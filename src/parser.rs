#[derive(Debug, Clone)]
pub struct KeyValuePair {
    key: String,
    value: String,
}

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    keys: Vec<KeyValuePair>,
    children: Vec<Node>,
}

// this is base64 of "root-section" the root of the file
//
// Just to make it harder to conflict with user specified sections
pub const ROOT_SECTION: &str = "cm9vdC1zZWN0aW9uCg==";

impl Node {
    pub fn new(name: String) -> Self {
        Self {
            name,
            keys: Vec::new(),
            children: Vec::new(),
        }
    }
}

const SECTION_PREFIX: &str = "@";
const SECTION_END: &str = "}";
const KEY_VALUE_SEP: &str = "=";

pub fn parse(input: String) -> Node {
    let mut root_node = Node::new(ROOT_SECTION.into());
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
                push_kv_pair(&mut root_node.keys, key.to_owned(), value.to_owned());
            } else {
                let parent: &mut Node = &mut root_node;
                let kv_pair = KeyValuePair {
                    key: key.into(),
                    value: value.into(),
                };

                create_nodes(kv_pair, &mut sections.clone(), parent);
            }
        }
    }

    println!("{:#?}", root_node);
    return root_node;
}

fn create_nodes(kv_pair: KeyValuePair, sections: &mut Vec<String>, parent: &mut Node) {
    let section_name = sections.get(0);

    if section_name.is_some() {
        let node = Node::new(section_name.unwrap().to_owned());

        push_child(&mut parent.children, node);

        sections.remove(0);

        let tnode_index = parent.children.len() - 1;

        create_nodes(kv_pair, sections, &mut parent.children[tnode_index]);
    } else {
        // add kv pair to last node
        push_kv_pair(&mut parent.keys, kv_pair.key, kv_pair.value);
    }
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

fn push_child(target: &mut Vec<Node>, node: Node) {
    match target.iter_mut().find(|p| *p.name == node.name.to_owned()) {
        Some(_) => {}
        None => {
            target.push(node);
        }
    }
}
