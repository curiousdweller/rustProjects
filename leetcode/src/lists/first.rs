use self::lists::List;

mod lists {
    use std::{
        io::Empty,
        mem::{self, replace},
    };

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]

    pub struct List {
        head: Link,
    }

    // Either there is another item in the list, or there isn't
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub enum Link {
        Empty,
        Pointer(Box<Node>),
    }

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    pub struct Node {
        elem: u32,
        next: Link,
    }

    impl List {
        pub fn new() -> Self {
            List { head: Link::Empty }
        }

        pub fn push(&mut self, num: u32) {
            let node = Node {
                elem: num,
                next: mem::replace(&mut self.head, Link::Empty),
            };
            self.head = Link::Pointer(Box::new(node));
        }

        pub fn pop(&mut self) -> Option<u32> {
            let res;
            // Take self.head (LINK) out of self
            // mem::replace is useful when woriking with an exclusive mutable reference
            match mem::replace(&mut self.head, Link::Empty) {
                Link::Empty => {
                    res = None;
                }
                Link::Pointer(boxed_link) => {
                    res = Some((*boxed_link).elem);
                    // we can move out of boxed_link because we own the link we are matching
                    // which in turn owns the Boxed node.

                    self.head = boxed_link.next;
                }
            }

            res
        }
    }
    impl Drop for List {
        fn drop(&mut self) {
            let mut current_node = replace(&mut self.head, Link::Empty);

            while let Link::Pointer(ref mut boxed_node) = &mut current_node {
                current_node = replace(&mut boxed_node.next, Link::Empty);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic() {
        use super::lists::List;

        let mut list = List::new();
        let mut list_second = List::new();
        assert_ne!(list_second, list);
        list.push(5);

        assert_eq!(5, list.pop().unwrap());
    }
}
