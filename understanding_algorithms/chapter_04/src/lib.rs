mod q_sort {
    use std::cmp::Ordering;

    pub fn quick_sort<T: Clone>(arr: &mut [T], cmp_fn: impl Fn(&T, &T) -> Ordering + Copy) {
        match arr.len() {
            1 | 0 => {}
            2 => {
                if cmp_fn(&arr[0], &arr[1]) == Ordering::Greater {
                    arr.swap(0, 1);
                }
            }
            len => {
                let pivot = find_pivot(arr);
                let pivot_value = &arr[pivot];
                let mut vec_smaller_than_pivot = Vec::with_capacity(len - 1);
                let mut vec_greater_than_pivot = Vec::with_capacity(len - 1);
                for (idx, v) in arr.iter().enumerate() {
                    if idx != pivot {
                        if cmp_fn(v, pivot_value) == Ordering::Greater {
                            vec_greater_than_pivot.push(v.clone());
                        } else {
                            vec_smaller_than_pivot.push(v.clone());
                        }
                    }
                }
                quick_sort(vec_smaller_than_pivot.as_mut_slice(), cmp_fn);
                quick_sort(vec_greater_than_pivot.as_mut_slice(), cmp_fn);
                let mut ordered_vec = vec_smaller_than_pivot;
                ordered_vec.push(pivot_value.clone());
                ordered_vec.append(&mut vec_greater_than_pivot);
                arr.swap_with_slice(ordered_vec.as_mut_slice());
            }
        }
    }

    fn find_pivot<T>(arr: &[T]) -> usize {
        arr.len() / 2
    }
}

#[cfg(test)]
mod tests {
    use crate::q_sort::quick_sort;

    #[test]
    fn it_works() {
        let v_size = 100000000;
        let mut v = (0..v_size).rev().collect::<Vec<_>>();
        quick_sort(&mut v, i32::cmp);
        assert_eq!(v, (0..v_size).collect::<Vec<_>>());
    }

    // #[test]
    // fn it_works1() {
    //     let mut v = vec![9, 7, 4, 3, 2, 8, 6, 5, 1, 0];
    //     quick_sort(&mut v, i32::cmp);
    //     assert_eq!(v, (0..10).collect::<Vec<_>>());
    // }

    // #[test]
    // fn it_works2() {
    //     let v_size = 100000000;
    //     let mut v = (0..v_size).collect::<Vec<_>>();
    //     quick_sort(&mut v, |a, b| match a.cmp(b) {
    //         std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
    //         std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
    //         std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
    //     });
    //     assert_eq!(v, (0..v_size).rev().collect::<Vec<_>>());
    // }
}
