pub fn execute_instructions(n: i32, start_pos: Vec<i32>, s: String) -> Vec<i32> {
    let chars: Vec<char> = s.chars().collect();
    let mut result = Vec::new();

    for i in 0..chars.len() {
        let mut count = 0;
        let mut position = vec![start_pos[0], start_pos[1]];

        for j in i..chars.len() {
            let char = chars[j];

            if char == 'U' {
                position[0] = position[0] - 1;
            } else if char == 'D' {
                position[0] = position[0] + 1;
            } else if char == 'L' {
                position[1] = position[1] - 1;
            } else {
                position[1] = position[1] + 1;
            }

            if (position[0] < 0 || position[1] < 0 || position[0] >= n || position[1] >= n) {
                break;
            }

            count = count + 1;
        }

        result.push(count)
    }

    return result;
}
fn main() {
    println!("Hello, world!");
}
