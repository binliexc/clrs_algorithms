pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, ele: T) {
        let e = Box::new(Node {
            val: ele,
            next: self.head.take(),
        });
        self.head = Some(e);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(n) => Some(&n.val),
        }
    }

    pub fn peed_mut(&mut self) -> Option<&mut T> {
        match &mut self.head {
            None => None,
            Some(n) => Some(&mut n.val),
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // 利用生命周期消除原则, 不必显示标注生命周期
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: match &self.head {
                None => None,
                Some(e) => Some(e),
            },
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: match &mut self.head {
                None => None,
                Some(e) => Some(e),
            },
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut e = self.head.take();
        while let Some(mut n) = e {
            e = n.next.take();
        }
    }
}

// 拿走所有权的迭代器
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
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

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.val
        })
    }
}

#[cfg(test)]
mod second_test {
    use std::time::Instant;

    use super::List;

    // 测试基本的push和pop功能
    #[test]
    fn second_test1() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(3, list.pop().unwrap());
        assert_eq!(2, list.pop().unwrap());

        list.push(4);
        list.push(5);

        assert_eq!(5, list.pop().unwrap());
        assert_eq!(4, list.pop().unwrap());
        assert_eq!(1, list.pop().unwrap());
    }

    // 测试drop功能
    #[test]
    fn second_test2() {
        let now = Instant::now();
        let mut list = List::new();
        for i in 0..1_000_000 {
            list.push(i);
        }
        drop(list);
        println!("{}", now.elapsed().as_millis());
    }

    // 测试peek功能
    #[test]
    fn second_test3() {
        let mut list = List::new();
        list.push(1);
        list.push(2);

        assert_eq!(2, *list.peek().unwrap());

        list.pop();
        assert_eq!(1, *list.peek().unwrap());
        assert_eq!(1, *list.peek().unwrap());

        list.push(3);

        *list.peed_mut().unwrap() += 1;
        assert_eq!(4, *list.peek().unwrap());
    }

    // 测试IntoIter
    #[test]
    fn second_test4() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn second_test5() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&1), iter.next());
    }

    #[test]
    fn second_test6() {
        let mut list = List::new();
        list.push(1);
    }
}
