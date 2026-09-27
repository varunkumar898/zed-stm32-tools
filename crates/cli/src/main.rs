//! Native STM32 Tools CLI for Zed Editor.
//!
//! Provides CLI execution for toolchain diagnostics, project inspection,
//! device querying, building, flashing, and memory analytics.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use stm32_zed::devices::{find_device, ArmCore, STM32_DEVICES};
use stm32_zed::project::{BuildSystem, IocProject, LinkerScript, Stm32Project};
use stm32_zed::toolchain::{DoctorReport, ToolCategory, ToolStatus, REQUIRED_TOOLS};
use stm32_zed::utils::{format_bytes, render_progress_bar};

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_BOLD: &str = "\x1b[1m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_DIM: &str = "\x1b[2m";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return ExitCode::SUCCESS;
    }

    match args[1].as_str() {
        "doctor" => run_doctor(),
        "info" => run_info(args.get(2).map(|s| s.as_str())),
        "devices" | "device" => run_devices(args.get(2).map(|s| s.as_str())),
        "build" => run_build(args.get(2).map(|s| s.as_str())),
        "clean" => run_clean(),
        "size" => run_size(args.get(2).map(|s| s.as_str())),
        "flash" => run_flash(args.get(2).map(|s| s.as_str())),
        "cubemx" => run_cubemx(args.get(2).map(|s| s.as_str())),
        "new" => {
            if args.len() < 4 {
                eprintln!(
                    "{}Error:{} 'new' requires MCU part number and project name.\nUsage: stm32-tools-cli new <MCU> <project_name>",
                    COLOR_RED, COLOR_RESET
                );
                return ExitCode::FAILURE;
            }
            run_new(&args[2], &args[3])
        }
        "help" | "--help" | "-h" => {
            print_usage();
            ExitCode::SUCCESS
        }
        unknown => {
            eprintln!(
                "{}Error:{} Unknown command '{}'.\nRun 'stm32-tools-cli help' for available commands.",
                COLOR_RED, COLOR_RESET, unknown
            );
            ExitCode::FAILURE
        }
    }
}

fn print_usage() {
    println!(
        "{bold}STM32 Tools for Zed — Embedded Development Suite{reset}

{cyan}USAGE:{reset}
    stm32-tools-cli <COMMAND> [OPTIONS]

{cyan}COMMANDS:{reset}
    {green}doctor{reset}             Run diagnostics on Arm GCC, GDB, CubeMX, CubeProg, OpenOCD, probe-rs
    {green}info{reset} [PATH]         Inspect .ioc, linker scripts, and build configuration
    {green}devices{reset} [QUERY]     Search STM32 MCU database (flash, ram, core, packages, openocd)
    {green}build{reset} [--release]   Build firmware using detected build system (Make, CMake, Ninja)
    {green}clean{reset}               Clean firmware build artifacts
    {green}size{reset} [ELF_FILE]     Analyze firmware Flash & RAM consumption against device limits
    {green}flash{reset} [--tool TOOL] Flash target board (auto, cubeprog, probe-rs, openocd)
    {green}cubemx{reset} [--gui|--gen] Open STM32CubeMX or trigger headless code generation
    {green}new{reset} <MCU> <NAME>    Scaffold new STM32 project with build files & Zed tasks
    {green}help{reset}                Print this help message
",
        bold = COLOR_BOLD,
        reset = COLOR_RESET,
        cyan = COLOR_CYAN,
        green = COLOR_GREEN,
    );
}

