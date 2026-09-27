//! STM32 Toolchain Discovery and Diagnostics.
//!
//! Handles cross-platform discovery of ARM GCC, GDB, STM32CubeMX, STM32CubeProgrammer,
//! OpenOCD, probe-rs, clangd, Make, CMake, and Ninja.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolCategory {
    Compiler,
    Debugger,
    Programmer,
    LanguageServer,
    BuildSystem,
    Generator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStatus {
    pub name: String,
    pub binary_name: String,
    pub category: ToolCategory,
    pub required: bool,
    pub installed: bool,
    pub path: Option<String>,
    pub version: Option<String>,
    pub install_hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorReport {
    pub tools: Vec<ToolStatus>,
}

impl DoctorReport {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn add(&mut self, tool: ToolStatus) {
        self.tools.push(tool);
    }

    pub fn all_required_present(&self) -> bool {
        self.tools
            .iter()
            .filter(|t| t.required)
            .all(|t| t.installed)
    }

    pub fn missing_tools(&self) -> Vec<&ToolStatus> {
        self.tools.iter().filter(|t| !t.installed).collect()
    }

    pub fn format_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# STM32 Toolchain Diagnostics (Doctor)\n\n");

        let mut current_cat: Option<ToolCategory> = None;
        for tool in &self.tools {
            if current_cat != Some(tool.category) {
                current_cat = Some(tool.category);
                let cat_name = match tool.category {
                    ToolCategory::Compiler => "### Compilers & Assemblers",
                    ToolCategory::Debugger => "### Debuggers & Adapters",
                    ToolCategory::Programmer => "### Flashing & Programmers",
                    ToolCategory::LanguageServer => "### Language Intelligence",
                    ToolCategory::BuildSystem => "### Build Systems",
                    ToolCategory::Generator => "### Code Generators",
                };
                out.push_str(&format!("\n{}\n\n", cat_name));
            }

            let status_icon = if tool.installed { "✓" } else { "✗" };
            let req_str = if tool.required {
                " (Required)"
            } else {
                " (Optional)"
            };

            out.push_str(&format!(
                "* **[{}] {}**{}\n",
                status_icon, tool.name, req_str
            ));
            if let Some(p) = &tool.path {
                out.push_str(&format!("  - **Path:** `{}`\n", p));
            }
            if let Some(v) = &tool.version {
                out.push_str(&format!("  - **Version:** {}\n", v));
            }
            if !tool.installed {
                out.push_str(&format!("  - **Action Needed:** {}\n", tool.install_hint));
            }
        }

        out.push_str("\n---\n\n");
        if self.all_required_present() {
            out.push_str("✓ **Core STM32 development toolchain is ready.**\n");
        } else {
            out.push_str(
                "⚠️ **Some required tools are missing. Review the instructions above.**\n",
            );
        }

        out
    }
}

impl Default for DoctorReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard paths to probe on various operating systems for STM32CubeMX.
pub fn cubemx_search_paths() -> Vec<&'static str> {
    vec![
        // Linux
        "/opt/ST/STM32CubeMX/STM32CubeMX",
        "/opt/st/stm32cubemx/STM32CubeMX",
        "/usr/local/STMicroelectronics/STM32CubeMX/STM32CubeMX",
        "/home/cherry/STM32CubeMX/STM32CubeMX",
        // macOS
        "/Applications/STMicroelectronics/STM32CubeMX.app/Contents/MacOs/STM32CubeMX",
        "/Applications/STM32CubeMX.app/Contents/MacOs/STM32CubeMX",
        // Windows
        "C:\\Program Files\\STMicroelectronics\\STM32Cube\\STM32CubeMX\\STM32CubeMX.exe",
        "C:\\Program Files (x86)\\STMicroelectronics\\STM32Cube\\STM32CubeMX\\STM32CubeMX.exe",
    ]
}

/// Standard paths to probe on various operating systems for STM32CubeProgrammer CLI.
pub fn cubeprogrammer_search_paths() -> Vec<&'static str> {
    vec![
        // Linux
        "/usr/local/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_Programmer_CLI",
        "/opt/STMicroelectronics/STM32Cube/STM32CubeProgrammer/bin/STM32_Programmer_CLI",
        "/opt/st/stm32cubeprogrammer/bin/STM32_Programmer_CLI",
        "/usr/local/bin/STM32_Programmer_CLI",
        // macOS
        "/Applications/STMicroelectronics/STM32Cube/STM32CubeProgrammer/STM32CubeProgrammer.app/Contents/MacOs/bin/STM32_Programmer_CLI",
        // Windows
        "C:\\Program Files\\STMicroelectronics\\STM32Cube\\STM32CubeProgrammer\\bin\\STM32_Programmer_CLI.exe",
        "C:\\Program Files (x86)\\STMicroelectronics\\STM32Cube\\STM32CubeProgrammer\\bin\\STM32_Programmer_CLI.exe",
    ]
}

/// Probe-rs binary search names
pub fn probers_binary_names() -> &'static [&'static str] {
    &["probe-rs", "probe-rs-cli"]
}

