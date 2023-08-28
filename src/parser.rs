use std::fs::File;

use pest::{iterators::Pair, Parser};

use crate::structs::Passage;

#[derive(pest_derive::Parser)]
#[grammar = "verses.pest"]
pub struct VerseParser;

pub(crate) fn parse(s: &str) -> Vec<Passage> {
    let file = VerseParser::parse(Rule::references, s)
        .expect("Parsing failed")
        .next()
        .unwrap();
    let passages = file.into_inner().filter_map(parse_reference).collect();
    passages
}

fn parse_reference(reference: Pair<Rule>) -> Option<Passage> {
    assert_eq!(reference.as_rule(), Rule::reference);

    match reference.as_rule() {
        Rule::reference => {
            let mut inner = reference.into_inner();

            let book = inner.next().unwrap().as_str();
            let chapter = inner.next().unwrap().as_str();

            let mut verses = inner.next().unwrap().into_inner();
            let start_verse = verses.next().unwrap().as_str().parse::<u8>().unwrap();
            let end_verse = if let Some(end_verse) = verses.next() {
                end_verse.as_str().parse::<u8>().unwrap()
            } else {
                start_verse
            };

            Some(Passage {
                book: book.parse().unwrap(),
                chapter: chapter.parse().unwrap(),
                start_verse,
                end_verse,
            })
        }
        _ => None,
    }
}
