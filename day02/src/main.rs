fn main() {
    println!("Hello, world!");

    let input_string: &str = 
    "B Z
    C Z
    B X
    A Y
    B X
    B X
    A X
    B Z
    C Z
    B Y
    A Z
    C X
    B X
    C X
    B Z
    B Z
    C Y
    B Z
    B Z
    C Z
    B Z
    B Y
    B X
    B Y
    C Z
    C Y
    C Z
    A X
    C Z
    B X
    C Z
    B Y
    B X
    A Y
    A X
    A Y
    B Y
    B X
    B X
    A Z
    B Z
    B Y
    C Z
    B X
    C Y
    B Z
    B Y
    C Y
    A X
    A Y
    C Y
    C Z
    B Z
    B X
    C Z
    A X
    B X
    A Y
    B Z
    C Y
    A Y
    C Z
    C Z
    A X
    B X
    C Z
    A Z
    A Z
    B X
    B X
    B X
    A Y
    B X
    B X
    C Y
    B X
    C Z
    C Y
    B Z
    A X
    B X
    B X
    A X
    C Y
    C Y
    A X
    A X
    B Z
    B X
    C Z
    B X
    B Z
    A Z
    B Z
    A X
    A X
    B Z
    A X
    B X
    B X
    B X
    A Y
    A Y
    A Y
    B X
    C Y
    B Z
    A Y
    B X
    A Z
    C X
    A Z
    B Y
    B Z
    C Z
    B Z
    A Y
    B X
    B Z
    B Z
    B Z
    C Y
    B X
    A Y
    B Z
    B Y
    B Z
    B X
    A X
    A X
    B Y
    B X
    C Y
    A Y
    A Z
    B Z
    B Z
    B Y
    B X
    B Z
    B X
    B Z
    B Z
    B X
    B Z
    B Z
    B Z
    B X
    A Y
    B X
    B Z
    A X
    B Z
    B Z
    B X
    C Y
    A Z
    B Z
    C Z
    B X
    A Z
    B X
    A Z
    C Y
    C Y
    A Y
    A Y
    B Z
    A Y
    A Y
    C Z
    A X
    B X
    B X
    C Y
    A Z
    B Y
    C Y
    B Z
    B Y
    B Z
    A X
    B Z
    C Z
    B X
    B Y
    A X
    C Z
    B Y
    B Z
    B Z
    A Z
    B X
    A Y
    C Y
    C Y
    B Z
    B Z
    B X
    B Z
    B Z
    B Y
    B Z
    B Z
    B Z
    B X
    B X
    B Z
    B Y
    B Z
    C Y
    A Z
    A Y
    B X
    A X
    B Z
    B Z
    A Z
    B Z
    B Z
    B X
    A Y
    C Z
    C Y
    B Z
    B Z
    C Z
    A X
    B Z
    B Z
    B X
    A Y
    A Z
    B Z
    C Y
    C Z
    A Y
    B Z
    B X
    C Z
    A X
    C Z
    B Z
    C Z
    B X
    C Y
    B X
    B Z
    B X
    A Y
    A Z
    B Z
    B X
    B Z
    C Z
    C Z
    C Y
    B X
    B Y
    B Z
    C Z
    C Z
    B Z
    B X
    B Z
    C Y
    A Y
    C Y
    B X
    C Y
    B Y
    C X
    B X
    A Y
    C Z
    B X
    B Z
    B Y
    B X
    B X
    A Z
    B Z
    C Z
    B X
    B X
    A X
    B X
    B X
    C X
    C Y
    A X
    B X
    B Z
    B X
    A Y
    B X
    B Y
    B X
    B X
    B Y
    B X
    A Z
    B X
    B Z
    B X
    C Z
    A Z
    C X
    B Z
    A Y
    B X
    B X
    A Z
    A Y
    B Z
    B Z
    A Y
    C Z
    B X
    B X
    C Z
    B Z
    B Z
    B Z
    A Y
    A Y
    A X
    B X
    C X
    B X
    B Z
    B X
    A X
    B X
    B Z
    B X
    A Y
    B X
    C Y
    B X
    B Z
    B Z
    C Z
    C Z
    C Y
    B Z
    B X
    B Z
    A Y
    C Y
    B X
    C Y
    C Z
    A X
    B Z
    A X
    B X
    C Y
    A X
    A Y
    B X
    A Y
    A Z
    C X
    B X
    B Y
    B Z
    B X
    A Y
    B Z
    B X
    A X
    B X
    B X
    B Y
    B Z
    B Z
    B X
    B X
    A X
    B Z
    B Z
    B Z
    A Y
    C Z
    A X
    A Y
    B Z
    B X
    C Y
    A X
    B X
    B Z
    B Y
    A Y
    B Z
    B Z
    B X
    A Y
    A Y
    A Y
    C Y
    B X
    B X
    B Z
    B Z
    A Z
    A Y
    B Z
    B X
    B X
    B X
    A X
    B X
    C Y
    B Z
    A Y
    B X
    A Y
    A X
    C Y
    B Z
    C Z
    B Z
    B X
    B Z
    A Y
    B Z
    C Z
    B Z
    B Z
    A X
    B Z
    B Z
    B X
    B Z
    B Z
    B Z
    C X
    B X
    B X
    A Y
    A Y
    B X
    B X
    C Z
    B X
    B X
    C Y
    C Y
    C Z
    B X
    B Z
    B Z
    B Z
    A Y
    A Y
    A X
    C Z
    A Z
    A X
    B Z
    C Z
    A X
    B X
    B Y
    C Z
    B Y
    B Z
    C Y
    C Y
    C Y
    B Z
    B Z
    B X
    B Z
    B Y
    B X
    B Z
    A X
    B X
    A Y
    A X
    B Z
    A Y
    C Z
    C X
    B X
    B X
    C Y
    B Z
    A Y
    B Z
    B X
    C X
    B Z
    A X
    A Y
    A Y
    C Y
    C Z
    C Z
    B X
    A X
    B X
    B X
    B Z
    C Y
    B X
    C Z
    C Y
    B X
    B Z
    B X
    B X
    C Z
    A Y
    B Z
    B X
    A X
    B Z
    B Z
    B X
    B X
    C Z
    C Z
    B Y
    C Z
    B Z
    C Y
    B X
    B X
    C Y
    B X
    B X
    A Z
    B X
    B X
    A Y
    B X
    B X
    B X
    B Z
    B Z
    C Z
    A Y
    B X
    B Z
    B X
    B Z
    B Z
    B X
    B Y
    C Z
    A Z
    A Y
    C Y
    B X
    A X
    B Y
    A X
    B Y
    A Y
    C X
    B X
    A Y
    B Z
    B Z
    B X
    B Z
    B Z
    B X
    B X
    A Y
    B Z
    A Y
    B Y
    B Y
    B Y
    B X
    B Z
    B Z
    A Y
    A X
    C Y
    B X
    B X
    A Y
    B X
    A Y
    B Z
    B Z
    B Y
    B Z
    C Z
    C Z
    C Y
    B Z
    C Y
    A X
    B X
    C Y
    C Z
    B X
    C Y
    A Y
    B Z
    B Z
    A X
    C Y
    B X
    A Y
    C Z
    B Z
    B X
    A Y
    C Z
    A X
    A Y
    C X
    C Y
    A Y
    B Z
    B X
    A Z
    B Z
    B Z
    B X
    C Z
    B X
    B Z
    B X
    B X
    B Z
    B X
    A X
    A Y
    B Y
    A Y
    A Y
    B Z
    C Z
    B X
    B Z
    A Y
    A Y
    A Z
    B X
    A Y
    A Y
    B X
    A X
    B Y
    B Z
    B Y
    A Z
    C Y
    B Z
    A X
    A Z
    C Z
    B X
    B Z
    B X
    C Y
    A X
    B Z
    A Y
    B X
    B Z
    B X
    C Y
    B Z
    B X
    A Y
    B X
    C Y
    C X
    A X
    B Z
    B X
    A X
    A Z
    B X
    B Y
    A Z
    B Z
    C Y
    A Y
    C X
    B Z
    A Y
    C Y
    C Z
    B Y
    C Y
    A Z
    B Z
    B Z
    A Y
    B X
    C X
    A Y
    A Y
    A Z
    B X
    B X
    B Y
    A X
    B X
    B Z
    B Z
    B X
    B Y
    C Z
    C Y
    A Y
    A Y
    C X
    C Z
    C Y
    C Y
    A Y
    A Y
    B Z
    C Y
    C Y
    A Y
    A Y
    C Z
    B Z
    A X
    B Y
    B Z
    B Z
    B Z
    B Z
    B X
    A Y
    A Y
    B Z
    A Y
    C X
    A X
    C Z
    B Z
    B Z
    A X
    C Y
    B Z
    B X
    B X
    B X
    B Z
    C Y
    B Z
    B X
    B X
    B Z
    C Y
    B X
    A Z
    B X
    B Z
    A X
    C X
    A X
    B Z
    B Z
    B Z
    B Z
    A Y
    B Z
    B Y
    B Z
    B Z
    B X
    B Z
    B Z
    A Y
    A X
    B X
    A Z
    B Z
    A Z
    B X
    B X
    B Z
    A Y
    B Y
    A X
    B X
    B X
    A Y
    B X
    B X
    B Z
    B Y
    B Z
    C Z
    B X
    C Y
    B Z
    B Z
    C Y
    B Z
    B Z
    B Z
    B Z
    B Z
    C Y
    A Y
    A X
    B Z
    C Y
    B Y
    C Z
    B Z
    B Z
    C Z
    B Z
    B Z
    B X
    A Y
    B Z
    B Z
    C Y
    A Y
    B Z
    A Y
    B Z
    C Z
    C Y
    A Y
    A Z
    B X
    A X
    B Z
    B Y
    A X
    A X
    A Y
    B X
    C Y
    B X
    B Z
    A Y
    B X
    B X
    A Y
    A Z
    C Z
    C Y
    A Y
    B X
    B Z
    B X
    C Z
    A Y
    B X
    A Y
    B Z
    B X
    C Y
    A Y
    B Z
    C Y
    B Y
    B X
    C Y
    A Y
    B Z
    C Y
    B X
    A X
    B Z
    B Z
    C Y
    A Y
    B Z
    C Y
    B X
    A X
    A Z
    C X
    B Z
    B Z
    C Y
    B Y
    C Z
    C Y
    A X
    A Y
    A X
    A Y
    C Z
    C Y
    C Z
    C Z
    C Y
    A X
    C Z
    B X
    C X
    B X
    A Z
    B X
    C Z
    A Y
    A X
    A Z
    C Z
    B X
    C Y
    A Y
    C Y
    C Z
    C Y
    C Y
    C X
    B Z
    B X
    B Y
    A X
    B Z
    B Y
    C Y
    C Y
    C Z
    A Z
    A X
    A Y
    C Z
    B Z
    B X
    B Z
    B Z
    B X
    B Z
    C Y
    A X
    B X
    A Z
    B X
    C Y
    B Z
    B X
    B Z
    C Z
    C Z
    A X
    B Z
    B X
    A Y
    B Z
    A Y
    B Y
    C Z
    C Y
    A X
    A Y
    C Y
    C Z
    B X
    C Y
    B Z
    B Z
    B Z
    C Z
    B X
    C Y
    B Z
    C Z
    B X
    A Y
    A X
    B Z
    B Z
    C Y
    B X
    B Z
    C Y
    A Y
    C Y
    A Y
    B X
    C Z
    A X
    A Y
    C Y
    C Z
    B Z
    B Z
    B Z
    A Y
    A Y
    C Z
    A Z
    B X
    A X
    B Z
    C Z
    B X
    C Y
    B Z
    B X
    B Z
    B Z
    B X
    C Z
    B X
    B Z
    B X
    A X
    B X
    A X
    B Z
    B Z
    A Y
    B X
    B Z
    B Z
    C Z
    C Y
    B X
    B X
    B Y
    C Z
    C Y
    A X
    B Z
    C Y
    A Y
    B X
    B X
    A X
    A Y
    C X
    B Z
    B Z
    A Y
    A X
    C Y
    B Z
    B Z
    C X
    C Y
    A Y
    B Z
    C Y
    B Z
    B X
    B Z
    C Y
    B X
    B Z
    B X
    B X
    B X
    B Z
    B Z
    C Y
    B X
    B Z
    A Z
    A Y
    A Z
    A Y
    B X
    C Z
    A Y
    B X
    B X
    C Z
    B Z
    A X
    B X
    C Y
    A Y
    A X
    B Z
    A X
    A Y
    B Z
    B X
    B Z
    C Y
    A X
    A X
    B X
    B Y
    C X
    A X
    B X
    B X
    A Y
    C Y
    B Z
    B Z
    C Z
    B X
    B Z
    B X
    B Y
    B Z
    B Z
    B X
    B Z
    A X
    B X
    A Y
    A Z
    B Z
    B X
    A Y
    A X
    B Z
    B X
    C Z
    A Y
    A Y
    C Z
    B X
    A X
    C Y
    B X
    B Z
    B X
    B Z
    B X
    C Y
    B Z
    C Y
    C Z
    A Z
    C Z
    A X
    C Z
    B Y
    B X
    B Z
    C Z
    A Y
    A Z
    A X
    B Z
    A X
    B Z
    B X
    A Z
    C Z
    C Y
    B Z
    C X
    A X
    A X
    B X
    A Y
    A X
    B Z
    B Z
    B Z
    B Z
    A X
    A X
    A Y
    B Z
    B Y
    B Z
    A Y
    B Y
    A X
    A Z
    B Z
    A Y
    B X
    A X
    A X
    B Z
    A Z
    B Y
    B Z
    C Z
    C Y
    B X
    B Y
    A X
    A Z
    B Z
    A Z
    B Z
    A X
    B Z
    A Z
    B Y
    A Z
    C Y
    B Z
    C Z
    B Z
    B X
    B Z
    C X
    A X
    B X
    C Z
    B X
    B Y
    A X
    B X
    B X
    A Z
    B X
    B Z
    C Z
    B X
    B X
    B X
    A Y
    A Z
    C Y
    A Y
    B X
    A Z
    A Z
    B Y
    B Y
    C Z
    C Z
    B Z
    C Z
    B Z
    A Y
    A X
    C Y
    B X
    B Z
    B Z
    B Z
    B X
    B Z
    B Z
    A X
    A Y
    B X
    B X
    B X
    C X
    C Y
    C X
    B X
    B Z
    B Y
    C X
    A Y
    A Y
    B Z
    C Y
    C Z
    C Z
    C Z
    A Y
    B Y
    B Z
    B X
    B Z
    B Y
    A X
    C Y
    C Z
    A Y
    B Z
    A X
    A X
    A X
    B Z
    B X
    C Y
    B Y
    C Z
    B Z
    B Z
    C Y
    B Z
    C Z
    B X
    B Y
    A Y
    C Z
    A Y
    B Z
    B Z
    B X
    B X
    B Z
    B Z
    B X
    B X
    C Z
    B Y
    B Z
    B X
    C Y
    C Z
    A Y
    C Z
    B Z
    C Y
    B X
    C Z
    A X
    B Z
    B X
    C X
    C Z
    B Z
    C Z
    A X
    B Z
    C X
    B Z
    C Z
    A Y
    B Z
    B Z
    C Z
    B Y
    B Z
    B X
    B X
    A X
    A Y
    A Y
    C Y
    C Y
    C Y
    B Z
    B Z
    A Y
    B X
    A Z
    C Y
    C Z
    B X
    A Y
    A Y
    C Z
    C Z
    C Y
    A Z
    B Z
    B Z
    B Z
    A Y
    A Y
    C Z
    B Z
    B X
    C Z
    B Z
    C Y
    A Z
    B X
    B Z
    A Z
    B X
    A X
    B X
    A X
    B X
    B Z
    B Z
    B Z
    C Y
    C Z
    A Y
    B X
    A X
    C Z
    C Y
    C Z
    B Z
    B X
    A Y
    A X
    C Z
    B X
    C Z
    C Y
    A X
    B X
    C Z
    B X
    B Z
    C Y
    B X
    A X
    A Y
    A X
    B Z
    B Z
    C Z
    B X
    A Y
    B X
    B X
    A Z
    B Y
    B Z
    B X
    B Z
    B X
    B Y
    B X
    B X
    A Y
    A Y
    A X
    C Y
    A Y
    B X
    C Y
    B Z
    B Z
    A Y
    B X
    C Y
    C Z
    C Y
    B Z
    C Z
    C Y
    A Y
    A Y
    B Z
    B X
    A X
    A Y
    B X
    B Z
    B X
    C Z
    C Z
    A Y
    B X
    B Z
    B Y
    C X
    C Y
    B Z
    A X
    B Z
    A Y
    A X
    A Y
    B X
    B Z
    B Z
    B X
    B Z
    C Z
    B Z
    A Y
    B Z
    C Z
    B X
    B X
    B X
    B X
    B X
    B X
    B Y
    B Z
    B X
    B Z
    A Z
    B Z
    C Y
    A X
    B Z
    B Z
    C Z
    B X
    A Z
    C Y
    B Z
    B X
    A X
    A Y
    C Y
    B Y
    A X
    B Y
    B X
    B Z
    C Y
    B Z
    C Y
    A Z
    B Z
    C Y
    C Z
    A Y
    C X
    C Y
    B Z
    B X
    B Z
    B X
    B X
    A Y
    B Y
    B X
    B X
    C Y
    B X
    C X
    B Y
    A Y
    C Y
    B X
    B X
    A X
    B X
    A X
    A X
    B X
    B X
    A Z
    C Z
    C Y
    B X
    B X
    C Z
    B X
    C Y
    C Z
    A Y
    B Z
    C Y
    B X
    B Y
    B X
    B X
    C X
    A X
    B X
    B Z
    B Z
    C Y
    C Y
    B Y
    A Y
    B Z
    B X
    B X
    A Z
    B Z
    B X
    B X
    A Y
    B X
    B X
    B X
    A X
    B X
    B X
    B X
    B Z
    B X
    A Z
    B Y
    B X
    B Z
    B Z
    B Z
    A X
    B Z
    B Z
    B X
    B Z
    C Z
    C Y
    A Z
    C X
    C Y
    A Y
    B X
    B Z
    C Z
    B X
    C Z
    B Z
    A Z
    A Y
    B Y
    B Z
    B X
    A X
    B Z
    C Z
    C Y
    B Z
    A X
    A Y
    A Z
    B Z
    C Y
    A Y
    B X
    C Z
    A Y
    B X
    B Z
    B X
    C Y
    B X
    B X
    B X
    A Z
    B Z
    C Z
    B Z
    C Y
    B Z
    C Z
    B Z
    B X
    C Y
    C Z
    A X
    C Z
    C Y
    C Y
    B X
    A Y
    A Z
    B X
    B Z
    B Z
    B Z
    A X
    A Z
    B Z
    A Z
    A Y
    C Z
    B Y
    B Z
    B X
    B X
    C Z
    B Z
    B Z
    B Z
    B Z
    B X
    B X
    A X
    A X
    A Z
    B Z
    B X
    B Z
    B Z
    C X
    A Y
    B Y
    B X
    B X
    B Z
    B X
    B X
    B X
    C Y
    B Z
    B X
    C Y
    B Z
    A Y
    B Y
    B Z
    A Y
    A X
    B X
    B X
    B Z
    A X
    B Z
    A Y
    B Z
    B X
    A X
    A X
    A X
    A Y
    B Z
    A Y
    A X
    B X
    B Z
    A Y
    B Z
    B X
    B X
    A Z
    B Z
    B Z
    B Z
    B Z
    A X
    B Z
    B Z
    B X
    B Z
    C Y
    B Z
    B X
    B Z
    B X
    C Y
    B X
    B Y
    B Z
    B X
    A X
    C Y
    B Z
    B Z
    B X
    A Y
    B X
    B Z
    C X
    C Y
    A Y
    B X
    B X
    A Y
    B Z
    C Y
    B Z
    A Y
    C Y
    B Z
    A X
    A X
    A Y
    C Y
    C Z
    B Z
    C Z
    B X
    A X
    B X
    A Y
    A Y
    C Z
    C Y
    A Z
    B Z
    A Y
    B X
    B X
    B Z
    C Z
    B X
    B X
    B Y
    C Y
    C Z
    A Y
    A Z
    A X
    A Y
    A Y
    A Z
    B Z
    B Z
    C Z
    B Z
    B X
    C Y
    A Y
    B X
    C Z
    A X
    B Z
    B Y
    A Y
    B X
    B X
    A X
    C Z
    C Z
    C Y
    C Y
    A X
    B X
    B X
    B Y
    A Z
    C Z
    A Y
    C Z
    C Y
    B X
    C Y
    B Z
    A Z
    B Y
    B X
    C Y
    B Y
    B Z
    A Z
    A X
    B X
    C Z
    C Z
    B Z
    B Z
    C Z
    B X
    B X
    C Y
    A Y
    C Y
    C Z
    B Z
    B X
    A Y
    B Y
    B Z
    C X
    B X
    B Z
    B Z
    B X
    B Z
    B Z
    C Y
    A Y
    B Z
    B X
    A Y
    A Y
    B Z
    B Y
    C Y
    B Z
    B Y
    B Z
    A Y
    B X
    C Z
    A X
    B X
    C Y
    B Z
    B Y
    B Y
    B Z
    A X
    A X
    B Z
    B X
    A X
    B Z
    B Z
    A Y
    B X
    A X
    B X
    B X
    A Y
    C Z
    C Y
    B Z
    B X
    B X
    B X
    A X
    B X
    B Z
    B Z
    B Z
    B X
    B Z
    B Z
    A Z
    C Y
    B X
    B X
    B X
    A X
    C Z
    A Y
    A Y
    B Z
    B X
    C Z
    B Y
    C Z
    B X
    A Y
    C X
    B Z
    B X
    B X
    C Y
    B X
    B X
    B Z
    A X
    B X
    A Z
    B X
    B X
    B Z
    B X
    B X
    B Z
    A Y
    B X
    B Z
    B X
    B X
    C Y
    B X
    B Z
    B Z
    B X
    B Z
    C Z
    B Y
    A Y
    B Z
    B X
    B X
    A X
    B Z
    B X
    B X
    B Z
    C Z
    B Z
    B X
    C Y
    B Z
    B Y
    B Z
    B Z
    A Z
    B X
    B X
    B X
    B X
    C Y
    A Y
    B X
    B Z
    B X
    B X
    C Z
    C X
    B Z
    B Z
    B X
    B Z
    A Y
    A Z
    C Z
    A X
    A X
    A Y
    A Y
    A Y
    C Z
    B Z
    A Y
    C Z
    B X
    A Z
    C Y
    A X
    A X
    A Y
    B Z
    A X
    B X
    B X
    C Z
    B Z
    C Z
    B Z
    A X
    C Y
    C Z
    A X
    A Z
    B X
    C Y
    A X
    B X
    B Z
    B X
    A Y
    A Z
    C Z
    B Z
    A X
    B X
    C Y
    B X
    B X
    C Y
    B X
    A Z
    A X
    C X
    B Z
    B Y
    C Z
    C Z
    A Y
    A Y
    A Y
    B X
    A Z
    B X
    A X
    C Y
    B X
    A X
    B X
    B X
    B Z
    B Z
    B Y
    A Y
    C Z
    C Z
    B X
    B Z
    C Z
    B X
    C Z
    B Z
    A Y
    A X
    B X
    C Z
    C Y
    A Y
    B Z
    C Y
    B X
    C X
    A X
    B Z
    C Z
    B Z
    B X
    A Y
    B Z
    A Y
    A Y
    A Y
    C Y
    C Y
    C Z
    A Y
    C Y
    C Y
    B Z
    B X
    C Y
    C Y
    B X
    B Z
    B X
    C Z
    C Y
    C Y
    B X
    B X
    A Y
    B Z
    B Z
    B X
    B X
    C Z
    A X
    B Y
    B X
    A Z
    B Z
    B X
    B Z
    C Y
    B Z
    C Y
    B Z
    A Z
    C Y
    A Z
    C Z
    B X
    B X
    C Y
    B X
    C Y
    A Z
    B X
    B Z
    B Z
    B X
    B Z
    B Y
    A Y
    B Z
    B X
    B Z
    A Y
    A Y
    C Z
    A Y
    C X
    B Z
    A Y
    A Y
    B X
    B Z
    A Y
    C Z
    B Z
    B Z
    C Y
    A X
    B Z
    C Z
    B X
    B X
    B Z
    A Y
    B Y
    C Z
    A Y
    B X
    C Z
    B X
    B Z
    B Z
    B Z
    B X
    B Z
    B X
    A Z
    B X
    B X
    B X
    B Z
    A Z
    A Y
    B Z
    B X
    B X
    C X
    A Y
    A X
    C Y
    A Z
    A X
    C Y
    A Y
    C Y
    A Z
    B X
    B Z
    B Z
    C Y
    A Y
    B X
    B X
    A Y
    C Z
    B Z
    B Z
    A X
    A Y
    B Z
    A Z
    A Y
    C Z
    B X
    B Z
    B Z
    C Y
    A Y
    C Z
    B Z
    A Y
    C Y
    C Y
    B Z
    B Z
    B Z
    A Y
    B Y
    A Y
    B Z
    C Z
    A Y
    B X
    C Y
    A Z
    A X
    B X
    A Z
    B Z
    A Z
    C Z
    B Z
    A Y
    A X
    A X
    C Y
    A Y
    B Z
    A Y
    B Z
    B X
    A Y
    A X
    A Y
    A X
    C Z
    A Y
    B X
    C Z
    B X
    B Z
    B Z
    C Y
    B Z
    B X
    A X
    B Y
    A Y
    B X
    C Z
    A Y
    B Z
    B X
    A Y
    C Z
    C Y
    B Z
    B Z
    B Z
    A X
    B X
    A Y
    B Z
    C Y
    A X
    A Z
    B Y
    B X
    C Y
    B Z
    C Y
    A Z
    B X
    A Y
    A Y
    C Y
    A Z
    B X
    A Z
    B X
    B X
    A X
    B X
    B Z
    B X
    B Y
    B Z
    B Y
    B X
    A Y
    A X
    C Z
    B Y
    C Y
    B X
    C Y
    B X
    B Z
    B X
    B X
    B Z
    B Z
    B Z
    C Z
    B Z
    B Z
    A X
    A Y
    B X
    B X
    B X
    C Z
    B X
    B Z
    C Y
    A X
    A Z
    B X
    C Z
    A X
    A X
    B Z
    A Y
    B Z
    A X
    C X
    C Z
    B X
    B X
    C Y
    A X
    B X
    A X
    C Y
    C Y
    A X
    A X
    B X
    B Y
    B Z
    A X
    B X
    B X
    B X
    B Y
    A Z
    B Z
    C Z
    B X
    B X
    B Z
    A Y
    C Y
    B X
    B X
    A X
    C Y
    C X
    C Z
    B X
    B Y
    A Z
    C Z
    A X
    C Y
    B Z
    B X
    A X
    B X
    B X
    B Z
    C Y
    A Y
    A X
    C Z
    B Z
    A Y
    B X
    B X
    B X
    A Y
    B X
    C Y
    B Y
    A X
    A X
    B Y
    B X
    B X
    B Z
    B Z
    A X
    C Z
    C Z
    A X
    C Y
    B X
    C Z
    B X
    B Z
    B X
    B Z
    A Y
    C Y
    B X
    B X
    B X
    B X
    B Y
    C Y
    B Y
    B Y
    A Y
    B Z
    C Y
    A X
    C Z
    B X
    B Z
    C Y
    A Y
    B X
    C Z
    B Z
    A Z
    A Z
    A Z
    A Y
    C Z
    A Z
    B X
    C Z
    B Z
    B X
    C X
    A Z
    B Y
    A Y
    B Y
    C Y
    B X
    A X
    A X
    A Y
    A Y
    A X
    B Y
    B Z
    B X
    A X
    C Y
    B X
    B Y
    A Y
    C Y
    A Y
    B Z
    B X
    B Z
    B Z
    B X
    B X
    B Z
    B Z
    C Z
    C Y
    A Z
    B X
    B X
    A Y
    C Y
    B X
    B Z
    B X
    C Z
    A Z
    B X
    C Z
    B X
    B X
    B X
    B Z
    C Z
    C Z
    B X
    C Y
    B Z
    C Z
    B Z
    C Z
    B Y
    B X
    C Z
    A X
    B X
    B X
    C Y
    B Z";

    fn split_line_to_inputs(input: &str) -> [char;2]{
        let input_vec: Vec<&str> = input.trim().split(" ").collect();
        [input_vec[0].chars().next().unwrap(), input_vec[1].chars().next().unwrap()] // Unwrap for fast prototype with controlled inputs
    }

    

    fn decode_inputs(inputs:Vec<[char;2]>) -> Vec<[u16;2]>{
        let mut outputs = Vec::<[u16;2]>::new();
        let mut output:[u16;2];
        for input in inputs{
            output = [0,0];
            output[0] = match input[0] {
                'A' => 1, // rock
                'B' => 2, // paper
                'C' => 3, // scissors
                _ => panic!("Illegal first input: {} .", input[0])
            };
            output[1] = match input[1] {
                // 'X' => 1,
                // 'Y' => 2,
                // 'Z' => 3,
                'X' => {
                    // lose
                    match output[0] {
                        1 => 3,
                        2 => 1,
                        3 => 2, 
                        _ => panic!("Illegal first value: {} .", output[0])
                    }
                },
                'Y' => {
                    // draw
                    match output[0] {
                        1 => 1,
                        2 => 2,
                        3 => 3, 
                        _ => panic!("Illegal first value: {} .", output[0])
                    }
                },
                'Z' => {
                    // win
                    match output[0] {
                        1 => 2,
                        2 => 3,
                        3 => 1, 
                        _ => panic!("Illegal first value: {} .", output[0])
                    }
                },
                _ => panic!("Illegal second input: {} .", input[1])
            };
            outputs.push(output);
        }
        
        outputs
    }

    fn get_inputs(input_string: &str) -> Vec<[u16;2]>{
        let lines: Vec<&str> = input_string.lines().collect();
        let mut inputs_char:Vec<[char;2]> = Vec::<[char;2]>::new();

        for line in lines {
            inputs_char.push(split_line_to_inputs(line));
        }
        
        
        decode_inputs(inputs_char)
    }

    fn score_round(round:[u16;2]) -> u16{
        match round[0]{
            1 => {
                match round[1]{
                    1 => 1 + 3, // rock rock
                    2 => 2 + 6, // rock paper
                    3 => 3 + 0, // rock scissors
                    _ => panic!("Illegal value: {} .", round[1])
                }
            },
            2 => {
                match round[1]{
                    1 => 1 + 0, // paper rock
                    2 => 2 + 3, // paper paper
                    3 => 3 + 6, // paper scissors
                    _ => panic!("Illegal value: {} .", round[1])
                }
            },
            3 => {
                match round[1]{
                    1 => 1 + 6, // scissors rock
                    2 => 2 + 0, // scissors paper
                    3 => 3 + 3, // scissors scissors
                    _ => panic!("Illegal value: {} .", round[1])
                }
            },
            _ => panic!("Illegal value: {} .", round[0])
        }
    }

    fn score_all_rounds(rounds:Vec<[u16;2]>) -> Vec<u16> {
        let mut output: Vec<u16> = Vec::<u16>::new();
        for round in rounds{
            output.push(score_round(round));
        }
        output
    }

    fn tally_scores(round_scores:Vec<u16>) -> u16{
        round_scores.into_iter().sum()
    }

    let inputs = get_inputs(input_string);
    let round_scores = score_all_rounds(inputs);
    let final_score = tally_scores(round_scores);

    println!("Total scores: {} .", final_score);
    
}
