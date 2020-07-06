use std::io;

fn main() {
    
    println!("Calculate perimeter of a Square.\n");

    println!("What is length?: ");

    let mut l = String::new();

    io::stdin().read_line(&mut l).expect("failed");

    let length = match l.trim().parse::<f32>() {
        Ok(length) => length,
        Err(_) => panic!("fuck!"),
    };

    println!("What is breadth?: ");

    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("failed");
    let breadth = match b.trim().parse::<f32>() {
        Ok(breadth) => breadth,
        Err(_) => panic!("fuck!"),
    };
        
    println!("perimeter is {}", length + breadth); 
}