fn which(binary: &str) -> Option<PathBuf> {
    if let Ok(path_var) = env::var("PATH") {
        for dir in env::split_paths(&path_var) {
            let candidate = dir.join(binary);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

fn get_version(binary: &str) -> Option<String> {
    let output = match binary {
        "arm-none-eabi-gcc" | "arm-none-eabi-gdb" | "gdb-multiarch" | "clangd" | "make"
        | "cmake" | "ninja" | "openocd" => Command::new(binary).arg("--version").output().ok()?,
        "probe-rs" => Command::new(binary).arg("--version").output().ok()?,
        "STM32_Programmer_CLI" => Command::new(binary).arg("--version").output().ok()?,
        _ => return None,
    };

    if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout);
        let first_line = text.lines().next().unwrap_or("").trim();
        if !first_line.is_empty() {
            return Some(first_line.to_string());
        }
    }
    None
}

fn run_doctor() -> ExitCode {
    println!(
        "\n{bold}======================================================================{reset}",
        bold = COLOR_BOLD,
        reset = COLOR_RESET
    );
    println!(
        "                 {bold}{cyan}STM32 Toolchain Doctor Diagnostics{reset}",
        bold = COLOR_BOLD,
        cyan = COLOR_CYAN,
        reset = COLOR_RESET
    );
    println!(
        "{bold}======================================================================{reset}\n",
        bold = COLOR_BOLD,
        reset = COLOR_RESET
    );

    let mut report = DoctorReport::new();

    for tool in REQUIRED_TOOLS {
        let (installed, path) = if let Some(p) = which(tool.binary) {
            (true, Some(p.to_string_lossy().to_string()))
        } else if tool.binary == "arm-none-eabi-gdb" {
            if let Some(p) = which("gdb-multiarch") {
                (true, Some(p.to_string_lossy().to_string()))
            } else {
                (false, None)
            }
        } else {
            (false, None)
        };

        let version = if installed {
            get_version(tool.binary)
        } else {
            None
        };

        report.add(ToolStatus {
            name: tool.name.to_string(),
            binary_name: tool.binary.to_string(),
            category: tool.category,
            required: tool.required,
            installed,
            path,
            version,
            install_hint: tool.install_hint.to_string(),
        });
    }

    let categories = [
        (ToolCategory::Compiler, "Compiler & Assembler"),
        (ToolCategory::Debugger, "Debuggers"),
        (ToolCategory::Programmer, "Hardware Programmers / Flashers"),
        (
            ToolCategory::LanguageServer,
            "Language Server & Intelligence",
        ),
        (ToolCategory::BuildSystem, "Build Systems"),
        (ToolCategory::Generator, "Configuration & Code Generators"),
    ];

    for (cat, cat_title) in &categories {
        println!(
            "{bold}[ {title} ]{reset}",
            bold = COLOR_BOLD,
            title = cat_title,
            reset = COLOR_RESET
        );

        for tool in report.tools.iter().filter(|t| t.category == *cat) {
            if tool.installed {
                let ver_str = tool
                    .version
                    .as_deref()
                    .map(|v| format!(" ({})", v))
                    .unwrap_or_default();
                let path_str = tool.path.as_deref().unwrap_or("");
                println!(
                    "  {green}✓{reset} {bold}{name:<22}{reset} {dim}{path}{reset}{ver}",
                    green = COLOR_GREEN,
                    reset = COLOR_RESET,
                    bold = COLOR_BOLD,
                    name = tool.name,
                    dim = COLOR_DIM,
                    path = path_str,
                    ver = ver_str
                );
            } else {
                let req_marker = if tool.required {
                    format!("{} [REQUIRED]{}", COLOR_RED, COLOR_RESET)
                } else {
                    format!("{} [Optional]{}", COLOR_DIM, COLOR_RESET)
                };
                println!(
                    "  {red}✗{reset} {bold}{name:<22}{reset}{req}",
                    red = COLOR_RED,
                    reset = COLOR_RESET,
                    bold = COLOR_BOLD,
                    name = tool.name,
                    req = req_marker
                );
                println!(
                    "      {dim}Binary: {bin} | Install: {hint}{reset}",
                    dim = COLOR_DIM,
                    bin = tool.binary_name,
                    hint = tool.install_hint,
                    reset = COLOR_RESET
                );
            }
        }
        println!();
    }

    if report.all_required_present() {
        println!(
            "{green}{bold}✓ All required STM32 development tools are installed and operational.{reset}\n",
            green = COLOR_GREEN,
            bold = COLOR_BOLD,
            reset = COLOR_RESET
        );
        ExitCode::SUCCESS
    } else {
        println!(
            "{yellow}{bold}! Some required STM32 tools are missing. Review the install hints above.{reset}\n",
            yellow = COLOR_YELLOW,
            bold = COLOR_BOLD,
            reset = COLOR_RESET
        );
        ExitCode::FAILURE
    }
}

fn run_info(path_opt: Option<&str>) -> ExitCode {
    let dir = path_opt
        .map(PathBuf::from)
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    println!(
        "\n{bold}STM32 Project Inspection:{reset} {dim}{}{reset}\n",
        dir.display(),
        bold = COLOR_BOLD,
        reset = COLOR_RESET,
        dim = COLOR_DIM
    );

    let mut project = Stm32Project::new(dir.to_string_lossy().to_string());

    // 1. Detect build system
    if dir.join("CMakeLists.txt").exists() {
        project.build_system = BuildSystem::CMake;
    } else if dir.join("build.ninja").exists() {
        project.build_system = BuildSystem::Ninja;
    } else if dir.join("Makefile").exists() {
        project.build_system = BuildSystem::Make;
    }

    // 2. Detect .ioc file
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("ioc") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let filename = path.file_name().unwrap_or_default().to_string_lossy();
                    let parsed = IocProject::parse(&filename, &content);
                    project.mcu = parsed.mcu_name.clone();
                    project.family = parsed.family;
                    project.has_freertos = parsed.has_freertos;
                    project.has_hal = parsed.has_hal;
                    project.has_ll = parsed.has_ll;
                    project.ioc = Some(parsed);
                    break;
                }
            }
        }
    }

    // 3. Detect linker script
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("ld") {
                if let Ok(content) = fs::read_to_string(&path) {
                    let filename = path.file_name().unwrap_or_default().to_string_lossy();
                    let parsed = LinkerScript::parse(&filename, &content);
                    project.linker_script = Some(parsed);
                    break;
                }
            }
        }
    }

    // 4. FreeRTOS config check
    if dir.join("Core/Inc/FreeRTOSConfig.h").exists()
        || dir.join("FreeRTOSConfig.h").exists()
        || dir.join("include/FreeRTOSConfig.h").exists()
    {
        project.has_freertos = true;
    }

    println!(
        "  * {bold}Build System:{reset} {}",
        project.build_system.name(),
        bold = COLOR_BOLD,
        reset = COLOR_RESET
    );
    if let Some(mcu) = &project.mcu {
        println!(
            "  * {bold}Target MCU:{reset}   {green}{}{reset}",
            mcu,
            bold = COLOR_BOLD,
            green = COLOR_GREEN,
            reset = COLOR_RESET
        );
    } else {
        println!(
            "  * {bold}Target MCU:{reset}   {yellow}Not detected (no .ioc found){reset}",
            bold = COLOR_BOLD,
            yellow = COLOR_YELLOW,
            reset = COLOR_RESET
        );
    }

    if let Some(fam) = project.family {
        println!(
            "  * {bold}Family:{reset}       {}",
            fam.name(),
            bold = COLOR_BOLD,
            reset = COLOR_RESET
        );
    }

    println!(
        "  * {bold}FreeRTOS:{reset}     {}",
        if project.has_freertos {
            "Enabled ✓"
        } else {
            "Disabled"
        },
        bold = COLOR_BOLD,
        reset = COLOR_RESET
    );
    println!(
        "  * {bold}HAL Drivers:{reset}  {}",
        if project.has_hal {
            "Present ✓"
        } else {
            "None"
        },
        bold = COLOR_BOLD,
        reset = COLOR_RESET
    );

    if let Some(ld) = &project.linker_script {
        println!(
            "\n  {cyan}{bold}Linker Memory Regions:{reset}",
            cyan = COLOR_CYAN,
            bold = COLOR_BOLD,
            reset = COLOR_RESET
        );
        for reg in &ld.regions {
            println!(
                "    - {bold}{:<8}{reset} Origin: 0x{:08X} | Length: {} (0x{:X})",
                reg.name,
                reg.origin,
                format_bytes(reg.length),
                reg.length,
                bold = COLOR_BOLD,
                reset = COLOR_RESET
            );
        }
    }

    println!();
    ExitCode::SUCCESS
}

