fn main() {
    let x: u32 = 16;
    let y: f32 = 12.4;
    let a: bool = true;
    let ch: char = '😺'; // char in rust is 4 bytes
    let s: &str = "Hello, world! 😺";

    println!("x: {}, y: {}", x , y);
    println!("a: {}", a);
    println!("ch: {}\ns: {}", ch, s);
}
