// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    let mut previous = head.clone();
    let mut current = head.unwrap().next;
    let mut current_position = 1;

    while current.is_some() {
        if current_position >= m && current_position < n {
            let mut node = current.unwrap();
            let following = node.next;
            node.next = previous;
            previous = Some(node);
            current = following;
            println!("current_position >= m");
        } else {
            let node = current.unwrap();
            let following = node.next.clone();
            previous = Some(node);
            current = following;
        }

        current_position += 1;
        println!("--> prev: {:?}", previous.clone());
    }
    
    previous
}
