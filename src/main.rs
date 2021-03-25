fn main() {
    let my_tup: (u8, char, &str) = (255, 'h', "Hello!");
    let my_arr: [&str; 3] = [
        "string1",
        "second string",
        "another string"
    ];
    let arr2 = ["string1", "string2"];
    
    println!("Tuple: {:?}", my_tup);
    println!("Array: {:?}", my_arr);
    println!("Array2: {:?}", arr2);
}
