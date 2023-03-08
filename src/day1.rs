use std::io;

fn main() {
    let mut mod_ar = vec![];

    println!("amount: ");
  
    let mut amt = String::new();

    io::stdin()
        .read_line(&mut amt)
        .expect("Fatal Error");

    let amt = match amt.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Defaulting to 10.");
            10
        }
    };

    for _ in 0..amt+1{
        mod_ar.push(amt);
    }

    for (item, value) in mod_ar.iter().enumerate(){
        println!("{} : {}",item,value);
    }
}
