include std::io;
fn main() {
    
    let arr=[1,2,3,5,9,3,8];
    let tup=(1,2,3,5,9,3,8);

    let mut index = String::new;
    io::stdin()
        .read_line(&mut index)
        .expect("not valid");

    let index = index.trim().parse().expect("not valid");

    a= arr[index];
    println!("val in array at {index} is {a}");
}
