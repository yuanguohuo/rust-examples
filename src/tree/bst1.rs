use std::collections::LinkedList;
use std::ops::Deref;

pub struct Node<T> {
    val: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct BSTree<T> {
    root: Link<T>,
}

impl<T: Ord> BSTree<T> {
    pub fn new() -> Self {
        BSTree { root: None }
    }

    pub fn insert(&mut self, val: T) {
        //target_place points to the link which will be replaced at last;
        //   1. it is initialized to root;
        //   2. traverse down until a 'none-link' is reached;
        //   3. replace that 'none-link' with a Node (note `target_place` is a ref, which points
        //      to the 'none-link', so we can modify it by `*target_place`, the same as what we do
        //      in C/C++);

        let mut target_place: &mut Link<T> = &mut self.root;

        //`while let` is a shortcut for match; here we are actually matching `target_place` (a ref)
        // with `Some(boxed_node)` (a value), so `boxed_node` is a (mut) ref to the Box<Node<T>>
        // inside `target_place`;
        //See https://www.yuanguohuo.com/2020/01/13/rust-pattern-match/
        while let Some(boxed_node) = target_place {
            target_place = if val <= boxed_node.val {
                &mut boxed_node.left
            } else {
                &mut boxed_node.right
            }
        }

        //modify a value by its pointer/ref, the same as what we do in C/C++;
        *target_place = Some(Box::new(Node {
            val,
            left: None,
            right: None,
        }));
    }

    pub fn preorder<F>(&self, f: F)
    where
        F: Fn(&T) + Copy,
    {
        preorder(&self.root, f);
    }

    pub fn preorder_recursive<F>(&self, f: F)
    where
        F: Fn(&T) + Copy,
    {
        preorder_recursive(&self.root, f);
    }

    pub fn get_preorder_itr(&self) -> PreorderItr<T> {
        PreorderItr::new(&self.root)
    }

    pub fn inorder<F>(&self, f: F)
    where
        F: Fn(&T) + Copy,
    {
        inorder(&self.root, f);
    }

    pub fn inorder_recursive<F>(&self, f: F)
    where
        F: Fn(&T) + Copy,
    {
        inorder_recursive(&self.root, f);
    }

    pub fn get_inorder_itr(&self) -> InorderItr<T> {
        InorderItr::new(&self.root)
    }
}

pub fn preorder_recursive<T, F>(link: &Link<T>, f: F)
where
    F: Fn(&T) + Copy,
{
    //`if let` is a shortcut for match, same as `while let` in BSTree::insert(); see comments there;
    if let Some(boxed_node) = link {
        f(&boxed_node.val);
        preorder(&boxed_node.left, f);
        preorder(&boxed_node.right, f);
    }
}

pub fn preorder<T, F>(link: &Link<T>, f: F)
where
    F: Fn(&T) + Copy,
{
    let mut stack: LinkedList<&Node<T>> = LinkedList::new();

    // if link is not None, push it into stack;
    if let Some(boxed_node) = link {
        stack.push_back(boxed_node.deref());
    }

    // same as while(!stack.is_empty()) {...}
    while let Some(node) = stack.pop_back() {
        f(&node.val);

        // why not `if let Some(boxed_node) = node.right`?
        // because that's match value by value, causing node.right partial moved;
        if let Some(boxed_node) = &node.right {
            stack.push_back(boxed_node.deref());
        }
        if let Some(boxed_node) = &node.left {
            stack.push_back(boxed_node.deref());
        }
    }
}

pub fn inorder_recursive<T, F>(link: &Link<T>, f: F)
where
    F: Fn(&T) + Copy,
{
    if let Some(boxed_node) = link {
        inorder_recursive(&boxed_node.left, f);
        f(&boxed_node.val);
        inorder_recursive(&boxed_node.right, f);
    }
}

pub fn inorder<T, F>(mut link: &Link<T>, f: F)
where
    F: Fn(&T) + Copy,
{
    let mut stack: LinkedList<&Node<T>> = LinkedList::new();

    loop {
        while let Some(boxed_node) = link {
            stack.push_back(boxed_node.deref());
            link = &boxed_node.left;
        }

        if stack.is_empty() {
            break;
        }

        let node = stack.pop_back().unwrap();
        f(&node.val);

        link = &node.right;
    }
}

pub struct PreorderItr<'a, T> {
    stack: LinkedList<&'a Node<T>>,
}

