use crate::Node;
use std::rc::Rc;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct ParseSpec<T: Node<T>> {
    pub root: Option<Rc<RwLock<T>>>,
    pub is_terminal: bool,
    pub start_index: usize,
    pub end_index: usize,
}

impl<T: Node<T>> ParseSpec<T> {
    pub fn create_nonterminal(
        root: Option<T>,
        start_index: usize,
        end_index: usize,
    ) -> ParseSpec<T> {
        ParseSpec {
            root: root.map(|r| Rc::new(RwLock::new(r))),
            is_terminal: false,
            start_index,
            end_index,
        }
    }

    pub fn create_terminal(root: Option<T>, start_index: usize, end_index: usize) -> ParseSpec<T> {
        ParseSpec {
            root: root.map(|r| Rc::new(RwLock::new(r))),
            is_terminal: true,
            start_index,
            end_index,
        }
    }

    pub fn create_wrapped_nonterminal(
        root: Option<Rc<RwLock<T>>>,
        start_index: usize,
        end_index: usize,
    ) -> ParseSpec<T> {
        ParseSpec {
            root,
            is_terminal: false,
            start_index,
            end_index,
        }
    }

    pub fn create_wrapped_terminal(root: Option<Rc<RwLock<T>>>) -> ParseSpec<T> {
        ParseSpec {
            root,
            is_terminal: true,
            start_index: 0,
            end_index: 0,
        }
    }

    pub fn apply_offset(&mut self, offset: usize) {
        self.start_index += offset;
        self.end_index += offset
    }
}