fn run_devices(query_opt: Option<&str>) -> ExitCode {
    if let Some(q) = query_opt {
        let q_upper = q.to_uppercase();
        let matches: Vec<_> = STM32_DEVICES
            .iter()
            .filter(|d| {
                d.part_number.to_uppercase().contains(&q_upper)
                    || d.series.to_uppercase().contains(&q_upper)
                    || d.family.name().to_uppercase().contains(&q_upper)
            })
            .collect();

        if matches.is_empty() {
            println!(
                "No STM32 devices found matching '{}'. Try 'f4', 'g4', 'h7', or 'stm32f401re'.",
                q
            );
            return ExitCode::FAILURE;
        }

        println!(
            "\n{bold}Found {} matching STM32 device(s):{reset}\n",
            matches.len(),
            bold = COLOR_BOLD,
            reset = COLOR_RESET
        );

        for dev in matches {
            println!(
                "  {green}{bold}{:<18}{reset} | {family:<8} | Core: {core:<14} | Max: {mhz:>3} MHz | Flash: {:<8} | RAM: {:<6} | Pkg: {}",
                dev.part_number,
                format_bytes(dev.flash_bytes),
                format_bytes(dev.ram_bytes),
                dev.package,
                green = COLOR_GREEN,
                bold = COLOR_BOLD,
                reset = COLOR_RESET,
                family = dev.family.name(),
                core = dev.core.name(),
                mhz = dev.max_frequency_mhz,
            );
            println!(
                "    {dim}OpenOCD: target/{openocd} | probe-rs: {probers} | SVD: {svd}{reset}",
                openocd = dev.openocd_target,
                probers = dev.probe_rs_target,
                svd = dev.svd_filename,
                dim = COLOR_DIM,
                reset = COLOR_RESET
            );
        }
        println!();
    } else {
        println!(
            "\n{bold}STM32 Device Families in Database ({total} curated parts):{reset}\n",
            total = STM32_DEVICES.len(),
            bold = COLOR_BOLD,
            reset = COLOR_RESET
        );
        for dev in STM32_DEVICES.iter().take(15) {
            println!(
                "  * {bold}{:<18}{reset} ({:<8}, {}, Flash: {}, RAM: {})",
                dev.part_number,
                dev.family.name(),
                dev.core.name(),
                format_bytes(dev.flash_bytes),
                format_bytes(dev.ram_bytes),
                bold = COLOR_BOLD,
                reset = COLOR_RESET
            );
        }
        println!(
            "\n{dim}Use 'stm32-tools-cli devices <query>' to search specific MCUs.{reset}\n",
            dim = COLOR_DIM,
            reset = COLOR_RESET
        );
    }

    ExitCode::SUCCESS
}

