use neoconf::neoconf::Neoconf;

fn main() {
    let mut config = Neoconf::new(String::from("test.neoconf"));

    config.load();

    match config.get("", "testkey") {
        Some(value) => {
            println!("testkey: {}", value);
        },
        None => {}
    }
}
