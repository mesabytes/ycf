use neoconf::neoconf::Neoconf;

fn main() {
    let mut config = Neoconf::new(String::from("test.neoconf"));

    config.load();

    match config.get(Some("testsection"), "testkey") {
        Some(value) => {
            println!("testkey: {}", value);
        },
        None => {}
    }

    match config.get(Some("main"), "host") {
        Some(value) => {
            println!("host: {}", value);
        },
        None => {}
    }

    match config.get(Some("main"), "port") {
        Some(value) => {
            println!("port: {}", value);
        },
        None => {}
    }

    match config.get(None, "name") {
        Some(value) => {
            println!("name: {}", value);
        },
        None => {}
    }

    // config.remove(None, "host");
}
