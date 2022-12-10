use std::{cell::RefCell, rc::Rc};

pub struct TreeNode<T> {
    value: T,
    left: TreeLink<T>,
    right: TreeLink<T>,
}
type TreeLink<T> = Option<Rc<RefCell<TreeNode<T>>>>;

#[macro_export]
macro_rules! tree {
    ($value:expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            value: $value,
            left: None,
            right: None,
        })))
    };
    ($value:expr, $left:expr, $right:expr) => {
        Some(Rc::new(RefCell::new(TreeNode {
            value: $value,
            left: $left,
            right: $right,
        })))
    };
}

trait Inorder<T, U> {
    fn traverse_inorder(&self, visit: &mut dyn FnMut(&T) -> U) -> Vec<U>
    where
        U: Clone;
}

impl<T, U> Inorder<T, U> for TreeLink<T> {
    fn traverse_inorder(&self, visit: &mut dyn FnMut(&T) -> U) -> Vec<U>
    where
        U: Clone,
    {
        match self {
            Some(node) => {
                let node = node.borrow();
                let left = Self::traverse_inorder(&node.left, visit);
                let root = vec![visit(&node.value)];
                let right = Self::traverse_inorder(&node.right, visit);
                [left, root, right].concat()
            }
            None => vec![],
        }
    }
}

fn main() {
    let root = tree!(1, None, tree!(2, tree!(3), None));
    assert_eq!(root.traverse_inorder(&mut |&e| e), vec![1, 3, 2])
}
