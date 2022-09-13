use std::collections::HashSet;
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
}

fn solve() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line,i32);
    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    let nums: Vec<i32> = input_line.trim().split(" ").map(|v| v.parse::<i32>().unwrap()).collect();
    sort_zero(nums, n);
}

fn sort_zero(mut nums: Vec<i32>, _n: i32) {
    let mut ret = 0;
    while !is_sorted(&nums) {
        for i in (0..nums.len()).rev() {
            if nums[i - 1] > nums[i] {
                ret += unique_len(&mut nums, i - 1);
                break;
            }
        }
    }
    println!("{}",ret);
}

fn is_sorted(nums: &[i32]) -> bool {
    for i in 0..nums.len() - 1 {
        if nums[i] > nums[i + 1] {
            return false;
        }
    }
    true
}

fn unique_len(nums: &mut [i32], idx: usize) -> i32 {
    let mut set = HashSet::new();
    for i in 0..=idx {
        if nums[i] == 0 { continue; }
        set.insert(nums[i]);
        nums[i] = 0;
    }
    for i in idx + 1..nums.len() {
        if set.contains(&nums[i]) {
            nums[i] = 0;
        }
    }
    set.len() as i32
}
