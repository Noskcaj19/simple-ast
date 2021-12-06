use crate::regex::{Captures, Regex};
use crate::{MarkdownNode, ParseSpec, Rule};
use lazy_static::lazy_static;
use std::cell::RefCell;

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
    Emoji,
    ChannelMention,
    UserMention,
    RoleMention,
}

pub struct BlockQuote {
    in_quote: RefCell<bool>,
}

impl BlockQuote {
    pub fn new() -> BlockQuote {
        BlockQuote {
            in_quote: RefCell::new(false),
        }
    }
}

lazy_static! {
    static ref ESCAPE: Regex = Regex::new(r"^\\([^0-9A-Za-z\s])").unwrap();
    static ref NEWLINE: Regex = Regex::new(r"^(?:\n *)*\n").unwrap();
    static ref BOLD: Regex =
        Regex::new(r"^\*\*([\s\S]+?)\*\*(?!\*)").unwrap();
    static ref UNDERLINE: Regex = Regex::new(r"^__([\s\S]+?)__(?!_)").unwrap();
    static ref ITALICS: Regex = Regex::new(concat!(
        "^\\b_", "((?:__|\\\\[\\s\\S]|[^\\\\_])+?)_", "\\b",
        "|",
        // Or match *s that are followed by a non-space:
        "^\\*(?=\\S)(",
        // Match any of:
        //  - `**`: so that bolds inside italics don't close the
        // italics
        //  - whitespace
        //  - non-whitespace, non-* characters
        "(?:\\*\\*|\\s+(?:[^*\\s]|\\*\\*)|[^\\s*])+?",
        // followed by a non-space, non-* then *
        ")\\*(?!\\*)"
    )).unwrap();
    static ref STRIKETHROUGH: Regex = Regex::new(r"^~~([\s\S]+?)~~(?!_)").unwrap();
    static ref TEXT: Regex =
        Regex::new(r"^[\s\S]+?(?=[^0-9A-Za-z\s\x{00c0}-\x{ffff}]|\n| {2,}\n|\w+:\S|$)")
            .unwrap();

    // Additional Discord rules
    static ref INLINE_CODE: Regex = Regex::new(r"^(`+)(\s*([\s\S]*?[^`])\s*)\1(?!`)").unwrap();
    static ref CODE: Regex = Regex::new(
        r"^```((([A-z0-9-]+?)\n+)?\n*([\S\s]+?)\n*)```",
    ).unwrap();
    static ref SPOILER: Regex = Regex::new(r"^\|\|([\s\S]+?)\|\|").unwrap();
    static ref BLOCK_QUOTE: Regex = Regex::new(r"^( *>>> +([\s\S]*))|^( *>(?!>>) +([^\n]*(\n *>(?!>>) +[^\n]*)*\n?))").unwrap();
    static ref CHANNEL_MENTION: Regex = Regex::new(r"^<#(\d+?)>").unwrap();
    static ref ROLE_MENTION: Regex = Regex::new(r"^<@&(\d+?)>").unwrap();
    static ref EMOJI: Regex = Regex::new(r"^<a?:(.+?):(\d+?)>").unwrap();
    static ref USER_MENTION: Regex = Regex::new(r"^<@!?(\d+?)>").unwrap();
}

