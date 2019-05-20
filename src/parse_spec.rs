use crate::MarkdownNode;
use std::rc::Rc;
use std::sync::RwLock;

type RootType = Option<Rc<RwLock<MarkdownNode>>>;

#[derive(Debug, Clone)]
pub struct ParseSpec {
    pub root: RootType,
    pub is_terminal: bool,
    pub start_index: usize,
    pub end_index: usize,
}

impl ParseSpec {
    pub fn create_nonterminal(
        root: Option<MarkdownNode>,
        start_index: usize,
        end_index: usize,
    ) -> ParseSpec {
        ParseSpec {
            root: root.map(|r| Rc::new(RwLock::new(r))),
            is_terminal: false,
            start_index,
            end_index,
        }
    }

    pub fn create_terminal(
        root: Option<MarkdownNode>,
        start_index: usize,
        end_index: usize,
    ) -> ParseSpec {
        ParseSpec {
            root: root.map(|r| Rc::new(RwLock::new(r))),
            is_terminal: true,
            start_index,
            end_index,
        }
    }

    pub fn create_wrapped_nonterminal(
        root: RootType,
        start_index: usize,
        end_index: usize,
    ) -> ParseSpec {
        ParseSpec {
            root,
            is_terminal: false,
            start_index,
            end_index,
        }
    }
    pub fn create_wrapped_terminal(root: RootType) -> ParseSpec {
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
