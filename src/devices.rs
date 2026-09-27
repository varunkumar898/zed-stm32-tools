//! STM32 Device Database and Hardware Abstraction.
//!
//! Provides accurate device specifications, memory boundaries, ARM cores,
//! OpenOCD target scripts, probe-rs targets, CMSIS definitions, and peripheral inventories.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stm32Family {
    F0,
    F1,
    F2,
    F3,
    F4,
    F7,
    G0,
    G4,
    H5,
    H7,
    L0,
    L1,
    L4,
    L5,
    U5,
    WB,
    WL,
}

impl Stm32Family {
    pub fn name(&self) -> &'static str {
        match self {
            Self::F0 => "STM32F0",
            Self::F1 => "STM32F1",
            Self::F2 => "STM32F2",
            Self::F3 => "STM32F3",
            Self::F4 => "STM32F4",
            Self::F7 => "STM32F7",
            Self::G0 => "STM32G0",
            Self::G4 => "STM32G4",
            Self::H5 => "STM32H5",
            Self::H7 => "STM32H7",
            Self::L0 => "STM32L0",
            Self::L1 => "STM32L1",
            Self::L4 => "STM32L4",
            Self::L5 => "STM32L5",
            Self::U5 => "STM32U5",
            Self::WB => "STM32WB",
            Self::WL => "STM32WL",
        }
    }

    pub fn default_openocd_target(&self) -> &'static str {
        match self {
            Self::F0 => "target/stm32f0x.cfg",
            Self::F1 => "target/stm32f1x.cfg",
            Self::F2 => "target/stm32f2x.cfg",
            Self::F3 => "target/stm32f3x.cfg",
            Self::F4 => "target/stm32f4x.cfg",
            Self::F7 => "target/stm32f7x.cfg",
            Self::G0 => "target/stm32g0x.cfg",
            Self::G4 => "target/stm32g4x.cfg",
            Self::H5 => "target/stm32h5x.cfg",
            Self::H7 => "target/stm32h7x.cfg",
            Self::L0 => "target/stm32l0.cfg",
            Self::L1 => "target/stm32l1.cfg",
            Self::L4 => "target/stm32l4x.cfg",
            Self::L5 => "target/stm32l5x.cfg",
            Self::U5 => "target/stm32u5x.cfg",
            Self::WB => "target/stm32wbx.cfg",
            Self::WL => "target/stm32wlx.cfg",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArmCore {
    CortexM0,
    CortexM0Plus,
    CortexM3,
    CortexM4F,
    CortexM7,
    CortexM33,
}

impl ArmCore {
    pub fn arch_flag(&self) -> &'static str {
        match self {
            Self::CortexM0 => "-mcpu=cortex-m0 -mthumb",
            Self::CortexM0Plus => "-mcpu=cortex-m0plus -mthumb",
            Self::CortexM3 => "-mcpu=cortex-m3 -mthumb",
            Self::CortexM4F => "-mcpu=cortex-m4 -mthumb -mfpu=fpv4-sp-d16 -mfloat-abi=hard",
            Self::CortexM7 => "-mcpu=cortex-m7 -mthumb -mfpu=fpv5-d16 -mfloat-abi=hard",
            Self::CortexM33 => "-mcpu=cortex-m33 -mthumb -mfpu=fpv5-sp-d16 -mfloat-abi=hard",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::CortexM0 => "Arm Cortex-M0",
            Self::CortexM0Plus => "Arm Cortex-M0+",
            Self::CortexM3 => "Arm Cortex-M3",
            Self::CortexM4F => "Arm Cortex-M4 with FPU",
            Self::CortexM7 => "Arm Cortex-M7 with DP-FPU",
            Self::CortexM33 => "Arm Cortex-M33 with FPU",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Stm32Device {
    pub part_number: &'static str,
    pub family: Stm32Family,
    pub series: &'static str,
    pub core: ArmCore,
    pub flash_bytes: u64,
    pub ram_bytes: u64,
    pub max_frequency_mhz: u32,
    pub package: &'static str,
    pub cmsis_device_name: &'static str,
    pub openocd_target: &'static str,
    pub probe_rs_target: &'static str,
    pub svd_filename: &'static str,
    pub peripherals: &'static [&'static str],
}

pub static STM32_DEVICES: &[Stm32Device] = &[
    // --- STM32F0 ---
    Stm32Device {
        part_number: "STM32F030R8",
        family: Stm32Family::F0,
        series: "STM32F0x0",
        core: ArmCore::CortexM0,
        flash_bytes: 64 * 1024,
        ram_bytes: 8 * 1024,
        max_frequency_mhz: 48,
        package: "LQFP64",
        cmsis_device_name: "STM32F030x8",
        openocd_target: "target/stm32f0x.cfg",
        probe_rs_target: "STM32F030R8Tx",
        svd_filename: "STM32F030.svd",
        peripherals: &[
            "USART1", "USART2", "SPI1", "SPI2", "I2C1", "I2C2", "ADC1", "TIM1", "TIM3", "TIM14",
        ],
    },
    Stm32Device {
        part_number: "STM32F072RB",
        family: Stm32Family::F0,
        series: "STM32F0x2",
        core: ArmCore::CortexM0,
        flash_bytes: 128 * 1024,
        ram_bytes: 16 * 1024,
        max_frequency_mhz: 48,
        package: "LQFP64",
        cmsis_device_name: "STM32F072xB",
        openocd_target: "target/stm32f0x.cfg",
        probe_rs_target: "STM32F072RBTx",
        svd_filename: "STM32F072.svd",
        peripherals: &[
            "USART1", "USART2", "USART3", "USART4", "SPI1", "SPI2", "I2C1", "I2C2", "CAN", "USB",
            "ADC1", "DAC1",
        ],
    },
    // --- STM32F1 ---
    Stm32Device {
        part_number: "STM32F103C8",
        family: Stm32Family::F1,
        series: "STM32F103",
        core: ArmCore::CortexM3,
        flash_bytes: 64 * 1024,
        ram_bytes: 20 * 1024,
        max_frequency_mhz: 72,
        package: "LQFP48",
        cmsis_device_name: "STM32F103xB",
        openocd_target: "target/stm32f1x.cfg",
        probe_rs_target: "STM32F103C8",
        svd_filename: "STM32F103.svd",
        peripherals: &[
            "USART1", "USART2", "USART3", "SPI1", "SPI2", "I2C1", "I2C2", "USB", "CAN", "ADC1",
            "ADC2", "TIM1", "TIM2", "TIM3", "TIM4",
        ],
    },
    Stm32Device {
        part_number: "STM32F103RB",
        family: Stm32Family::F1,
        series: "STM32F103",
        core: ArmCore::CortexM3,
        flash_bytes: 128 * 1024,
        ram_bytes: 20 * 1024,
        max_frequency_mhz: 72,
        package: "LQFP64",
        cmsis_device_name: "STM32F103xB",
        openocd_target: "target/stm32f1x.cfg",
        probe_rs_target: "STM32F103RBTx",
        svd_filename: "STM32F103.svd",
        peripherals: &[
            "USART1", "USART2", "USART3", "SPI1", "SPI2", "I2C1", "I2C2", "USB", "CAN", "ADC1",
            "ADC2",
        ],
    },
    // --- STM32F2 ---
    Stm32Device {
        part_number: "STM32F207ZG",
        family: Stm32Family::F2,
        series: "STM32F207",
        core: ArmCore::CortexM3,
        flash_bytes: 1024 * 1024,
        ram_bytes: 128 * 1024,
        max_frequency_mhz: 120,
        package: "LQFP144",
        cmsis_device_name: "STM32F207xx",
        openocd_target: "target/stm32f2x.cfg",
        probe_rs_target: "STM32F207ZGTx",
        svd_filename: "STM32F207.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "USART3",
            "UART4",
            "UART5",
            "USART6",
            "SPI1",
            "SPI2",
            "SPI3",
            "I2C1",
            "I2C2",
            "I2C3",
            "ETH",
            "USB_OTG_FS",
            "USB_OTG_HS",
        ],
    },
    // --- STM32F3 ---
    Stm32Device {
        part_number: "STM32F303RE",
        family: Stm32Family::F3,
        series: "STM32F303",
        core: ArmCore::CortexM4F,
        flash_bytes: 512 * 1024,
        ram_bytes: 64 * 1024,
        max_frequency_mhz: 72,
        package: "LQFP64",
        cmsis_device_name: "STM32F303xE",
        openocd_target: "target/stm32f3x.cfg",
        probe_rs_target: "STM32F303RETx",
        svd_filename: "STM32F303E.svd",
        peripherals: &[
            "USART1", "USART2", "USART3", "UART4", "UART5", "SPI1", "SPI2", "SPI3", "I2C1", "I2C2",
            "CAN", "USB", "ADC1", "ADC2", "ADC3", "ADC4", "DAC1", "DAC2",
        ],
    },
    // --- STM32F4 ---
    Stm32Device {
        part_number: "STM32F401RE",
        family: Stm32Family::F4,
        series: "STM32F401",
        core: ArmCore::CortexM4F,
        flash_bytes: 512 * 1024,
        ram_bytes: 96 * 1024,
        max_frequency_mhz: 84,
        package: "LQFP64",
        cmsis_device_name: "STM32F401xE",
        openocd_target: "target/stm32f4x.cfg",
        probe_rs_target: "STM32F401RETx",
        svd_filename: "STM32F401.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "USART6",
            "SPI1",
            "SPI2",
            "SPI3",
            "I2C1",
            "I2C2",
            "I2C3",
            "USB_OTG_FS",
            "ADC1",
            "TIM1",
            "TIM2",
            "TIM3",
            "TIM4",
            "TIM5",
            "TIM9",
            "TIM10",
            "TIM11",
        ],
    },
    Stm32Device {
        part_number: "STM32F411CE",
        family: Stm32Family::F4,
        series: "STM32F411",
        core: ArmCore::CortexM4F,
        flash_bytes: 512 * 1024,
        ram_bytes: 128 * 1024,
        max_frequency_mhz: 100,
        package: "UFQFPN48",
        cmsis_device_name: "STM32F411xE",
        openocd_target: "target/stm32f4x.cfg",
        probe_rs_target: "STM32F411CEUx",
        svd_filename: "STM32F411.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "USART6",
            "SPI1",
            "SPI2",
            "SPI3",
            "SPI4",
            "SPI5",
            "I2C1",
            "I2C2",
            "I2C3",
            "USB_OTG_FS",
            "ADC1",
        ],
    },
    Stm32Device {
        part_number: "STM32F446RE",
        family: Stm32Family::F4,
        series: "STM32F446",
        core: ArmCore::CortexM4F,
        flash_bytes: 512 * 1024,
        ram_bytes: 128 * 1024,
        max_frequency_mhz: 180,
        package: "LQFP64",
        cmsis_device_name: "STM32F446xx",
        openocd_target: "target/stm32f4x.cfg",
        probe_rs_target: "STM32F446RETx",
        svd_filename: "STM32F446.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "USART3",
            "UART4",
            "UART5",
            "USART6",
            "SPI1",
            "SPI2",
            "SPI3",
            "SPI4",
            "I2C1",
            "I2C2",
            "I2C3",
            "CAN1",
            "CAN2",
            "USB_OTG_FS",
            "USB_OTG_HS",
            "ADC1",
            "ADC2",
            "ADC3",
            "DAC1",
            "DAC2",
        ],
    },
    // --- STM32F7 ---
    Stm32Device {
        part_number: "STM32F746NG",
        family: Stm32Family::F7,
        series: "STM32F746",
        core: ArmCore::CortexM7,
        flash_bytes: 1024 * 1024,
        ram_bytes: 320 * 1024,
        max_frequency_mhz: 216,
        package: "TFBGA216",
        cmsis_device_name: "STM32F746xx",
        openocd_target: "target/stm32f7x.cfg",
        probe_rs_target: "STM32F746NGHx",
        svd_filename: "STM32F746.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "USART3",
            "UART4",
            "UART5",
            "USART6",
            "UART7",
            "UART8",
            "SPI1",
            "SPI2",
            "SPI3",
            "SPI4",
            "SPI5",
            "SPI6",
            "ETH",
            "LTDC",
            "FMC",
            "QUADSPI",
            "USB_OTG_FS",
            "USB_OTG_HS",
        ],
    },
    Stm32Device {
        part_number: "STM32F767ZI",
        family: Stm32Family::F7,
        series: "STM32F767",
        core: ArmCore::CortexM7,
        flash_bytes: 2048 * 1024,
        ram_bytes: 512 * 1024,
        max_frequency_mhz: 216,
        package: "LQFP144",
        cmsis_device_name: "STM32F767xx",
        openocd_target: "target/stm32f7x.cfg",
        probe_rs_target: "STM32F767ZITx",
        svd_filename: "STM32F767.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "USART3",
            "UART4",
            "UART5",
            "USART6",
            "ETH",
            "CAN1",
            "CAN2",
            "CAN3",
            "USB_OTG_FS",
            "USB_OTG_HS",
            "ADC1",
            "ADC2",
            "ADC3",
        ],
    },
    // --- STM32G0 ---
    Stm32Device {
        part_number: "STM32G071RB",
        family: Stm32Family::G0,
        series: "STM32G071",
        core: ArmCore::CortexM0Plus,
        flash_bytes: 128 * 1024,
        ram_bytes: 36 * 1024,
        max_frequency_mhz: 64,
        package: "LQFP64",
        cmsis_device_name: "STM32G071xx",
        openocd_target: "target/stm32g0x.cfg",
        probe_rs_target: "STM32G071RBTx",
        svd_filename: "STM32G071.svd",
        peripherals: &[
            "USART1", "USART2", "USART3", "USART4", "LPUART1", "SPI1", "SPI2", "I2C1", "I2C2",
            "USB_PD", "ADC1", "DAC1",
        ],
    },
    // --- STM32G4 ---
    Stm32Device {
        part_number: "STM32G431RB",
        family: Stm32Family::G4,
        series: "STM32G431",
        core: ArmCore::CortexM4F,
        flash_bytes: 128 * 1024,
        ram_bytes: 32 * 1024,
        max_frequency_mhz: 170,
        package: "LQFP64",
        cmsis_device_name: "STM32G431xx",
        openocd_target: "target/stm32g4x.cfg",
        probe_rs_target: "STM32G431RBTx",
        svd_filename: "STM32G431.svd",
        peripherals: &[
            "USART1", "USART2", "USART3", "LPUART1", "SPI1", "SPI2", "SPI3", "I2C1", "I2C2",
            "I2C3", "FDCAN1", "USB", "ADC1", "ADC2", "DAC1", "DAC3", "COMP1", "COMP2", "OPAMP1",
            "OPAMP2",
        ],
    },
    Stm32Device {
        part_number: "STM32G474RE",
        family: Stm32Family::G4,
        series: "STM32G474",
        core: ArmCore::CortexM4F,
        flash_bytes: 512 * 1024,
        ram_bytes: 128 * 1024,
        max_frequency_mhz: 170,
        package: "LQFP64",
        cmsis_device_name: "STM32G474xx",
        openocd_target: "target/stm32g4x.cfg",
        probe_rs_target: "STM32G474RETx",
        svd_filename: "STM32G474.svd",
        peripherals: &[
            "USART1", "USART2", "USART3", "UART4", "UART5", "LPUART1", "SPI1", "SPI2", "SPI3",
            "I2C1", "I2C2", "I2C3", "I2C4", "FDCAN1", "FDCAN2", "FDCAN3", "HRTIM1", "ADC1", "ADC2",
            "ADC3", "ADC4", "ADC5",
        ],
    },
    // --- STM32H5 ---
    Stm32Device {
        part_number: "STM32H563ZI",
        family: Stm32Family::H5,
        series: "STM32H563",
        core: ArmCore::CortexM33,
        flash_bytes: 2048 * 1024,
        ram_bytes: 640 * 1024,
        max_frequency_mhz: 250,
        package: "LQFP144",
        cmsis_device_name: "STM32H563xx",
        openocd_target: "target/stm32h5x.cfg",
        probe_rs_target: "STM32H563ZITx",
        svd_filename: "STM32H563.svd",
        peripherals: &[
            "USART1", "USART2", "USART3", "UART4", "UART5", "USART6", "UART7", "UART8", "UART9",
            "UART12", "LPUART1", "SPI1", "SPI2", "SPI3", "SPI4", "SPI5", "SPI6", "ETH", "FDCAN1",
            "FDCAN2", "USB_FS",
        ],
    },
    // --- STM32H7 ---
    Stm32Device {
        part_number: "STM32H743ZI",
        family: Stm32Family::H7,
        series: "STM32H743",
        core: ArmCore::CortexM7,
        flash_bytes: 2048 * 1024,
        ram_bytes: 1024 * 1024,
        max_frequency_mhz: 480,
        package: "LQFP144",
        cmsis_device_name: "STM32H743xx",
        openocd_target: "target/stm32h7x.cfg",
        probe_rs_target: "STM32H743ZITx",
        svd_filename: "STM32H743.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "USART3",
            "UART4",
            "UART5",
            "USART6",
            "UART7",
            "UART8",
            "SPI1",
            "SPI2",
            "SPI3",
            "SPI4",
            "SPI5",
            "SPI6",
            "ETH",
            "FDCAN1",
            "FDCAN2",
            "USB_OTG_FS",
            "USB_OTG_HS",
            "FMC",
            "QUADSPI",
        ],
    },
    Stm32Device {
        part_number: "STM32H750VB",
        family: Stm32Family::H7,
        series: "STM32H750",
        core: ArmCore::CortexM7,
        flash_bytes: 128 * 1024,
        ram_bytes: 1024 * 1024,
        max_frequency_mhz: 480,
        package: "LQFP100",
        cmsis_device_name: "STM32H750xx",
        openocd_target: "target/stm32h7x.cfg",
        probe_rs_target: "STM32H750VBTx",
        svd_filename: "STM32H750.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "USART3",
            "UART4",
            "UART5",
            "USART6",
            "UART7",
            "UART8",
            "SPI1",
            "SPI2",
            "SPI3",
            "SPI4",
            "SPI5",
            "SPI6",
            "QUADSPI",
            "FMC",
            "USB_OTG_FS",
            "USB_OTG_HS",
        ],
    },
    // --- STM32L0 ---
    Stm32Device {
        part_number: "STM32L053R8",
        family: Stm32Family::L0,
        series: "STM32L053",
        core: ArmCore::CortexM0Plus,
        flash_bytes: 64 * 1024,
        ram_bytes: 8 * 1024,
        max_frequency_mhz: 32,
        package: "LQFP64",
        cmsis_device_name: "STM32L053xx",
        openocd_target: "target/stm32l0.cfg",
        probe_rs_target: "STM32L053R8Tx",
        svd_filename: "STM32L053.svd",
        peripherals: &[
            "USART1", "USART2", "LPUART1", "SPI1", "SPI2", "I2C1", "I2C2", "USB", "ADC1", "DAC1",
            "LCD",
        ],
    },
    // --- STM32L1 ---
    Stm32Device {
        part_number: "STM32L152RE",
        family: Stm32Family::L1,
        series: "STM32L152",
        core: ArmCore::CortexM3,
        flash_bytes: 512 * 1024,
        ram_bytes: 80 * 1024,
        max_frequency_mhz: 32,
        package: "LQFP64",
        cmsis_device_name: "STM32L152xE",
        openocd_target: "target/stm32l1.cfg",
        probe_rs_target: "STM32L152RETx",
        svd_filename: "STM32L152.svd",
        peripherals: &[
            "USART1", "USART2", "USART3", "UART4", "UART5", "SPI1", "SPI2", "SPI3", "I2C1", "I2C2",
            "USB", "ADC1", "DAC1", "DAC2", "LCD",
        ],
    },
    // --- STM32L4 ---
    Stm32Device {
        part_number: "STM32L476RG",
        family: Stm32Family::L4,
        series: "STM32L476",
        core: ArmCore::CortexM4F,
        flash_bytes: 1024 * 1024,
        ram_bytes: 128 * 1024,
        max_frequency_mhz: 80,
        package: "LQFP64",
        cmsis_device_name: "STM32L476xx",
        openocd_target: "target/stm32l4x.cfg",
        probe_rs_target: "STM32L476RGTx",
        svd_filename: "STM32L476.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "USART3",
            "UART4",
            "UART5",
            "LPUART1",
            "SPI1",
            "SPI2",
            "SPI3",
            "I2C1",
            "I2C2",
            "I2C3",
            "CAN1",
            "USB_OTG_FS",
            "ADC1",
            "ADC2",
            "ADC3",
            "DAC1",
            "DAC2",
        ],
    },
    Stm32Device {
        part_number: "STM32L432KC",
        family: Stm32Family::L4,
        series: "STM32L432",
        core: ArmCore::CortexM4F,
        flash_bytes: 256 * 1024,
        ram_bytes: 64 * 1024,
        max_frequency_mhz: 80,
        package: "UFQFPN32",
        cmsis_device_name: "STM32L432xx",
        openocd_target: "target/stm32l4x.cfg",
        probe_rs_target: "STM32L432KCUx",
        svd_filename: "STM32L432.svd",
        peripherals: &[
            "USART1", "USART2", "LPUART1", "SPI1", "I2C1", "I2C3", "CAN1", "USB", "ADC1", "DAC1",
        ],
    },
    // --- STM32L5 ---
    Stm32Device {
        part_number: "STM32L552ZE",
        family: Stm32Family::L5,
        series: "STM32L552",
        core: ArmCore::CortexM33,
        flash_bytes: 512 * 1024,
        ram_bytes: 256 * 1024,
        max_frequency_mhz: 110,
        package: "LQFP144",
        cmsis_device_name: "STM32L552xx",
        openocd_target: "target/stm32l5x.cfg",
        probe_rs_target: "STM32L552ZETx",
        svd_filename: "STM32L552.svd",
        peripherals: &[
            "USART1", "USART2", "USART3", "UART4", "UART5", "LPUART1", "SPI1", "SPI2", "SPI3",
            "I2C1", "I2C2", "I2C3", "I2C4", "FDCAN1", "USB_FS", "ADC1", "ADC2",
        ],
    },
    // --- STM32U5 ---
    Stm32Device {
        part_number: "STM32U575ZI",
        family: Stm32Family::U5,
        series: "STM32U575",
        core: ArmCore::CortexM33,
        flash_bytes: 2048 * 1024,
        ram_bytes: 786 * 1024,
        max_frequency_mhz: 160,
        package: "LQFP144",
        cmsis_device_name: "STM32U575xx",
        openocd_target: "target/stm32u5x.cfg",
        probe_rs_target: "STM32U575ZITx",
        svd_filename: "STM32U575.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "USART3",
            "UART4",
            "UART5",
            "LPUART1",
            "SPI1",
            "SPI2",
            "SPI3",
            "I2C1",
            "I2C2",
            "I2C3",
            "I2C4",
            "FDCAN1",
            "USB_OTG_FS",
            "OCTOSPI1",
            "OCTOSPI2",
            "ADC1",
            "ADC4",
        ],
    },
    // --- STM32WB ---
    Stm32Device {
        part_number: "STM32WB55RG",
        family: Stm32Family::WB,
        series: "STM32WB55",
        core: ArmCore::CortexM4F,
        flash_bytes: 1024 * 1024,
        ram_bytes: 256 * 1024,
        max_frequency_mhz: 64,
        package: "VFQFPN68",
        cmsis_device_name: "STM32WB55xx",
        openocd_target: "target/stm32wbx.cfg",
        probe_rs_target: "STM32WB55RGVx",
        svd_filename: "STM32WB55.svd",
        peripherals: &[
            "USART1",
            "LPUART1",
            "SPI1",
            "SPI2",
            "I2C1",
            "I2C3",
            "USB",
            "BLE_RF",
            "802.15.4_RF",
            "ADC1",
        ],
    },
    // --- STM32WL ---
    Stm32Device {
        part_number: "STM32WL55JC",
        family: Stm32Family::WL,
        series: "STM32WL55",
        core: ArmCore::CortexM4F,
        flash_bytes: 256 * 1024,
        ram_bytes: 64 * 1024,
        max_frequency_mhz: 48,
        package: "UFBGA73",
        cmsis_device_name: "STM32WL55xx",
        openocd_target: "target/stm32wlx.cfg",
        probe_rs_target: "STM32WL55JCIx",
        svd_filename: "STM32WL55.svd",
        peripherals: &[
            "USART1",
            "USART2",
            "LPUART1",
            "SPI1",
            "SPI2",
            "I2C1",
            "I2C2",
            "I2C3",
            "SUBGHZ_RADIO",
            "ADC1",
            "DAC1",
        ],
    },
];

