use std::rc::Rc;

struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None }
    }

    fn prepend(&self, elem: T) -> List<T> {
        let node = Node { elem, next: self.head.clone() };
        List { head: Some(Rc::new(node)) }
    }

    fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(node) = link {
            if let Ok(node) = Rc::try_unwrap(node) {
                link = node.next;
            } else {
                break
            }
        }
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

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
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

    #[test]
    fn iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}