/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// 

fn sort<T: Ord>(array: &mut [T]) {
    //TODO
    let len = array.len();
    if len <= 1 {
        return;
    }
    for start in (0..len / 2).rev() {
        sift_down(array, start, len);
    }
    for end in (1..len).rev() {
        array.swap(0, end);
        sift_down(array, 0, end);
    }
}

fn sift_down<T: Ord>(array: &mut [T], mut root: usize, len: usize) {
    loop {
        let left_child = 2 * root + 1;
        if left_child >= len {
            break;
        }
        let mut swap_idx = root;
        if array[left_child] > array[swap_idx] {
            swap_idx = left_child;
        }
        let right_child = left_child + 1;
        if right_child < len && array[right_child] > array[swap_idx] {
            swap_idx = right_child;
        }
        if swap_idx == root {
            break;
        }
        array.swap(root, swap_idx);
        root = swap_idx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
