// 35, Search Insert Position

use std::io;

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let left = 0;
        let right = nums.len() as i32 - 1;

        Solution::solve(&nums, left, right, target)
    }

    pub fn solve(nums: &Vec<i32>, left: i32, right: i32, target: i32) -> i32 {
        if left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            }

            if nums[mid as usize] > target {
                return Solution::solve(nums, left, mid - 1, target);
            }
            return Solution::solve(nums, mid + 1, right, target);
        }
        left
    }
}

fn main() {
    let mut inp = String::new();
    let mut t = String::new();

    io::stdin().read_line(&mut inp).expect("No input");
    io::stdin().read_line(&mut t).expect("No input");

    let arr: Vec<i32> = inp
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let target: i32 = t.trim().parse().expect("Not a number");

    let r = Solution::search_insert(arr, target);

    println!("{:?}", r);
}

/////////////////////////////////////////////////////////

// // 28, Find the index of the first occurence in a string
//
// use std::io;
//
// struct Solution {}
//
// impl Solution {
//     pub fn str_str(haystack: String, needle: String) -> i32 {
//         if haystack.len() == needle.len() && haystack == needle {
//             return 0;
//         }
//
//         if haystack.len() < needle.len() {
//             return -1;
//         }
//
//         let window_size = needle.len();
//
//         let mut cur_word: &str;
//
//         for i in 0..haystack.len() {
//             if i + window_size > haystack.len() {
//                 return -1;
//             }
//             cur_word = &haystack[i..(i + window_size)];
//
//             if cur_word == needle {
//                 return i as i32;
//             }
//         }
//         -1
//     }
// }
//
// fn main() {
//     let mut inp1 = String::new();
//     let mut inp2 = String::new();
//
//     io::stdin().read_line(&mut inp1).expect("No input");
//     io::stdin().read_line(&mut inp2).expect("No input");
//
//     inp1 = inp1.trim().to_string();
//     inp2 = inp2.trim().to_string();
//
//     let r = Solution::str_str(inp1, inp2);
//     println!("{:?}", r);
// }

///////////////////////////////////////////////

// // 27, Remove element
//
// use std::io;
//
// struct Solution {}
//
// impl Solution {
//     pub fn remove_element(nums: &mut [i32], val: i32) -> i32 {
//         if nums.is_empty() {
//             return 0;
//         }
//         let mut k: i32 = 0;
//
//         for i in 0..nums.len() {
//             if nums[i] != val {
//                 nums[k as usize] = nums[i];
//                 k += 1;
//             }
//         }
//         k
//     }
// }
//
// fn main() {
//     let mut inp_str = String::new();
//     let mut val_str = String::new();
//
//     io::stdin().read_line(&mut inp_str).expect("Wrong input");
//     io::stdin().read_line(&mut val_str).expect("Wrong input");
//
//     let mut vec_inp: Vec<i32> = inp_str
//         .trim()
//         .chars()
//         .map(|x| x.to_string().parse().expect("Not a number"))
//         .collect();
//
//     let val: i32 = val_str.trim().parse().expect("Not a number");
//
//     let k = Solution::remove_element(&mut vec_inp, val);
//
//     println!("{:?}", k);
// }
//
////////////////////////////////////////////

