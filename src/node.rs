use std::cell::RefCell;
use std::collections::HashMap;

use std::ffi::OsString;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Clone)]
pub struct Node<V> {
    pub value: Option<V>,
    pub parent: Option<Pointer<V>>,
    pub children: Option<HashMap<OsString, Pointer<V>>>,
}

impl<V> fmt::Debug for Node<V>
where
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value: {:?}, children: {:?}", self.value, self.children)
    }
}

pub type Pointer<T> = Rc<RefCell<Node<T>>>;

#[derive(Clone, Debug)]
pub struct NodeData {
    pub selected: bool,
    pub path: PathBuf,
}

impl FromStr for NodeData {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let selected = false;
        let path = PathBuf::from_str(s)?;

        Ok(NodeData { selected, path })
    }
}

#[derive(Clone, Debug)]
pub struct NodeItem {
    pub key: OsString,
    pub node: Node<NodeData>,
}

pub trait NameGettable {
    fn get_name(&self) -> &str;
}

pub trait ItemMarkable {
    fn marked(&self) -> bool;
}

impl ItemMarkable for Pointer<NodeData> {
    fn marked(&self) -> bool {
        let borrowed = self.borrow();

        match borrowed.value.as_ref() {
            Some(value) => value.selected,
            None => match borrowed.children.as_ref() {
                Some(children) => {
                    if let Some(marked) =
                        children.iter().map(|item| item.1.marked()).max()
                    {
                        marked
                    } else {
                        false
                    }
                }
                None => false,
            },
        }
    }
}
