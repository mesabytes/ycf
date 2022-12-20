use std::process::exit;

use crate::constants::PREPROCS_LIST;

pub fn preprocessor(file: Option<String>, input: String) -> String {
    let mut new_input = String::new();

    for (index, line) in input.lines().enumerate() {
        let mut line = line.trim().to_string();

        if line.starts_with("#") {
            line.remove(0);

            let tokens: Vec<&str> = line.split_whitespace().collect();

            let preprocessor_name = tokens[0];
            if PREPROCS_LIST.contains(&preprocessor_name) {
                match preprocessor_name {
                    "import" => {
                        let config_file: &String = match file {
                            Some(ref f) => f,
                            None => continue,
                        };

                        let mut fpath = std::path::PathBuf::new();

                        fpath.push(config_file);

                        // remove config_file from path
                        fpath.pop();

                        let file_name = tokens.get(1).unwrap_or_else(|| {
                            eprintln!("{line_number} | {line}", line_number = index + 1);
                            eprintln!("[ycf] Error: import path is missing!");
                            exit(1);
                        });

                        let file_to_import = fpath.join(file_name);

                        if !file_to_import.exists() {
                            eprintln!("{line_number} | {line}", line_number = index + 1);
                            eprintln!("[ycf] Error: import path does not exists!");
                            exit(1);
                        }
                        if file_to_import.is_dir() {
                            eprintln!("{line_number} | {line}", line_number = index + 1);
                            eprintln!("[ycf] Error: import path is a directory!");
                            exit(1);
                        }

                        let file_to_import_content =
                            std::fs::read_to_string(&file_to_import).expect("Failed to read file");

                        // preprocess imported file content
                        let content = preprocessor(file.clone(), file_to_import_content);

                        new_input.push_str(&content);
                        new_input.push('\n');
                    }
                    _ => {}
                }
            }
        } else {
            new_input.push_str(&line);
            new_input.push('\n');
        }
    }

    new_input
}
