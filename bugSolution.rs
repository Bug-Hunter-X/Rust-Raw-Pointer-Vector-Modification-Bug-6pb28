fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Safe and correct way to modify the vector
    println!("v: {:?}", v);
}