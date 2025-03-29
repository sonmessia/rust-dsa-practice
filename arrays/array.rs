fn main() {
    let arr = [1, 2, 3, 4, 5];
    for i in 0..arr.len() {
        println!("{}", arr[i]);
    }
    for &num in arr.iter() {
        println!("{}", num);
    }
}