// // 21. Merge two sorted list
//
// use std::io;
//
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }
//
// struct Solution {}
//
// impl Solution {
//     pub fn merge_two_lists(
//         l1: Option<Box<ListNode>>,
//         l2: Option<Box<ListNode>>,
//     ) -> Option<Box<ListNode>> {
//         let mut head: Box<ListNode> = Box::new(ListNode::new(0));
//         let mut current = &mut head;
//         let mut list1 = l1;
//         let mut list2 = l2;
//
//         while list1.is_some() || list2.is_some() {
//             if list1.is_none() {
//                 current.next = list2;
//                 break;
//             } else if list2.is_none() {
//                 current.next = list1;
//                 break;
//             } else {
//                 let node1 = list1.as_mut().unwrap();
//                 let node2 = list2.as_mut().unwrap();
//
//                 if node1.val < node2.val {
//                     current.next = Some(Box::new(ListNode::new(node1.val)));
//                     list1 = node1.next.take();
//                 } else {
//                     current.next = Some(Box::new(ListNode::new(node2.val)));
//                     list2 = node2.next.take();
//                 }
//                 current = current.next.as_mut().unwrap();
//             }
//         }
//
//         head.next
//     }
//
//     pub fn array_to_linked_list(arr: Vec<i32>) -> Option<Box<ListNode>> {
//         if arr.is_empty() {
//             return None;
//         }
//
//         let mut head = Box::new(ListNode::new(arr[0]));
//         let mut current = &mut head;
//
//         for &el in arr.iter().skip(1) {
//             let node = Box::new(ListNode::new(el));
//             current.next = Some(node);
//             current = current.next.as_mut().unwrap();
//         }
//         Some(head)
//     }
//
//     pub fn print_linked_list(head: &mut Option<Box<ListNode>>) {
//         let mut current = head.as_mut();
//
//         while let Some(node) = current {
//             // Safe pattern matching
//             print!("{:?} ", node.val); // Print the value
//             current = node.next.as_mut();
//         }
//     }
// }
//
// fn main() {
//     let mut inp1 = String::new();
//     io::stdin().read_line(&mut inp1).expect("Wrong input");
//
//     let mut inp2 = String::new();
//     io::stdin().read_line(&mut inp2).expect("Wrong input");
//
//     let arr1_num: Vec<i32> = inp1
//         .trim()
//         .split(" ")
//         .map(|x| x.parse().expect("Not a number"))
//         .collect();
//
//     let arr2_num: Vec<i32> = inp2
//         .trim()
//         .split(" ")
//         .map(|x| x.parse().expect("Not a number"))
//         .collect();
//
//     let head1 = Solution::array_to_linked_list(arr1_num);
//     let head2 = Solution::array_to_linked_list(arr2_num);
//
//     let mut h1 = Solution::merge_two_lists(head1, head2);
//
//     Solution::print_linked_list(&mut h1);
// }

///////////////////////////////////////////

// // 20, Valid parenthesis
//
// use std::io;
//
// struct Solution {}
//
// impl Solution {
//     pub fn is_valid(s: String) -> bool {
//         let mut stack: Vec<char> = Vec::new();
//         let char_arr: Vec<char> = s.chars().collect();
//
//         for el in char_arr {
//             if el == '{' || el == '(' || el == '[' {
//                 stack.push(el);
//             } else {
//                 let last = match stack.pop() {
//                     Some(l) => l,
//                     None => return false,
//                 };
//
//                 if (el == '}' && last != '{')
//                     || (el == ')' && last != '(')
//                     || (el == ']' && last != '[')
//                 {
//                     return false;
//                 }
//             }
//         }
//
//         stack.is_empty()
//     }
// }
//
// fn main() {
//     let mut str_inp = String::new();
//
//     io::stdin().read_line(&mut str_inp).expect("Wrong input");
//
//     let result = Solution::is_valid(str_inp.trim().to_string());
//
//     println!("{:?}", result);
// }

///////////////////////////////////////////////////////////

// // 14, Longesr Common Prefix
//
// use std::{cmp::min, io};
//
// struct Solution {}
//
// impl Solution {
//     pub fn longest_common_prefix(strs: Vec<String>) -> String {
//         if strs.is_empty() {
//             return String::new();
//         }
//         Solution::solve(&strs, 0, (strs.len() - 1) as i32)
//     }
//
//     pub fn solve(strs: &Vec<String>, left: i32, right: i32) -> String {
//         if left == right {
//             return strs[left as usize].to_string();
//         }
//
//         let mid = left + (right - left) / 2;
//
//         let left_el = Solution::solve(strs, left, mid);
//         let right_el = Solution::solve(strs, mid + 1, right);
//
//         Solution::common_prefix(&left_el, &right_el)
//     }
//
//     pub fn common_prefix(left_el: &str, right_el: &str) -> String {
//         let len_min = min(left_el.len(), right_el.len());
//
//         let vec_left: Vec<char> = left_el.chars().collect();
//         let vec_right: Vec<char> = right_el.chars().collect();
//
//         let mut res = String::new();
//         for i in 0..len_min {
//             if vec_left[i] != vec_right[i] {
//                 return res;
//             } else {
//                 res.push(vec_left[i]);
//             }
//         }
//         res
//     }
// }
//
// fn main() {
//     let mut inp_str = String::new();
//
//     io::stdin()
//         .read_line(&mut inp_str)
//         .expect("No line provide");
//
//     let words: Vec<&str> = inp_str.trim().split(" ").collect();
//
//     let words_vec: Vec<String> = words.iter().map(|&s| s.to_string()).collect();
//
//     let result = Solution::longest_common_prefix(words_vec);
//
//     println!("{:?}", result);
// }