fn find_elf_file(dir: &Path) -> Option<PathBuf> {
    let candidate_dirs = [
        dir.join("build"),
        dir.join("Debug"),
        dir.join("Release"),
        dir.to_path_buf(),
    ];

    for d in &candidate_dirs {
        if let Ok(entries) = fs::read_dir(d) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.extension().and_then(|s| s.to_str()) == Some("elf") {
                    return Some(p);
                }
            }
        }
    }
    None
}

fn run_size(elf_opt: Option<&str>) -> ExitCode {
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let elf_path = if let Some(p) = elf_opt {
        PathBuf::from(p)
    } else if let Some(found) = find_elf_file(&current_dir) {
        found
    } else {
        eprintln!(
            "{}Error:{} No .elf binary found. Build the project first or specify path.\nUsage: stm32-tools-cli size [path/to/binary.elf]",
            COLOR_RED, COLOR_RESET
        );
        return ExitCode::FAILURE;
    };

    println!(
        "\n{bold}Analyzing Firmware Size:{reset} {dim}{}{reset}\n",
        elf_path.display(),
        bold = COLOR_BOLD,
        reset = COLOR_RESET,
        dim = COLOR_DIM
    );

    let output = match Command::new("arm-none-eabi-size")
        .arg("-B")
        .arg(&elf_path)
        .output()
    {
        Ok(out) => out,
        Err(err) => {
            eprintln!(
                "{}Error running arm-none-eabi-size: {}{}",
                COLOR_RED, err, COLOR_RESET
            );
            return ExitCode::FAILURE;
        }
    };

    if !output.status.success() {
        eprintln!(
            "{}arm-none-eabi-size failed: {}{}",
            COLOR_RED,
            String::from_utf8_lossy(&output.stderr),
            COLOR_RESET
        );
        return ExitCode::FAILURE;
    }

    let size_stdout = String::from_utf8_lossy(&output.stdout);
    // Parse default Berkeley format
    let lines: Vec<&str> = size_stdout.lines().collect();
    if lines.len() >= 2 {
        let parts: Vec<&str> = lines[1].split_whitespace().collect();
        if parts.len() >= 3 {
            let text: u64 = parts[0].parse().unwrap_or(0);
            let data: u64 = parts[1].parse().unwrap_or(0);
            let bss: u64 = parts[2].parse().unwrap_or(0);

            let flash_used = text + data;
            let ram_used = data + bss;

            // Try to infer device limits from project
            let mut max_flash = 512 * 1024;
            let mut max_ram = 128 * 1024;

            // Read .ioc if present
            if let Ok(entries) = fs::read_dir(&current_dir) {
                for entry in entries.flatten() {
                    let p = entry.path();
                    if p.extension().and_then(|s| s.to_str()) == Some("ioc") {
                        if let Ok(content) = fs::read_to_string(&p) {
                            let parsed = IocProject::parse("", &content);
                            if let Some(mcu) = &parsed.mcu_name {
                                if let Some(dev) = find_device(mcu) {
                                    max_flash = dev.flash_bytes;
                                    max_ram = dev.ram_bytes;
                                }
                            }
                        }
                    }
                }
            }

            let flash_pct = (flash_used as f64 / max_flash as f64) * 100.0;
            let ram_pct = (ram_used as f64 / max_ram as f64) * 100.0;

            println!(
                "  {bold}FLASH Usage:{reset} [{bar}] {pct:>5.1}%  ({used} / {total})",
                bar = render_progress_bar(flash_pct, 24),
                pct = flash_pct,
                used = format_bytes(flash_used),
                total = format_bytes(max_flash),
                bold = COLOR_BOLD,
                reset = COLOR_RESET
            );

            println!(
                "  {bold}RAM   Usage:{reset} [{bar}] {pct:>5.1}%  ({used} / {total})\n",
                bar = render_progress_bar(ram_pct, 24),
                pct = ram_pct,
                used = format_bytes(ram_used),
                total = format_bytes(max_ram),
                bold = COLOR_BOLD,
                reset = COLOR_RESET
            );

            println!(
                "  {dim}Sections: .text = {} | .data = {} | .bss = {}{reset}\n",
                format_bytes(text),
                format_bytes(data),
                format_bytes(bss),
                dim = COLOR_DIM,
                reset = COLOR_RESET
            );

            return ExitCode::SUCCESS;
        }
    }

    println!("{}", size_stdout);
    ExitCode::SUCCESS
}

