use std::iter::IntoIterator;
use std::iter::Iterator;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct Itr<'a, T> {
    curr: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Itr<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. curr.take():  it's mem::replace under the hood, setting curr to None and returning
        //    its original value, say origin;
        // 2. origin.map(): note that map() takes self (not &self) as argument, so origin is moved;
        // 3. if origin is None, do nothing (remember that curr was left None in step-1);
        // 4. if origin is Some(T), T is moved (or copied if it's Copy) to closure;
        self.curr.take().map(|node| {
            self.curr = node.next.as_ref().map(|next| next.as_ref());
            &node.val
        })
    }
}

pub struct MutItr<'a, T> {
    curr: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for MutItr<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr.take().map(|node| {
            self.curr = node.next.as_mut().map(|next| next.as_mut());
            &mut node.val
        })
    }
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

    //Yuanguo: the accurate version of 'fn itr()' should be this:
    //       pub fn itr<'a>(&'a self) -> Itr<'a, T>
    //meaning that we create Itr with lifetime 'a from &self whose lifetime is also 'a;
    //but that can be elided into:
    //       pub fn itr(&self) -> Itr<T>
    pub fn itr(&self) -> Itr<T> {
        let curr = self.head.as_ref().map(|node| node.as_ref());
        Itr { curr }
    }

    pub fn mut_itr(&mut self) -> MutItr<T> {
        let curr = self.head.as_mut().map(|node| node.as_mut());
        MutItr { curr }
    }
}

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

pub struct IntoItr<T> {
    curr: Link<T>,
}

impl<T> Iterator for IntoItr<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. curr.take():  it's mem::replace under the hood, setting curr to None and returning
        //    its original value, say origin;
        // 2. origin.map(): note that map() takes self (not &self) as argument, so origin is moved;
        // 3. if origin is None, do nothing (remember that curr was left None in step-1);
        // 4. if origin is Some(T), T is moved (or copied if it's Copy) to closure, that's to say
        //    the Box<Node<T>> object inside origin is moved to node, which will be destroyed when
        //    the closure ends; that is where each node of the list gets destroyed;
        self.curr.take().map(|node| {
            self.curr = node.next;
            node.val
        })
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoItr<T>;

    /*
     * this will not work, see test_drop_move() below.
     *
    fn into_iter(self) -> Self::IntoIter {
        IntoItr { curr: self.head }
    }
    */

    fn into_iter(mut self) -> Self::IntoIter {
        IntoItr {
            curr: self.head.take(),
        }
    }
}

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

    #[test]
    fn test_into_itr() {
        let mut l: List<i32> = List::new();
        assert_eq!(l.front(), None);
        assert_eq!(l.front_mut(), None);

        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        let mut itr = l.into_iter(); // l is moved here;
        assert_eq!(Some(3), itr.next());
        assert_eq!(Some(2), itr.next());
        assert_eq!(Some(1), itr.next());
        assert_eq!(None, itr.next());

        //l is moved, so we cannot use it again;
        //let _itr1 = l.into_iter(); //value used here after move
    }

    #[test]
    fn test_itr() {
        let mut l: List<i32> = List::new();
        assert_eq!(l.front(), None);
        assert_eq!(l.front_mut(), None);

        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        let mut itr = l.itr();
        assert_eq!(Some(&3), itr.next());
        assert_eq!(Some(&2), itr.next());
        assert_eq!(Some(&1), itr.next());
        assert_eq!(None, itr.next());

        //list l is not moved, so we can iterator over it again by itr1;
        let mut itr1 = l.itr();
        assert_eq!(Some(&3), itr1.next());
        assert_eq!(Some(&2), itr1.next());
        assert_eq!(Some(&1), itr1.next());
        assert_eq!(None, itr1.next());
    }

    #[test]
    fn test_mut_itr() {
        let mut l: List<i32> = List::new();
        assert_eq!(l.front(), None);
        assert_eq!(l.front_mut(), None);

        l.push_front(1);
        l.push_front(2);
        l.push_front(3);

        let mut mut_itr = l.mut_itr();
        assert_eq!(Some(&mut 3), mut_itr.next());
        assert_eq!(Some(&mut 2), mut_itr.next());
        assert_eq!(Some(&mut 1), mut_itr.next());
        assert_eq!(None, mut_itr.next());

        let mut mut_itr1 = l.mut_itr();
        mut_itr1.next().map(|v| *v = *v * 2);
        mut_itr1.next().map(|v| *v = *v * 2);
        mut_itr1.next().map(|v| *v = *v * 2);
        mut_itr1.next().map(|v| *v = *v * 2);
        mut_itr1.next().map(|v| *v = *v * 2);

        let mut mut_itr2 = l.mut_itr();
        assert_eq!(Some(&mut 6), mut_itr2.next());
        assert_eq!(Some(&mut 4), mut_itr2.next());
        assert_eq!(Some(&mut 2), mut_itr2.next());
        assert_eq!(None, mut_itr2.next());
    }

    #[test]
    fn test_drop_move() {
        let l: List<i32> = List::new();

        // rustc --explain E0509
        //
        // We tried to move a field out of a List<T> instance which
        // implements the `Drop` trait. However, a struct cannot be dropped if one or
        // more of its fields have been moved.
        //
        // Structs implementing the `Drop` trait have an implicit destructor that gets
        // called when they go out of scope. This destructor may use the fields of the
        // struct, so moving out of the struct could make it impossible to run the
        // destructor. Therefore, we must think of all values whose type implements the
        // `Drop` trait as single units whose fields cannot be moved.

        //1. List<T> cannot be partially moved;
        //let _head = l.head;   //E0509

        //2. List<T> can be moved as a unit;
        let _l = l; //OK!

        //3. List<T> can be partially mutated;
        let mut l2: List<i32> = List::new();
        l2.head.take();

        //总之：实现Drop的类型
        //   1. 可以被move；
        //   2. 但不能被 partially move；
        //   3. 可以被partially mutated；
    }
}
