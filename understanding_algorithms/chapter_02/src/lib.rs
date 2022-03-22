mod simple_list {
    use std::ptr::{self};

    pub struct List<T> {
        head: Link<T>,
        tail: *mut Node<T>,
    }

    pub type Link<T> = Option<Box<Node<T>>>;

    pub struct Node<T> {
        data: T,
        link: Link<T>,
        prev_link: *mut Node<T>,
    }

    impl<T> Node<T> {
        pub fn new(data: T) -> Self {
            Self {
                data,
                link: None,
                prev_link: ptr::null_mut(),
            }
        }
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            Self {
                head: None,
                tail: ptr::null_mut(),
            }
        }

        pub fn push(&mut self, data: T) {
            let mut new_node = Box::new(Node::new(data));
            new_node.prev_link = self.tail;
            let raw_node: *mut _ = &mut *new_node;
            if self.tail.is_null() {
                self.head = Some(new_node);
            } else {
                unsafe {
                    (*self.tail).link = Some(new_node);
                }
            }
            self.tail = raw_node;
        }

        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|head| {
                let head = *head;
                self.head = head.link;
                match &mut self.head {
                    Some(head) => head.prev_link = ptr::null_mut(),
                    None => self.tail = ptr::null_mut(),
                }
                head.data
            })
        }

        pub fn take(&mut self, pos: usize) -> Option<T> {
            let mut node = &mut self.head;
            if pos == 0 {
                node.take().map(|mut node| {
                    let raw_node: *mut _ = &mut *node;
                    if raw_node == self.tail {
                        self.tail = ptr::null_mut();
                    }
                    self.head = node.link;
                    node.data
                })
            } else {
                let mut idx = 1;
                loop {
                    match node {
                        Some(inner_node) => {
                            if idx == pos {
                                break inner_node.link.take().map(|mut next_node| {
                                    let raw_node: *mut _ = &mut *next_node;
                                    if raw_node == self.tail {
                                        self.tail = &mut **inner_node;
                                    }
                                    inner_node.link = next_node.link;
                                    next_node.data
                                });
                            }
                            node = &mut inner_node.link;
                        }
                        None => break None,
                    }
                    idx += 1;
                }
            }
        }

        pub fn get(&self, pos: usize) -> Option<&T> {
            self.iter()
                .enumerate()
                .find_map(|(idx, val)| if idx == pos { Some(val) } else { None })
        }

        pub fn iter(&self) -> impl Iterator<Item = &T> {
            ListIter(&self.head)
        }

        pub fn into_iter(self) -> impl Iterator<Item = T> {
            ListLinkIter(self.head)
        }

        pub fn len(&self) -> usize {
            self.iter().count()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    pub struct ListIter<'a, T>(&'a Link<T>);

    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            match self.0 {
                None => None,
                Some(link) => {
                    self.0 = &link.link;
                    Some(&link.data)
                }
            }
        }
    }

    pub struct ListLinkIter<T>(Link<T>);

    impl<T> Iterator for ListLinkIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.0.take().map(|link| {
                self.0 = link.link;
                link.data
            })
        }
    }

    impl<T> FromIterator<T> for List<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            let mut list = Self::new();
            iter.into_iter().for_each(|item| list.push(item));
            list
        }
    }

    impl<T> std::ops::Index<usize> for List<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            self.get(index).unwrap()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::List;

        #[test]
        fn it_works() {
            let mut list = List::new();
            assert!(list.is_empty());
            for i in 0..10 {
                list.push(i);
            }
            assert_eq!(list.len(), 10);
            for i in 0..10 {
                assert_eq!(list.pop(), Some(i));
            }
            assert!(list.is_empty());
        }
        #[test]
        fn from_iter() {
            let vec = (0..10).collect::<Vec<_>>();
            let list = (0..10).collect::<List<_>>();
            assert_eq!(vec.len(), list.len());
            for i in 0..vec.len() {
                assert_eq!(vec[i], list[i]);
            }
        }

        #[test]
        fn take() {
            let mut list = (0..10).collect::<List<_>>();
            assert_eq!(list.take(5), Some(5));
            assert_eq!(list.len(), 9);
            assert_eq!(list.take(8), Some(9));
            assert_eq!(list.len(), 8);
        }
    }
}
