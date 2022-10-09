fn main() {
    // Declare a variable
    let mut a_number;

    // Declare a second variable and bind the value
    let a_word = "Ten";

    // Bind a value to the first variable
    a_number = 10;

    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);

    // Change the value of an immutable variable
    a_number = 15;
    println!("Now the number is {}.", a_number);
}
