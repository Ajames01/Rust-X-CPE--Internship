use std::io; //import user input package

fn main() {
    println!("Enter the first number");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");

    println!("Enter the second number");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");

    println!("Enter the operation");
    let mut opp = String::new();
    io::stdin().read_line(&mut opp).expect("Failed to read the line");

    //eliminate white spaces from inputs
    let num1: f32 = input1.trim().parse().expect("Please input a number");
    let num2: f32 = input2.trim().parse().expect("Please input a number");
    let opps = opp.trim();

    //operations
    if opps == "+" {
        let result = num1 + num2;
        println!("The result is: {}", result);
    } else if opps == "-" {
        let result = num1 - num2;
        println!("The result is: {}", result);
    } else if opps == "/" {
        if num2 == 0.0 {
            println!("Error: Division by zero is not allowed");
        } else { 
            let result = num1 / num2;
            println!("The Quotient is {}", result);
        }
    } else if opps == "*" {
        let result = num1 * num2;
        println!("The product is {}", result); // Corrected output message
    } else {
        println!("Invalid input");
    }
}

