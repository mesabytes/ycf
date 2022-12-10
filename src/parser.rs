use std::collections::HashMap;

const SECTION_PREFIX: &str = "@";

pub fn parse(file: &str, input: String, storage: &mut HashMap<String, String>) {
    let mut section = String::new();
    let mut inside_section = false;

    for (index, mut line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let line_number = index + 1;

        line = line.trim();

        if line.starts_with(SECTION_PREFIX) {
            // FIX: When move than one section are inside a section section var gets cleared
            let s: Vec<&str> = line.split_whitespace().collect();

            let current_section = {
                if s[0] == SECTION_PREFIX {
                    s[1].to_string()
                } else {
                    let mut t = s[0].to_string();

                    // remove `SECTION_PREFIX` from section name
                    t.remove(SECTION_PREFIX.len() - 1);

                    t
                }
            };

            if inside_section {
                section = format!("{section}.{current_section}");
            } else {
                section = current_section
            }

            assert!(
                section.is_empty() == false,
                "{} ({}): No section name is provided",
                file,
                line_number
            );
        }

        if line.starts_with("{") || line.ends_with("{") {
            inside_section = true;
        }

        if line.starts_with("}") || line.ends_with("}") {
            inside_section = false;
        }

        if line.contains("=") {
            let (mut key, mut value) = line.split_once("=").expect("Corrupt config file!");

            key = key.trim();
            value = value.trim();

            let skey;

            if !section.is_empty() {
                skey = format!("{section}.{key}");
            } else {
                skey = key.to_string();
            }

            storage.insert(skey, value.to_string());
        }
    }
}
