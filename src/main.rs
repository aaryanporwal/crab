fn main() {
    // 3. For 
    let arr = [ 2, 3, 5, 3 ];

    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }

    for a in &arr { // We don't need counter variable here
        println!("{}", a);
    }
}

