fn main() {
    let my_tup: (u8, char, &str) = (255, 'h', "Hello!");
    let my_arr: [&str; 3] = [
        "string1",
        "second string",
        "another string"
    ];
    let arr2 = ["string1", "string2"];
    
    println!("Tuple: ({}, {}, {})", my_tup.0, my_tup.1, my_tup.2);
    println!("Array: [{}, {}, {})]", my_arr[0], my_arr[1], my_arr[2]);
    println!("Array2: [{}, {}]", arr2[0], arr2[1]);
}
