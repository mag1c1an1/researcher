use std::io;
macro_rules! parse_input {
    ($s:expr,$t:ident) => {
        $s.trim().parse::<$t>().unwrap()
    };
}
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let loop_times = parse_input!(input_line,i32);
    for _ in 0..loop_times {
        solve()
    }
    println!("test");
}

fn solve() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let nums: Vec<i32> = input_line.trim().split(" ").map(|v| v.parse::<i32>().unwrap()).collect();
    println!("{:?}",nums)
}