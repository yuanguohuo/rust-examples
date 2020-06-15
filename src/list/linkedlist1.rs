use std::iter::Iterator;

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

    pub fn push_front(&mut self, val: T) {
        let new_head = Box::new(Node {
            val,
            next: self.head.take(),
        });
        self.head = Some(new_head);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }
}

pub struct IntoItr<T> {
    next: Link<T>,
}

impl<T> Iterator for IntoItr<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node|{
            self.next = node.next;
            node.val
        })
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoItr<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoItr{next:self.head}
    }
}

/*
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(mut node) = head {
            head = node.next.take();
            // node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to None by take() method,
            // so no unbounded recursion occurs.

            //Yuanguo:
            //  head = node.next;
            //should be fine too, because node.next is moved (not a None or Some, just uninitialized),
            //so, no destruction work to do;
        }
    }
}
*/

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut l: List<i32> = List::new();
        assert_eq!(l.front(), None);
        assert_eq!(l.front_mut(), None);

        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        assert_eq!(l.front(), Some(&3));
        assert_eq!(l.front_mut(), Some(&mut 3));

        l.pop_front();
        assert_eq!(l.front(), Some(&2));
        assert_eq!(l.front_mut(), Some(&mut 2));

        l.push_front(4);
        l.push_front(5);

        if let Some(v) = l.front_mut() {
            *v = *v * 2;
        }
        assert_eq!(l.front(), Some(&10));
        l.pop_front();

        l.front_mut().map(|v| {
            *v = *v * 2;
        });

        assert_eq!(l.front(), Some(&8));
        l.pop_front();

        assert_eq!(l.front(), Some(&2));

        let f1 = l.front_mut();
        f1.map(|v| {
            *v = *v * 2;
        });

        assert_eq!(l.front(), Some(&4));

        l.pop_front();
        assert_eq!(l.front(), Some(&1));
        l.pop_front();
        assert_eq!(l.front(), None);
        l.pop_front();
        assert_eq!(l.front(), None);
        assert_eq!(l.front_mut(), None);
    }
}
