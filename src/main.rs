use ycf::Ycf;

fn main() {
    let mut config = Ycf::load_from_file("test.ycf");

    let username = config.get("testsection1.user.username");

    config.set("hello".into(), "world".into());

    println!("{:?}", username);
    println!("{:?}", config.get("hello").unwrap());

    config.save(Some("output.ycf".into()));
}