/// GDB binary search candidates
pub fn gdb_binary_names() -> &'static [&'static str] {
    &["arm-none-eabi-gdb", "gdb-multiarch", "arm-none-eabi-gdb-py"]
}

/// Check tool definitions for the doctor
pub struct ToolQuery {
    pub name: &'static str,
    pub binary: &'static str,
    pub category: ToolCategory,
    pub required: bool,
    pub install_hint: &'static str,
}

pub static REQUIRED_TOOLS: &[ToolQuery] = &[
    ToolQuery {
        name: "ARM GNU GCC Compiler",
        binary: "arm-none-eabi-gcc",
        category: ToolCategory::Compiler,
        required: true,
        install_hint: "Install `arm-none-eabi-gcc` via your package manager (e.g. `apt install gcc-arm-none-eabi` or `pacman -S arm-none-eabi-gcc`).",
    },
    ToolQuery {
        name: "ARM GNU G++ Compiler",
        binary: "arm-none-eabi-g++",
        category: ToolCategory::Compiler,
        required: false,
        install_hint: "Install `arm-none-eabi-g++` for C++ embedded development.",
    },
    ToolQuery {
        name: "ARM GNU Size",
        binary: "arm-none-eabi-size",
        category: ToolCategory::Compiler,
        required: true,
        install_hint: "Part of the ARM GNU binutils package.",
    },
    ToolQuery {
        name: "ARM GNU Objcopy",
        binary: "arm-none-eabi-objcopy",
        category: ToolCategory::Compiler,
        required: true,
        install_hint: "Part of the ARM GNU binutils package.",
    },
    ToolQuery {
        name: "ARM GDB Debugger",
        binary: "arm-none-eabi-gdb",
        category: ToolCategory::Debugger,
        required: true,
        install_hint: "Install `arm-none-eabi-gdb` or `gdb-multiarch` for embedded hardware breakpoints and debugging.",
    },
    ToolQuery {
        name: "OpenOCD (Open On-Chip Debugger)",
        binary: "openocd",
        category: ToolCategory::Debugger,
        required: false,
        install_hint: "Install `openocd` via `apt install openocd` or `pacman -S openocd` for ST-LINK / JTAG debugging.",
    },
    ToolQuery {
        name: "probe-rs",
        binary: "probe-rs",
        category: ToolCategory::Debugger,
        required: false,
        install_hint: "Install probe-rs via `cargo install probe-rs-tools` for fast Rust/C hardware debugging and flashing.",
    },
    ToolQuery {
        name: "STM32CubeProgrammer CLI",
        binary: "STM32_Programmer_CLI",
        category: ToolCategory::Programmer,
        required: false,
        install_hint: "Download STM32CubeProgrammer from STMicroelectronics (st.com/stm32cubeprog) and add to PATH or configure `stm32.cubeprogrammer.path`.",
    },
    ToolQuery {
        name: "STM32CubeMX",
        binary: "STM32CubeMX",
        category: ToolCategory::Generator,
        required: false,
        install_hint: "Download STM32CubeMX from STMicroelectronics (st.com/stm32cubemx) or configure `stm32.cubemx.path`.",
    },
    ToolQuery {
        name: "clangd Language Server",
        binary: "clangd",
        category: ToolCategory::LanguageServer,
        required: true,
        install_hint: "Install `clangd` via your package manager (e.g. `apt install clangd` or `pacman -S clang`).",
    },
    ToolQuery {
        name: "GNU Make",
        binary: "make",
        category: ToolCategory::BuildSystem,
        required: false,
        install_hint: "Install `make` for Make-based CubeMX and STM32 projects.",
    },
    ToolQuery {
        name: "CMake",
        binary: "cmake",
        category: ToolCategory::BuildSystem,
        required: false,
        install_hint: "Install `cmake` (version 3.20+) for CMake-based STM32 builds.",
    },
    ToolQuery {
        name: "Ninja Build",
        binary: "ninja",
        category: ToolCategory::BuildSystem,
        required: false,
        install_hint: "Install `ninja` for high-speed parallel builds with CMake.",
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doctor_report_formatting() {
        let mut report = DoctorReport::new();
        report.add(ToolStatus {
            name: "ARM GNU GCC Compiler".to_string(),
            binary_name: "arm-none-eabi-gcc".to_string(),
            category: ToolCategory::Compiler,
            required: true,
            installed: true,
            path: Some("/usr/bin/arm-none-eabi-gcc".to_string()),
            version: Some("14.2.0".to_string()),
            install_hint: "N/A".to_string(),
        });
        report.add(ToolStatus {
            name: "STM32CubeProgrammer CLI".to_string(),
            binary_name: "STM32_Programmer_CLI".to_string(),
            category: ToolCategory::Programmer,
            required: false,
            installed: false,
            path: None,
            version: None,
            install_hint: "Install from st.com".to_string(),
        });

        let md = report.format_markdown();
        assert!(md.contains("[✓] ARM GNU GCC Compiler"));
        assert!(md.contains("[✗] STM32CubeProgrammer CLI"));
        assert!(md.contains("Install from st.com"));
        assert!(report.all_required_present());
    }
}