impl<'a, T> Iterator for PreorderItr<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop_back() {
            None => None,
            Some(node) => {
                if let Some(boxed_node) = &node.right {
                    self.stack.push_back(boxed_node.deref());
                }
                if let Some(boxed_node) = &node.left {
                    self.stack.push_back(boxed_node.deref());
                }
                Some(&node.val)
            }
        }
    }
}

impl<'a, T> PreorderItr<'a, T> {
    pub fn new(link: &'a Link<T>) -> Self {
        let mut stack: LinkedList<&'a Node<T>> = LinkedList::new();
        if let Some(boxed_node) = link {
            stack.push_back(boxed_node.deref());
        }
        PreorderItr { stack }
    }
}

pub struct InorderItr<'a, T> {
    stack: LinkedList<&'a Node<T>>,
}

impl<'a, T> Iterator for InorderItr<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop_back() {
            None => None,
            Some(node) => {
                self.push_till_next(&node.right);
                Some(&node.val)
            }
        }
    }
}

impl<'a, T> InorderItr<'a, T> {
    pub fn new(link: &'a Link<T>) -> Self {
        let mut itr = InorderItr {
            stack: LinkedList::new(),
        };
        itr.push_till_next(link);
        itr
    }

    // `link` is a shared (immutable) ref to Link<T> instance (the instance cannot be mutated); but
    // `link` can ref different Link<T> instances, hence the `mut` before it;
    fn push_till_next(&mut self, mut link: &'a Link<T>) {
        while let Some(boxed_node) = link {
            self.stack.push_back(boxed_node.deref());
            link = &boxed_node.left;
        }
    }
}

#[cfg(test)]
mod test {
    use super::BSTree;

    #[test]
    pub fn test_preorder() {
        let mut bst: BSTree<i32> = BSTree::new();
        let insert_order = vec![8, 4, 10, 6, 5, 7, 9, 12, 13];
        for val in insert_order.iter() {
            bst.insert(*val);
        }

        let mut _i: usize = 0;
        let _preorder_list = vec![8, 4, 6, 5, 7, 10, 9, 12, 13];

        println!("preorder_recursive");
        bst.preorder_recursive(|v: &i32| {
            print!("{} ", v);
            //assert_eq!(*v, preorder_list[i]);
            //i = i+1;
        });
        println!();

        println!("preorder");
        bst.preorder(|v: &i32| {
            print!("{} ", v);
            //assert_eq!(*v, preorder_list[i]);
            //i = i+1;
        });
        println!();

        let mut pre_itr = bst.get_preorder_itr();

        //this would not compile, because we have an immutable reference
        //in `pre_itr`, so mutation is not allowed;
        //wonderful Rust!
        //bst.insert(100);

        println!("preorder_itr");
        while let Some(v) = pre_itr.next() {
            print!("{} ", v);
        }
        println!();

        let pre_itr2 = bst.get_preorder_itr();
        let c: Vec<&i32> = pre_itr2.collect();
        println!("{:?}", c);
        let expect: Vec<&i32> = vec![&8, &4, &6, &5, &7, &10, &9, &12, &13];
        assert_eq!(c, expect);
    }

    #[test]
    pub fn test_inorder() {
        let mut bst: BSTree<String> = BSTree::new();
        let insert_order = vec!["f", "h", "d", "e", "a", "b", "c", "g", "i"];
        for val in insert_order.iter() {
            bst.insert(String::from(*val));
        }

        println!("inorder");
        bst.inorder(|v| {
            print!("{} ", v);
        });
        println!();

        println!("inorder_recursive");
        bst.inorder_recursive(|v| {
            print!("{} ", v);
        });
        println!();

        println!("inorder_itr");
        let mut in_itr = bst.get_inorder_itr();
        while let Some(v) = in_itr.next() {
            print!("{}", v);
        }
        println!();

        let in_itr2 = bst.get_inorder_itr();
        let c: Vec<&String> = in_itr2.collect();
        println!("{:?}", c);

        let ra = &String::from("a");
        let rb = &String::from("b");
        let rc = &String::from("c");
        let rd = &String::from("d");
        let re = &String::from("e");
        let rf = &String::from("f");
        let rg = &String::from("g");
        let rh = &String::from("h");
        let ri = &String::from("i");
        let expect = vec![ra, rb, rc, rd, re, rf, rg, rh, ri];
        assert_eq!(c, expect);
    }
}
