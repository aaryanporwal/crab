#[derive(Debug)] // For easily printing our struct
struct Point(i32, i32, i32); // Structs without field names are possible!

#[derive(Debug)] 
struct Color(i32, i32, i32);

#[derive(Debug)]
struct What{} // Empty fields are possible too! // Notice no semicolon at end

fn main() {
    let black = Color(0,0,0);
    let zero = Point(1,1,1);
    let ikr = What{};

    println!("{:?}", black);
    println!("{:?}", zero);
    println!("{:?}", ikr);

}
