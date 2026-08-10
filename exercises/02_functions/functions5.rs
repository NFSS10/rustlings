fn square(num: i32) -> i32 {
    num * num // or `return num * num;`
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
