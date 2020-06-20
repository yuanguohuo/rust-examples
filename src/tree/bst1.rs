pub struct Node<T> {
    val: T,
    left: Link<T>,
    right: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub fn preorder<T, F>(link: &Link<T>, f: F)
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
}

#[cfg(test)]
mod test {
    use super::BSTree;

    #[test]
    pub fn test_basic() {
        let mut bst: BSTree<i32> = BSTree::new();
        bst.insert(2);
        bst.insert(1);
        bst.insert(3);

        println!("preorder");
        bst.preorder(|v: &i32| {
            println!("{}", v);
        });
    }
}
