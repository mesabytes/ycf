use neoconf::neoconf::Neoconf;

fn main() {
    let mut config = Neoconf::new(String::from("test.neoconf"));

    config.load();

    match config.get(None, "testkey") {
        Some(value) => {
            println!("testkey: {}", value);
        },
        None => {}
    }

    // config.remove("general", "somekey")
}
