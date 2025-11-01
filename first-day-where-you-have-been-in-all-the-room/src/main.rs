pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
    let n = next_visit.len();
    const M: i64 = 1_000_000_007;
    /*
     *
     *
     * 1 + (current - memo[next_visit]) + 1
     * 0 0 1 2
     * 0 2 6 12
     * 6 + 1 + (6 - 2) + 1
     *
     */

    let mut memo: Vec<i64> = vec![0; n];
    let mut current_day: i64 = 0;

    for i in 0..n {
        if (i == n - 1) {
            return (current_day % M) as i32;
        }

        let visit = next_visit[i] as usize;
        memo[i] = current_day;

        if visit == i {
            current_day = (current_day + 2) % M;
            continue;
        }

        current_day = (current_day + 1 + (current_day - memo[visit as usize]) + M + 1) % M;
    }

    return (current_day % M) as i32;
}

fn main() {
    println!("Hello, world!");
}
