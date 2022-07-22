use neoconf::Neoconf;

fn main() {
    let mut config = Neoconf::new(String::from("test.neoconf"));

    config.load();

    println!("testkey: {}", config.get(Some("testsection1"), "testkey").unwrap_or_default());

    // config.remove(Some("testsection2"), "host");

    println!("host: {}", config.get(Some("testsection2"), "host").unwrap_or_default());

    println!("port: {}", config.get(Some("testsection2"), "port").unwrap_or_default());

    println!("name: {}", config.get(None, "name").unwrap_or_default()); 

    config.save()
}
