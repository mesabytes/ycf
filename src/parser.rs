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
const ROOT_SECTION: &str = "cm9vdC1zZWN0aW9uCg==";

impl Node {
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

pub fn parse(input: String) -> Node {
    let mut root_node = Node::new();
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
                let mut inside_of: Vec<String> = Vec::new();

                for (index, section) in sections.clone().iter().enumerate() {
                    inside_of.push(section.to_owned());

                    let mut this_section = Node::new();

                    this_section.name = section.to_owned();
                    push_kv_pair(&mut this_section.keys, key.to_owned(), value.to_owned());

                    let mut target: Option<&mut Vec<Node>> = None;

                    if index == 0 {
                        target = Some(&mut root_node.children)
                    } else {
                        match root_node
                            .children
                            .iter_mut()
                            .position(|p| *p.name == this_section.name.to_owned())
                        {
                            Some(i) => target = Some(&mut root_node.children[i].children),
                            None => {
                                push_child(&mut root_node.children, this_section.clone());
                                let stchlen = root_node.children.len() - 1;

                                target = Some(&mut root_node.children[stchlen].children)
                            }
                        }
                    }

                    push_child(target.unwrap(), this_section);
                }
            }
        }
    }

    println!("{:#?}", root_node);
    return root_node;
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
    match target
        .iter_mut()
        .find(|p| *p.name == node.name.to_owned())
    {
        Some(_) => {}
        None => {
            target.push(node);
        }
    }
}
