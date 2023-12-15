use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::PathBuf;
use std::rc::Rc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TreeError {
    #[error("ChildrenIsNoneError")]
    ChildrenIsNoneError,
    #[error("StackIsEmptyError")]
    StackIsEmptyError,
    #[error("CannotGetFileNameError")]
    CannotGetFileNameError,
}

#[derive(Debug)]
struct Node<V> {
    data: Option<V>,
    children: Option<HashMap<OsString, Pointer<V>>>,
}

type Pointer<T> = Rc<RefCell<Node<T>>>;

#[derive(Debug)]
struct Tree<V> {
    root: Pointer<V>,
    size: u32,
}

impl<V> Tree<V>
where
    V: Copy,
{
    pub fn new() -> Tree<V> {
        Tree {
            root: Rc::new(RefCell::new(Node {
                data: None,
                children: Some(HashMap::new()),
            })),
            size: 0,
        }
    }

    pub fn add(&self, path: PathBuf, value: V) -> Result<(), TreeError> {
        let mut curr = self.root.clone();

        let filename =
            path.file_name().ok_or(TreeError::CannotGetFileNameError)?;

        for c in path.components() {
            let comp_str = c.as_os_str();
            let is_leaf_node = comp_str == filename;

            let child = Self::get_child(&curr, comp_str)?;

            match child {
                Some(found) => {
                    curr = found;
                }
                None => {
                    let new_node = Rc::new(RefCell::new(Node {
                        data: if is_leaf_node { Some(value) } else { None },
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

    fn get_child(
        curr: &Rc<RefCell<Node<V>>>,
        comp_str: &std::ffi::OsStr,
    ) -> Result<Option<Rc<RefCell<Node<V>>>>, TreeError> {
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
        let tree: Tree<u32> = Tree::new();
        assert_eq!(tree.size, 0);
    }

    #[test]
    fn test_add() {
        let tree: Tree<u32> = Tree::new();

        let x = PathBuf::from_str("./a/b/c/file.txt").expect("");
        let y = PathBuf::from_str("./a/b/file.txt").expect("");
        let z = PathBuf::from_str("./a/c/file.txt").expect("");
        let z2 = PathBuf::from_str("./a/b/c/file2.txt").expect("");

        tree.add(x, 0);
        tree.add(y, 0);
        tree.add(z, 0);
        tree.add(z2, 0);

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
