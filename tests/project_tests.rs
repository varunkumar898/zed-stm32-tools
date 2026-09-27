use stm32_zed::devices::Stm32Family;
use stm32_zed::project::{
    generate_project_scaffold, BuildSystem, IocProject, LinkerScript, ProjectTemplateConfig,
    Stm32Project,
};

#[test]
fn test_complex_ioc_parsing() {
    let ioc = r#"
# MicroXplorer Configuration settings - do not modify
File.Version=6
KeepUserCode=true
Mcu.CPUType=ARM Cortex-M4
Mcu.Family=STM32F4
Mcu.IP0=NVIC
Mcu.IP1=RCC
Mcu.IP2=SYS
Mcu.IP3=USART2
Mcu.IPNb=4
Mcu.Name=STM32F446RETx
Mcu.Package=LQFP64
Mcu.Pin0=PA2
Mcu.Pin1=PA3
Mcu.PinsNb=2
Mcu.ThirdPartyNb=0
Mcu.UserConstants=
Mcu.UserName=STM32F446RETx
MxCube.Version=6.12.0
MxDb.Version=DB.6.0.120
ProjectManager.CustomerFirmwarePackage=STM32Cube_FW_F4_V1.28.0
ProjectManager.TargetToolchain=Makefile
ProjectManager.ToolChainLocation=
    "#;

    let parsed = IocProject::parse("STM32F446RE.ioc", ioc);
    assert_eq!(parsed.mcu_name, Some("STM32F446RETx".to_string()));
    assert_eq!(parsed.family, Some(Stm32Family::F4));
    assert_eq!(parsed.toolchain_target, Some("Makefile".to_string()));
    assert_eq!(parsed.cubemx_version, Some("6.12.0".to_string()));
    assert!(!parsed.has_freertos);
}

#[test]
fn test_linker_script_memory_regions() {
    let ld = r#"
/* Linker script for STM32H743ZITx Device with 2048KByte FLASH, 1024KByte RAM */

ENTRY(Reset_Handler)

_estack = ORIGIN(RAM_D1) + LENGTH(RAM_D1);

MEMORY
{
  DTCMRAM (xrw)   : ORIGIN = 0x20000000, LENGTH = 128K
  RAM_D1  (xrw)   : ORIGIN = 0x24000000, LENGTH = 512K
  FLASH   (rx)    : ORIGIN = 0x08000000, LENGTH = 2048K
}
    "#;

    let parsed = LinkerScript::parse("STM32H743ZITx_FLASH.ld", ld);
    assert_eq!(parsed.regions.len(), 3);

    let flash = parsed.flash_region().expect("Flash region");
    assert_eq!(flash.origin, 0x08000000);
    assert_eq!(flash.length, 2048 * 1024);

    let dtcm = parsed
        .regions
        .iter()
        .find(|r| r.name == "DTCMRAM")
        .expect("DTCMRAM");
    assert_eq!(dtcm.origin, 0x20000000);
    assert_eq!(dtcm.length, 128 * 1024);
}

#[test]
fn test_project_summary_formatting() {
    let mut project = Stm32Project::new("/home/user/my_stm32_app".to_string());
    project.mcu = Some("STM32F401RE".to_string());
    project.family = Some(Stm32Family::F4);
    project.build_system = BuildSystem::CMake;
    project.has_freertos = true;
    project
        .elf_artifacts
        .push("build/my_stm32_app.elf".to_string());

    let summary = project.summary_markdown();
    assert!(summary.contains("/home/user/my_stm32_app"));
    assert!(summary.contains("STM32F401RE"));
    assert!(summary.contains("Family:** STM32F4"));
    assert!(summary.contains("Build System:** CMake"));
    assert!(summary.contains("FreeRTOS Detected:** Yes"));
    assert!(summary.contains("build/my_stm32_app.elf"));
}

#[test]
fn test_generate_cmake_scaffold() {
    let cfg = ProjectTemplateConfig {
        name: "SensorNode".to_string(),
        mcu: "STM32G474RE".to_string(),
        build_system: BuildSystem::CMake,
        use_freertos: false,
    };

    let files = generate_project_scaffold(&cfg).expect("generation should succeed");
    assert!(files.iter().any(|f| f.relative_path == "CMakeLists.txt"));
    assert!(files.iter().any(|f| f.relative_path == "src/main.c"));
    assert!(files.iter().any(|f| f.relative_path == "linker.ld"));

    let cmake_file = files
        .iter()
        .find(|f| f.relative_path == "CMakeLists.txt")
        .unwrap();
    assert!(cmake_file.content.contains("project(SensorNode C ASM)"));
}
