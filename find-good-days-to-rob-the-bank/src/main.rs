use std::cmp::{max, min};

pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut streak = 0;
    let n = security.len();
    let mut backward_streak = 0;
    let mut suffix: Vec<bool> = vec![false; n];

    for i in (0..n).rev() {
        if security[i] <= security[min(i + 1, n - 1)] {
            backward_streak = backward_streak + 1;
        } else {
            backward_streak = 1;
        }

        suffix[i] = backward_streak - 1 >= time;
    }

    for i in (0..n) {
        let greeting = if i == 0 { 0 } else { i - 1 };
        if security[greeting] >= security[i] {
            streak = streak + 1;
        } else {
            streak = 1;
        }

        if (streak - 1 >= time && suffix[i]) {
            result.push(i as i32);
        }
    }

    return result;
}

fn main() {
    println!("Hello, world!");
}
