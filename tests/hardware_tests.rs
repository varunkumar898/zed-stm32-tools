//! Hardware Integration Tests.
//!
//! Unit and mock tests run by default in CI without physical hardware.
//! Tests requiring physical hardware (ST-LINK probe, STM32 board connected via USB/SWD)
//! are marked with `#[ignore]` and can be executed with `cargo test --test hardware_tests -- --ignored`.

use stm32_zed::programmer::{
    build_detect_probes_command, build_flash_command, build_reset_command,
    parse_cubeprog_probe_list, ProgrammerBackend, ProgrammerConfig, ProgrammerInterface,
};

#[test]
fn test_simulated_probe_detection_mock() {
    let mock_cli_output = r#"
      -------------------------------------------------------------------
                        STM32CubeProgrammer v2.16.0
      -------------------------------------------------------------------

===== ST-LINK Probe List =====

ST-LINK Probe 0 :
   SN        : 066EFF535053717867144225
   Root Port : 1-4.2
   VID/PID   : 0483:374B
   Version   : V2J37M27
   Voltage   : 3.26V

ST-LINK Probe 1 :
   SN        : 002B00174741500320383734
   Root Port : 1-4.3
   VID/PID   : 0483:374E
   Version   : V3J7M2
   Voltage   : 3.30V
"#;

    let probes = parse_cubeprog_probe_list(mock_cli_output);
    assert_eq!(probes.len(), 2);
    assert_eq!(probes[0].serial_number, "066EFF535053717867144225");
    assert_eq!(probes[0].target_voltage, Some("3.26V".to_string()));
    assert_eq!(probes[1].serial_number, "002B00174741500320383734");
    assert_eq!(probes[1].target_voltage, Some("3.30V".to_string()));
}

#[test]
fn test_mock_command_generation() {
    let config = ProgrammerConfig {
        backend: ProgrammerBackend::CubeProgrammer,
        cli_path: Some("/custom/bin/STM32_Programmer_CLI".to_string()),
        interface: ProgrammerInterface::SWD,
        frequency_khz: Some(8000),
        flash_address: 0x08000000,
        openocd_interface: "interface/stlink.cfg".to_string(),
        openocd_target: Some("target/stm32f4x.cfg".to_string()),
        probe_rs_chip: Some("STM32F401RETx".to_string()),
    };

    let (exe, args) = build_flash_command(&config, "build/app.bin", true);
    assert_eq!(exe, "/custom/bin/STM32_Programmer_CLI");
    assert!(args.contains(&"port=SWD freq=8000".to_string()));
    assert!(args.contains(&"-rst".to_string()));

    let (reset_exe, reset_args) = build_reset_command(&config);
    assert_eq!(reset_exe, "/custom/bin/STM32_Programmer_CLI");
    assert_eq!(reset_args, vec!["-c", "port=SWD", "-rst"]);
}

/// Physical hardware test: Connects to a physical ST-LINK probe if plugged in.
/// Run with: `cargo test --test hardware_tests -- --ignored`
#[test]
#[ignore = "Requires physical ST-LINK and STM32 board connected via USB"]
fn test_physical_stlink_detection() {
    let (exe, args) = build_detect_probes_command(ProgrammerBackend::CubeProgrammer, None);
    let output = std::process::Command::new(&exe).args(&args).output();

    match output {
        Ok(out) => {
            let stdout_str = String::from_utf8_lossy(&out.stdout);
            println!("ST-LINK Probe Output:\n{}", stdout_str);
            assert!(out.status.success());
        }
        Err(e) => {
            panic!("Failed to execute {}: {}", exe, e);
        }
    }
}