/// Infers the STM32 family from a part number or string.
pub fn infer_family(mcu_str: &str) -> Option<Stm32Family> {
    let clean = mcu_str.trim().to_uppercase();
    if !clean.starts_with("STM32") {
        return None;
    }
    let rest = &clean["STM32".len()..];

    if rest.starts_with("F0") {
        Some(Stm32Family::F0)
    } else if rest.starts_with("F1") {
        Some(Stm32Family::F1)
    } else if rest.starts_with("F2") {
        Some(Stm32Family::F2)
    } else if rest.starts_with("F3") {
        Some(Stm32Family::F3)
    } else if rest.starts_with("F4") {
        Some(Stm32Family::F4)
    } else if rest.starts_with("F7") {
        Some(Stm32Family::F7)
    } else if rest.starts_with("G0") {
        Some(Stm32Family::G0)
    } else if rest.starts_with("G4") {
        Some(Stm32Family::G4)
    } else if rest.starts_with("H5") {
        Some(Stm32Family::H5)
    } else if rest.starts_with("H7") {
        Some(Stm32Family::H7)
    } else if rest.starts_with("L0") {
        Some(Stm32Family::L0)
    } else if rest.starts_with("L1") {
        Some(Stm32Family::L1)
    } else if rest.starts_with("L4") {
        Some(Stm32Family::L4)
    } else if rest.starts_with("L5") {
        Some(Stm32Family::L5)
    } else if rest.starts_with("U5") {
        Some(Stm32Family::U5)
    } else if rest.starts_with("WB") {
        Some(Stm32Family::WB)
    } else if rest.starts_with("WL") {
        Some(Stm32Family::WL)
    } else {
        None
    }
}

