fn main() {
    const N: i32 = 5;
    let x: i32 = 100;
    let add = |a, b| a + b;
    println!("{} + {} = {} {}", N, x, sum(N, x), add(N, x));
}

fn sum(a: i32, b: i32) -> i32 {
  a + b
}
