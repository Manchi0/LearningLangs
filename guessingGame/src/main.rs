use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing Numbers!!");

    let secret_num= rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter a number between 1-100");
        let mut numba=String::new();

        io::stdin()
            .read_line(&mut numba)
            .expect("Failed to read the line");
    
    

        let numba: u32= match numba.trim().parse(){
            Ok(num) =>num,
            Err(_) => continue,
        };

        println!("You guessed: {}", numba);

        match numba.cmp(&secret_num){
            Ordering::Less =>println!("Too small"),
            Ordering:: Greater=> println!("Too big"),
            Ordering::Equal => {
                println!("Omg waow so smart!!");
                break;
            }
        }
    }
}

// fn main() {
//     println!("Guessing Numbers!!");

//     let secret_num= rand::thread_rng()::gen_range(1..=100);
//     println!("Sec Num={secret_num}");
//     println!("Enter a number between 1-100");

//     loop {
//         let mut numba=String::new();

//         io::stdin()
//             .read_line(&mut numba)
//             .expect("Failed to read the line");
    
    

//         let numba: u32= numba.trim().parse().expect("Please input a Number!!!!")
//         println!("You guessed: {}", numba);

//         match numba.cmp(&secret_num){
//             Ordering::Less =>println!("Too small"),
//             Ordering::Equal => println!("Correct!!"),
//             Ordering::Greater => {
//                 println!("Too big!!");
//                 break
//              }
//         }
//     }
// }






// fn main() {
//     println!("Guessing Numbers!!");

//     let secret_num= rand::thread_rng()::gen_range(1..=100);
//     println!("Sec Num={secret_num}");
//     println!("Enter a number between 1-100");
//     let mut numba=String::new();

//     io::stdin()
//         .read_line(&mut numba)
//         .expect("Failed to read the line");
//     println!("You guessed: {}", numba);
// }
