use crate::parser::Styled;
use std::{rc::Rc, sync::RwLock};

type NodeType = Rc<RwLock<MarkdownNode>>;

pub trait Node<T> {
    fn get_children(&self) -> Option<&[Rc<RwLock<T>>]>;
    fn add_child(&mut self, child: Rc<RwLock<T>>);
}

#[derive(Debug, Clone)]
pub enum MarkdownNode {
    Italic(Vec<NodeType>),
    Bold(Vec<NodeType>),
    Underline(Vec<NodeType>),
    Strikethrough(Vec<NodeType>),
    Text(String),
    InlineCode(String),
    Code(String, String),
    Spoiler(Vec<NodeType>),
    SingleBlockQuote(Vec<NodeType>),
    BlockQuote(Vec<NodeType>),
}

impl Node<MarkdownNode> for MarkdownNode {
    fn get_children(&self) -> Option<&[NodeType]> {
        match self {
            MarkdownNode::Italic(children) => Some(children),
            MarkdownNode::Bold(children) => Some(children),
            MarkdownNode::Underline(children) => Some(children),
            MarkdownNode::Strikethrough(children) => Some(children),
            MarkdownNode::Text(_) => None,
            MarkdownNode::InlineCode(_) => None,
            MarkdownNode::Code(_, _) => None,
            MarkdownNode::Spoiler(children) => Some(children),
            MarkdownNode::BlockQuote(children) => Some(children),
            MarkdownNode::SingleBlockQuote(children) => Some(children),
        }
    }

    fn add_child(&mut self, child: NodeType) {
        match self {
            MarkdownNode::Italic(ref mut children) => children.push(child),
            MarkdownNode::Bold(ref mut children) => children.push(child),
            MarkdownNode::Underline(ref mut children) => children.push(child),
            MarkdownNode::Strikethrough(ref mut children) => children.push(child),
            MarkdownNode::Text(_) => {}
            MarkdownNode::InlineCode(_) => {}
            MarkdownNode::Code(_, _) => {}
            MarkdownNode::Spoiler(ref mut children) => children.push(child),
            MarkdownNode::BlockQuote(ref mut children) => children.push(child),
            MarkdownNode::SingleBlockQuote(ref mut children) => children.push(child),
        }
    }
}

impl Styled<MarkdownNode> {
    pub fn as_markdown(&self) -> String {
        self.0
            .iter()
            .map(|s| MarkdownNode::as_markdown(&*s.read().unwrap()))
            .collect::<Vec<_>>()
            .join("")
    }
}

impl MarkdownNode {
    fn collect(styles: &[Rc<RwLock<MarkdownNode>>]) -> String {
        styles
            .iter()
            .map(|s| MarkdownNode::as_markdown(&*s.read().unwrap()))
            .collect::<Vec<_>>()
            .join("")
    }

    // TODO: take into account _/* italic syntax
    pub fn as_markdown(&self) -> String {
        use MarkdownNode::*;
        match self {
            Bold(styles) => format!("**{}**", MarkdownNode::collect(styles)),
            Italic(styles) => format!("_{}_", MarkdownNode::collect(styles)),
            Underline(styles) => format!("__{}__", MarkdownNode::collect(styles)),
            Strikethrough(styles) => format!("~~{}~~", MarkdownNode::collect(styles)),
            Spoiler(styles) => format!("||{}||", MarkdownNode::collect(styles)),
            Text(string) => string.to_owned(),
            InlineCode(string) => format!("`{}`", string),
            Code(language, string) => format!("```{}\n{}```", language, string),
            SingleBlockQuote(styles) => format!(
                "> {}",
                MarkdownNode::collect(styles)
                    .lines()
                    .collect::<Vec<_>>()
                    .join("\n> ")
            ),
            BlockQuote(styles) => format!(">>> {}", MarkdownNode::collect(styles)),
        }
    }
}
