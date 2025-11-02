use std::cmp::max;

pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
    let mut result_when_add_pattern_0 = 0;
    let mut result_when_add_pattern_1 = 0;
    let mut count_pattern_0 = 0;
    let mut count_pattern_1 = 0;
    let chars: Vec<char> = pattern.chars().collect();
    let pattern_char_0 = chars[0];
    let pattern_char_1 = chars[1];

    for char in text.chars() {
        if (char == pattern_char_1) {
            result_when_add_pattern_0 = result_when_add_pattern_0 + count_pattern_0;
            count_pattern_1 = count_pattern_1 + 1;
        }

        if (char == pattern_char_0) {
            count_pattern_0 = count_pattern_0 + 1;
        }
    }

    result_when_add_pattern_0 = result_when_add_pattern_0 + count_pattern_1;
    count_pattern_0 = 0;
    count_pattern_1 = 0;

    for char in text.chars().rev() {
        if (char == pattern_char_0) {
            result_when_add_pattern_1 = result_when_add_pattern_1 + count_pattern_1;
            count_pattern_0 = count_pattern_0 + 1;
        }

        if (char == pattern_char_1) {
            count_pattern_1 = count_pattern_1 + 1;
        }
    }

    result_when_add_pattern_1 = result_when_add_pattern_1 + count_pattern_0;

    return max(result_when_add_pattern_0, result_when_add_pattern_1);
}
fn main() {
    println!("Hello, world!");
}
