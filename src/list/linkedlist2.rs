use std::ops::Deref;
use std::rc::Rc;

//      link1                    link2
//   +----------+            +----------+
//   |   Some   |            |   Some   |
//   +----------+            +----------+
//   |    Rc    |            |    Rc    |
//   +----------+            +----------+
//         ^                       ^
//         |                       |
//         +--- -------+-----------+
//                     |
//                     V
//            +-----------------+
//            |   ref_cnt = 2   |
//            +-----------------+
//            |     Node<T>     |
//            +-----------------+
type Link<T> = Option<Rc<Node<T>>>;

pub struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct Itr<'a, T> {
    curr: Option<&'a Node<T>>,
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    // return a new List<T>: val->(self); and `self` is kept as is;
    pub fn prepend(&self, val: T) -> Self {
        let node = Node {
            val,
            // 1. self.head.as_ref(), we get Option<&Rc<Node<T>>> instance, say opt_ref; not self.head
            //    is kept as is;
            // 2. opt_ref.map(), note that `map()` takes `self` (rather than `&self`) as the 1st argument,
            //    so, opt_ref is moved to `self` in `map()`;
            // 3. `map()` is a match under the hood: match value by value by value, (again self is an Option
            //    value, not ref), as a result, the &Rc<Node<T>> instance in self is copied (&T is Copy) to
            //    `rc_node`;
            // 4. now, we have `rc_node`, which is an &Rc<Node<T>> instance; by calling its `clone()` method,
            //    we get another Rc<Node<T>> instance, say rc_node2;
            // 5. `map()` creates a new Option value, Some(rc_node2), and moves it to `Node.next`;
            next: self.head.as_ref().map(|rc_node| rc_node.clone()),
            //the above can be shorted as:
            //next: self.head.clone(),
        };

        List {
            head: Some(Rc::new(node)),
        }
    }

    // return a new List<T> with self's head removed; and `self` is kept as is;
    pub fn tail(&self) -> Self {
        //see comments in `prepend` method above; `map()` is changed to `and_then()` here;
        //`and_then` is similar to `map()`, the only difference is:
        //   `map` wraps the return value of the enclosure into Option::Some;
        //   `and_then` does not wrap (so, your enclosure should return Option::Some);
        let link = self.head.as_ref().and_then(|rc_node| rc_node.next.clone());
        List { head: link }
    }

    pub fn itr(&self) -> Itr<T> {
        Itr {
            //see comments in `prepend` method above;
            curr: self.head.as_ref().map(|rc_node| rc_node.deref()),
        }
    }
}

impl<'a, T> Iterator for Itr<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // 1. self.curr.take():  it's mem::replace under the hood, setting self.curr to None and returning
        //    its original value, say `origin`;
        // 2. origin.map(): note that `map()` takes `self` (not `&self`) as argument, so `origin`
        //    is moved to `self` argument in `map()`;
        // 3. in `map()`
        //      a. if `self` is None, return None; we do nothing (remember that curr was left None in step-1);
        //      b. if `self` is Some(T), T is moved (or copied if it's Copy) to closure;
        // 4. the closure returns a &Node<T>, say `tmp_node_ref`;
        // 5. `map()` wraps `tmp_node_ref` in Some, and move the Some to self.curr;
        self.curr.take().map(|node| {
            self.curr = node.next.as_ref().map(|next| next.deref());
            &node.val
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    pub fn test_basic() {
        let l: List<i32> = List::new();
        let l1 = l.prepend(1);
        let l2 = l1.prepend(2);
        let l3 = l2.prepend(3);

        let mut l_itr = l.itr();
        assert_eq!(None, l_itr.next());
        assert_eq!(None, l_itr.next());

        let mut l1_itr = l1.itr();
        assert_eq!(Some(&1), l1_itr.next());
        assert_eq!(None, l1_itr.next());

        let mut l2_itr = l2.itr();
        assert_eq!(Some(&2), l2_itr.next());
        assert_eq!(Some(&1), l2_itr.next());
        assert_eq!(None, l2_itr.next());

        let mut l3_itr = l3.itr();
        assert_eq!(Some(&3), l3_itr.next());
        assert_eq!(Some(&2), l3_itr.next());
        assert_eq!(Some(&1), l3_itr.next());
        assert_eq!(None, l3_itr.next());
    }
}
