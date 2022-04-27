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

    // the first and last index
    let mut low:usize = 0;
    let mut high:usize = haystack.len() - 1;

    loop {

        let mid:usize = (low + high) / 2;

        let guess: u8 = haystack[mid];

        println!("Step {}", index + 1);
        
        println!("Is it {} ?", guess);

        if guess == needle {
            println!("YESS!! Hooray!");
            break;
        } else if guess < needle {
                low = mid + 1;
                println!("it is higher than {}", guess);
        } else {
            high = mid - 1;
            println!("it is lower than {}", guess);
        }

        index = index + 1;
    }

    println!("We found it in {} steps!", index + 1);
}
