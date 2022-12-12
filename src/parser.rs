#[derive(Debug, Clone)]
pub struct KeyValuePair {
    key: String,
    value: String,
}

#[derive(Debug, Clone)]
pub struct Section {
    name: String,
    keys: Vec<KeyValuePair>,
    children: Vec<Section>,
}

// this is base64 of "root-section" the root of the file
//
// Just to make it harder to conflict with user specified sections
const ROOT_SECTION: &str = "cm9vdC1zZWN0aW9uCg==";

impl Section {
    pub fn new() -> Self {
        Self {
            name: ROOT_SECTION.into(),
            keys: Vec::new(),
            children: Vec::new(),
        }
    }
}

const SECTION_PREFIX: &str = "@";
const SECTION_START: &str = "{";
const SECTION_END: &str = "}";
const KEY_VALUE_SEP: &str = "=";

pub fn parse(input: String) -> Section {
    let mut storage = Section::new();
    let mut sections: Vec<String> = Vec::new();
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
                    for _ in 0..SECTION_PREFIX.len() {
                        t.remove(0);
                    }

                    t
                }
            };

            sections.push(current_section);

            assert!(
                sections.is_empty() == false,
                "line {}: No section name is provided",
                line_number
            );
        }

        if line.starts_with(SECTION_START) || line.ends_with(SECTION_START) {
            inside_section = true;
        }

        if line.starts_with(SECTION_END) || line.ends_with(SECTION_END) {
            inside_section = false;
            sections.pop();
        }

        if line.contains(KEY_VALUE_SEP) {
            let (mut key, mut value) = line
                .split_once(KEY_VALUE_SEP)
                .expect("Corrupt config file!");

            key = key.trim();
            value = value.trim();

            if sections.is_empty() {
                // if key is already in keys overwrite it with new value
                // to prevent dublicated keys
                match storage.keys.iter_mut().find(|p| *p.key == key.to_owned()) {
                    Some(pair) => {
                        pair.value = value.to_owned();
                    }
                    None => {
                        storage.keys.push(KeyValuePair {
                            key: key.into(),
                            value: value.into(),
                        });
                    }
                }
            } else {
                println!("key: {:?}\n\tsection: {:?}", key, sections.join("."));
            }
        }
    }

    println!("{:#?}", storage);
    return storage;
}
