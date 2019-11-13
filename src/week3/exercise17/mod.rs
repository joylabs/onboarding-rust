#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, mut n: i32) -> Option<Box<ListNode>> {
    let mut vec = vec![];
    let mut current = head.clone();
    let mut i = 1;
    let dummy: Option<Box<ListNode>> = None;

    if !head.is_some() {
        return dummy;
    }

    //println!("curr = {:?}", current.clone());
    while current.is_some() {
        let mut draft = current.unwrap();
        if i == n + 1 {
            vec.push(Some(draft.clone()));
            break;
        }
        let previous = draft.next;
        draft.next = dummy.clone();
        vec.push(Some(draft.clone()));
        current = previous;
        i += 1;
    }

    let mut helper = vec.clone();
    for (i, val) in vec.into_iter().enumerate() {
        if i as i32 >= m - 1 {
            if i as i32 == n || i as i32 == n - 1 {
                break;
            }
            let before = val;
            let after = helper[(n - 1) as usize].clone();
            helper[i] = after;
            helper[(n - 1) as usize] = before;

            n -= 1;
        }
    }

    let mut previous: Option<Box<ListNode>> = helper[helper.len() - 1].clone().unwrap().next;
    for list in helper.into_iter().rev() {
        let mut node = list.unwrap();
        node.next = previous;
        previous = Some(node);
    }

    previous
}
