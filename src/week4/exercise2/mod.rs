// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    let mut start = head.clone();
    let mut previous_start: Option<Box<ListNode>> = None;

    for _ in 0..(m - 1) {
        previous_start = start.clone();
        start = start.unwrap().next;
    }

    let mut end = start.clone();
    let mut previous_end: Option<Box<ListNode>> = None;
    let mut tail: Option<Box<ListNode>> = None;

    for _ in m..=n {
        let mut node = end.unwrap();
        let following = node.next;
        tail = following.clone();
        node.next = previous_end;
        previous_end = Some(node);
        end = following;
    }

    // let mut i = 1;
    // let mut head  = head;
    // while head.is_some() {

    //     if i == m {

    //     }

    //     if i == n {

    //     }

    //     i += 1;
    // }

    println!("previous_end: {:?}, tail: {:?}", previous_end, tail);

    None
}

fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut previous: Option<Box<ListNode>> = None;
    let mut current = head;

    while current.is_some() {
        let mut node = current.unwrap();
        let following = node.next;
        node.next = previous;
        previous = Some(node);
        current = following;
    }

    previous
}
