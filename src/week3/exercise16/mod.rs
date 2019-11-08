#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
