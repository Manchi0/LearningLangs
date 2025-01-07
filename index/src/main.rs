use std::io;

fn main() {
    println!("enter a val");
    let arr=[1,2,3,5,9,3,8];

    let mut ind = String::new();
    io::stdin()
        .read_line(&mut ind)
        .expect("not valid");

    let ind: usize = ind.trim().parse().expect("not valid");


    let a= arr[ind];
    println!("val in array at {ind} is {a}");
}
