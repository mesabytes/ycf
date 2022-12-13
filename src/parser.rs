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
const SECTION_END: &str = "}";
const KEY_VALUE_SEP: &str = "=";

pub fn parse(input: String) -> Section {
    let mut storage = Section::new();
    let mut sections: Vec<String> = Vec::new();

    for (index, mut line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let line_number = index + 1;

        line = line.trim();

        if line.starts_with(SECTION_PREFIX) {
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

        if line.starts_with(SECTION_END) || line.ends_with(SECTION_END) {
            sections.pop();
        }

        if line.contains(KEY_VALUE_SEP) {
            let (mut key, mut value) = line
                .split_once(KEY_VALUE_SEP)
                .expect("Corrupt config file!");

            key = key.trim();
            value = value.trim();

            if sections.is_empty() {
                push_kv_pair(&mut storage.keys, key.to_owned(), value.to_owned());
            } else {
                let mut inside_of: Vec<String> = Vec::new();

                for (index, section) in sections.clone().iter().enumerate() {
                    inside_of.push(section.to_owned());

                    let mut this_section = Section::new();

                    this_section.name = section.to_owned();
                    push_kv_pair(&mut this_section.keys, key.to_owned(), value.to_owned());

                    let mut target: Option<&mut Vec<Section>> = None;

                    if index != 0 {
                        match storage
                            .children
                            .iter_mut()
                            .position(|p| *p.name == this_section.name.to_owned())
                        {
                            Some(i) => target = Some(&mut storage.children[i].children),
                            None => {
                                push_child(&mut storage.children, this_section.clone());
                                let stchlen = storage.children.len()-1;

                                target = Some(&mut storage.children[stchlen].children)
                            }
                        }
                    } else {
                        target = Some(&mut storage.children)
                    }

                    push_child(target.unwrap(), this_section);
                }
            }
        }
    }

    println!("{:#?}", storage);
    return storage;
}

fn push_kv_pair(target: &mut Vec<KeyValuePair>, k: String, v: String) {
    match target.iter_mut().find(|p| *p.key == k.to_owned()) {
        Some(pair) => {
            pair.value = v.to_owned();
        }
        None => {
            target.push(KeyValuePair {
                key: k.into(),
                value: v.into(),
            });
        }
    }
}

fn push_child(target: &mut Vec<Section>, section: Section) {
    match target
        .iter_mut()
        .find(|p| *p.name == section.name.to_owned())
    {
        Some(_) => {}
        None => {
            target.push(section);
        }
    }
}
