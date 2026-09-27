//! STM32 Project Detection, .ioc Analysis, Linker Script Parsing, and Project Generation.

use crate::devices::{find_device, infer_family, Stm32Family};
use crate::utils::{format_bytes, parse_size_or_hex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildSystem {
    Make,
    CMake,
    Ninja,
    Unknown,
}

impl BuildSystem {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Make => "Make",
            Self::CMake => "CMake",
            Self::Ninja => "Ninja",
            Self::Unknown => "Unknown",
        }
    }

    pub fn default_build_command(&self) -> &'static str {
        match self {
            Self::Make => "make -j",
            Self::CMake => "cmake --build build",
            Self::Ninja => "ninja -C build",
            Self::Unknown => "make",
        }
    }

    pub fn default_clean_command(&self) -> &'static str {
        match self {
            Self::Make => "make clean",
            Self::CMake => "cmake --build build --target clean",
            Self::Ninja => "ninja -C build -t clean",
            Self::Unknown => "make clean",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRegion {
    pub name: String,
    pub origin: u64,
    pub length: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkerScript {
    pub path: String,
    pub regions: Vec<MemoryRegion>,
}

impl LinkerScript {
    pub fn parse(path: &str, content: &str) -> Self {
        let mut regions = Vec::new();
        let mut in_memory_block = false;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("/*") && trimmed.ends_with("*/") {
                continue;
            }

            if trimmed.starts_with("MEMORY") || trimmed == "MEMORY" {
                in_memory_block = true;
                continue;
            }

            if in_memory_block {
                if trimmed == "{" {
                    continue;
                }
                if trimmed.contains('}') {
                    in_memory_block = false;
                    continue;
                }

                // Format: REGION_NAME (attr) : ORIGIN = 0x08000000, LENGTH = 512K
                // or: RAM (xrw) : ORIGIN = 0x20000000, LENGTH = 128K
                if let Some((name_part, rest)) = trimmed.split_once(':') {
                    let reg_name = name_part.split('(').next().unwrap_or("").trim().to_string();

                    if !reg_name.is_empty() && rest.contains("ORIGIN") && rest.contains("LENGTH") {
                        let mut origin = 0u64;
                        let mut length = 0u64;

                        for part in rest.split(',') {
                            let part = part.trim();
                            if let Some((k, v)) = part.split_once('=') {
                                let key = k.trim().to_uppercase();
                                let val_str = v.trim().trim_end_matches(';').trim();
                                if key == "ORIGIN" {
                                    if let Some(num) = parse_size_or_hex(val_str) {
                                        origin = num;
                                    }
                                } else if key == "LENGTH" {
                                    if let Some(num) = parse_size_or_hex(val_str) {
                                        length = num;
                                    }
                                }
                            }
                        }

                        if length > 0 {
                            regions.push(MemoryRegion {
                                name: reg_name,
                                origin,
                                length,
                            });
                        }
                    }
                }
            }
        }

        Self {
            path: path.to_string(),
            regions,
        }
    }

    pub fn flash_region(&self) -> Option<&MemoryRegion> {
        self.regions
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case("FLASH") || r.name.eq_ignore_ascii_case("ROM"))
    }

    pub fn ram_region(&self) -> Option<&MemoryRegion> {
        self.regions
            .iter()
            .find(|r| r.name.eq_ignore_ascii_case("RAM") || r.name.eq_ignore_ascii_case("SRAM"))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IocProject {
    pub ioc_path: String,
    pub mcu_name: Option<String>,
    pub board_name: Option<String>,
    pub cubemx_version: Option<String>,
    pub family: Option<Stm32Family>,
    pub has_freertos: bool,
    pub has_hal: bool,
    pub has_ll: bool,
    pub toolchain_target: Option<String>,
    pub properties: HashMap<String, String>,
}

impl IocProject {
    pub fn parse(path: &str, content: &str) -> Self {
        let mut props = HashMap::new();
        let mut mcu_name = None;
        let mut board_name = None;
        let mut cubemx_version = None;
        let mut has_freertos = false;
        let mut has_hal = false;
        let mut has_ll = false;
        let mut toolchain_target = None;

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if let Some((k, v)) = trimmed.split_once('=') {
                let key = k.trim().to_string();
                let value = v.trim().to_string();

                if key == "Mcu.UserName" || key == "Mcu.Name" {
                    mcu_name = Some(value.clone());
                } else if key == "Mcu.Family" {
                    // e.g. STM32F4
                } else if key == "Board" || key == "Mcu.Board" {
                    board_name = Some(value.clone());
                } else if key == "MxCube.Version" {
                    cubemx_version = Some(value.clone());
                } else if key == "ProjectManager.TargetToolchain" {
                    toolchain_target = Some(value.clone());
                }

                if key.contains("FREERTOS") || value.contains("FREERTOS") {
                    has_freertos = true;
                }
                if value.contains("HAL") {
                    has_hal = true;
                }
                if value.contains("LL") {
                    has_ll = true;
                }

                props.insert(key, value);
            }
        }

        let family = mcu_name.as_deref().and_then(infer_family);

        Self {
            ioc_path: path.to_string(),
            mcu_name,
            board_name,
            cubemx_version,
            family,
            has_freertos,
            has_hal,
            has_ll,
            toolchain_target,
            properties: props,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stm32Project {
    pub root_path: String,
    pub ioc: Option<IocProject>,
    pub build_system: BuildSystem,
    pub linker_script: Option<LinkerScript>,
    pub elf_artifacts: Vec<String>,
    pub mcu: Option<String>,
    pub family: Option<Stm32Family>,
    pub has_freertos: bool,
    pub has_cmsis: bool,
    pub has_hal: bool,
    pub has_ll: bool,
}

impl Stm32Project {
    pub fn new(root_path: String) -> Self {
        Self {
            root_path,
            ioc: None,
            build_system: BuildSystem::Unknown,
            linker_script: None,
            elf_artifacts: Vec::new(),
            mcu: None,
            family: None,
            has_freertos: false,
            has_cmsis: true,
            has_hal: true,
            has_ll: false,
        }
    }

    pub fn summary_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# STM32 Project Information\n\n");
        out.push_str(&format!("* **Root Path:** `{}`\n", self.root_path));

        if let Some(ioc) = &self.ioc {
            out.push_str(&format!("* **CubeMX .ioc:** `{}`\n", ioc.ioc_path));
            if let Some(mcu) = &ioc.mcu_name {
                out.push_str(&format!("* **Target MCU:** `{}`\n", mcu));
            }
            if let Some(board) = &ioc.board_name {
                out.push_str(&format!("* **Target Board:** `{}`\n", board));
            }
            if let Some(tc) = &ioc.toolchain_target {
                out.push_str(&format!("* **CubeMX Target Toolchain:** `{}`\n", tc));
            }
            if let Some(v) = &ioc.cubemx_version {
                out.push_str(&format!("* **CubeMX Version:** `{}`\n", v));
            }
        } else if let Some(mcu) = &self.mcu {
            out.push_str(&format!("* **Target MCU:** `{}`\n", mcu));
        }

        if let Some(fam) = self.family {
            out.push_str(&format!("* **Family:** {}\n", fam.name()));
        }

        out.push_str(&format!(
            "* **Build System:** {}\n",
            self.build_system.name()
        ));
        out.push_str(&format!(
            "* **FreeRTOS Detected:** {}\n",
            if self.has_freertos { "Yes" } else { "No" }
        ));

        if let Some(ld) = &self.linker_script {
            out.push_str(&format!("* **Linker Script:** `{}`\n", ld.path));
            out.push_str("  - **Configured Memory Regions:**\n");
            for r in &ld.regions {
                out.push_str(&format!(
                    "    - **{}:** Origin `0x{:08X}`, Length `{}`\n",
                    r.name,
                    r.origin,
                    format_bytes(r.length)
                ));
            }
        }

        if !self.elf_artifacts.is_empty() {
            out.push_str("* **Firmware Artifacts (ELF):**\n");
            for elf in &self.elf_artifacts {
                out.push_str(&format!("  - `{}`\n", elf));
            }
        }

        out
    }
}

/// Project creation template configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplateConfig {
    pub name: String,
    pub mcu: String,
    pub build_system: BuildSystem,
    pub use_freertos: bool,
}

/// Scaffold generation result containing files to write
pub struct GeneratedFile {
    pub relative_path: String,
    pub content: String,
}

/// Generates a starter STM32 C project with Linker Script, Startup code, Makefile / CMakeLists.txt,
/// and `.zed/tasks.json`.
pub fn generate_project_scaffold(
    config: &ProjectTemplateConfig,
) -> Result<Vec<GeneratedFile>, String> {
    let dev = find_device(&config.mcu).ok_or_else(|| {
        format!(
            "Unknown STM32 MCU '{}'. Choose from supported devices.",
            config.mcu
        )
    })?;

    let flash_kb = dev.flash_bytes / 1024;
    let ram_kb = dev.ram_bytes / 1024;
    let core_flags = dev.core.arch_flag();

    let mut files = Vec::new();

    // 1. Linker script (linker.ld)
    let linker_content = format!(
        r#"/* Generated Linker Script for {part_number} */
ENTRY(Reset_Handler)

_estack = ORIGIN(RAM) + LENGTH(RAM); /* End of RAM stack */

_Min_Heap_Size = 0x200;  /* Required heap */
_Min_Stack_Size = 0x400; /* Required stack */

MEMORY
{{
  RAM    (xrw) : ORIGIN = 0x20000000, LENGTH = {ram_kb}K
  FLASH  (rx)  : ORIGIN = 0x08000000, LENGTH = {flash_kb}K
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
    *(.rodata)
    *(.rodata*)
    . = ALIGN(4);
    _etext = .;
  }} >FLASH

  _sidata = LOADADDR(.data);

  .data :
  {{
    . = ALIGN(4);
    _sdata = .;
    *(.data)
    *(.data*)
    . = ALIGN(4);
    _edata = .;
  }} >RAM AT> FLASH

  .bss :
  {{
    . = ALIGN(4);
    _sbss = .;
    __bss_start__ = _sbss;
    *(.bss)
    *(.bss*)
    *(COMMON)
    . = ALIGN(4);
    _ebss = .;
    __bss_end__ = _ebss;
  }} >RAM

  ._user_heap_stack :
  {{
    . = ALIGN(8);
    PROVIDE(end = .);
    PROVIDE(_end = .);
    . = . + _Min_Heap_Size;
    . = . + _Min_Stack_Size;
    . = ALIGN(8);
  }} >RAM
}}
"#,
        part_number = dev.part_number,
        ram_kb = ram_kb,
        flash_kb = flash_kb,
    );
    files.push(GeneratedFile {
        relative_path: "linker.ld".to_string(),
        content: linker_content,
    });

    // 2. main.c
    let main_content = format!(
        r#"/**
 * @file main.c
 * @brief STM32 Application Entry Point for {part_number}
 *
 * Generated by Zed STM32 Tools
 */

#include <stdint.h>

/* Register addresses for {family} */
#define RCC_BASE      0x40023800UL

void SystemInit(void) {{
    /* Configure system clocks */
}}

int main(void) {{
    SystemInit();

    /* USER CODE BEGIN 1 */
    volatile uint32_t counter = 0;
    /* USER CODE END 1 */

    while (1) {{
        /* USER CODE BEGIN WHILE */
        counter++;
        /* USER CODE END WHILE */
    }}

    return 0;
}}
"#,
        part_number = dev.part_number,
        family = dev.family.name(),
    );
    files.push(GeneratedFile {
        relative_path: "src/main.c".to_string(),
        content: main_content,
    });

    // 3. Startup code (startup.c)
    let startup_content = format!(
        r#"/**
 * @file startup.c
 * @brief Minimal Reset and Exception Vectors for {part_number} ({core_name})
 */

#include <stdint.h>

extern uint32_t _estack;
extern uint32_t _sidata;
extern uint32_t _sdata;
extern uint32_t _edata;
extern uint32_t _sbss;
extern uint32_t _ebss;

extern int main(void);
extern void SystemInit(void);

void Reset_Handler(void) {{
    uint32_t *src = &_sidata;
    uint32_t *dst = &_sdata;

    while (dst < &_edata) {{
        *dst++ = *src++;
    }}

    dst = &_sbss;
    while (dst < &_ebss) {{
        *dst++ = 0;
    }}

    SystemInit();
    main();

    while (1) {{}}
}}

void Default_Handler(void) {{
    while (1) {{}}
}}

__attribute__((weak, alias("Default_Handler"))) void NMI_Handler(void);
__attribute__((weak, alias("Default_Handler"))) void HardFault_Handler(void);

__attribute__((section(".isr_vector"), used))
const uint32_t g_pfnVectors[] = {{
    (uint32_t)&_estack,
    (uint32_t)&Reset_Handler,
    (uint32_t)&NMI_Handler,
    (uint32_t)&HardFault_Handler,
}};
"#,
        part_number = dev.part_number,
        core_name = dev.core.name(),
    );
    files.push(GeneratedFile {
        relative_path: "src/startup.c".to_string(),
        content: startup_content,
    });

    // 4. Build system file (Makefile or CMakeLists.txt)
    match config.build_system {
        BuildSystem::CMake => {
            let cmake_content = format!(
                r#"cmake_minimum_required(VERSION 3.20)
set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR arm)

set(CMAKE_C_COMPILER arm-none-eabi-gcc)
set(CMAKE_ASM_COMPILER arm-none-eabi-gcc)
set(CMAKE_OBJCOPY arm-none-eabi-objcopy)
set(CMAKE_SIZE arm-none-eabi-size)

project({name} C ASM)

set(MCU_FLAGS "{core_flags}")
set(CMAKE_C_FLAGS "${{CMAKE_C_FLAGS}} ${{MCU_FLAGS}} -Wall -Wextra -Og -g3 -fdata-sections -ffunction-sections")
set(CMAKE_EXE_LINKER_FLAGS "${{CMAKE_EXE_LINKER_FLAGS}} ${{MCU_FLAGS}} -T${{CMAKE_SOURCE_DIR}}/linker.ld -Wl,-Map=${{CMAKE_BINARY_DIR}}/${{PROJECT_NAME}}.map,--gc-sections")

add_executable(${{PROJECT_NAME}}.elf
    src/main.c
    src/startup.c
)

add_custom_command(TARGET ${{PROJECT_NAME}}.elf POST_BUILD
    COMMAND ${{CMAKE_OBJCOPY}} -O binary ${{PROJECT_NAME}}.elf ${{PROJECT_NAME}}.bin
    COMMAND ${{CMAKE_OBJCOPY}} -O ihex ${{PROJECT_NAME}}.elf ${{PROJECT_NAME}}.hex
    COMMAND ${{CMAKE_SIZE}} ${{PROJECT_NAME}}.elf
)
"#,
                name = config.name,
                core_flags = core_flags,
            );
            files.push(GeneratedFile {
                relative_path: "CMakeLists.txt".to_string(),
                content: cmake_content,
            });
        }
        _ => {
            let makefile_content = format!(
                r#"# Makefile for {name} ({part_number})
TARGET = build/{name}
CC = arm-none-eabi-gcc
CP = arm-none-eabi-objcopy
SZ = arm-none-eabi-size

CPU = {core_flags}
CFLAGS = $(CPU) -Wall -Wextra -Og -g3 -fdata-sections -ffunction-sections -Iinclude -Isrc
LDFLAGS = $(CPU) -Tlinker.ld -Wl,-Map=$(TARGET).map,--gc-sections

SRCS = src/main.c src/startup.c
OBJS = $(SRCS:.c=.o)

all: $(TARGET).elf $(TARGET).bin $(TARGET).hex
	$(SZ) $(TARGET).elf

$(TARGET).elf: $(OBJS) linker.ld
	@mkdir -p build
	$(CC) $(OBJS) $(LDFLAGS) -o $@

%.o: %.c
	$(CC) -c $(CFLAGS) $< -o $@

$(TARGET).bin: $(TARGET).elf
	$(CP) -O binary $< $@

$(TARGET).hex: $(TARGET).elf
	$(CP) -O ihex $< $@

clean:
	rm -rf build src/*.o

.PHONY: all clean
"#,
                name = config.name,
                part_number = dev.part_number,
                core_flags = core_flags,
            );
            files.push(GeneratedFile {
                relative_path: "Makefile".to_string(),
                content: makefile_content,
            });
        }
    }

    // 5. Tasks configuration (.zed/tasks.json)
    let build_cmd = config.build_system.default_build_command();
    let clean_cmd = config.build_system.default_clean_command();
    let tasks_json = format!(
        r#"[
  {{
    "label": "STM32: Build",
    "command": "{build_cmd}",
    "cwd": "$ZED_WORKTREE_ROOT",
    "use_new_terminal": false,
    "allow_concurrent_runs": false
  }},
  {{
    "label": "STM32: Clean",
    "command": "{clean_cmd}",
    "cwd": "$ZED_WORKTREE_ROOT",
    "use_new_terminal": false
  }},
  {{
    "label": "STM32: Flash (CubeProgrammer)",
    "command": "STM32_Programmer_CLI -c port=SWD -w build/{name}.bin 0x08000000 -v -rst",
    "cwd": "$ZED_WORKTREE_ROOT",
    "use_new_terminal": false
  }},
  {{
    "label": "STM32: Flash (probe-rs)",
    "command": "probe-rs run --chip {probe_chip} build/{name}.elf",
    "cwd": "$ZED_WORKTREE_ROOT",
    "use_new_terminal": false
  }}
]
"#,
        build_cmd = build_cmd,
        clean_cmd = clean_cmd,
        name = config.name,
        probe_chip = dev.probe_rs_target,
    );
    files.push(GeneratedFile {
        relative_path: ".zed/tasks.json".to_string(),
        content: tasks_json,
    });

    // 6. Debug configuration (.zed/debug.json)
    let debug_json = format!(
        r#"[
  {{
    "label": "STM32: Debug (probe-rs)",
    "adapter": "probe-rs",
    "request": "launch",
    "program": "$ZED_WORKTREE_ROOT/build/{name}.elf",
    "chip": "{probe_chip}",
    "cwd": "$ZED_WORKTREE_ROOT"
  }}
]
"#,
        name = config.name,
        probe_chip = dev.probe_rs_target,
    );
    files.push(GeneratedFile {
        relative_path: ".zed/debug.json".to_string(),
        content: debug_json,
    });

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_linker_script() {
        let ld_text = r#"
        MEMORY
        {
          RAM    (xrw) : ORIGIN = 0x20000000, LENGTH = 128K
          FLASH  (rx)  : ORIGIN = 0x08000000, LENGTH = 512K
          CCMRAM (rw)  : ORIGIN = 0x10000000, LENGTH = 64KB
        }
        "#;

        let script = LinkerScript::parse("STM32F401RETx_FLASH.ld", ld_text);
        assert_eq!(script.regions.len(), 3);

        let flash = script.flash_region().expect("flash region exists");
        assert_eq!(flash.origin, 0x08000000);
        assert_eq!(flash.length, 512 * 1024);

        let ram = script.ram_region().expect("ram region exists");
        assert_eq!(ram.origin, 0x20000000);
        assert_eq!(ram.length, 128 * 1024);
    }

    #[test]
    fn test_parse_ioc_project() {
        let ioc_text = r#"
        #STM32CubeMX project file
        Mcu.Family=STM32F4
        Mcu.Name=STM32F401RETx
        Mcu.Package=LQFP64
        Mcu.UserName=STM32F401RETx
        MxCube.Version=6.12.0
        ProjectManager.TargetToolchain=Makefile
        ProjectManager.CustomerFirmwarePackage=STM32Cube_FW_F4_V1.28.0
        "#;

        let ioc = IocProject::parse("test.ioc", ioc_text);
        assert_eq!(ioc.mcu_name, Some("STM32F401RETx".to_string()));
        assert_eq!(ioc.family, Some(Stm32Family::F4));
        assert_eq!(ioc.toolchain_target, Some("Makefile".to_string()));
    }

    #[test]
    fn test_generate_project_scaffold() {
        let config = ProjectTemplateConfig {
            name: "Blinky".to_string(),
            mcu: "STM32F401RE".to_string(),
            build_system: BuildSystem::Make,
            use_freertos: false,
        };

        let files = generate_project_scaffold(&config).expect("scaffold generation should succeed");
        assert!(files.iter().any(|f| f.relative_path == "linker.ld"));
        assert!(files.iter().any(|f| f.relative_path == "src/main.c"));
        assert!(files.iter().any(|f| f.relative_path == "Makefile"));
        assert!(files.iter().any(|f| f.relative_path == ".zed/tasks.json"));
    }
}
