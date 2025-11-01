use core::num;

pub fn decode_ciphertext(encoded_text: String, rows: i32) -> String {
    let len = encoded_text.len();
    if (len == 0) {
        return encoded_text;
    }
    let num_of_columns = len / (rows as usize);
    let parts: Vec<Vec<char>> = encoded_text
        .as_bytes()
        .chunks(num_of_columns)
        .map(|chunk| chunk.iter().map(|&b| b as char).collect())
        .collect();
    let mut result = String::new();
    let mut extra_indent = 0;

    for i in 0..num_of_columns {
        let mut tmp_string = String::from("");
        for row in 0..rows {
            let part = &parts[row as usize];
            let index = row as usize + extra_indent;
            if (index >= num_of_columns) {
                break;
            }

            tmp_string.push(part[index]);
        }

        result += &tmp_string;

        extra_indent = extra_indent + 1;
    }

    result.trim_end().to_string()
}

fn main() {
    let result = decode_ciphertext(String::from("ch   ie   pr"), 3);
    let result1 = decode_ciphertext(String::from("iveo    eed   l te   olc"), 4);
    let result2 = decode_ciphertext(String::from("coding"), 1);
    println!("{}", result);
    println!("{}", result1);
    println!("{}", result)
}
