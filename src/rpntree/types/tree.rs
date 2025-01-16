use std::fmt::Display;
pub struct Tree<T: Copy> {
    pub root_node: TreeNode<T>,
}
impl<T: Copy> Tree<T> {
    pub fn reduce<R>(&self, reducer: fn(Option<R>, T, Option<R>) -> R) -> R {
        self.root_node.reduce(reducer)
    }
    pub fn new(root_val: T) -> Tree<T> {
        Tree::<T> {
            root_node: TreeNode::<T> {
                value: root_val,
                left: None,
                right: None,
            },
        }
    }
}
impl<T: Display + Copy> Display for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root_node)
    }
}
pub struct TreeNode<T: Copy> {
    pub value: T,
    pub left: Option<Box<TreeNode<T>>>,
    pub right: Option<Box<TreeNode<T>>>,
}
impl<T: Copy + Display> Display for TreeNode<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            match &self.left {
                Some(v) => format!("({})<-", v.to_string()),
                None => "".to_string(),
            },
            format!("({})", self.value),
            match &self.right {
                Some(v) => format!("->({})", v.to_string()),
                None => "".to_string(),
            }
        )
    }
}
impl<T: Copy> TreeNode<T> {
    pub fn new(v: T) -> TreeNode<T> {
        TreeNode {
            value: v,
            left: None,
            right: None,
        }
    }
    pub fn reduce<R>(&self, reducer: fn(Option<R>, T, Option<R>) -> R) -> R {
        let left_val = match &self.left {
            Some(node) => Some(node.reduce(reducer)),
            None => None,
        };
        let right_val = match &self.right {
            Some(node) => Some(node.reduce(reducer)),
            None => None,
        };
        reducer(left_val, self.value, right_val)
    }
    pub fn insert_left(&mut self, value: T) {
        self.left = Some(Box::new(TreeNode {
            value,
            left: None,
            right: None,
        }));
    }
    pub fn insert_right(&mut self, value: T) {
        self.right = Some(Box::new(TreeNode {
            value,
            left: None,
            right: None,
        }));
    }
}
