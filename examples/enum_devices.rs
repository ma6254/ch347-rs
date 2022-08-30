use ch347_rs::ch347lib::ch347lib;

fn main() {
    println!("Hello, world {:?}", ch347lib::get_version(1));
    println!("enum_device: {:?}", ch347lib::enum_device());
    println!("Exit");
}
