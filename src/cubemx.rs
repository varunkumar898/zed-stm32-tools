//! STM32CubeMX Integration and User Code Protection.
//!
//! Provides CubeMX executable resolution, command builders for GUI and headless code generation,
//! and preservation of `USER CODE BEGIN` / `USER CODE END` blocks across code regenerations.

use std::collections::HashMap;

/// Extracts all user-written code between `/* USER CODE BEGIN <tag> */` and `/* USER CODE END <tag> */`.
pub fn extract_user_code_blocks(source: &str) -> HashMap<String, String> {
    let mut blocks = HashMap::new();
    let mut current_tag: Option<String> = None;
    let mut current_lines = Vec::new();

    for line in source.lines() {
        if let Some(start_idx) = line.find("USER CODE BEGIN") {
            let after = &line[start_idx + "USER CODE BEGIN".len()..];
            let tag = after.trim().trim_end_matches("*/").trim().to_string();
            current_tag = Some(tag);
            current_lines.clear();
            continue;
        }

        if let Some(end_idx) = line.find("USER CODE END") {
            let after = &line[end_idx + "USER CODE END".len()..];
            let tag = after.trim().trim_end_matches("*/").trim().to_string();
            if let Some(cur) = current_tag.take() {
                if cur == tag {
                    blocks.insert(cur, current_lines.join("\n"));
                }
            }
            continue;
        }

        if current_tag.is_some() {
            current_lines.push(line.to_string());
        }
    }

    blocks
}

/// Injects preserved user code blocks back into freshly generated source code.
pub fn restore_user_code_blocks(
    fresh_source: &str,
    saved_blocks: &HashMap<String, String>,
) -> String {
    let mut result = Vec::new();
    let mut skipping_original = false;

    for line in fresh_source.lines() {
        if let Some(start_idx) = line.find("USER CODE BEGIN") {
            let after = &line[start_idx + "USER CODE BEGIN".len()..];
            let tag = after.trim().trim_end_matches("*/").trim().to_string();
            result.push(line.to_string());

            if let Some(saved_content) = saved_blocks.get(&tag) {
                if !saved_content.is_empty() {
                    result.push(saved_content.clone());
                }
                skipping_original = true;
            }
            continue;
        }

        if let Some(end_idx) = line.find("USER CODE END") {
            let after = &line[end_idx + "USER CODE END".len()..];
            let _tag = after.trim().trim_end_matches("*/").trim().to_string();
            skipping_original = false;
            result.push(line.to_string());
            continue;
        }

        if !skipping_original {
            result.push(line.to_string());
        }
    }

    result.join("\n")
}

/// Generates an STM32CubeMX script for headless code generation.
pub fn generate_headless_script(ioc_abs_path: &str) -> String {
    format!(
        r#"config load "{ioc_path}"
project generate
exit
"#,
        ioc_path = ioc_abs_path.replace('\\', "/")
    )
}

/// Builds execution command and arguments to launch STM32CubeMX GUI with an `.ioc` file.
pub fn build_open_cubemx_command(cubemx_path: &str, ioc_path: &str) -> (String, Vec<String>) {
    (cubemx_path.to_string(), vec![ioc_path.to_string()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_and_restore_user_code() {
        let original_source = r#"
        /* USER CODE BEGIN Includes */
        #include "my_driver.h"
        #include "custom.h"
        /* USER CODE END Includes */

        void SystemClock_Config(void);

        int main(void) {
            /* USER CODE BEGIN 1 */
            int custom_var = 42;
            /* USER CODE END 1 */

            while (1) {
                /* USER CODE BEGIN WHILE */
                toggle_led();
                /* USER CODE END WHILE */
            }
        }
        "#;

        let blocks = extract_user_code_blocks(original_source);
        assert_eq!(blocks.len(), 3);
        assert!(blocks
            .get("Includes")
            .unwrap()
            .contains("#include \"my_driver.h\""));
        assert!(blocks.get("1").unwrap().contains("int custom_var = 42;"));
        assert!(blocks.get("WHILE").unwrap().contains("toggle_led();"));

        // Simulate fresh generation by CubeMX where user sections are empty
        let fresh_source = r#"
        /* USER CODE BEGIN Includes */
        /* USER CODE END Includes */

        void SystemClock_Config(void);

        int main(void) {
            /* USER CODE BEGIN 1 */
            /* USER CODE END 1 */

            while (1) {
                /* USER CODE BEGIN WHILE */
                /* USER CODE END WHILE */
            }
        }
        "#;

        let restored = restore_user_code_blocks(fresh_source, &blocks);
        assert!(restored.contains("#include \"my_driver.h\""));
        assert!(restored.contains("int custom_var = 42;"));
        assert!(restored.contains("toggle_led();"));
    }
}
