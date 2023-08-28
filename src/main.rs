mod parser;
mod structs;

fn main() {
    let file = include_str!("../verses.txt");
    let parsed = parser::parse(file);
    dbg!(parsed.len());

    let mut merged = structs::Passage::merge_all(&parsed);
    merged.sort();

    for passage in merged.clone() {
        println!("{}", passage.to_string());
    }

    println!(
        "Total num verses: {}",
        merged.iter().map(|p| p.num_verses()).sum::<u32>()
    );
}