///////////////////////////////////////////////////////////
// 13, Roman to Integer

// use std::{char, collections::HashMap, io};
//
// struct Solution {}
//
// impl Solution {
//     pub fn roman_to_int(s: String) -> i32 {
//         let map: HashMap<char, i32> = HashMap::from([
//             ('I', 1),
//             ('V', 5),
//             ('X', 10),
//             ('L', 50),
//             ('C', 100),
//             ('D', 500),
//             ('M', 1000),
//         ]);
//
//         let mut result: i32 = 0;
//
//         let mut last_el = 0;
//         for el in s.chars().rev() {
//             match map.get(&el) {
//                 Some(&val) => {
//                     if val < last_el {
//                         result -= val;
//                     } else {
//                         result += val;
//                     }
//
//                     last_el = val;
//                 }
//                 None => {
//                     break;
//                 }
//             }
//         }
//
//         result
//     }
// }
//
// fn main() {
//     let mut inp_str = String::new();
//
//     io::stdin()
//         .read_line(&mut inp_str)
//         .expect("No input provided");
//
//     let inp_str = inp_str.trim().to_string();
//     let result = Solution::roman_to_int(inp_str);
//     println!("{:?}", result);
// }

////////////////////////////////////////////////

// // 9. Palindrom number
//
// use std::io;
//
// struct Solution {}
//
// impl Solution {
//     pub fn is_palindrome(x: i32) -> bool {
//         let x_str: Vec<char> = x.to_string().chars().collect();
//
//         let mut left: usize = 0;
//         let mut right: usize = x_str.len() - 1;
//
//         while left < right {
//             if x_str[left] != x_str[right] {
//                 return false;
//             }
//             left += 1;
//             right -= 1;
//         }
//
//         true
//     }
// }
// fn main() {
//     let mut inp_str = String::new();
//
//     io::stdin()
//         .read_line(&mut inp_str)
//         .expect("No string provided");
//
//     let inp_num: i32 = inp_str.trim().parse().expect("Not a number");
//
//     let result = Solution::is_palindrome(inp_num);
//
//     println!("{:?}", result);
// }
//
// ///////////////////////////////////////////////////
//
// // 1. TWO SUM
//
// use std::{collections::HashMap, io};
//
// struct Solution {}
//
// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         let mut map = HashMap::new();
//
//         for (i, &num) in nums.iter().enumerate() {
//             let remain = target - num;
//
//             match map.get(&remain) {
//                 Some(&index) => {
//                     return vec![index as i32, i as i32];
//                 }
//                 None => {
//                     map.insert(num, i);
//                 }
//             };
//         }
//
//         vec![]
//     }
// }
//
// fn main() {
//     let mut nums_input = String::new();
//     let mut target_input = String::new();
//
//     io::stdin()
//         .read_line(&mut nums_input)
//         .expect("failed to read");
//
//     io::stdin()
//         .read_line(&mut target_input)
//         .expect("Failed to read");
//
//     let nums: Vec<i32> = nums_input
//         .trim()
//         .split(' ')
//         .map(|s| s.parse().expect("Not a number"))
//         .collect();
//
//     let target: i32 = target_input.trim().parse().expect("Expect a number");
//
//     let result = Solution::two_sum(nums, target);
//
//     println!("{:?}", result);
// }
