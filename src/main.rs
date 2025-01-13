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
    );
    print!("{esc}c", esc = 27 as char); //that is use to clear screen

    let mut stack = basic::dsa::Stack::new(10);
    println!("{:?}", stack.get_data());
    let _ = stack.push(1);
    let _ = stack.push(2);
    let _ = stack.push(3);
    let _ = stack.push(4);
    let _ = stack.push(5);
    let _ = stack.push(6);
    let _ = stack.push(7);
    let _ = stack.push(8);
    let _ = stack.push(9);
    let _ = stack.push(10);
    println!("{:?}", stack.get_data());
    let _ = stack.pop();
    let _ = stack.pop();
    let _ = stack.pop();
    let _ = stack.pop();
    let _ = stack.pop();
    let _ = stack.pop();
    let _ = stack.pop();
    let _ = stack.pop();
    let _ = stack.pop();
    println!("{:?}", stack.get_data());
    let _ = stack.pop();

    let mut link_l = basic::dsa::list::new(10);
    // link_l.show_data();
    link_l.add_data(1);
    link_l.add_data(2);
    link_l.add_data(3);
    link_l.add_data(3);
    link_l.add_data(4);
    link_l.add_data(1);
    link_l.add_data(5);
    link_l.add_data(7);
    link_l.add_data(8);
    link_l.add_data(9);
    link_l.add_data(10);

    link_l.show_data();
}
