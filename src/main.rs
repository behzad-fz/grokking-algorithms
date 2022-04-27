fn main() {
    println!("Simple Search!");

    // create haystack
    let mut haystack: [u8; 100] = [0; 100];

    // initiate haystack
    for number in 0u8..100 {
        haystack[number as usize] = number + 1;
    }

    // get needle from user
    let needle: u8 = 50;

    // index for walking on array
    let mut index:usize = 0;

    loop {
        println!("Step {}", index + 1);
        
        println!("Is it {} ?", haystack[index]);

        if haystack[index] == needle {
            println!("Hooray");
            break;
        }   

        index = index + 1;
    }

    println!("We found it in {} steps!", index + 1);
}