fn run_build(release_flag: Option<&str>) -> ExitCode {
    let dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let is_release = release_flag == Some("--release");

    println!(
        "\n{bold}Building STM32 Firmware ({mode})...{reset}\n",
        mode = if is_release { "Release" } else { "Debug" },
        bold = COLOR_BOLD,
        reset = COLOR_RESET
    );

    let status = if dir.join("CMakeLists.txt").exists() {
        if !dir.join("build").exists() {
            println!(
                "{cyan}Configuring CMake project...{reset}",
                cyan = COLOR_CYAN,
                reset = COLOR_RESET
            );
            let config_status = Command::new("cmake")
                .args(["-B", "build", "-GNinja"])
                .status();
            if let Ok(st) = config_status {
                if !st.success() {
                    // Fallback to standard generator
                    let _ = Command::new("cmake").args(["-B", "build"]).status();
                }
            }
        }
        Command::new("cmake").args(["--build", "build"]).status()
    } else if dir.join("build.ninja").exists() {
        Command::new("ninja").status()
    } else if dir.join("Makefile").exists() {
        Command::new("make").arg("-j").status()
    } else {
        eprintln!(
            "{}Error:{} No recognized build system (CMakeLists.txt, Makefile, or build.ninja) found in current directory.",
            COLOR_RED, COLOR_RESET
        );
        return ExitCode::FAILURE;
    };

    match status {
        Ok(st) if st.success() => {
            println!(
                "\n{green}{bold}✓ Build succeeded!{reset}",
                green = COLOR_GREEN,
                bold = COLOR_BOLD,
                reset = COLOR_RESET
            );
            // Automatically run size report on output
            let _ = run_size(None);
            ExitCode::SUCCESS
        }
        Ok(st) => {
            eprintln!(
                "\n{red}{bold}✗ Build failed with exit code {:?}{reset}",
                st.code(),
                red = COLOR_RED,
                bold = COLOR_BOLD,
                reset = COLOR_RESET
            );
            ExitCode::FAILURE
        }
        Err(err) => {
            eprintln!(
                "\n{red}Failed to execute build tool: {}{reset}",
                err,
                red = COLOR_RED,
                reset = COLOR_RESET
            );
            ExitCode::FAILURE
        }
    }
}

