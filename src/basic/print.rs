pub fn print_fn() {
    println!("Hello world");
}

pub fn greet_fn(name: &str) {
    println!("Hello {} sir! good morning", name);
}

pub fn add(num1: i32, num2: i32) {
    println!("{} + {} = {}", num1, num2, num1 + num2);
}

pub fn table(num1: i32) {
    for i in 1..10 + 1 {
        println!("{} x {} = {}", num1, i, num1 * i);
    }
}

pub fn table2(num1: i32) {
    let mut i = 1;
    while i <= 10 {
        println!("{} x {} = {}", num1, i, num1 * i);
        i = i + 1;
    }
}
