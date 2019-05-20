use crate::{MarkdownNode, ParseSpec, Rule};
use lazy_static::lazy_static;
use onig::{Captures, Regex};

macro_rules! styles {
    ( $( $style:ident ),* $(,)? ) => {
        $(
            pub struct $style;
        )*
    };
}

styles! {
    Escape,
    Newline,
    Bold,
    Underline,
    Italic,
    Strikethrough,
    Text,
    InlineCode,
    Code,
    Spoiler,
}

lazy_static! {
    static ref ESCAPE: Regex = Regex::new(r"^\\([^0-9A-Za-z\s])").unwrap();
    static ref NEWLINE: Regex = Regex::new(r"^(?:\n *)*\n").unwrap();
    static ref BOLD: Regex =
        Regex::new(r"^\*\*([\s\S]+?)\*\*(?!\*)").unwrap();
    static ref UNDERLINE: Regex = Regex::new(r"^__([\s\S]+?)__(?!_)").unwrap();
    static ref ITALICS: Regex = Regex::new(concat!(
        "^\\b_" , "((?:__|\\\\[\\s\\S]|[^\\\\_])+?)_" , "\\b",
        "|" ,
        // Or match *s that are followed by a non-space:
        "^\\*(?=\\S)(" ,
        // Match any of:
        //  - `**`: so that bolds inside italics don't close the
        // italics
        //  - whitespace
        //  - non-whitespace, non-* characters
        "(?:\\*\\*|\\s+(?:[^*\\s]|\\*\\*)|[^\\s*])+?" ,
        // followed by a non-space, non-* then *
        ")\\*(?!\\*)"
    )).unwrap();
    static ref STRIKETHROUGH: Regex = Regex::new(r"^~~(?=\S)([\s\S]*?\S)~~").unwrap();
    static ref TEXT: Regex =
        Regex::new(r"^[\s\S]+?(?=[^0-9A-Za-z\s\u00c0-\uffff]|\n| {2,}\n|\w+:\S|$)")
            .unwrap();

    // Additional Discord rules
    static ref INLINE_CODE: Regex = Regex::new(r"^`([\s\S]+?)`").unwrap();
    static ref CODE: Regex = Regex::with_options(
        r"^```(([^\n]+)?(?:\n)(.+?))```",
        onig::RegexOptions::REGEX_OPTION_MULTILINE,
        onig::Syntax::default(),
    ).unwrap();
    static ref SPOILER: Regex = Regex::with_options(
        r"^\|\|(.+?)\|\|",
        onig::RegexOptions::REGEX_OPTION_MULTILINE,
        onig::Syntax::default(),
    ).unwrap();
}

impl Rule for Escape {
    fn parse(&self, captures: Captures) -> ParseSpec {
        let (start, end) = captures.pos(1).unwrap();
        let text = captures.at(1).unwrap();
        ParseSpec::create_terminal(Some(MarkdownNode::Text(text.to_owned())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        ESCAPE.captures(src)
    }

    fn name(&self) -> String {
        "Escape".to_owned()
    }
}

impl Rule for Newline {
    fn parse(&self, captures: Captures) -> ParseSpec {
        let (start, end) = captures.pos(0).unwrap();
        ParseSpec::create_terminal(Some(MarkdownNode::Text("\n".to_owned())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        NEWLINE.captures(src)
    }

    fn name(&self) -> String {
        "Newline".to_owned()
    }
}

impl Rule for Bold {
    fn parse(&self, captures: Captures) -> ParseSpec {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_nonterminal(Some(MarkdownNode::Bold(Vec::new())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        BOLD.captures(src)
    }

    fn name(&self) -> String {
        "Bold".to_owned()
    }
}

impl Rule for Underline {
    fn parse(&self, captures: Captures) -> ParseSpec {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_nonterminal(Some(MarkdownNode::Underline(vec![])), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        UNDERLINE.captures(&src)
    }

    fn name(&self) -> String {
        "Underline".to_owned()
    }
}

impl Rule for Italic {
    fn parse(&self, captures: Captures) -> ParseSpec {
        let (start, end) = match captures.pos(1) {
            Some(pos) => pos,
            None => captures.pos(2).unwrap(),
        };
        ParseSpec::create_nonterminal(Some(MarkdownNode::Italic(vec![])), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        ITALICS.captures(&src)
    }

    fn name(&self) -> String {
        "Italic".to_owned()
    }
}

impl Rule for Strikethrough {
    fn parse(&self, captures: Captures) -> ParseSpec {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_nonterminal(Some(MarkdownNode::Strikethrough(vec![])), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        STRIKETHROUGH.captures(&src)
    }

    fn name(&self) -> String {
        "Strikethrough".to_owned()
    }
}

impl Rule for Text {
    fn parse(&self, captures: Captures) -> ParseSpec {
        let (start, end) = captures.pos(0).unwrap();
        let text = captures.at(0).unwrap();
        ParseSpec::create_terminal(Some(MarkdownNode::Text(text.to_owned())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        TEXT.captures(src)
    }

    fn name(&self) -> String {
        "Text".to_owned()
    }
}

impl Rule for InlineCode {
    fn parse(&self, captures: Captures) -> ParseSpec {
        let (start, end) = captures.pos(1).unwrap();
        let text = captures.at(1).unwrap();
        ParseSpec::create_terminal(Some(MarkdownNode::InlineCode(text.to_owned())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        INLINE_CODE.captures(src)
    }

    fn name(&self) -> String {
        "Inline Code".to_owned()
    }
}

impl Rule for Code {
    fn parse(&self, captures: Captures) -> ParseSpec {
        let (start, end) = captures.pos(1).unwrap();
        let language = captures.at(2).unwrap();
        let text = captures.at(3).unwrap();
        ParseSpec::create_terminal(
            Some(MarkdownNode::Code(language.to_owned(), text.to_owned())),
            start,
            end,
        )
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        CODE.captures(src)
    }

    fn name(&self) -> String {
        "Code".to_owned()
    }
}

impl Rule for Spoiler {
    fn parse(&self, captures: Captures) -> ParseSpec {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_nonterminal(Some(MarkdownNode::Spoiler(Vec::new())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        SPOILER.captures(src)
    }

    fn name(&self) -> String {
        "Spoiler".to_owned()
    }
}
