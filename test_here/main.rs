// Definition for singly-linked list.
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

// impl Solution {
//     pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
//     }
// }

fn main() {

    let list1 = Some(Box::new(ListNode{
                    val: 2,
                    next: Some(Box::new(ListNode{
                        val: 4,
                        next: Some(Box::new(ListNode::new(3)))
                    }))
                }));

    let list2 = Some(Box::new(ListNode{
        val: 5,
        next: Some(Box::new(ListNode{
            val: 6,
            next: Some(Box::new(ListNode::new(4)))
        }))
    }));

    //println!("{:?}", list2);

    some_to_string(list1, list2);
}

fn some_to_string(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>){


    let mut l1 = &list1;
    let mut s1: String = "".to_string();

    let mut l2 = &list2;
    let mut s2: String = "".to_string();

    let mut sum: i32;

    // Create 2 Strings of val
    loop {
        if let None = l1 {
            sum = s1.parse::<i32>().unwrap() + s2.parse::<i32>().unwrap();
            break;
        } else {
            s1 += &l1.as_ref().unwrap().val.to_string();
            s2 += &l2.as_ref().unwrap().val.to_string();

            l1 = &l1.as_ref().unwrap().next;
            l2 = &l2.as_ref().unwrap().next;
        }
    }

    for (i, c) in s1.chars().enumerate() {

        println!("{} {:?}", i, c.to_digit(10).unwrap());
    }
    
    println!("{}", sum);


    // 807
    let base = 10;
    while sum != 0 {

        println!("{:?}", sum % base);
        // sum % base = 807 / 10 => 7
        // sum % base = 80 / 10 => 0

        sum /= base;
        // sum /= base
        // 80.7 => 80
        // 80 => 8
        println!("{:?}", sum);

    }

    println!("{}", sum);

    // let mut list3 = Some(Box::new(ListNode{
    //     val: ,
    // }))

    // loop {
    //     if sum == 0{

    //         break;
    //     }else{
    //         return Some(Box::new(ListNode{
    //             val:
    //         }));
    //     }
    // }

    // Some(Box::new(ListNode{
    //     val: 5,
    //     next: Some(Box::new(ListNode{
    //         val: 6,
    //         next: Some(Box::new(ListNode::new(4)))
    //     }))
    // }));
}