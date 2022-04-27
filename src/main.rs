use std::io;

fn main() {
    println!("Simple Search!");

    // create haystack
    let mut haystack: [u8; 100] = [0; 100];

    // initiate haystack
    for number in 0u8..100 {
        haystack[number as usize] = number + 1;
    }

    // get the needle from console
    let mut buffer = String::new();
    
    // Error handling
    match io::stdin().read_line(&mut buffer) {
        Ok(n) => {
            println!("{} bytes read", n);
        }
        Err(error) => println!("error: {}", error),
    }

    // convert string to integer
    let needle: u8 = buffer.trim().parse().unwrap();

    println!("We are looking for {}", needle);

    // index for walking on array
    let mut index:usize = 0;

    loop {
        println!("Step {}", index + 1);
        
        println!("Is it {} ?", haystack[index]);

        if haystack[index] == needle {
            println!("Hooray!");
            break;
        }   

        index = index + 1;
    }

    println!("We found it in {} steps!", index + 1);
}
