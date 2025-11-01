use std::cmp;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// 1 - 1 - 1 - 1 - 1 -1
pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
    let mut half_of_the_list: Vec<i32> = Vec::new();
    let mut result = 0;
    let mut current = head.as_ref();
    let mut faster_pointer = head.as_ref();

    while faster_pointer.is_some() && faster_pointer.unwrap().next.is_some() {
        half_of_the_list.push(current.unwrap().val);

        if (current.is_some() && current.unwrap().next.is_some()) {
            current = current.unwrap().next.as_ref();
        }

        faster_pointer = faster_pointer.unwrap().next.as_ref();

        if faster_pointer.is_some() && faster_pointer.unwrap().next.is_some() {
            faster_pointer = faster_pointer.unwrap().next.as_ref();
        } else {
            break;
        }
    }

    let mut index = half_of_the_list.len() - 1;

    while (current.is_some()) {
        result = cmp::max(result, current.unwrap().val + half_of_the_list[index]);

        current = current.unwrap().next.as_ref();
        index = index - 1;
    }

    return result;
}

fn main() {
    println!("Hello, world!");
}
