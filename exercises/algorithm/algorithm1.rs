/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    // 合并两个有序链表。注意这里要求 T 实现 PartialOrd
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
        where
            T: PartialOrd,
    {
        let mut merged_list = LinkedList::new();

        // 使用闭包将链表中的所有节点转换为 Vec<T>，同时清空链表中的指针以避免 double free
        let collect = |mut list: LinkedList<T>| -> Vec<T> {
            let mut elements = Vec::new();
            let mut current = list.start;
            while let Some(ptr) = current {
                unsafe {
                    let node = Box::from_raw(ptr.as_ptr());
                    elements.push(node.val);
                    current = node.next;
                }
            }
            // 清空链表指针，防止 Drop 重复释放
            list.start = None;
            list.end = None;
            list.length = 0;
            elements
        };

        let mut a_iter = collect(list_a).into_iter().peekable();
        let mut b_iter = collect(list_b).into_iter().peekable();

        // 合并两个有序序列
        loop {
            match (a_iter.peek(), b_iter.peek()) {
                (Some(a), Some(b)) => {
                    if a <= b {
                        merged_list.add(a_iter.next().unwrap());
                    } else {
                        merged_list.add(b_iter.next().unwrap());
                    }
                }
                (Some(_), None) => merged_list.add(a_iter.next().unwrap()),
                (None, Some(_)) => merged_list.add(b_iter.next().unwrap()),
                (None, None) => break,
            }
        }

        merged_list
    }
}

impl<T> Display for LinkedList<T>
    where
        T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
    where
        T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

// 实现 Drop trait 以释放链表所有节点，防止内存泄露
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start;
        while let Some(node_ptr) = current {
            unsafe {
                current = (*node_ptr.as_ptr()).next;
                Box::from_raw(node_ptr.as_ptr());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, target) in target_vec.iter().enumerate() {
            assert_eq!(target, list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, target) in target_vec.iter().enumerate() {
            assert_eq!(target, list_c.get(i as i32).unwrap());
        }
    }
}
