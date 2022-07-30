use neoconf::{Neoconf, Options};

fn main() {
    let mut config = Neoconf::new("test.neoconf", Options::default());

    config.load();

    println!("testkey: {}", config.get(Some("testsection1"), "testkey").unwrap_or_default());
    
    println!("host: {}", config.get(Some("testsection2"), "host").unwrap_or_default());

    println!("port: {}", config.get(Some("testsection2"), "port").unwrap_or_default());

    println!("name: {}", config.get(None, "name").unwrap_or_default()); 

    // config.save().unwrap();
}
