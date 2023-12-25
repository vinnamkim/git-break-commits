use std::cell::RefCell;
use std::collections::HashMap;

use std::path::PathBuf;
use std::rc::Rc;
use thiserror::Error;

use crate::node::{Node, NodeData, Pointer};

#[derive(Error, Debug)]
pub enum TreeError {
    #[error("ChildrenIsNoneError")]
    ChildrenIsNoneError,
    #[error("StackIsEmptyError")]
    StackIsEmptyError,
    #[error("CannotGetFileNameError")]
    CannotGetFileNameError,
    #[error("EmptyTreeError")]
    EmptyTreeError,
}

#[derive(Debug)]
pub struct Tree {
    root: Pointer<NodeData>,
    size: u32,
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            root: Rc::new(RefCell::new(Node {
                value: None,
                parent: None,
                children: Some(HashMap::new()),
            })),
            size: 0,
        }
    }

    pub fn add(&self, value: NodeData) -> Result<(), TreeError> {
        let mut curr = self.root.clone();

        let filename = value
            .path
            .file_name()
            .ok_or(TreeError::CannotGetFileNameError)?;

        for c in value.path.components() {
            let comp_str = c.as_os_str();
            let is_leaf_node = comp_str == filename;

            let child = Self::get_child(&curr, comp_str)?;

            match child {
                Some(found) => {
                    curr = found;
                }
                None => {
                    let new_node = Rc::new(RefCell::new(Node {
                        value: if is_leaf_node {
                            Some(value.clone())
                        } else {
                            None
                        },
                        parent: Some(curr.clone()),
                        children: if is_leaf_node {
                            None
                        } else {
                            Some(HashMap::new())
                        },
                    }));
                    {
                        let borrowed = &mut curr.borrow_mut();
                        let children = borrowed
                            .children
                            .as_mut()
                            .ok_or(TreeError::ChildrenIsNoneError)?;
                        children
                            .insert(comp_str.to_os_string(), new_node.clone());
                    }
                    curr = new_node;
                }
            }
        }
        Ok(())
    }

    pub fn get_root(&self) -> Pointer<NodeData> {
        self.root.clone()
    }

    fn get_child(
        curr: &Pointer<NodeData>,
        comp_str: &std::ffi::OsStr,
    ) -> Result<Option<Pointer<NodeData>>, TreeError> {
        let children = &curr.borrow().children;
        let child = children
            .as_ref()
            .ok_or(TreeError::ChildrenIsNoneError)?
            .get(comp_str)
            .cloned();
        Ok(child)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    #[test]
    fn test_new() {
        let tree: Tree = Tree::new();
        assert_eq!(tree.size, 0);
    }

    #[test]
    fn test_add() {
        let tree: Tree = Tree::new();

        let x = NodeData::from_str("./a/b/c/file.txt").expect("");
        let y = NodeData::from_str("./a/b/file.txt").expect("");
        let z = NodeData::from_str("./a/c/file.txt").expect("");
        let z2 = NodeData::from_str("./a/b/c/file2.txt").expect("");

        tree.add(x);
        tree.add(y);
        tree.add(z);
        tree.add(z2);

        // let cmp = x.ancestors().partial_cmp(y.ancestors()).expect("msg");
        // println!("Compare x={:?} y={:?} {:?}", x, y, cmp);

        // let cmp = x.ancestors().partial_cmp(z.ancestors()).expect("msg");
        // println!("Compare x={:?} z={:?} {:?}", x, z, cmp);

        // let cmp = x.ancestors().partial_cmp(z2.ancestors()).expect("msg");
        // println!("Compare x={:?} z2={:?} {:?}", x, z2, cmp);

        // for c in x.components() {
        //     println!("Component x={:?}, {:?}", x, c);
        // }
        dbg!(tree);
    }
}
