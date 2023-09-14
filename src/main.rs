use std::io;
fn main() {
    println!("Hello, world!");
    let x = 4;
    println!("x is {}", x);
    println!("x is {}", x);
    const SECONDS_IN_MINUTE: u32 = 60;
    let tup: (i32, bool, char)  = (1, true, 's');
    println!("{}", tup.0);
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[4] = 3;

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");
    println!("{}", input);
}
