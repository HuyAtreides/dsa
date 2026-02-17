pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut result = Vec::new();
    potions.sort();

    for spell in spells {
        result.push(count(spell, &potions, success));
    }
    // 1 2 3 4 5 6 7
    // 0 1 2 3 4 5 6
    return result;
}

pub fn count(spell: i32, potions: &Vec<i32>, success: i64) -> i32 {
    let mut start = 0;
    let mut end = potions.len() - 1;

    while start < end {
        let middle = (end + start) / 2;

        if (spell as i64) * (potions[middle] as i64) >= success {
            end = middle;
        } else {
            start = middle + 1;
        }
    }

    if ((spell as i64) * (potions[end] as i64) < success) {
        return 0;
    }

    return (potions.len() - start) as i32;
}

fn main() {
    println!("Hello, world!");
}
