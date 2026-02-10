/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
//   todo!("Implement this")
let mut res = 1;
while n != 1 {
    if n % 2 == 0 {
        n = n / 2;
    }
    else {
        n = 3*n + 1;
    }
    res = res + 1
}
res
}

fn main() {
    println!("Length: {}", collatz_length(11)); // should be 15
}