use onig::Captures;

pub trait Rule {
    fn parse(&self, captures: Captures) -> crate::ParseSpec;
    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>>;
    fn name(&self) -> String {
        "unnamed".to_string()
    }
}
