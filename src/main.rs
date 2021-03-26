// This is how you declare a function in rust
//
// fn function_name(parameters) -> return type {

//}

fn main() {
    let mut arr = [ 2, 3, 5, 3 ];
    arr[1] = square(arr[1]);
    println!("{:?}", arr);
    
    arr = [1,2,3,4];
    arr[1] = square2(arr[1]);
    println!("{:?}", arr);

}

fn square(x: u32) -> u32 {
    return x*x;
}

fn square2(x: u32) -> u32 {
    x*x
}

