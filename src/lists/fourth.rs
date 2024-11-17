use std::{
    cell::{Ref, RefCell}, rc::Rc
};

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    val: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, ele: T) {
        let n = Rc::new(RefCell::new({
            Node {
                val: ele,
                prev: None,
                next: None,
            }
        }));
        match self.head.take() {
            Some(node) => {
                n.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(n.clone());
                self.head = Some(n);
            }
            None => {
                self.head = Some(n.clone());
                self.tail = Some(n);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.head = node.borrow().next.clone().map(|n| {
                    n.borrow_mut().prev = None;
                    n
                });

                // 如果链表只有1个节点，head和tail都会指向同一个节点，如果不消费掉tail，后续Rc::try_unwarp()
                // 就会返回None而不是返回节点
                if Rc::ptr_eq(&node, self.tail.as_ref().unwrap()) {
                    self.tail.take();
                }

                println!("rc count: {}", Rc::strong_count(&node));
                Some(Rc::try_unwrap(node).ok().unwrap().into_inner().val)
            }
            None => None,
        }
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |n| &n.val))
    }


}

impl<T> Node<T> {
    pub fn new(ele: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val: ele,
            prev: None,
            next: None,
        }))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

// pub struct Iter<'a, T>(Option<Ref<'a, Node<T>>>);

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = Ref<'a, T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.take().map(|node| {
//             let (next, elem) = Ref::map_split(node, |n| {
//                 (&n.next, &n.val)
//             });
//             self.0 = next.as_ref().map(|n| n.borrow());
//             elem
//         }) 
//     }
// }

#[cfg(test)]
mod fourth_test {
    use super::List;

    #[test]
    fn fourth_test1() {
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        // assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn fourth_test2() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);
    }
}
