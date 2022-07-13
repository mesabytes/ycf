use neoconf::Neoconf;

fn main() {
    let mut config = Neoconf::new(String::from("test.neoconf"));

    config.load();

    println!("testkey: {}", config.get(Some("testsection1"), "testkey").unwrap_or(&String::from("None")));

    println!("host: {}", config.get(Some("testsection2"), "host").unwrap_or(&String::from("None")));

    println!("port: {}", config.get(Some("testsection2"), "port").unwrap_or(&String::from("None")));

    println!("name: {}", config.get(None, "name").unwrap_or(&String::from("None")));

    // config.remove(None, "host");
}
