//! STM32 Debugger and DAP Configuration.
//!
//! Generates Zed DAP debug scenarios for probe-rs, OpenOCD, and ARM GDB.
//! Handles SVD file discovery for register inspection and FreeRTOS thread awareness.

use crate::devices::find_device;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DebugAdapterKind {
    ProbeRs,
    GdbOpenOcd,
    GdbMultiarch,
}

impl DebugAdapterKind {
    pub fn adapter_name(&self) -> &'static str {
        match self {
            Self::ProbeRs => "probe-rs",
            Self::GdbOpenOcd => "gdb",
            Self::GdbMultiarch => "gdb-multiarch",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZedDebugScenario {
    pub label: String,
    pub adapter: String,
    pub request: String,
    pub program: String,
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svd_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gdbpath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openocd_config: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtos: Option<String>,
}

/// Generates a set of Zed `.zed/debug.json` configurations tailored to the project.
pub fn generate_zed_debug_configs(
    project_name: &str,
    elf_path: &str,
    mcu: Option<&str>,
    svd_path: Option<&str>,
    has_freertos: bool,
) -> Vec<ZedDebugScenario> {
    let mut configs = Vec::new();
    let dev = mcu.and_then(find_device);

    let probe_chip = dev
        .map(|d| d.probe_rs_target.to_string())
        .or_else(|| mcu.map(|m| m.to_string()))
        .unwrap_or_else(|| "STM32F401RETx".to_string());

    let target_cfg = dev
        .map(|d| d.openocd_target.to_string())
        .unwrap_or_else(|| "target/stm32f4x.cfg".to_string());

    let rtos_opt = if has_freertos {
        Some("FreeRTOS".to_string())
    } else {
        None
    };

    // 1. probe-rs DAP configuration
    configs.push(ZedDebugScenario {
        label: format!("STM32: Debug with probe-rs ({project_name})"),
        adapter: "probe-rs".to_string(),
        request: "launch".to_string(),
        program: elf_path.to_string(),
        cwd: Some("$ZED_WORKTREE_ROOT".to_string()),
        chip: Some(probe_chip),
        svd_file: svd_path.map(|s| s.to_string()),
        server: None,
        gdbpath: None,
        openocd_config: None,
        rtos: rtos_opt.clone(),
    });

    // 2. OpenOCD + GDB configuration
    let openocd_args = vec![
        "-f".to_string(),
        "interface/stlink.cfg".to_string(),
        "-f".to_string(),
        target_cfg,
    ];

    configs.push(ZedDebugScenario {
        label: format!("STM32: Debug with GDB & OpenOCD ({project_name})"),
        adapter: "gdb".to_string(),
        request: "launch".to_string(),
        program: elf_path.to_string(),
        cwd: Some("$ZED_WORKTREE_ROOT".to_string()),
        chip: None,
        svd_file: svd_path.map(|s| s.to_string()),
        server: Some("localhost:3333".to_string()),
        gdbpath: Some("arm-none-eabi-gdb".to_string()),
        openocd_config: Some(openocd_args),
        rtos: rtos_opt,
    });

    configs
}

/// Locates CMSIS-SVD register definition file for the target device.
pub fn discover_svd_file(
    worktree_root: &Path,
    custom_svd_dir: Option<&str>,
    device_name: Option<&str>,
) -> Option<PathBuf> {
    let dev = device_name.and_then(find_device);
    let expected_filename = dev.map(|d| d.svd_filename);

    // 1. Custom configured SVD directory
    if let Some(custom_dir) = custom_svd_dir {
        let p = Path::new(custom_dir);
        if let Some(filename) = expected_filename {
            let candidate = p.join(filename);
            if candidate.exists() {
                return Some(candidate);
            }
        }
    }

    // 2. Worktree local `svd/` directory
    let local_svd = worktree_root.join("svd");
    if local_svd.exists() {
        if let Some(filename) = expected_filename {
            let candidate = local_svd.join(filename);
            if candidate.exists() {
                return Some(candidate);
            }
        }
        // Any .svd in worktree/svd
        if let Ok(entries) = std::fs::read_dir(&local_svd) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "svd") {
                    return Some(path);
                }
            }
        }
    }

    // 3. Worktree root `*.svd`
    if let Ok(entries) = std::fs::read_dir(worktree_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "svd") {
                return Some(path);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_zed_debug_configs() {
        let configs = generate_zed_debug_configs(
            "Blinky",
            "$ZED_WORKTREE_ROOT/build/Blinky.elf",
            Some("STM32F401RE"),
            Some("$ZED_WORKTREE_ROOT/svd/STM32F401.svd"),
            true,
        );

        assert_eq!(configs.len(), 2);
        assert_eq!(configs[0].adapter, "probe-rs");
        assert_eq!(configs[0].chip, Some("STM32F401RETx".to_string()));
        assert_eq!(configs[0].rtos, Some("FreeRTOS".to_string()));
        assert_eq!(configs[1].adapter, "gdb");
        assert_eq!(configs[1].server, Some("localhost:3333".to_string()));
    }
}