fn run_clean() -> ExitCode {
    let dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    println!(
        "{cyan}Cleaning build artifacts...{reset}",
        cyan = COLOR_CYAN,
        reset = COLOR_RESET
    );

    if dir.join("build").exists() {
        let _ = fs::remove_dir_all(dir.join("build"));
    }

    if dir.join("Makefile").exists() {
        let _ = Command::new("make").arg("clean").status();
    }

    println!(
        "{green}✓ Clean complete.{reset}",
        green = COLOR_GREEN,
        reset = COLOR_RESET
    );
    ExitCode::SUCCESS
}

fn run_flash(tool_opt: Option<&str>) -> ExitCode {
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let elf_file = match find_elf_file(&current_dir) {
        Some(f) => f,
        None => {
            eprintln!(
                "{}Error:{} No .elf binary found. Run 'stm32-tools-cli build' first.",
                COLOR_RED, COLOR_RESET
            );
            return ExitCode::FAILURE;
        }
    };

    let tool = tool_opt.unwrap_or("auto");
    println!(
        "\n{bold}Flashing STM32 Firmware:{reset} {dim}{}{reset} using {cyan}{}{reset}\n",
        elf_file.display(),
        tool,
        bold = COLOR_BOLD,
        reset = COLOR_RESET,
        dim = COLOR_DIM,
        cyan = COLOR_CYAN
    );

    // Try probe-rs first if installed, else STM32_Programmer_CLI, else OpenOCD
    let status = if tool == "probe-rs" || (tool == "auto" && which("probe-rs").is_some()) {
        println!(
            "{cyan}Running probe-rs run...{reset}",
            cyan = COLOR_CYAN,
            reset = COLOR_RESET
        );
        Command::new("probe-rs")
            .args(["run", elf_file.to_str().unwrap_or("")])
            .status()
    } else if tool == "cubeprog" || (tool == "auto" && which("STM32_Programmer_CLI").is_some()) {
        println!(
            "{cyan}Running STM32_Programmer_CLI...{reset}",
            cyan = COLOR_CYAN,
            reset = COLOR_RESET
        );
        Command::new("STM32_Programmer_CLI")
            .args([
                "-c",
                "port=SWD",
                "-d",
                elf_file.to_str().unwrap_or(""),
                "-v",
                "-rst",
            ])
            .status()
    } else if tool == "openocd" || (tool == "auto" && which("openocd").is_some()) {
        println!(
            "{cyan}Running OpenOCD flash...{reset}",
            cyan = COLOR_CYAN,
            reset = COLOR_RESET
        );
        Command::new("openocd")
            .args([
                "-f",
                "interface/stlink.cfg",
                "-f",
                "target/stm32f4x.cfg",
                "-c",
                &format!("program {} verify reset exit", elf_file.display()),
            ])
            .status()
    } else {
        eprintln!(
            "{}Error:{} No supported hardware flasher found (probe-rs, STM32_Programmer_CLI, or openocd). Run 'stm32-tools-cli doctor' to diagnose.",
            COLOR_RED, COLOR_RESET
        );
        return ExitCode::FAILURE;
    };

    match status {
        Ok(st) if st.success() => {
            println!(
                "\n{green}{bold}✓ Flashing completed successfully! Target restarted.{reset}\n",
                green = COLOR_GREEN,
                bold = COLOR_BOLD,
                reset = COLOR_RESET
            );
            ExitCode::SUCCESS
        }
        _ => {
            eprintln!(
                "\n{red}{bold}✗ Flashing failed. Check target power and ST-LINK connection.{reset}\n",
                red = COLOR_RED,
                bold = COLOR_BOLD,
                reset = COLOR_RESET
            );
            ExitCode::FAILURE
        }
    }
}

