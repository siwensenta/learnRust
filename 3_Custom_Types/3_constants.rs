// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;
static mut LANGUAGE2: &str = "Rust";
fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    //THRESHOLD = 5;
    // FIXME ^ Comment out this line
    
    unsafe{
        println!("It is {}", LANGUAGE2);
        LANGUAGE2 = "python";
        println!("But now it is {}", LANGUAGE2);
    }
    
    
}
