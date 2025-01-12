mod basic;

fn main() {
    basic::print::print_fn();
    basic::print::greet_fn("parag");
    basic::print::add(12, 52);
    basic::print::table(17);
    basic::print::table2(55);
    print!("{esc}c", esc = 27 as char); //that is use to clear screen
    basic::function::fun1("name");
    println!("fibonacci 10 = {}", basic::function::fibonacci_number(10));
    println!("factorial 10! = {}", basic::function::factorial_number(10));
    println!(
        "array {:?} = {:?}",
        [10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        basic::function::sort_bobble(&[10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
    );
    let mut array = vec![10, 7, 8, 9, 1, 5];
    let len = array.len();
    basic::function::quick_sort(&mut array, 0, len - 1);
    let sorted_array = array.clone();
    println!("array {:?} = {:?}", [10, 7, 8, 9, 1, 5], sorted_array);

    println!(
        "array {:?} search element {:?} index [{:?}]",
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        10,
        basic::function::liner_search([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec(), 10),
    );
    println!(
        "array {:?} search element {:?} index [{:?}]",
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        10,
        basic::function::binary_search([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec(), 10),
    )
}
