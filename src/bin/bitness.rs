extern crate bitness;

fn main() {
    use std::process;

    match bitness::os_bitness() {
        Ok(bn) => println!("{:?}", bn),
        Err(err) => {
            eprintln!("{}", err.to_string());
            process::exit(2);
        }
    };
}
