use core::num;
use std::collections::HashMap;

use path_clean::PathClean;
use std::ffi::OsString;
use std::path::{self, PathBuf};
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
    leaf_node_ids: Vec<NodeId>,
    pub num_leaf_node: usize,
    pub num_selected: usize,
}

pub type TreePtr = Rc<RefCell<Tree>>;

impl Default for Tree {
    fn default() -> Self {
        Self {
            nodes: vec![Node::new_root()],
            leaf_node_ids: vec![],
            num_leaf_node: 0,
            num_selected: 0,
        }
    }
}

impl Tree {
    pub fn new_from_paths<I>(file_paths: I) -> Result<TreePtr, TreeError>
    where
        I: IntoIterator<Item = PathBuf>,
    {
        let tree = Rc::new(RefCell::new(Tree::default()));

        for file_path in file_paths {
            tree.borrow_mut().add(file_path)?;
        }

        Ok(tree)
    }

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

                    if is_leaf_node {
                        self.num_leaf_node += 1;
                        self.leaf_node_ids.push(child_id);
                    }

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
            self.update_num_selected();
        }
    }

    fn correct_parents_mark(&mut self, node_id: NodeId) {
        if let Some(parent_id) = self.get_node(node_id).parent {
            self.nodes[parent_id].mark =
                self.compute_mark_from_children(parent_id);
            self.correct_parents_mark(parent_id)
        }
    }

    fn update_num_selected(&mut self) {
        let mut num_selected = 0;
        let mut stack = vec![self.root_id()];

        while let Some(node_id) = stack.pop() {
            let node = self.get_node(node_id);

            if node.mark == Mark::Unselected {
                continue;
            } else {
                for (_, node_id) in &node.children {
                    stack.push(*node_id);
                }
            }

            if node.is_leaf_node() && node.mark == Mark::Selected {
                num_selected += 1;
            }
        }
        self.num_selected = num_selected
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

    pub fn get_path_buf(&self, node_id: NodeId) -> PathBuf {
        // let mut node_id = node_id;
        let mut stack = vec![];

        let mut curr_id = Some(node_id);

        while let Some(node_id) = curr_id {
            let node = self.get_node(node_id);
            if let Some(key) = node.key.clone() {
                stack.push(key);
            }
            curr_id = node.parent;
        }

        let path_buf: PathBuf = stack.iter().rev().collect();

        path_buf
    }

    pub fn get_selected_file_paths(&self) -> Vec<PathBuf> {
        let selected: Vec<PathBuf> = self
            .leaf_node_ids
            .iter()
            .filter_map(|node_id| {
                let node = self.get_node(*node_id);
                if let Mark::Selected = node.mark {
                    node.fullpath.clone()
                } else {
                    None
                }
            })
            .collect();

        selected
    }

    pub fn get_remaining_tree(&self) -> Result<TreePtr, TreeError> {
        let new_tree = Tree::new_ptr();

        for node_id in &self.leaf_node_ids {
            let node = self.get_node(*node_id);
            if node.mark == Mark::Unselected {
                if let Some(path) = node.fullpath.clone() {
                    new_tree.borrow_mut().add(path)?;
                }
            }
        }

        Ok(new_tree)
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

        let file_paths = vec![
            PathBuf::from_str("./a/b/c/file.txt").expect(""),
            PathBuf::from_str("./a/b/file.txt").expect(""),
            PathBuf::from_str("./a/c/file.txt").expect(""),
            PathBuf::from_str("a/b/c/file2.txt").expect(""),
        ];
        let tree = Tree::new_from_paths(file_paths).expect("");

        tree
    }

    #[test]
    fn test_add() -> Result<(), io::Error> {
        let tree = prepare_tree();

        dbg!(&tree);

        assert_eq!(tree.borrow().size(), 8 + 1);
        assert_eq!(tree.borrow().num_leaf_node, 4);

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

        assert_eq!(tree.borrow().num_selected, 2);

        let selected = tree.borrow().get_selected_file_paths();

        assert_eq!(selected.len(), 2);

        let remaining = tree.borrow().get_remaining_tree().expect("");

        assert_eq!(remaining.borrow().num_leaf_node, 2);
        assert_eq!(remaining.borrow().size(), 8 + 1 - 3);

        dbg!(selected);

        Ok(())
    }
}
