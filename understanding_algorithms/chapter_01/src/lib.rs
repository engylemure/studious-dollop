// This mod covers the implementation of a simple binary_search algorithm.
pub mod binary_search {
    use std::cmp::PartialOrd;
    /*
     * The binary search has some small constraints related to it:
     *  - It's elements should be PartialOrd
     *  - The array should be ordered
     */
    pub fn binary_search<T: PartialOrd>(arr: &[T], element_searched: &T) -> Option<usize> {
        if arr.len() == 0 {
            None
        } else if arr[0] < arr[arr.len() - 1] {
            binary_search_inc(arr, element_searched)
        } else {
            binary_search_dec(arr, element_searched)
        }
    }
    // Binary search for arrays that are in a increasing order
    fn binary_search_inc<T: PartialOrd>(arr: &[T], element_searched: &T) -> Option<usize> {
        let len = arr.len();
        let mut low_idx = 0;
        let mut high_idx = len - 1;
        loop {
            if low_idx > high_idx {
                break None;
            }
            let between_idx = (low_idx + high_idx) / 2;
            let guess = &arr[between_idx];
            if guess == element_searched {
                break Some(between_idx);
            }
            if guess > element_searched {
                high_idx = between_idx.checked_sub(1)?;
            } else {
                low_idx = between_idx + 1;
            }
        }
    }
    // Binary search for arrays that are in a decreasing order
    fn binary_search_dec<T: PartialOrd>(arr: &[T], element_searched: &T) -> Option<usize> {
        let len = arr.len();
        let mut low_idx = len - 1;
        let mut high_idx = 0;
        loop {
            if low_idx < high_idx {
                break None;
            }
            let between_idx = (low_idx + high_idx) / 2;
            let guess = &arr[between_idx];
            if guess == element_searched {
                break Some(between_idx);
            }
            if guess > element_searched {
                high_idx = between_idx + 1;
            } else {
                low_idx = between_idx.checked_sub(1)?;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::binary_search;

        #[test]
        fn increasing_order_in_arr() {
            let arr = vec![1, 3, 5, 7, 9];
            assert_eq!(binary_search(&arr, &3), Some(1));
        }

        #[test]
        fn increasing_order_not_in_arr() {
            let arr = vec![1, 3, 5, 7, 9];
            assert_eq!(binary_search(&arr, &-1), None);
        }

        #[test]
        fn decreasing_order_in_arr() {
            let arr = vec![9, 7, 5, 3, 1];
            assert_eq!(binary_search(&arr, &3), Some(3));
        }

        #[test]
        fn decreasing_order_not_in_arr() {
            let arr = vec![9, 7, 5, 3, 1];
            assert_eq!(binary_search(&arr, &-1), None);
        }
    }
}
