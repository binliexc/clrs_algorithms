pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    val: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, ele: i32) {
        let e = Box::new(Node {
            val: ele,
            next: std::mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(e);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(e) => {
                self.head = e.next;
                Some(e.val)
            }
        } 
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod first_test {
    use std::time::Instant;

    use super::List;

    // 测试基本的push和pop功能
    #[test]
    fn test1() {
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
    fn test2() {
        let now = Instant::now();
        let mut list = List::new();
        for i in 0..1_000_000 {
            list.push(i);
        }
        drop(list);
        println!("{}", now.elapsed().as_millis());
    }
}