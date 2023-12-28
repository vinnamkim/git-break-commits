use std::collections::HashMap;

use path_clean::PathClean;
use std::ffi::OsString;
use std::path::PathBuf;
use std::{cell::RefCell, rc::Rc};
use thiserror::Error;

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
    #[error("OutofIndexError")]
    OutofIndexError,
    #[error("LeafNodeHasNoneMarkError")]
    LeafNodeHasNoneMarkError,
}

pub type NodeId = usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mark {
    Unselected,
    PartiallySelected,
    Selected,
}

#[derive(Debug)]
pub struct Node {
    pub key: Option<OsString>,
    pub mark: Mark,
    pub fullpath: Option<PathBuf>,
    pub parent: Option<NodeId>,
    pub children: HashMap<OsString, NodeId>,
}

impl Node {
    pub fn new_root() -> Node {
        Node {
            key: None,
            mark: Mark::Unselected,
            fullpath: None,
            parent: None,
            children: HashMap::new(),
        }
    }

    pub fn is_leaf_node(&self) -> bool {
        self.children.is_empty()
    }
}

#[derive(Debug)]
pub struct Tree {
    nodes: Vec<Node>,
}

pub type TreePtr = Rc<RefCell<Tree>>;

impl Default for Tree {
    fn default() -> Self {
        Self {
            nodes: vec![Node::new_root()],
        }
    }
}

impl Tree {
    pub fn new_ptr() -> TreePtr {
        Rc::new(RefCell::new(Tree::default()))
    }

    pub fn root_id(&self) -> NodeId {
        0
    }

    pub fn add(&mut self, value: PathBuf) -> Result<(), TreeError> {
        let cleaned_path = value.clean();

        let mut curr_id = self.root_id();
        let mut stack = PathBuf::new();

        for component in cleaned_path.components() {
            stack.push(component);
            let key = component.as_os_str();
            let is_leaf_node = stack == cleaned_path;

            match self.nodes[curr_id].children.get(key) {
                Some(child_id) => {
                    curr_id = *child_id;
                }
                None => {
                    let child_id = self.nodes.len();

                    let new_node = Node {
                        key: Some(key.to_owned()),
                        mark: Mark::Unselected,
                        fullpath: if is_leaf_node {
                            Some(cleaned_path.clone())
                        } else {
                            None
                        },
                        parent: Some(curr_id),
                        children: HashMap::new(),
                    };

                    self.nodes.push(new_node);
                    self.nodes[curr_id]
                        .children
                        .insert(key.to_owned(), child_id);
                    curr_id = child_id;
                }
            }
        }
        Ok(())
    }

    pub fn get_root(&self) -> &Node {
        let node_id = self.root_id();
        self.get_node(node_id)
    }

    pub fn get_node(&self, node_id: NodeId) -> &Node {
        &self.nodes[node_id]
    }

    pub fn find_node(&self, value: PathBuf) -> Option<NodeId> {
        let cleaned_path = value.clean();

        let mut curr_id = self.root_id();
        let mut stack = PathBuf::new();

        for component in cleaned_path.components() {
            stack.push(component);
            let key = component.as_os_str();

            match self.nodes[curr_id].children.get(key) {
                Some(child_id) => {
                    curr_id = *child_id;
                }
                None => {
                    return None;
                }
            }
        }

        let is_leaf_node = stack == cleaned_path;

        if is_leaf_node {
            return Some(curr_id);
        } else {
            return None;
        }
    }

    pub fn mark(&mut self, node_id: NodeId, new_mark: Mark) {
        self.mark_impl(node_id, new_mark, true)
    }

    fn mark_impl(
        &mut self,
        node_id: NodeId,
        new_mark: Mark,
        is_first_call: bool,
    ) {
        self.nodes[node_id].mark = new_mark;

        let children_ids: Vec<NodeId> = self
            .get_node(node_id)
            .children
            .iter()
            .map(|child| *child.1)
            .collect();

        for child_id in children_ids {
            self.mark_impl(child_id, new_mark, false)
        }

        if is_first_call {
            self.correct_parents_mark(node_id);
        }
    }

    fn correct_parents_mark(&mut self, node_id: NodeId) {
        if let Some(parent_id) = self.get_node(node_id).parent {
            self.nodes[parent_id].mark =
                self.compute_mark_from_children(parent_id);
            self.correct_parents_mark(parent_id)
        }
    }

    fn compute_mark_from_children(&self, node_id: NodeId) -> Mark {
        let node = self.get_node(node_id);

        if node.is_leaf_node() {
            return node.mark;
        }

        let children_marks: Vec<Mark> = node
            .children
            .iter()
            .map(|child| self.get_node(*child.1).mark)
            .collect();

        if children_marks.iter().all(|mark| *mark == Mark::Selected) {
            return Mark::Selected;
        } else if children_marks.iter().all(|mark| *mark == Mark::Unselected) {
            return Mark::Unselected;
        } else {
            return Mark::PartiallySelected;
        }
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs, io, str::FromStr};

    use tempfile::tempdir;

    use super::*;
    #[test]
    fn test_new() {
        let tree = Tree::new_ptr();
        assert_eq!(tree.borrow().size(), 1);
    }

    fn prepare_tree() -> Rc<RefCell<Tree>> {
        // .
        // └── a
        //     ├── b
        //     │   ├── c
        //     │   │   ├── file.txt
        //     │   │   └── file2.txt
        //     │   └── file.txt
        //     └── c
        //         └── file.txt
        // # of nodes => 8 + 1 (including root)

        let tree = Tree::new_ptr();

        let x = PathBuf::from_str("./a/b/c/file.txt").expect("");
        let y = PathBuf::from_str("./a/b/file.txt").expect("");
        let z = PathBuf::from_str("./a/c/file.txt").expect("");
        let z2 = PathBuf::from_str("a/b/c/file2.txt").expect("");

        tree.borrow_mut().add(x);
        tree.borrow_mut().add(y);
        tree.borrow_mut().add(z);
        tree.borrow_mut().add(z2);

        tree
    }

    #[test]
    fn test_add() -> Result<(), io::Error> {
        let tree = prepare_tree();

        dbg!(&tree);

        assert_eq!(tree.borrow().size(), 8 + 1);

        Ok(())
    }

    #[test]
    fn test_mark() -> Result<(), io::Error> {
        let tree = prepare_tree();

        let path = PathBuf::from_str("a/b/c").expect("");
        let node_id = tree.borrow().find_node(path).expect("");

        tree.borrow_mut().mark(node_id, Mark::Selected);

        let node_mark = tree.borrow().get_node(node_id).mark;
        assert_eq!(node_mark, Mark::Selected);
        for child in &tree.borrow().get_node(node_id).children {
            let node_mark = tree.borrow().get_node(*child.1).mark;
            assert_eq!(node_mark, Mark::Selected);
        }

        fn check_parent(node_id: NodeId, tree: &TreePtr) {
            if let Some(parent_id) = tree.borrow().get_node(node_id).parent {
                assert_eq!(
                    tree.borrow().get_node(parent_id).mark,
                    Mark::PartiallySelected
                );
                check_parent(parent_id, tree);
            }
        }
        check_parent(node_id, &tree);

        dbg!(&tree);

        Ok(())
    }
}