fn run_cubemx(mode_opt: Option<&str>) -> ExitCode {
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut ioc_file = None;

    if let Ok(entries) = fs::read_dir(&current_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) == Some("ioc") {
                ioc_file = Some(p);
                break;
            }
        }
    }

    let cubemx_bin = which("stm32cubemx")
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "stm32cubemx".to_string());

    if mode_opt == Some("--generate") || mode_opt == Some("-g") {
        if let Some(ioc) = ioc_file {
            println!(
                "{cyan}Running STM32CubeMX headless code generation for {}...{reset}",
                ioc.display(),
                cyan = COLOR_CYAN,
                reset = COLOR_RESET
            );
            let script_content = format!("config load {}\ngenerate code .\nexit\n", ioc.display());
            let script_path = current_dir.join(".cubemx_gen.tmp");
            let _ = fs::write(&script_path, script_content);

            let status = Command::new(&cubemx_bin)
                .args(["-q", script_path.to_str().unwrap_or("")])
                .status();
            let _ = fs::remove_file(script_path);

            if let Ok(st) = status {
                if st.success() {
                    println!(
                        "{green}✓ Code generation complete.{reset}",
                        green = COLOR_GREEN,
                        reset = COLOR_RESET
                    );
                    return ExitCode::SUCCESS;
                }
            }
            eprintln!(
                "{}CubeMX code generation failed. Ensure stm32cubemx is configured.{}",
                COLOR_RED, COLOR_RESET
            );
            ExitCode::FAILURE
        } else {
            eprintln!(
                "{}No .ioc file found in current directory.{}",
                COLOR_RED, COLOR_RESET
            );
            ExitCode::FAILURE
        }
    } else {
        println!(
            "{cyan}Launching STM32CubeMX GUI...{reset}",
            cyan = COLOR_CYAN,
            reset = COLOR_RESET
        );
        let mut cmd = Command::new(&cubemx_bin);
        if let Some(ioc) = ioc_file {
            cmd.arg(ioc);
        }
        match cmd.spawn() {
            Ok(_) => {
                println!(
                    "{green}✓ STM32CubeMX launched successfully.{reset}",
                    green = COLOR_GREEN,
                    reset = COLOR_RESET
                );
                ExitCode::SUCCESS
            }
            Err(err) => {
                eprintln!(
                    "{}Failed to launch STM32CubeMX: {}{}",
                    COLOR_RED, err, COLOR_RESET
                );
                ExitCode::FAILURE
            }
        }
    }
}

