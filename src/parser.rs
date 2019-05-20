use crate::{Node, ParseSpec, Rule};
use std::rc::Rc;
use std::sync::{RwLock, RwLockWriteGuard};

pub struct Parser<T: Node<T>> {
    rules: Vec<Box<Rule<T>>>,
}

#[derive(Debug)]
pub struct Styled<T: Node<T>>(pub Vec<Rc<RwLock<T>>>);

impl<T: Node<T>> Parser<T> {
    pub fn with_rules(rules: Vec<Box<Rule<T>>>) -> Parser<T> {
        Parser { rules }
    }

    pub fn parse(&self, src: &str) -> Styled<T> {
        let mut remaining_parses = Vec::new();
        let mut top_level_nodes: Vec<Rc<RwLock<T>>> = Vec::new();

        if !src.is_empty() {
            remaining_parses.push(ParseSpec::create_nonterminal(None, 0, src.len()));
        }

        while !remaining_parses.is_empty() {
            let mut builder = remaining_parses
                .pop()
                .expect("remaining parses must not be empty");

            if builder.start_index >= builder.end_index {
                break;
            }

            let inspection_source = &src[builder.start_index..builder.end_index];
            let offset = builder.start_index;

            for rule in &self.rules {
                let captures = rule.captures(inspection_source);
                if let Some(matcher) = captures {
                    let matcher_source_end = matcher.pos(0).unwrap().1 + offset;

                    let mut new_builder = rule.parse(matcher);
                    let parent = &mut builder.root;

                    if let Some(it) = new_builder.root.clone() {
                        if let Some(ref mut parent) = parent {
                            let mut parent: RwLockWriteGuard<T> = parent.write().unwrap();
                            parent.add_child(it)
                        } else {
                            top_level_nodes.push(it)
                        }
                    }

                    // In case the last match didn't consume the rest of the source for this subtree,
                    // make sure the rest of the source is consumed.
                    if matcher_source_end != builder.end_index {
                        remaining_parses.push(ParseSpec::create_wrapped_nonterminal(
                            builder.root,
                            matcher_source_end,
                            builder.end_index,
                        ))
                    }

                    // We want to speak in terms of indices within the source string,
                    // but the Rules only see the matchers in the context of the substring
                    // being examined. Adding this offset addresses that issue.
                    if !new_builder.is_terminal {
                        new_builder.apply_offset(offset);
                        remaining_parses.push(new_builder);
                    }
                    break;
                }
            }
        }

        Styled(top_level_nodes)
    }
}
