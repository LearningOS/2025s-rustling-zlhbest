/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]) {
    //TODO
    if array.len() > 1 {
        quick_sort(array, 0, array.len() - 1);
    }
}
fn quick_sort<T: PartialOrd>(array: &mut [T], start: usize, end: usize) {
    // 快排就是分级处理
    let (mut left, mut right) = (start, end);
    let base = start;
    while left < right {
        while array[left] <= array[base] && left < right {
            left += 1;
        }
        // 找到比base大的数字
        array.swap(left, right);

        while array[right] >= array[base] && left <= right {
            right -= 1;
        }
        array.swap(right, base);
    }
    if left - 1 > start {
        quick_sort(array, start, left - 1);
    }
    if right + 1 < end {
        quick_sort(array, right + 1, end);
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
