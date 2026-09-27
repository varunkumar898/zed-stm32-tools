//! Compiler Diagnostics and Firmware Memory Usage Analysis.
//!
//! Parses GCC / Clang build outputs to extract errors, warnings, and notes.
//! Parses `arm-none-eabi-size` outputs and calculates exact Flash and RAM usage against device limits.

use crate::utils::{calculate_percentage, format_bytes, render_progress_bar};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Note,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerDiagnostic {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub severity: DiagnosticSeverity,
    pub message: String,
}

/// Parses GCC / Clang compiler output for diagnostics.
pub fn parse_gcc_diagnostics(output: &str) -> Vec<CompilerDiagnostic> {
    let re = Regex::new(r#"^([^:\r\n]+):(\d+):(\d+):\s+(error|warning|note):\s+(.*)$"#).unwrap();
    let mut diagnostics = Vec::new();

    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(caps) = re.captures(trimmed) {
            let file = caps.get(1).map_or("", |m| m.as_str()).to_string();
            let line_num = caps
                .get(2)
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(0);
            let col_num = caps
                .get(3)
                .and_then(|m| m.as_str().parse::<u32>().ok())
                .unwrap_or(0);
            let sev_str = caps.get(4).map_or("", |m| m.as_str());
            let msg = caps.get(5).map_or("", |m| m.as_str()).to_string();

            let severity = match sev_str {
                "error" => DiagnosticSeverity::Error,
                "warning" => DiagnosticSeverity::Warning,
                _ => DiagnosticSeverity::Note,
            };

            diagnostics.push(CompilerDiagnostic {
                file,
                line: line_num,
                column: col_num,
                severity,
                message: msg,
            });
        }
    }

    diagnostics
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryUsageReport {
    pub text: u64,
    pub data: u64,
    pub bss: u64,
    pub flash_used: u64,
    pub flash_total: Option<u64>,
    pub flash_percentage: Option<f64>,
    pub ram_used: u64,
    pub ram_total: Option<u64>,
    pub ram_percentage: Option<f64>,
}

impl MemoryUsageReport {
    pub fn format_report(&self) -> String {
        let mut out = String::new();
        out.push_str("=====================================================\n");
        out.push_str("              STM32 Memory Usage Report              \n");
        out.push_str("=====================================================\n");
        out.push_str(&format!(
            "Sections Breakdown:\n  .text (code/constants): {}\n  .data (initialized):    {}\n  .bss  (zero-init):       {}\n\n",
            format_bytes(self.text),
            format_bytes(self.data),
            format_bytes(self.bss)
        ));

        // FLASH
        out.push_str("FLASH (text + data):\n");
        if let Some(total) = self.flash_total {
            let pct = self.flash_percentage.unwrap_or(0.0);
            let bar = render_progress_bar(pct, 20);
            out.push_str(&format!(
                "  Used:      {} / {} (Available: {})\n  Usage:     {}\n\n",
                format_bytes(self.flash_used),
                format_bytes(total),
                format_bytes(total.saturating_sub(self.flash_used)),
                bar
            ));
        } else {
            out.push_str(&format!(
                "  Used:      {}\n\n",
                format_bytes(self.flash_used)
            ));
        }

        // RAM
        out.push_str("RAM (data + bss):\n");
        if let Some(total) = self.ram_total {
            let pct = self.ram_percentage.unwrap_or(0.0);
            let bar = render_progress_bar(pct, 20);
            out.push_str(&format!(
                "  Used:      {} / {} (Available: {})\n  Usage:     {}\n",
                format_bytes(self.ram_used),
                format_bytes(total),
                format_bytes(total.saturating_sub(self.ram_used)),
                bar
            ));
        } else {
            out.push_str(&format!("  Used:      {}\n", format_bytes(self.ram_used)));
        }

        out.push_str("=====================================================\n");
        out
    }
}

/// Parses the output of `arm-none-eabi-size` (both Berkeley and SysV formats).
pub fn parse_arm_size_output(
    output: &str,
    flash_total: Option<u64>,
    ram_total: Option<u64>,
) -> Option<MemoryUsageReport> {
    let mut text = None;
    let mut data = None;
    let mut bss = None;

    // Check Berkeley format: text\tdata\tbss\tdec\thex\tfilename
    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 {
            if parts[0].eq_ignore_ascii_case("text") {
                continue; // Header
            }
            if let (Ok(t), Ok(d), Ok(b)) = (
                parts[0].parse::<u64>(),
                parts[1].parse::<u64>(),
                parts[2].parse::<u64>(),
            ) {
                text = Some(t);
                data = Some(d);
                bss = Some(b);
                break;
            }
        }
    }

    if let (Some(t), Some(d), Some(b)) = (text, data, bss) {
        let flash_used = t + d;
        let ram_used = d + b;

        let flash_percentage = flash_total.map(|tot| calculate_percentage(flash_used, tot));
        let ram_percentage = ram_total.map(|tot| calculate_percentage(ram_used, tot));

        return Some(MemoryUsageReport {
            text: t,
            data: d,
            bss: b,
            flash_used,
            flash_total,
            flash_percentage,
            ram_used,
            ram_total,
            ram_percentage,
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gcc_diagnostics() {
        let output = r#"
src/main.c:42:10: error: 'GPIOA' undeclared (first use in this function)
   42 |   HAL_GPIO_WritePin(GPIOA, GPIO_PIN_5, GPIO_PIN_SET);
      |          ^~~~~
src/main.c:55:18: warning: unused variable 'timeout' [-Wunused-variable]
   55 |   uint32_t timeout = 1000;
      |            ^~~~~~~
"#;

        let diags = parse_gcc_diagnostics(output);
        assert_eq!(diags.len(), 2);
        assert_eq!(diags[0].file, "src/main.c");
        assert_eq!(diags[0].line, 42);
        assert_eq!(diags[0].column, 10);
        assert_eq!(diags[0].severity, DiagnosticSeverity::Error);
        assert!(diags[0].message.contains("'GPIOA' undeclared"));

        assert_eq!(diags[1].severity, DiagnosticSeverity::Warning);
        assert!(diags[1].message.contains("unused variable 'timeout'"));
    }

    #[test]
    fn test_parse_arm_size() {
        let size_output = r#"
   text	   data	    bss	    dec	    hex	filename
  14280	    112	   1572	  15964	   3e5c	build/Blinky.elf
"#;

        let report = parse_arm_size_output(size_output, Some(512 * 1024), Some(128 * 1024))
            .expect("should parse size output");

        assert_eq!(report.text, 14280);
        assert_eq!(report.data, 112);
        assert_eq!(report.bss, 1572);
        assert_eq!(report.flash_used, 14280 + 112);
        assert_eq!(report.ram_used, 112 + 1572);

        let formatted = report.format_report();
        assert!(formatted.contains("FLASH (text + data)"));
        assert!(formatted.contains("RAM (data + bss)"));
        assert!(formatted.contains("14.1 KB / 512.0 KB"));
    }
}
