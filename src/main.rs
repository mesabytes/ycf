use neoconf::neoconf::Neoconf;

fn main() {
    let mut config = Neoconf::new(String::from("test.neoconf"));

    config.load();

    println!("testkey: {}", config.get(Some("testsection"), "testkey").unwrap_or(&String::from("None")));

    println!("host: {}", config.get(Some("main"), "host").unwrap_or(&String::from("None")));

    println!("port: {}", config.get(Some("main"), "port").unwrap_or(&String::from("None")));

    println!("name: {}", config.get(None, "name").unwrap_or(&String::from("None")));

    // config.remove(None, "host");
}
