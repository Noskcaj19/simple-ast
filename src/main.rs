use simple_ast::{MarkdownNode, Parser, Rule};

use simple_ast::markdown_rules::*;

fn main() {
    let rules: Vec<&Rule<MarkdownNode>> = vec![
        &Escape,
        &Newline,
        &Bold,
        &Underline,
        &Italic,
        &Strikethrough,
        &Spoiler,
        &Code,
        &InlineCode,
        &Text,
    ];

    let parser = Parser::with_rules(rules);
    let i = "_fooff_ **bar _foo_**";
    let result = parser.parse(i);
    println!("\nResult:\n{:#?}", result);
    println!("{}", i);
    println!("{:#?}", result.as_markdown());

    let mut line = String::new();
    let mut buffer = String::new();
    loop {
        std::io::stdin().read_line(&mut line).unwrap();
        if !line.trim().is_empty() {
            buffer.push('\n');
            buffer.push_str(&line);
            line.clear();
            continue;
        }
        println!("===========");

        let i = buffer.trim();
        let result = parser.parse(i);
        println!("Result: {:#?}", result);
        println!("Input: {}", i);
        println!("Generated: {:#?}", result.as_markdown());
        buffer.clear();
        println!("+++++++++++");
    }
}