/// Finds a device by exact part number, or by closest prefix.
pub fn find_device(query: &str) -> Option<&'static Stm32Device> {
    let q = query.trim().to_uppercase();
    // 1. Exact match
    if let Some(dev) = STM32_DEVICES.iter().find(|d| d.part_number == q) {
        return Some(dev);
    }
    // 2. Query starts with part number or part number starts with query
    if let Some(dev) = STM32_DEVICES
        .iter()
        .find(|d| q.starts_with(d.part_number) || d.part_number.starts_with(&q))
    {
        return Some(dev);
    }
    // 3. Series match (e.g. "STM32F401" matches "STM32F401RE")
    if let Some(dev) = STM32_DEVICES.iter().find(|d| q.starts_with(d.series)) {
        return Some(dev);
    }
    None
}

/// Returns all devices belonging to a family.
pub fn find_devices_by_family(family: Stm32Family) -> Vec<&'static Stm32Device> {
    STM32_DEVICES
        .iter()
        .filter(|d| d.family == family)
        .collect()
}

/// Returns the OpenOCD target configuration script for an MCU name.
pub fn openocd_target_for_mcu(mcu: &str) -> &'static str {
    if let Some(dev) = find_device(mcu) {
        dev.openocd_target
    } else if let Some(fam) = infer_family(mcu) {
        fam.default_openocd_target()
    } else {
        "target/stm32f4x.cfg" // Fallback
    }
}

