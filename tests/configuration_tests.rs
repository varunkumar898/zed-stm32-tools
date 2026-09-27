use stm32_zed::commands::{generate_all_tasks, Stm32Settings};
use stm32_zed::project::BuildSystem;

#[test]
fn test_config_roundtrip() {
    let original = Stm32Settings {
        cubemx_path: Some("/opt/ST/STM32CubeMX/STM32CubeMX".to_string()),
        cubeprogrammer_path: Some("/usr/local/bin/STM32_Programmer_CLI".to_string()),
        arm_toolchain_path: Some("/usr/bin".to_string()),
        openocd_path: Some("/usr/bin/openocd".to_string()),
        probe_rs_path: Some("/home/cherry/.cargo/bin/probe-rs".to_string()),
        gdb_path: Some("/usr/bin/gdb-multiarch".to_string()),
        svd_path: Some("./svd/STM32F401.svd".to_string()),
        programmer_interface: "SWD".to_string(),
        programmer_frequency_khz: 8000,
        build_system: Some("cmake".to_string()),
        debug_adapter: "probe-rs".to_string(),
    };

    let toml_str = original.to_toml_string().expect("serialize to toml");
    let deserialized = Stm32Settings::parse_toml(&toml_str).expect("deserialize from toml");

    assert_eq!(deserialized.cubemx_path, original.cubemx_path);
    assert_eq!(deserialized.programmer_frequency_khz, 8000);
    assert_eq!(deserialized.programmer_interface, "SWD");
    assert_eq!(deserialized.debug_adapter, "probe-rs");
}

#[test]
fn test_task_generation_cmake() {
    let tasks = generate_all_tasks(
        BuildSystem::CMake,
        Some("firmware.ioc"),
        "build/firmware.elf",
        "STM32F401RETx",
    );

    let build_task = tasks.iter().find(|t| t.label == "STM32: Build").unwrap();
    assert_eq!(build_task.command, "cmake");
    assert_eq!(build_task.args, vec!["--build", "build"]);

    let clean_task = tasks.iter().find(|t| t.label == "STM32: Clean").unwrap();
    assert_eq!(clean_task.command, "cmake");
    assert_eq!(
        clean_task.args,
        vec!["--build", "build", "--target", "clean"]
    );
}

#[test]
fn test_task_generation_make() {
    let tasks = generate_all_tasks(
        BuildSystem::Make,
        None,
        "build/firmware.elf",
        "STM32F401RETx",
    );

    let build_task = tasks.iter().find(|t| t.label == "STM32: Build").unwrap();
    assert_eq!(build_task.command, "make");

    // No .ioc was provided, so CubeMX tasks should not be generated
    assert!(!tasks.iter().any(|t| t.label == "STM32: Open CubeMX"));
}
