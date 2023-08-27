use std::cmp::{min, max};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Book {
    Genesis,
    Exodus,
    Leviticus,
    Numbers,
    Deuteronomy,
    Joshua,
    Judges,
    Ruth,
    Samuel1,
    Samuel2,
    Kings1,
    Kings2,
    Chronicles1,
    Chronicles2,
    Ezra,
    Nehemiah,
    Esther,
    Job,
    Psalms,
    Proverbs,
    Ecclesiastes,
    SongOfSolomon,
    Isaiah,
    Jeremiah,
    Lamentations,
    Ezekiel,
    Daniel,
    Hosea,
    Joel,
    Amos,
    Obadiah,
    Jonah,
    Micah,
    Nahum,
    Habakkuk,
    Zephaniah,
    Haggai,
    Zechariah,
    Malachi,
    Matthew,
    Mark,
    Luke,
    John,
    Acts,
    Romans,
    Corinthians1,
    Corinthians2,
    Galatians,
    Ephesians,
    Philippians,
    Colossians,
    Thessalonians1,
    Thessalonians2,
    Timothy1,
    Timothy2,
    Titus,
    Philemon,
    Hebrews,
    James,
    Peter1,
    Peter2,
    John1,
    John2,
    John3,
    Jude,
    Revelation,
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Book::Exodus => write!(f, "Exodus"),
            Book::Genesis => write!(f, "Genesis"),
            Book::Leviticus => write!(f, "Leviticus"),
            Book::Numbers => write!(f, "Numbers"),
            Book::Deuteronomy => write!(f, "Deuteronomy"),
            Book::Joshua => write!(f, "Joshua"),
            Book::Judges => write!(f, "Judges"),
            Book::Ruth => write!(f, "Ruth"),
            Book::Samuel1 => write!(f, "1 Samuel"),
            Book::Samuel2 => write!(f, "2 Samuel"),
            Book::Kings1 => write!(f, "1 Kings"),
            Book::Kings2 => write!(f, "2 Kings"),
            Book::Chronicles1 => write!(f, "1 Chronicles"),
            Book::Chronicles2 => write!(f, "2 Chronicles"),
            Book::Ezra => write!(f, "Ezra"),
            Book::Nehemiah => write!(f, "Nehemiah"),
            Book::Esther => write!(f, "Esther"),
            Book::Job => write!(f, "Job"),
            Book::Psalms => write!(f, "Psalms"),
            Book::Proverbs => write!(f, "Proverbs"),
            Book::Ecclesiastes => write!(f, "Ecclesiastes"),
            Book::SongOfSolomon => write!(f, "SongOfSolomon"),
            Book::Isaiah => write!(f, "Isaiah"),
            Book::Jeremiah => write!(f, "Jeremiah"),
            Book::Lamentations => write!(f, "Lamentations"),
            Book::Ezekiel => write!(f, "Ezekiel"),
            Book::Daniel => write!(f, "Daniel"),
            Book::Hosea => write!(f, "Hosea"),
            Book::Joel => write!(f, "Joel"),
            Book::Amos => write!(f, "Amos"),
            Book::Obadiah => write!(f, "Obadiah"),
            Book::Jonah => write!(f, "Jonah"),
            Book::Micah => write!(f, "Micah"),
            Book::Nahum => write!(f, "Nahum"),
            Book::Habakkuk => write!(f, "Habakkuk"),
            Book::Zephaniah => write!(f, "Zephaniah"),
            Book::Haggai => write!(f, "Haggai"),
            Book::Zechariah => write!(f, "Zechariah"),
            Book::Malachi => write!(f, "Malachi"),
            Book::Matthew => write!(f, "Matthew"),
            Book::Mark => write!(f, "Mark"),
            Book::Luke => write!(f, "Luke"),
            Book::John => write!(f, "John"),
            Book::Acts => write!(f, "Acts"),
            Book::Romans => write!(f, "Romans"),
            Book::Corinthians1 => write!(f, "1 Corinthians"),
            Book::Corinthians2 => write!(f, "2 Corinthians"),
            Book::Galatians => write!(f, "Galatians"),
            Book::Ephesians => write!(f, "Ephesians"),
            Book::Philippians => write!(f, "Philippians"),
            Book::Colossians => write!(f, "Colossians"),
            Book::Thessalonians1 => write!(f, "1 Thessalonians"),
            Book::Thessalonians2 => write!(f, "2 Thessalonians"),
            Book::Timothy1 => write!(f, "1 Timothy"),
            Book::Timothy2 => write!(f, "2 Timothy"),
            Book::Titus => write!(f, "Titus"),
            Book::Philemon => write!(f, "Philemon"),
            Book::Hebrews => write!(f, "Hebrews"),
            Book::James => write!(f, "James"),
            Book::Peter1 => write!(f, "1 Peter"),
            Book::Peter2 => write!(f, "2 Peter"),
            Book::John1 => write!(f, "1 John"),
            Book::John2 => write!(f, "2 John"),
            Book::John3 => write!(f, "3 John"),
            Book::Jude => write!(f, "Jude"),
            Book::Revelation => write!(f, "Revelation"),
        }
    }
}

#[derive(Clone, Copy)]
struct Passage {
    book: Book,
    chapter: u8,
    start_verse: u8,
    end_verse: u8,
}

impl Passage {
    fn new(book: Book, chapter: u8, start_verse: u8, end_verse: u8) -> Passage {
        Passage {
            book,
            chapter,
            start_verse,
            end_verse,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{} {}:{}-{}",
            self.book.to_string(),
            self.chapter,
            self.start_verse,
            self.end_verse
        )
    }

    fn merge(&self, other: &Passage) -> Passage {
        assert!(self.book == other.book);
        assert!(self.chapter == other.chapter);
        let start_verse = min(self.start_verse, other.start_verse);
        let end_verse = max(self.end_verse, other.end_verse);
        
        Passage {
            book: self.book,
            chapter: self.chapter,
            start_verse,
            end_verse,
        }
    }

    fn merge_all(passages: &Vec<Passage>) -> Vec<Passage> {
        let mut result: HashMap<(Book, u8), Passage> = HashMap::new();
        
        for passage in passages {
            let key = (passage.book, passage.chapter);
            if result.contains_key(&key) {
                let existing = result.get(&key).unwrap().clone();
                result.insert(key, existing.merge(passage));
            } else {
                result.insert(key, passage.clone());
            }
        }

        result.values().cloned().collect()
    }
}
