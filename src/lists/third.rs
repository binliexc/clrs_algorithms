use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn head(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(ele) => Some(&ele.val),
            None => None,
        }
    }

    pub fn prepend(&self, ele: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                val: ele,
                next: self.head.clone(),
            })),
        }
    }

    // 移除链表首个元素，并返回剩余链表
    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn into_iter(&self) -> IntoIter<T>
    where
        T: Clone,
    {
        IntoIter(List {
            head: self.head.clone(),
        })
    }

     pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| {
                &**node
            }),
        }
     }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(n) = head {
            if let Ok(mut node) = Rc::try_unwrap(n) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

// 拿走所有权的迭代器
pub struct IntoIter<T: Clone>(List<T>);

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.head.take().map(|node| {
            self.0 = List {
                head: node.next.clone(),
            };
            node.val.clone()
        })
    }
}

// 不可变借用迭代器
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.val
        })
    }
}

#[cfg(test)]
mod third_test {
    use super::List;

    #[test]
    fn third_test1() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    // 测试IntoIter
    #[test]
    fn third_test2() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn third_test3() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
    }
}
