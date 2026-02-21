use std::cmp::max;

// https://leetcode.com/problems/maximum-white-tiles-covered-by-a-carpet/description/?envType=problem-list-v2&envId=5ah4tt07
fn main() {}

pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
    let mut stack: Vec<(i32, i32)> = tiles.iter().map(|tile| (tile[0], tile[1])).collect();
    stack.sort_by(|a, b| a.0.cmp(&b.0));
    let mut prefix: Vec<i64> = vec![0; stack.len()];

    for i in 0..stack.len() {
        if i == 0 {
            prefix[i] = prefix[0] + (stack[i].1 - stack[i].0 + 1) as i64;
        } else {
            prefix[i] = prefix[i - 1] + (stack[i].1 - stack[i].0 + 1) as i64;
        }
    }

    let mut result = 0;

    for i in 0..stack.len() {
        let l = stack[i].0 + carpet_len - 1;
        let index = binary_search(i, l, &stack);
        let end = stack[index].1;

        let acquire = if l < end {
            prefix[index] - prefix[i] + (stack[i].1 - stack[i].0 + 1 - (end - l)) as i64
        } else {
            prefix[index] - prefix[i] + (stack[i].1 - stack[i].0 + 1) as i64
        };

        result = max(result, acquire);
    }

    result as i32
}

fn binary_search(start: usize, size: i32, stack: &[(i32, i32)]) -> usize {
    let mut s = start;
    let mut end = stack.len() - 1;

    while s <= end {
        let middle = (s + end) / 2;

        if stack[middle].1 < size {
            s = middle + 1;
        } else if stack[middle].1 >= size && size >= stack[middle].0 {
            return middle;
        } else {
            end = middle - 1;
        }
    }

    end
}

#[cfg(test)]
mod test {
    use crate::maximum_white_tiles;

    #[test]
    pub fn basic_case_test() {
        let actual = maximum_white_tiles(vec![vec![1, 2], vec![3, 4]], 10);
        assert_eq!(actual, 4);
    }
    #[test]
    pub fn unsorted() {
        let actual = maximum_white_tiles(vec![vec![3, 4], vec![1, 2]], 10);
        assert_eq!(actual, 4);
    }

    #[test]
    pub fn more_complicated() {
        let actual = maximum_white_tiles(vec![vec![1, 2], vec![3, 4]], 3);
        assert_eq!(actual, 4);
    }

    #[test]
    pub fn multiple_cover() {
        let actual = maximum_white_tiles(
            vec![
                vec![1, 5],
                vec![10, 11],
                vec![12, 18],
                vec![20, 25],
                vec![30, 32],
            ],
            10,
        );
        assert_eq!(actual, 9);
    }
}
