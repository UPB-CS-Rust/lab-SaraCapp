fn main() {
    let input: [i32; 0] = [];

    let largest = input.iter().max().unwrap_or(&0);
    let smallest = input.iter().min().unwrap_or(&0);

    println!("{} is largest and {} is smallest", largest, smallest);
}
