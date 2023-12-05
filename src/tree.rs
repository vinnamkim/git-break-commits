use std::path::PathBuf;
use std::rc::Rc;

struct Node<V> {
    key: PathBuf,
    data: Option<V>,
    parent: Pointer<V>,
    children: Vec<Pointer<V>>,
}

type Pointer<T> = Option<Rc<Node<T>>>;

struct Tree<V> {
    root: Pointer<V>,
    size: u32,
}

impl<V> Tree<V> {
    pub fn new() -> Tree<V> {
        Tree {
            root: Some(Rc::new(Node{key: PathBuf::new(), data: None, parent: None, children: vec![]})),
            size: 0,
        }
    }

    pub fn add(self, key: PathBuf, value: V) {
        let curr = self.root;
        
        loop {

            match &curr {
                Some(x) => {
                    for child in x.children {
                        if let Some(node) = child {
                            node.key key.
                        }
                    }
                },
                None => {
                    break
                }
            }
        }

        // while curr
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
        // let tree: Tree<String, u32> = Tree::new();
        // tree.add()
        let x = PathBuf::from_str("./a/b/c").expect("");
        let y = PathBuf::from_str("./a/b").expect("");
        let z = PathBuf::from_str("./a/c").expect("");

        println!("Compare {}", x < y);
        println!("Compare {}", x < z);
    }
}