impl Rule<MarkdownNode> for Escape {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(1).unwrap();
        let text = captures.at(1).unwrap();
        ParseSpec::create_terminal(Some(MarkdownNode::Text(text.to_owned())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        ESCAPE.captures(src)
    }
}

impl Rule<MarkdownNode> for Newline {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(0).unwrap();
        ParseSpec::create_terminal(Some(MarkdownNode::Text("\n".to_owned())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        NEWLINE.captures(src)
    }
}

impl Rule<MarkdownNode> for Bold {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_nonterminal(Some(MarkdownNode::Bold(Vec::new())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        BOLD.captures(src)
    }
}

impl Rule<MarkdownNode> for Underline {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_nonterminal(Some(MarkdownNode::Underline(vec![])), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        UNDERLINE.captures(&src)
    }
}

impl Rule<MarkdownNode> for Italic {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = match captures.pos(1) {
            Some(pos) => pos,
            None => captures.pos(2).unwrap(),
        };
        ParseSpec::create_nonterminal(Some(MarkdownNode::Italic(vec![])), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        ITALICS.captures(&src)
    }
}

impl Rule<MarkdownNode> for Strikethrough {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_nonterminal(Some(MarkdownNode::Strikethrough(vec![])), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        STRIKETHROUGH.captures(&src)
    }
}

impl Rule<MarkdownNode> for Text {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(0).unwrap();
        let text = captures.at(0).unwrap();
        ParseSpec::create_terminal(Some(MarkdownNode::Text(text.to_owned())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        TEXT.captures(src)
    }
}

impl Rule<MarkdownNode> for InlineCode {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(2).unwrap();
        let text = captures.at(3).unwrap();
        ParseSpec::create_terminal(Some(MarkdownNode::InlineCode(text.to_owned())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        INLINE_CODE.captures(src)
    }
}

impl Rule<MarkdownNode> for Code {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(1).unwrap();
        let language = captures.at(3).unwrap_or("");
        let text = captures.at(4).unwrap();
        ParseSpec::create_terminal(
            Some(MarkdownNode::Code(language.to_owned(), text.to_owned())),
            start,
            end,
        )
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        CODE.captures(src)
    }
}

impl Rule<MarkdownNode> for Spoiler {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_nonterminal(Some(MarkdownNode::Spoiler(Vec::new())), start, end)
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        SPOILER.captures(src)
    }
}

impl Rule<MarkdownNode> for BlockQuote {
    fn accept_match(&self, last_capture: Option<&str>) -> bool {
        match last_capture {
            Some(last_capture) => {
                if last_capture.ends_with('\n') {
                    !*self.in_quote.borrow()
                } else {
                    false
                }
            }
            None => true,
        }
    }

    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        // group 2 for >>> and group 3 for >
        *self.in_quote.borrow_mut() = true;
        let single_line = captures.pos(2).is_none();
        if single_line {
            // group 4 excludes the leading >, which prevents infinite loops
            let (start, end) = captures.pos(4).unwrap();

            ParseSpec::create_nonterminal(
                Some(MarkdownNode::SingleBlockQuote(Vec::new())),
                start,
                end,
            )
        } else {
            let (start, end) = captures.pos(2).unwrap();

            ParseSpec::create_nonterminal(Some(MarkdownNode::BlockQuote(Vec::new())), start, end)
        }
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        BLOCK_QUOTE.captures(src)
    }
}

impl Rule<MarkdownNode> for UserMention {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_terminal(
            Some(MarkdownNode::UserMention(
                captures.at(1).and_then(|id| id.parse().ok()).unwrap(),
            )),
            start,
            end,
        )
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        USER_MENTION.captures(src)
    }
}

impl Rule<MarkdownNode> for ChannelMention {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_terminal(
            Some(MarkdownNode::ChannelMention(
                captures.at(1).and_then(|id| id.parse().ok()).unwrap(),
            )),
            start,
            end,
        )
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        CHANNEL_MENTION.captures(src)
    }
}

impl Rule<MarkdownNode> for RoleMention {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, end) = captures.pos(1).unwrap();
        ParseSpec::create_terminal(
            Some(MarkdownNode::RoleMention(
                captures.at(1).and_then(|id| id.parse().ok()).unwrap(),
            )),
            start,
            end,
        )
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        ROLE_MENTION.captures(src)
    }
}

impl Rule<MarkdownNode> for Emoji {
    fn parse(&self, captures: &Captures) -> ParseSpec<MarkdownNode> {
        let (start, _) = captures.pos(1).unwrap();
        let (_, end) = captures.pos(2).unwrap();
        ParseSpec::create_terminal(
            Some(MarkdownNode::Emoji(
                captures.at(1).unwrap().to_owned(),
                captures.at(2).and_then(|id| id.parse().ok()).unwrap(),
            )),
            start,
            end,
        )
    }

    fn captures<'a>(&self, src: &'a str) -> Option<Captures<'a>> {
        EMOJI.captures(src)
    }
}

