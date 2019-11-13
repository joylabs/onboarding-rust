use onboarding_rust::week3::exercise17::{reverse_between, ListNode};

#[test]
fn test_week4_exercise1_example1() {
    let input = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));

    let output = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    assert_eq!(output, reverse_between(input, 2, 4));
}
