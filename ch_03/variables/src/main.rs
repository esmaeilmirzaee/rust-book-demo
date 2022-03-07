fn main() {
    let mut x = 5;
    x += 1;
    println!("The outer x value is {}", x);
    {
        let mut x = "16";
        println!("The inner x value is {}", x);
        x = "161";
        println!("The inner x value is {}", x);
    }
    let x: u32 = "42".parse().expect("failed to cast");
    println!("The finished outer x value is {}", x);
    let sum = 3.2 + 2.7;
    let difference = 3.2 - 2.1;
    let product = 3.4 * 2.12;
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    let reminder = 43 % 2;
    println!("{} {} {} {} {} {}", sum, difference, product, quotient, floored, reminder);

    // prints the user's required value in an array
    let a: [u32; 5] = [1, 2, 3, 4, 5];
    let mut index = String::new();
    println!("Please enter an array index; between 0 and {}", a.len() - 1);
    std::io::stdin().read_line(&mut index).expect("failed to receive data");

    let index: usize = index.trim().parse().expect("failed to cast");
    let element = a[index];

    println!("The value of the element at index {} is {}.", index, element);
}
