use std::process::exit;

use crate::constants::PREPROCS_LIST;

pub fn preprocessor(file: Option<String>, input: String) -> String {
    let mut new_input = String::new();

    for (index, line) in input.lines().enumerate() {
        let mut line = line.trim().to_string();

        if line.starts_with("#") {
            line.remove(0);

            let tokens: Vec<&str> = line.split_whitespace().collect();

            let preprocessor = tokens[0];
            if PREPROCS_LIST.contains(&preprocessor) {
                match preprocessor {
                    "import" => {
                        let config_file: &String = match file {
                            Some(ref f) => f,
                            None => continue,
                        };

                        let file_to_import = tokens.get(1).unwrap_or_else(|| {
                            eprintln!("[ycf] Error: import path is missing!",);
                            eprintln!("{line_number} | {line}", line_number = index + 1);

                            exit(1);
                        });

                        // TODO: check if file_to_import exists if not throw an error
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
