//! STM32 Tasks and Configuration Generator for Zed.
//!
//! Generates Zed task definitions for build, clean, rebuild, flash, erase,
//! device detection, memory usage analysis, and CubeMX code generation.
//! Handles `.stm32/config.toml` project settings.

use crate::project::BuildSystem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stm32Settings {
    #[serde(default)]
    pub cubemx_path: Option<String>,
    #[serde(default)]
    pub cubeprogrammer_path: Option<String>,
    #[serde(default)]
    pub arm_toolchain_path: Option<String>,
    #[serde(default)]
    pub openocd_path: Option<String>,
    #[serde(default)]
    pub probe_rs_path: Option<String>,
    #[serde(default)]
    pub gdb_path: Option<String>,
    #[serde(default)]
    pub svd_path: Option<String>,
    #[serde(default = "default_interface")]
    pub programmer_interface: String,
    #[serde(default = "default_frequency")]
    pub programmer_frequency_khz: u32,
    #[serde(default)]
    pub build_system: Option<String>,
    #[serde(default = "default_debug_adapter")]
    pub debug_adapter: String,
}

fn default_interface() -> String {
    "SWD".to_string()
}

fn default_frequency() -> u32 {
    4000
}

fn default_debug_adapter() -> String {
    "probe-rs".to_string()
}

impl Default for Stm32Settings {
    fn default() -> Self {
        Self {
            cubemx_path: None,
            cubeprogrammer_path: None,
            arm_toolchain_path: None,
            openocd_path: None,
            probe_rs_path: None,
            gdb_path: None,
            svd_path: None,
            programmer_interface: default_interface(),
            programmer_frequency_khz: default_frequency(),
            build_system: None,
            debug_adapter: default_debug_adapter(),
        }
    }
}

