fn main() {
    let mut temp: String = String::new();
    std::io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let num: u128 = temp.trim().parse().expect("Failed to parse input");
    println!("{}", calc(num))
}

fn calc(x: u128) -> u128 {
    let mut i: u128 = 1;
    let mut a=0u128;
    let mut b=1u128;
    let mut c=1u128;
    while i <= x {
        i += 1;
        c=a+b;
        a=b;
        b=c;
    }
    a
}
