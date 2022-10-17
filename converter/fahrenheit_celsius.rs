// Create a Fahrenheit_Celsius converter and the other way around.

use std::io;

fn main() {
    let mut choose = String::new();
    println!("Choose between Celsius To Fahrenheit [1] or Fahrenheit to Celsius [2],\
    please introduce a corrected value (integer)");
    io::stdin().read_line(&mut choose).expect("Failed to read!");
    // Careful with the trim, cuz it's removing all characters and it causes and error that basically, closes the CMD
    // Carriage return BE CAREFUL MAN!
    if choose.trim() == "1" {
        println!("Please, Introduce a value ");
        choose.clear(); // With this we cleared the last value we storage.
        io::stdin().read_line(&mut choose).expect("Please, enter an integer");
        let choose: i32 = choose.trim().parse().expect("Failed to convert to i32");
        ctof(choose);
    } else if choose.trim() == "2" {
        println!("Please, Introduce a value");
        choose.clear(); // With this we cleared the last value we storage.
        io::stdin().read_line(&mut choose).expect("Please, enter an integer");
        let choose: i32 = choose.trim().parse().expect("Failed to convert to i32");
        ftpc(choose);
    }
}


fn ctof(c: i32) {
    let celsius_to_fahrenheit: i32 = (c * 9 / 5) + 32;
    println!("Here is your conversion: {celsius_to_fahrenheit}")
}


fn ftpc(f: i32) {
    let fahrenheit_to_celsius: i32 = (f-32) * 5 / 9;
    println!("Here is your conversion: {fahrenheit_to_celsius}")
}

