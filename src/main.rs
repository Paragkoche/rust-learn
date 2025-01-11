mod basic;

fn main() {
    basic::print::print_fn();
    basic::print::greet_fn("parag");
    basic::print::add(12, 52);
    basic::print::table(17);
    basic::print::table2(55);
    print!("{esc}c", esc = 27 as char); //that is use to clear screen
    basic::function::fun1("name");
}
