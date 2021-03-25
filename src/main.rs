fn main() {
    // pointers and references
    let a = 65;
    let b = &a;
    let c = *b;

    let d = &b; // &&a

    println!("{} {} {} {}", a, b , c, d); // Rust automatically dereferences pointers

    println!("{} {} {} {}", a, *b, c, **d); // Output will be same as above
}
