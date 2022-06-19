use neoconf::neoconf::Neoconf;

fn main() {
    let config = Neoconf::load(String::from("test.neoconf"));

    println!("hi");
}