impl Stm32Settings {
    pub fn parse_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    pub fn to_toml_string(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZedTask {
    pub label: String,
    pub command: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub args: Vec<String>,
    pub cwd: String,
    pub use_new_terminal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_concurrent_runs: Option<bool>,
}

/// Generates a comprehensive collection of Zed tasks conforming to the STM32 development workflow.
pub fn generate_all_tasks(
    build_system: BuildSystem,
    ioc_path: Option<&str>,
    firmware_elf: &str,
    probe_chip: &str,
) -> Vec<ZedTask> {
    let mut tasks = Vec::new();

    // 1. STM32: Build
    let (build_cmd, build_args) = match build_system {
        BuildSystem::CMake => (
            "cmake".to_string(),
            vec!["--build".to_string(), "build".to_string()],
        ),
        BuildSystem::Ninja => (
            "ninja".to_string(),
            vec!["-C".to_string(), "build".to_string()],
        ),
        _ => ("make".to_string(), vec!["-j".to_string()]),
    };
    tasks.push(ZedTask {
        label: "STM32: Build".to_string(),
        command: build_cmd,
        args: build_args,
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 2. STM32: Clean
    let (clean_cmd, clean_args) = match build_system {
        BuildSystem::CMake => (
            "cmake".to_string(),
            vec![
                "--build".to_string(),
                "build".to_string(),
                "--target".to_string(),
                "clean".to_string(),
            ],
        ),
        BuildSystem::Ninja => (
            "ninja".to_string(),
            vec![
                "-C".to_string(),
                "build".to_string(),
                "-t".to_string(),
                "clean".to_string(),
            ],
        ),
        _ => ("make".to_string(), vec!["clean".to_string()]),
    };
    tasks.push(ZedTask {
        label: "STM32: Clean".to_string(),
        command: clean_cmd,
        args: clean_args,
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 3. STM32: Rebuild
    let rebuild_cmd = match build_system {
        BuildSystem::CMake => {
            "cmake --build build --target clean && cmake --build build".to_string()
        }
        BuildSystem::Ninja => "ninja -C build -t clean && ninja -C build".to_string(),
        _ => "make clean && make -j".to_string(),
    };
    tasks.push(ZedTask {
        label: "STM32: Rebuild".to_string(),
        command: "sh".to_string(),
        args: vec!["-c".to_string(), rebuild_cmd],
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 4. STM32: Build Release
    let release_cmd = match build_system {
        BuildSystem::CMake => {
            "cmake -S . -B build -DCMAKE_BUILD_TYPE=Release && cmake --build build".to_string()
        }
        BuildSystem::Ninja => {
            "cmake -S . -B build -GNinja -DCMAKE_BUILD_TYPE=Release && ninja -C build".to_string()
        }
        _ => "make CFLAGS=\"-O3 -DNDEBUG\"".to_string(),
    };
    tasks.push(ZedTask {
        label: "STM32: Build Release".to_string(),
        command: "sh".to_string(),
        args: vec!["-c".to_string(), release_cmd],
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 5. STM32: Build Debug
    let debug_cmd = match build_system {
        BuildSystem::CMake => {
            "cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug && cmake --build build".to_string()
        }
        BuildSystem::Ninja => {
            "cmake -S . -B build -GNinja -DCMAKE_BUILD_TYPE=Debug && ninja -C build".to_string()
        }
        _ => "make CFLAGS=\"-Og -g3 -DDEBUG\"".to_string(),
    };
    tasks.push(ZedTask {
        label: "STM32: Build Debug".to_string(),
        command: "sh".to_string(),
        args: vec!["-c".to_string(), debug_cmd],
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 6. STM32: Flash
    tasks.push(ZedTask {
        label: "STM32: Flash".to_string(),
        command: "./scripts/flash.sh".to_string(),
        args: vec!["flash".to_string(), firmware_elf.to_string()],
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 7. STM32: Flash and Run
    tasks.push(ZedTask {
        label: "STM32: Flash and Run".to_string(),
        command: "./scripts/flash.sh".to_string(),
        args: vec!["flash-run".to_string(), firmware_elf.to_string()],
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 8. STM32: Erase
    tasks.push(ZedTask {
        label: "STM32: Erase (Full Chip)".to_string(),
        command: "./scripts/flash.sh".to_string(),
        args: vec!["erase".to_string()],
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 9. STM32: Reset Target
    tasks.push(ZedTask {
        label: "STM32: Reset".to_string(),
        command: "./scripts/flash.sh".to_string(),
        args: vec!["reset".to_string()],
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 10. STM32: Detect ST-LINK / Target
    tasks.push(ZedTask {
        label: "STM32: Detect ST-LINK".to_string(),
        command: "./scripts/flash.sh".to_string(),
        args: vec!["detect".to_string()],
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 11. STM32: Toolchain Doctor
    tasks.push(ZedTask {
        label: "STM32: Toolchain Doctor".to_string(),
        command: "./scripts/doctor.sh".to_string(),
        args: Vec::new(),
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 12. STM32: Memory Usage
    tasks.push(ZedTask {
        label: "STM32: Memory Usage".to_string(),
        command: "./scripts/build.sh".to_string(),
        args: vec!["size".to_string(), firmware_elf.to_string()],
        cwd: "$ZED_WORKTREE_ROOT".to_string(),
        use_new_terminal: false,
        allow_concurrent_runs: Some(false),
    });

    // 13. STM32: Open CubeMX (if .ioc present)
    if let Some(ioc) = ioc_path {
        tasks.push(ZedTask {
            label: "STM32: Open CubeMX".to_string(),
            command: "./scripts/cubemx.sh".to_string(),
            args: vec!["open".to_string(), ioc.to_string()],
            cwd: "$ZED_WORKTREE_ROOT".to_string(),
            use_new_terminal: false,
            allow_concurrent_runs: Some(false),
        });

        tasks.push(ZedTask {
            label: "STM32: Generate Code (CubeMX)".to_string(),
            command: "./scripts/cubemx.sh".to_string(),
            args: vec!["generate".to_string(), ioc.to_string()],
            cwd: "$ZED_WORKTREE_ROOT".to_string(),
            use_new_terminal: false,
            allow_concurrent_runs: Some(false),
        });
    }

    let _ = probe_chip; // referenced
    tasks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stm32_settings_serde() {
        let toml_str = r#"
        programmer_interface = "SWD"
        programmer_frequency_khz = 8000
        debug_adapter = "probe-rs"
        "#;

        let settings = Stm32Settings::parse_toml(toml_str).expect("parse toml");
        assert_eq!(settings.programmer_interface, "SWD");
        assert_eq!(settings.programmer_frequency_khz, 8000);
        assert_eq!(settings.debug_adapter, "probe-rs");
    }

    #[test]
    fn test_generate_all_tasks() {
        let tasks = generate_all_tasks(
            BuildSystem::Make,
            Some("firmware.ioc"),
            "build/firmware.elf",
            "STM32F401RETx",
        );

        assert!(tasks.iter().any(|t| t.label == "STM32: Build"));
        assert!(tasks.iter().any(|t| t.label == "STM32: Flash and Run"));
        assert!(tasks.iter().any(|t| t.label == "STM32: Open CubeMX"));
        assert!(tasks.iter().any(|t| t.label == "STM32: Toolchain Doctor"));
        assert!(tasks.iter().any(|t| t.label == "STM32: Memory Usage"));
    }
}
