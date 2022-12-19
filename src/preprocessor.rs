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
                        match file {
                            Some(ref base_config_file) => {
                                let file_to_import = match tokens.get(1) {
                                    Some(f) => f,
                                    None => {
                                        eprintln!(
                                            "[ycf] in {base_config_file} line {line_number}: no import path provided!",
                                            line_number = index + 1
                                        );
                                        exit(1);
                                    }
                                };

                                // TODO: check if file_to_import exists if not throw an error
                            }
                            None => {
                                continue;
                            }
                        };
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
