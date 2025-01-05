use std::io;

fn main() {

    loop{
        println!("1 for addition, 2 for subtraction, 3 for mul, 4 for divide");
        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("failed");

        let operation: u8 =operation.trim().parse().expect("enter a valid num");

        println!("num1 =");
        let mut num1= String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("failed");
        let num1: i32 =num1.trim().parse().expect("enter valid num");

        println!("num2=");
        let mut num2 =String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("failed");
        let num2: i32= num2.trim().parse().expect("num not valid");

        let ans = match operation {
            1 => num1 + num2,
            2 => num1 - num2,
            3 => num1 * num2,
            4 => {
                if num2 ==0{
                    println!("div  by zero not allowed");
                    continue;
                }
                num1 / num2
            }
            _ => break
        };
        println!("result={ans}");
    }
}
