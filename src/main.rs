use neoconf::neoconf::Neoconf;

fn main() {
    let config = Neoconf::new(String::from("test.neoconf"));

    config.load();
}
