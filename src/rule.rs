use crate::regex::Captures;
use crate::Node;

pub trait Rule<T: Node<T>> {
    fn parse(&self, captures: Captures) -> crate::ParseSpec<T>;
    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>>;
    fn name(&self) -> String {
        "unnamed".to_string()
    }
}
