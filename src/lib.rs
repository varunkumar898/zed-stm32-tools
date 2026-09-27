//! STM32 Development Extension for Zed.
//!
//! Provides language server orchestration (clangd with ARM GCC query-driver),
//! slash commands (/stm32-doctor, /stm32-info, /stm32-device),
//! DAP integration (probe-rs / GDB), and embedded workflow management.

pub mod commands;
pub mod cubemx;
pub mod debugger;
pub mod devices;
pub mod diagnostics;
pub mod programmer;
pub mod project;
pub mod toolchain;
pub mod utils;

use crate::devices::{find_device, STM32_DEVICES};
use crate::project::{BuildSystem, IocProject, LinkerScript, Stm32Project};
use crate::toolchain::{DoctorReport, ToolStatus, REQUIRED_TOOLS};
use zed_extension_api::*;

struct Stm32Extension;

impl Stm32Extension {
    fn run_doctor(&self, worktree: Option<&Worktree>) -> DoctorReport {
        let mut report = DoctorReport::new();

        for tool in REQUIRED_TOOLS {
            let mut installed = false;
            let mut path = None;

            if let Some(wt) = worktree {
                if let Some(p) = wt.which(tool.binary) {
                    installed = true;
                    path = Some(p);
                } else if tool.binary == "arm-none-eabi-gdb" {
                    // Check fallback gdb-multiarch
                    if let Some(p) = wt.which("gdb-multiarch") {
                        installed = true;
                        path = Some(p);
                    }
                }
            }

            report.add(ToolStatus {
                name: tool.name.to_string(),
                binary_name: tool.binary.to_string(),
                category: tool.category,
                required: tool.required,
                installed,
                path,
                version: None,
                install_hint: tool.install_hint.to_string(),
            });
        }

        report
    }

    fn inspect_worktree(&self, worktree: &Worktree) -> Stm32Project {
        let root = worktree.root_path();
        let mut project = Stm32Project::new(root);

        // 1. Detect build system
        if worktree.read_text_file("CMakeLists.txt").is_ok() {
            project.build_system = BuildSystem::CMake;
        } else if worktree.read_text_file("build.ninja").is_ok() {
            project.build_system = BuildSystem::Ninja;
        } else if worktree.read_text_file("Makefile").is_ok() {
            project.build_system = BuildSystem::Make;
        }

        // 2. Check for .ioc files (common CubeMX filenames or search pattern)
        let common_ioc_candidates = &["firmware.ioc", "project.ioc", "app.ioc"];
        for candidate in common_ioc_candidates {
            if let Ok(content) = worktree.read_text_file(candidate) {
                let parsed = IocProject::parse(candidate, &content);
                project.mcu = parsed.mcu_name.clone();
                project.family = parsed.family;
                project.has_freertos = parsed.has_freertos;
                project.has_hal = parsed.has_hal;
                project.has_ll = parsed.has_ll;
                project.ioc = Some(parsed);
                break;
            }
        }

        // 3. Check for linker scripts
        let common_ld_candidates = &[
            "linker.ld",
            "STM32F401RETx_FLASH.ld",
            "STM32F411CEUx_FLASH.ld",
            "STM32G474RETx_FLASH.ld",
            "STM32H743ZITx_FLASH.ld",
        ];
        for candidate in common_ld_candidates {
            if let Ok(content) = worktree.read_text_file(candidate) {
                let parsed = LinkerScript::parse(candidate, &content);
                project.linker_script = Some(parsed);
                break;
            }
        }

        // 4. Check for FreeRTOS
        if worktree.read_text_file("Core/Inc/FreeRTOSConfig.h").is_ok()
            || worktree.read_text_file("FreeRTOSConfig.h").is_ok()
            || worktree.read_text_file("include/FreeRTOSConfig.h").is_ok()
        {
            project.has_freertos = true;
        }

        project
    }
}

