use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
mod ansi;
mod lexer;

const TEXT_COUNT: usize = 10;
const THEME_COUNT: usize = 3;

const THEME_NAMES: [&'static str; THEME_COUNT] = ["rust_docs", "lex_wiki", "hobbit_book"];
const THEME_TEXT_COUNTS: [usize; THEME_COUNT] = [3, 4, 3];

const THEME_TEXT_MATRIX: TextThemeMatrix = [
    [1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
];

type WordTextMatrix = Vec<[bool; TEXT_COUNT]>;
type TextThemeMatrix = [[u32; TEXT_COUNT]; THEME_COUNT];

struct ThemeData {
    text_word_lists: Vec<Vec<String>>,
}

impl ThemeData {
    fn new() -> ThemeData {
        ThemeData {
            text_word_lists: Vec::new(),
        }
    }
}

struct Vocab {
    word_freq_set: HashMap<String, u32>,
    unique_ordered: Vec<String>,
}

impl Vocab {
    fn new() -> Vocab {
        Vocab {
            word_freq_set: HashMap::new(),
            unique_ordered: Vec::new(),
        }
    }

    fn extend(&mut self, words: &[String]) {
        for word in words {
            let new_word = !self.word_freq_set.contains_key(word);
            let entry = self.word_freq_set.entry(word.clone()).or_insert(0);
            *entry += 1;

            if new_word {
                self.unique_ordered.push(word.clone());
            }
        }
    }
}

fn main() {
    let mut theme_data = vec![];
    for theme_idx in 0..THEME_COUNT {
        let data = lex_theme_group(theme_idx);
        theme_data.push(data);
    }

    let vocal = create_vocab(&theme_data);

    pretty_print_theme_text_matrix();
    println!("");
}

fn lex_theme_group(theme_idx: usize) -> ThemeData {
    let cwd: std::path::PathBuf = std::env::current_dir().expect("failed to get cwd");
    let mut data: ThemeData = ThemeData::new();

    for text_idx in 0..THEME_TEXT_COUNTS[theme_idx] {
        let filename = format!("text/{}_{}.txt", theme_idx + 1, text_idx + 1);
        let filepath = cwd.join(filename);
        let words = lexer::lex(filepath, false);

        pretty_print_words(theme_idx, text_idx, &words);
        data.text_word_lists.push(words);
    }
    return data;
}

fn create_vocab(theme_data: &[ThemeData]) -> Vocab {
    let mut vocab = Vocab::new();

    for data in theme_data {
        for word_list in data.text_word_lists.iter() {
            vocab.extend(&word_list);
        }
    }
    return vocab;
}

fn pretty_print_words(theme_idx: usize, text_idx: usize, words: &[String]) {
    let g = ansi::GREEN_BOLD;
    let r = ansi::RESET;
    println!(
        "\n{g}GROUP: `{}`, TEXT: `{text_idx}`{r}",
        THEME_NAMES[theme_idx]
    );

    const PRINT_WIDTH: usize = 60;
    let mut width = 0;
    for word in words {
        print!("{word} ");
        width += word.len();
        if width >= PRINT_WIDTH {
            width = 0;
            print!("\n");
        }
    }
    if width != 0 {
        print!("\n");
    }
}

fn pretty_print_theme_text_matrix() {
    let g = ansi::GREEN_BOLD;
    let r = ansi::RESET;
    println!("\n{g}THEME x TEXT MATRIX:{r}");

    print!("{:12}", "text index");
    for text_idx in 0..TEXT_COUNT {
        print!("{:2} ", text_idx);
    }
    print!("\n");

    for theme_idx in 0..THEME_COUNT {
        let theme_name = THEME_NAMES[theme_idx];
        println!("{:12}{:?}", theme_name, THEME_TEXT_MATRIX[theme_idx]);
    }
}
