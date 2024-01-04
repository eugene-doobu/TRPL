use std::io;

fn main() {
    println!("Hello, world!");
    println!("Please input your guess.");

    // new 는 String 타입의 associated function
    let mut guess = String::new();
    // prelude 에 없는 타입을 이용하려면 명시적으로 use 문을 사용해서 가져와야 함
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
