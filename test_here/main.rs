struct Solution;

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn main() {

    // let l1 = Some(Box::new(ListNode{
    //     val: 2,
    //     next: Some(Box::new(ListNode{
    //         val: 4,
    //         next: Some(Box::new(ListNode::new(9)))
    //     }))
    // }));

    // let l2 = Some(Box::new(ListNode{
    //     val: 5,
    //     next: Some(Box::new(ListNode{
    //         val: 6,
    //         next: Some(Box::new(ListNode{
    //             val: 4,
    //             next: Some(Box::new(ListNode::new(9)))
    //         }))
    //     }))
    // }));

    // let l1 = Some(Box::new(ListNode {
    //     val: 9,
    //     next: Some(Box::new(ListNode {
    //         val: 9,
    //         next: Some(Box::new(ListNode {
    //             val: 9,
    //             next: Some(Box::new(ListNode {
    //                 val: 9,
    //                 next: Some(Box::new(ListNode {
    //                     val: 9,
    //                     next: Some(Box::new(ListNode {
    //                         val: 9,
    //                         next: Some(Box::new(ListNode::new(9)))
    //                     }))
    //                 }))
    //             }))
    //         }))
    //     }))
    // }));

    // let l2 = Some(Box::new(ListNode {
    //     val: 9,
    //     next: Some(Box::new(ListNode {
    //         val: 9, 
    //         next: Some(Box::new(ListNode {
    //             val: 9,
    //             next: Some(Box::new(ListNode::new(9)))
    //         }))
    //     }))
    // }));

    let l1 = Some(Box::new(ListNode{
        val: 1,
        next: Some(Box::new(ListNode{
            val: 2,
            next: Some(Box::new(ListNode{
                val: 3,
                next: Some(Box::new(ListNode::new(4)))
            }))
        }))
    }));

    let l2 = Some(Box::new(ListNode{
        val: 4,
        next: Some(Box::new(ListNode::new(5)))
    }));


    println!("{:?}", Solution::add_two_numbers(l1, l2));

}


impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>,l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut dummy_head = ListNode::new(0);
        let mut current = &mut dummy_head;
        let mut list1 = l1;
        let mut list2 = l2;
        let mut carry: i32 = 0;

        while list1 != None || list2 != None {
            let sum = match (&list1, &list2) {
                (Some(list1), Some(list2)) => list1.val + list2.val + carry,
                (Some(list1), None) => list1.val + carry,
                (None, Some(list2)) => list2.val + carry,
                (None, None) => carry
            };

            carry = sum / 10;

            current.next = Some(Box::new(ListNode::new(sum % 10)));

            current = current.next.as_mut().unwrap();

            list1 = if list1 != None { list1.unwrap().next } else { list1 };
            list2 = if list2 != None { list2.unwrap().next } else { list2 };
        }
        if carry > 0 {
            current.next = Some(Box::new(ListNode::new(carry)));
        }

        dummy_head.next
    }
}