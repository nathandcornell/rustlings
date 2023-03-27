// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);

    let answer2 = squareV2(5);
    println!("The square of 5 is {}", answer2);
}

fn square(num: i32) -> i32 {
    num * num
}

// Alternately:
fn squareV2(num: i32) -> i32 {
    return num * num;
}