impl Extension for Stm32Extension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let id_str = language_server_id.as_ref();
        if id_str != "clangd" && id_str != "stm32-clangd" {
            return Err(format!("Unsupported language server ID: {id_str}"));
        }

        let clangd_path = worktree.which("clangd").ok_or_else(|| {
            "clangd not found in PATH. Install clangd to enable C/C++ intelligence for STM32."
                .to_string()
        })?;

        // clangd arguments customized for STM32 cross-compilation:
        // --query-driver ensures clangd queries arm-none-eabi-gcc for correct system headers and defines
        let args = vec![
            "--background-index".to_string(),
            "--clang-tidy".to_string(),
            "--completion-style=detailed".to_string(),
            "--header-insertion=iwyu".to_string(),
            "--fallback-style=llvm".to_string(),
            "--query-driver=/**/*arm-none-eabi*".to_string(),
        ];

        Ok(Command {
            command: clangd_path,
            args,
            env: worktree.shell_env(),
        })
    }

    fn complete_slash_command_argument(
        &self,
        command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        if command.name == "stm32-device" {
            let completions = STM32_DEVICES
                .iter()
                .map(|dev| SlashCommandArgumentCompletion {
                    label: dev.part_number.to_string(),
                    new_text: dev.part_number.to_string(),
                    run_command: true,
                })
                .collect();
            return Ok(completions);
        }
        Ok(Vec::new())
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        match command.name.as_str() {
            "stm32-doctor" => {
                let report = self.run_doctor(worktree);
                let text = report.format_markdown();
                Ok(SlashCommandOutput {
                    text: text.clone(),
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "STM32 Toolchain Diagnostics".to_string(),
                    }],
                })
            }
            "stm32-info" => {
                let text = if let Some(wt) = worktree {
                    let project = self.inspect_worktree(wt);
                    project.summary_markdown()
                } else {
                    "# STM32 Project Information\n\nNo active worktree open.".to_string()
                };

                Ok(SlashCommandOutput {
                    text: text.clone(),
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: "STM32 Project Info".to_string(),
                    }],
                })
            }
            "stm32-device" => {
                let query = args.first().map(|s| s.as_str()).unwrap_or("STM32F401RE");
                let text = if let Some(dev) = find_device(query) {
                    format!(
                        "# STM32 Device: {}\n\n\
                        * **Family:** {}\n\
                        * **Series:** {}\n\
                        * **Core:** {}\n\
                        * **Max Clock Frequency:** {} MHz\n\
                        * **Flash Size:** {}\n\
                        * **RAM Size:** {}\n\
                        * **Package:** {}\n\
                        * **CMSIS Device Name:** `{}`\n\
                        * **OpenOCD Target Config:** `{}`\n\
                        * **probe-rs Chip Name:** `{}`\n\
                        * **CMSIS-SVD File:** `{}`\n\
                        * **Integrated Peripherals:** {}\n",
                        dev.part_number,
                        dev.family.name(),
                        dev.series,
                        dev.core.name(),
                        dev.max_frequency_mhz,
                        utils::format_bytes(dev.flash_bytes),
                        utils::format_bytes(dev.ram_bytes),
                        dev.package,
                        dev.cmsis_device_name,
                        dev.openocd_target,
                        dev.probe_rs_target,
                        dev.svd_filename,
                        dev.peripherals.join(", ")
                    )
                } else {
                    format!("# STM32 Device Database\n\nNo device found matching `{}`. Try `STM32F401RE`, `STM32G474RE`, or `STM32H743ZI`.", query)
                };

                Ok(SlashCommandOutput {
                    text: text.clone(),
                    sections: vec![SlashCommandOutputSection {
                        range: (0..text.len()).into(),
                        label: format!("Device Info: {query}"),
                    }],
                })
            }
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }

    fn get_dap_binary(
        &mut self,
        adapter_name: String,
        config: DebugTaskDefinition,
        user_installed_path: Option<String>,
        worktree: &Worktree,
    ) -> Result<DebugAdapterBinary, String> {
        match adapter_name.as_str() {
            "probe-rs" => {
                let bin = user_installed_path
                    .or_else(|| worktree.which("probe-rs"))
                    .ok_or_else(|| "probe-rs executable not found. Install it with `cargo install probe-rs-tools`.".to_string())?;

                Ok(DebugAdapterBinary {
                    command: Some(bin),
                    arguments: vec!["dap".to_string()],
                    envs: worktree.shell_env(),
                    cwd: Some(worktree.root_path()),
                    connection: None,
                    request_args: StartDebuggingRequestArguments {
                        configuration: config.config,
                        request: StartDebuggingRequestArgumentsRequest::Launch,
                    },
                })
            }
            "gdb" | "arm-none-eabi-gdb" => {
                let bin = user_installed_path
                    .or_else(|| worktree.which("arm-none-eabi-gdb"))
                    .or_else(|| worktree.which("gdb-multiarch"))
                    .ok_or_else(|| {
                        "ARM GDB not found. Install arm-none-eabi-gdb or gdb-multiarch.".to_string()
                    })?;

                Ok(DebugAdapterBinary {
                    command: Some(bin),
                    arguments: vec!["--interpreter=dap".to_string()],
                    envs: worktree.shell_env(),
                    cwd: Some(worktree.root_path()),
                    connection: None,
                    request_args: StartDebuggingRequestArguments {
                        configuration: config.config,
                        request: StartDebuggingRequestArgumentsRequest::Launch,
                    },
                })
            }
            _ => Err(format!("Unsupported debug adapter: {adapter_name}")),
        }
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        config: serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest, String> {
        let req_str = config
            .get("request")
            .and_then(|v| v.as_str())
            .unwrap_or("launch");

        if req_str.eq_ignore_ascii_case("attach") {
            Ok(StartDebuggingRequestArgumentsRequest::Attach)
        } else {
            Ok(StartDebuggingRequestArgumentsRequest::Launch)
        }
    }
}

register_extension!(Stm32Extension);
