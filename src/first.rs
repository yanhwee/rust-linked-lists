struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        let node = Node{ elem, next: self.head.take() };
        self.head = Some(Box::new(node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    fn peek(&mut self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(node) = link {
            link = node.next;
        }
    }
}

struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<T> Iter<'_, T> {
    fn new(list: &List<T>) -> Iter<T> {
        Iter { next: list.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}


// struct IterMut<T> {
//     next: Option<&
// }


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn list_push_pop() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn list_drop() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        list.push(6);
        list.push(7);
    }

    #[test]
    fn list_peek() {
        let mut list = List::new();
        list.push(10);
        assert_eq!(list.peek(), Some(&10));
        assert_eq!(list.pop(), Some(10));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn list_into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
    
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn list_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
    
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    // Benchmarks
    // #[test]
    fn list_drop_bench() {
        let mut list = List::new();
        let n = 100000000; // 100 million
        for i in 0..n {
            list.push(i);
        }
    }

    #[test]
    fn list_iter_bench() {
        let mut list = List::new();
        let n = 10_i32.pow(6); // 10 million
        for i in 0..n {
            list.push(i);
        }
        for _ in 0..1000 {
            let mut iter = list.iter();
            for i in (0..n).rev() {
                assert_eq!(iter.next(), Some(&i));
            }
            assert_eq!(iter.next(), None);
        }
    }
}
