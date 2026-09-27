use stm32_zed::toolchain::{
    cubemx_search_paths, cubeprogrammer_search_paths, gdb_binary_names, DoctorReport, ToolCategory,
    ToolStatus, REQUIRED_TOOLS,
};

#[test]
fn test_required_tools_list() {
    assert!(!REQUIRED_TOOLS.is_empty());
    let gcc_query = REQUIRED_TOOLS
        .iter()
        .find(|t| t.binary == "arm-none-eabi-gcc");
    assert!(gcc_query.is_some());
    assert!(gcc_query.unwrap().required);

    let gdb_query = REQUIRED_TOOLS
        .iter()
        .find(|t| t.binary == "arm-none-eabi-gdb");
    assert!(gdb_query.is_some());

    let clangd_query = REQUIRED_TOOLS.iter().find(|t| t.binary == "clangd");
    assert!(clangd_query.is_some());
}

#[test]
fn test_search_paths_contain_cross_platform_defaults() {
    let mx_paths = cubemx_search_paths();
    assert!(mx_paths.iter().any(|p| p.contains("STM32CubeMX")));
    assert!(mx_paths.iter().any(|p| p.contains("/opt/")));

    let prog_paths = cubeprogrammer_search_paths();
    assert!(prog_paths
        .iter()
        .any(|p| p.contains("STM32_Programmer_CLI")));
}

#[test]
fn test_gdb_binary_names_fallback() {
    let names = gdb_binary_names();
    assert!(names.contains(&"arm-none-eabi-gdb"));
    assert!(names.contains(&"gdb-multiarch"));
}

#[test]
fn test_doctor_report_status() {
    let mut report = DoctorReport::new();
    report.add(ToolStatus {
        name: "ARM GCC".to_string(),
        binary_name: "arm-none-eabi-gcc".to_string(),
        category: ToolCategory::Compiler,
        required: true,
        installed: true,
        path: Some("/usr/bin/arm-none-eabi-gcc".to_string()),
        version: Some("14.2.0".to_string()),
        install_hint: "".to_string(),
    });

    assert!(report.all_required_present());
    assert!(report.missing_tools().is_empty());

    report.add(ToolStatus {
        name: "Clangd".to_string(),
        binary_name: "clangd".to_string(),
        category: ToolCategory::LanguageServer,
        required: true,
        installed: false,
        path: None,
        version: None,
        install_hint: "Install clangd".to_string(),
    });

    assert!(!report.all_required_present());
    assert_eq!(report.missing_tools().len(), 1);

    let md = report.format_markdown();
    assert!(md.contains("[✓] ARM GCC"));
    assert!(md.contains("[✗] Clangd"));
}
