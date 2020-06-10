use add_one;
use add_two;

fn main() {
    let num = 10;
    println!(
        "{} plus one plus two is {}",
        num,
        add_two::add_two(add_one::add_one(num))
    );
}