fn run_new(mcu: &str, project_name: &str) -> ExitCode {
    let device = match find_device(mcu) {
        Some(d) => d,
        None => {
            eprintln!(
                "{}Error:{} Unknown MCU '{}'. Run 'stm32-tools-cli devices' to view valid part numbers.",
                COLOR_RED, COLOR_RESET, mcu
            );
            return ExitCode::FAILURE;
        }
    };

    let target_dir = PathBuf::from(project_name);
    if target_dir.exists() {
        eprintln!(
            "{}Error:{} Directory '{}' already exists.",
            COLOR_RED, COLOR_RESET, project_name
        );
        return ExitCode::FAILURE;
    }

    println!(
        "\n{bold}Creating new STM32 project:{reset} {cyan}{}{reset} for {green}{}{reset}...",
        project_name,
        device.part_number,
        bold = COLOR_BOLD,
        reset = COLOR_RESET,
        cyan = COLOR_CYAN,
        green = COLOR_GREEN
    );

    let _ = fs::create_dir_all(target_dir.join("src"));
    let _ = fs::create_dir_all(target_dir.join("inc"));
    let _ = fs::create_dir_all(target_dir.join(".zed"));

    // Write main.c
    let main_c = format!(
        r#"/**
 * Main application for {mcu} ({core})
 * Generated by Zed STM32 Tools
 */

#include <stdint.h>
#include <stdbool.h>

/* USER CODE BEGIN Includes */
/* USER CODE END Includes */

void SystemClock_Config(void);
void Error_Handler(void);

int main(void) {{
    /* USER CODE BEGIN 1 */
    /* USER CODE END 1 */

    SystemClock_Config();

    /* USER CODE BEGIN 2 */
    /* USER CODE END 2 */

    /* Infinite loop */
    while (1) {{
        /* USER CODE BEGIN 3 */
        /* Application logic */
        /* USER CODE END 3 */
    }}
}}

void SystemClock_Config(void) {{
    /* Configure target system clock for {mcu} ({freq} MHz max) */
}}

void Error_Handler(void) {{
    while (1) {{
        /* Loop on error */
    }}
}}
"#,
        mcu = device.part_number,
        core = device.core.name(),
        freq = device.max_frequency_mhz
    );
    let _ = fs::write(target_dir.join("src/main.c"), main_c);

    // Write linker script
    let ld_content = format!(
        r#"/* Linker script for {mcu} */
ENTRY(Reset_Handler)

MEMORY
{{
  FLASH (rx)      : ORIGIN = 0x08000000, LENGTH = {flash}K
  RAM (xrw)       : ORIGIN = 0x20000000, LENGTH = {ram}K
}}

SECTIONS
{{
  .isr_vector :
  {{
    . = ALIGN(4);
    KEEP(*(.isr_vector))
    . = ALIGN(4);
  }} >FLASH

  .text :
  {{
    . = ALIGN(4);
    *(.text)
    *(.text*)
    . = ALIGN(4);
  }} >FLASH

  .data :
  {{
    . = ALIGN(4);
    *(.data)
    *(.data*)
    . = ALIGN(4);
  }} >RAM AT> FLASH

  .bss :
  {{
    . = ALIGN(4);
    *(.bss)
    *(.bss*)
    . = ALIGN(4);
  }} >RAM
}}
"#,
        mcu = device.part_number,
        flash = device.flash_bytes / 1024,
        ram = device.ram_bytes / 1024
    );
    let _ = fs::write(target_dir.join("linker.ld"), ld_content);

    // Write CMakeLists.txt
    let cmake_content = format!(
        r#"cmake_minimum_required(VERSION 3.20)
project({proj} C ASM)

set(CMAKE_C_STANDARD 11)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

set(CPU_PARAMETERS -mcpu={cpu} -mthumb)

add_compile_options(
    ${{CPU_PARAMETERS}}
    -Wall
    -Wextra
    -Wpedantic
    -fdata-sections
    -ffunction-sections
)

add_link_options(
    ${{CPU_PARAMETERS}}
    -T${{CMAKE_CURRENT_SOURCE_DIR}}/linker.ld
    -Wl,--gc-sections
    -Wl,-Map=${{CMAKE_CURRENT_BINARY_DIR}}/${{PROJECT_NAME}}.map
    --specs=nano.specs
)

include_directories(inc)

file(GLOB_RECURSE SOURCES "src/*.c")
add_executable(${{PROJECT_NAME}}.elf ${{SOURCES}})
"#,
        proj = project_name,
        cpu = match device.core {
            ArmCore::CortexM0 => "cortex-m0",
            ArmCore::CortexM0Plus => "cortex-m0plus",
            ArmCore::CortexM3 => "cortex-m3",
            ArmCore::CortexM4F => "cortex-m4",
            ArmCore::CortexM7 => "cortex-m7",
            ArmCore::CortexM33 => "cortex-m33",
        }
    );
    let _ = fs::write(target_dir.join("CMakeLists.txt"), cmake_content);

    // Write .zed/tasks.json
    let zed_tasks = r#"[
  {
    "label": "STM32: Toolchain Doctor",
    "command": "stm32-tools-cli",
    "args": ["doctor"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "use_new_terminal": false,
    "reveal": "always"
  },
  {
    "label": "STM32: Project Info",
    "command": "stm32-tools-cli",
    "args": ["info"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "reveal": "always"
  },
  {
    "label": "STM32: Build",
    "command": "stm32-tools-cli",
    "args": ["build"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "reveal": "always"
  },
  {
    "label": "STM32: Clean",
    "command": "stm32-tools-cli",
    "args": ["clean"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "reveal": "always"
  },
  {
    "label": "STM32: Memory Usage (Size Report)",
    "command": "stm32-tools-cli",
    "args": ["size"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "reveal": "always"
  },
  {
    "label": "STM32: Flash & Run",
    "command": "stm32-tools-cli",
    "args": ["flash"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "reveal": "always"
  },
  {
    "label": "STM32: Open CubeMX GUI",
    "command": "stm32-tools-cli",
    "args": ["cubemx"],
    "cwd": "$ZED_WORKTREE_ROOT",
    "reveal": "always"
  }
]
"#;
    let _ = fs::write(target_dir.join(".zed/tasks.json"), zed_tasks);

    println!(
        "\n{green}{bold}✓ Successfully created project '{proj}'!{reset}
To open in Zed:
    {bold}cd {proj}{reset}
    Open in Zed and press {cyan}Ctrl+Alt+T{reset} (or {cyan}task: spawn{reset}) to build, flash, or run diagnostics.\n",
        proj = project_name,
        green = COLOR_GREEN,
        bold = COLOR_BOLD,
        reset = COLOR_RESET,
        cyan = COLOR_CYAN
    );

    ExitCode::SUCCESS
}
