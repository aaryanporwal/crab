fn main() {
    let mut arr = [ 2, 3, 5, 3 ];

    square(&mut arr[1]);
    println!("{:?}", arr);
}

fn square(x: &mut u32) {
    *x = (*x) * (*x)
}

