// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let dummy: Option<Box<ListNode>> = None;
    if head.is_none() {
        return dummy;
    }

    let mut odd = vec![];
    let mut even = vec![];

    let mut i = 0;

    while head.is_some() {
        let mut node = head.unwrap();
        let following = node.next;
        node.next = dummy.clone();
        if i % 2 == 0 {
            even.push(node);
        } else {
            odd.push(node);
        }
        head = following;
        i += 1;
    }

    even.append(&mut odd);
    let mut previous: Option<Box<ListNode>> = even[even.len() - 1].clone().next;
    for mut node in even.into_iter().rev() {
        node.next = previous;
        previous = Some(node);
    }
    previous
}
