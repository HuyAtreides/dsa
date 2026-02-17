pub fn is_valid_serialization(preorder: String) -> bool {
    let characters: Vec<&str> = preorder.split(",").collect();
    let n = characters.len();
    let mut count: usize = 0;
    dfs(&mut count, &characters);
    println!("{}", count);
    return count == n - 1;
}

fn dfs(index: &mut usize, preorder: &Vec<&str>) {
    if *index >= preorder.len() {
        return;
    }

    let char = preorder[*index];
    if char != "#" {
        *index += 1;
        dfs(index, preorder);

        *index += 1;
        dfs(index, preorder);
    }
}

fn main() {
    println!(
        "result 1: {}",
        is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()),
    );

    println!(
        "result 1: {}",
        is_valid_serialization("9,#,1,#,#".to_string())
    );
}