/// Returns the probe-rs target name for an MCU string.
pub fn probe_rs_target_for_mcu(mcu: &str) -> Option<&'static str> {
    find_device(mcu).map(|d| d.probe_rs_target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infer_family() {
        assert_eq!(infer_family("STM32F401RE"), Some(Stm32Family::F4));
        assert_eq!(infer_family("stm32g474re"), Some(Stm32Family::G4));
        assert_eq!(infer_family("STM32H743ZI"), Some(Stm32Family::H7));
        assert_eq!(infer_family("STM32WB55"), Some(Stm32Family::WB));
        assert_eq!(infer_family("UNKNOWN"), None);
    }

    #[test]
    fn test_find_device() {
        let f4 = find_device("STM32F401RE").expect("should find STM32F401RE");
        assert_eq!(f4.family, Stm32Family::F4);
        assert_eq!(f4.flash_bytes, 512 * 1024);
        assert_eq!(f4.ram_bytes, 96 * 1024);
        assert_eq!(f4.core, ArmCore::CortexM4F);

        let g4 = find_device("STM32G474").expect("should find STM32G474RE by prefix");
        assert_eq!(g4.family, Stm32Family::G4);
        assert_eq!(g4.max_frequency_mhz, 170);

        let h7 = find_device("STM32H743ZI").expect("should find STM32H743ZI");
        assert_eq!(h7.core, ArmCore::CortexM7);
        assert_eq!(h7.ram_bytes, 1024 * 1024);
    }

    #[test]
    fn test_openocd_target() {
        assert_eq!(openocd_target_for_mcu("STM32F401RE"), "target/stm32f4x.cfg");
        assert_eq!(openocd_target_for_mcu("STM32G431RB"), "target/stm32g4x.cfg");
        assert_eq!(openocd_target_for_mcu("STM32H743ZI"), "target/stm32h7x.cfg");
    }
}
