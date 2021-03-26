fn main() 
{
    
    // 1. loop
    
    // Values to variables can be assigned using loops ➰
    
    let mut counter = 0;

    let a = loop 
    {
        counter += 1;

        if counter >= 10 
        {
            break counter * 2;
        }
    }; // loop needs to end with a semicolon
    println!("counter: {}, a: {}", counter, a);
}

