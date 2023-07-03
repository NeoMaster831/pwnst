mod socket;

pub fn print_foo() {
    let to_print = socket::foo();
    println!("{}", to_print)
}