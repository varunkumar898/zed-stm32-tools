//! STM32 Programmer and Flashing Backends.
//!
//! Provides comprehensive support for STM32CubeProgrammer CLI, OpenOCD, and probe-rs.
//! Supports probe detection, flash programming, verification, reset, and sector erase.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgrammerBackend {
    CubeProgrammer,
    OpenOCD,
    ProbeRs,
}

impl ProgrammerBackend {
    pub fn name(&self) -> &'static str {
        match self {
            Self::CubeProgrammer => "STM32CubeProgrammer",
            Self::OpenOCD => "OpenOCD",
            Self::ProbeRs => "probe-rs",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProgrammerInterface {
    SWD,
    JTAG,
    UsbDfu,
    UART,
}

impl ProgrammerInterface {
    pub fn as_cubeprog_port(&self) -> &'static str {
        match self {
            Self::SWD => "port=SWD",
            Self::JTAG => "port=JTAG",
            Self::UsbDfu => "port=usb1",
            Self::UART => "port=/dev/ttyUSB0",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgrammerConfig {
    pub backend: ProgrammerBackend,
    pub cli_path: Option<String>,
    pub interface: ProgrammerInterface,
    pub frequency_khz: Option<u32>,
    pub flash_address: u64,
    pub openocd_interface: String,
    pub openocd_target: Option<String>,
    pub probe_rs_chip: Option<String>,
}

impl Default for ProgrammerConfig {
    fn default() -> Self {
        Self {
            backend: ProgrammerBackend::CubeProgrammer,
            cli_path: None,
            interface: ProgrammerInterface::SWD,
            frequency_khz: Some(4000),
            flash_address: 0x08000000,
            openocd_interface: "interface/stlink.cfg".to_string(),
            openocd_target: Some("target/stm32f4x.cfg".to_string()),
            probe_rs_chip: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedProbe {
    pub serial_number: String,
    pub description: String,
    pub target_voltage: Option<String>,
}

/// Parses the output of `STM32_Programmer_CLI -l`
pub fn parse_cubeprog_probe_list(output: &str) -> Vec<DetectedProbe> {
    let mut probes = Vec::new();
    let mut current_sn = None;
    let mut current_desc = None;
    let mut current_volt = None;

    for line in output.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("ST-LINK Probe") {
            if let Some(sn) = current_sn.take() {
                probes.push(DetectedProbe {
                    serial_number: sn,
                    description: current_desc.unwrap_or_else(|| "ST-LINK".to_string()),
                    target_voltage: current_volt.take(),
                });
            }
            current_desc = Some(trimmed.to_string());
        } else if trimmed.starts_with("SN") || trimmed.starts_with("Serial Number") {
            if let Some((_, sn)) = trimmed.split_once(':') {
                current_sn = Some(sn.trim().to_string());
            }
        } else if trimmed.starts_with("Voltage") || trimmed.starts_with("Target Voltage") {
            if let Some((_, v)) = trimmed.split_once(':') {
                current_volt = Some(v.trim().to_string());
            }
        }
    }

    if let Some(sn) = current_sn {
        probes.push(DetectedProbe {
            serial_number: sn,
            description: current_desc.unwrap_or_else(|| "ST-LINK".to_string()),
            target_voltage: current_volt,
        });
    }

    probes
}

/// Builds the probe detection command for the configured backend.
pub fn build_detect_probes_command(
    backend: ProgrammerBackend,
    cli_override: Option<&str>,
) -> (String, Vec<String>) {
    match backend {
        ProgrammerBackend::CubeProgrammer => {
            let exe = cli_override.unwrap_or("STM32_Programmer_CLI");
            (exe.to_string(), vec!["-l".to_string()])
        }
        ProgrammerBackend::OpenOCD => {
            let exe = cli_override.unwrap_or("openocd");
            (
                exe.to_string(),
                vec![
                    "-f".to_string(),
                    "interface/stlink.cfg".to_string(),
                    "-c".to_string(),
                    "init; shutdown".to_string(),
                ],
            )
        }
        ProgrammerBackend::ProbeRs => {
            let exe = cli_override.unwrap_or("probe-rs");
            (exe.to_string(), vec!["list".to_string()])
        }
    }
}

/// Builds the flashing command.
///
/// If `run_after` is true, automatically resets and executes the program.
pub fn build_flash_command(
    config: &ProgrammerConfig,
    firmware_path: &str,
    run_after: bool,
) -> (String, Vec<String>) {
    match config.backend {
        ProgrammerBackend::CubeProgrammer => {
            let exe = config.cli_path.as_deref().unwrap_or("STM32_Programmer_CLI");
            let mut port_arg = config.interface.as_cubeprog_port().to_string();
            if let Some(freq) = config.frequency_khz {
                port_arg.push_str(&format!(" freq={freq}"));
            }

            let mut args = vec![
                "-c".to_string(),
                port_arg,
                "-w".to_string(),
                firmware_path.to_string(),
            ];

            // Address is needed for raw binary files (.bin), optional for .elf
            if firmware_path.ends_with(".bin") {
                args.push(format!("0x{:08X}", config.flash_address));
            }

            args.push("-v".to_string()); // Verify flash contents

            if run_after {
                args.push("-rst".to_string()); // Reset and run
            }

            (exe.to_string(), args)
        }
        ProgrammerBackend::OpenOCD => {
            let exe = config.cli_path.as_deref().unwrap_or("openocd");
            let target_cfg = config
                .openocd_target
                .as_deref()
                .unwrap_or("target/stm32f4x.cfg");
            let reset_flag = if run_after { "reset" } else { "" };
            let cmd = format!(
                "program {} verify {} exit 0x{:08X}",
                firmware_path, reset_flag, config.flash_address
            );

            let args = vec![
                "-f".to_string(),
                config.openocd_interface.clone(),
                "-f".to_string(),
                target_cfg.to_string(),
                "-c".to_string(),
                cmd,
            ];

            (exe.to_string(), args)
        }
        ProgrammerBackend::ProbeRs => {
            let exe = config.cli_path.as_deref().unwrap_or("probe-rs");
            let chip = config.probe_rs_chip.as_deref().unwrap_or("STM32F401RETx");

            let subcmd = if run_after { "run" } else { "download" };
            let args = vec![
                subcmd.to_string(),
                "--chip".to_string(),
                chip.to_string(),
                firmware_path.to_string(),
            ];

            (exe.to_string(), args)
        }
    }
}

/// Builds an explicit erase command.
///
/// SAFETY: This is ONLY invoked upon explicit user command.
pub fn build_erase_command(config: &ProgrammerConfig) -> (String, Vec<String>) {
    match config.backend {
        ProgrammerBackend::CubeProgrammer => {
            let exe = config.cli_path.as_deref().unwrap_or("STM32_Programmer_CLI");
            let args = vec![
                "-c".to_string(),
                config.interface.as_cubeprog_port().to_string(),
                "-e".to_string(),
                "all".to_string(),
            ];
            (exe.to_string(), args)
        }
        ProgrammerBackend::OpenOCD => {
            let exe = config.cli_path.as_deref().unwrap_or("openocd");
            let target_cfg = config
                .openocd_target
                .as_deref()
                .unwrap_or("target/stm32f4x.cfg");
            let args = vec![
                "-f".to_string(),
                config.openocd_interface.clone(),
                "-f".to_string(),
                target_cfg.to_string(),
                "-c".to_string(),
                "init; reset halt; stm32x mass_erase 0; shutdown".to_string(),
            ];
            (exe.to_string(), args)
        }
        ProgrammerBackend::ProbeRs => {
            let exe = config.cli_path.as_deref().unwrap_or("probe-rs");
            let chip = config.probe_rs_chip.as_deref().unwrap_or("STM32F401RETx");
            let args = vec!["erase".to_string(), "--chip".to_string(), chip.to_string()];
            (exe.to_string(), args)
        }
    }
}

/// Builds a hardware reset command without reprogramming.
pub fn build_reset_command(config: &ProgrammerConfig) -> (String, Vec<String>) {
    match config.backend {
        ProgrammerBackend::CubeProgrammer => {
            let exe = config.cli_path.as_deref().unwrap_or("STM32_Programmer_CLI");
            let args = vec![
                "-c".to_string(),
                config.interface.as_cubeprog_port().to_string(),
                "-rst".to_string(),
            ];
            (exe.to_string(), args)
        }
        ProgrammerBackend::OpenOCD => {
            let exe = config.cli_path.as_deref().unwrap_or("openocd");
            let target_cfg = config
                .openocd_target
                .as_deref()
                .unwrap_or("target/stm32f4x.cfg");
            let args = vec![
                "-f".to_string(),
                config.openocd_interface.clone(),
                "-f".to_string(),
                target_cfg.to_string(),
                "-c".to_string(),
                "init; reset run; shutdown".to_string(),
            ];
            (exe.to_string(), args)
        }
        ProgrammerBackend::ProbeRs => {
            let exe = config.cli_path.as_deref().unwrap_or("probe-rs");
            let chip = config.probe_rs_chip.as_deref().unwrap_or("STM32F401RETx");
            let args = vec!["reset".to_string(), "--chip".to_string(), chip.to_string()];
            (exe.to_string(), args)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubeprog_probe_list_parsing() {
        let sample_output = r#"
        ===== ST-LINK Probe List =====
        ST-LINK Probe 0:
        SN: 066EFF535053717867144225
        Target Voltage: 3.26V
        "#;

        let probes = parse_cubeprog_probe_list(sample_output);
        assert_eq!(probes.len(), 1);
        assert_eq!(probes[0].serial_number, "066EFF535053717867144225");
        assert_eq!(probes[0].target_voltage, Some("3.26V".to_string()));
    }

    #[test]
    fn test_build_flash_command_cubeprog() {
        let config = ProgrammerConfig {
            backend: ProgrammerBackend::CubeProgrammer,
            cli_path: None,
            interface: ProgrammerInterface::SWD,
            frequency_khz: Some(4000),
            flash_address: 0x08000000,
            openocd_interface: "interface/stlink.cfg".to_string(),
            openocd_target: None,
            probe_rs_chip: None,
        };

        let (exe, args) = build_flash_command(&config, "build/firmware.bin", true);
        assert_eq!(exe, "STM32_Programmer_CLI");
        assert!(args.contains(&"-w".to_string()));
        assert!(args.contains(&"build/firmware.bin".to_string()));
        assert!(args.contains(&"0x08000000".to_string()));
        assert!(args.contains(&"-v".to_string()));
        assert!(args.contains(&"-rst".to_string()));
    }
}
