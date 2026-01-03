fn main() {
    let mut temp:String = String::new();
    println!("Welcome to the temperature converter! Press ctrl+c to exit at any time");
    loop {
        temp = String::new();
        println!("enter 1 to convert celsius to fahrenheit, 2 to convert fahrenheit to celsius");
        std::io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
        let choice: i32 = temp.trim().parse().expect("Failed to parse input");
        temp = String::new();
        println!("enter the temperature to convert");
        std::io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
        let num: f64 = temp.trim().parse().expect("Failed to parse input");
        match choice {
            1 => println!("the temperature in fahrenheit is {}", ctf(num)),
            2 => println!("the temperature in celsius is {}", ftc(num)),
            _ => println!("Invalid choice"),
        }
    }
}

fn ctf(x: f64) -> f64 {
    x * 9f64 / 5f64 + 32f64
}

fn ftc(x: f64) -> f64 {
    (x - 32f64) * 5f64 / 9f64
}
