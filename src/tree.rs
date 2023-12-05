use std::rc::Rc;

struct Tree<K, V> {
    root: Pointer<K, V>,
    size: u32,
}

struct Node<K, V> {
    key: K,
    data: V,
    parent: Pointer<K, V>,
    children: Vec<Pointer<K, V>>,
}

impl<K, V> Tree<K, V> {
    pub fn new() -> Tree<K, V> {
        Tree {
            root: None,
            size: 0,
        }
    }

    pub fn add(self, key: K, value: V) {
        
    }
}

type Pointer<K, T> = Option<Rc<Node<K, T>>>;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let tree: Tree<String, u32> = Tree::new();
        assert_eq!(tree.size, 0);
    }

    #[test]
    fn test_add() {
        let tree: Tree<String, u32> = Tree::new();
        tree.add()
    }
}
