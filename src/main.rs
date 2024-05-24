use std::collections::HashMap;
mod lexer;

const THEME_COUNT: usize = 3;
const THEME_NAMES: [&'static str; THEME_COUNT] = ["rust_docs", "lex_wiki", "hobbit_book"];

const TEXT_COUNT: usize = 10;
const TEXT_THEME_IDX: [usize; TEXT_COUNT] = [0, 0, 0, 1, 1, 1, 1, 2, 2, 2];

const THEME_TEXT_MATRIX: TextThemeMatrix = [
    [1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
];

type TextThemeMatrix = [[u8; TEXT_COUNT]; THEME_COUNT];

struct WordTextMatrix {
    entries: Vec<(String, [u8; TEXT_COUNT])>,
}

struct WordWordMatrix {
    entries: HashMap<(String, String), u32>,
}

struct TextData {
    theme_idx: usize,
    word_list: Vec<String>,
}

struct Vocab {
    word_freq_set: HashMap<String, u32>,
    unique_ordered: Vec<String>,
}

impl TextData {
    fn new(theme_idx: usize, word_list: Vec<String>) -> TextData {
        TextData {
            theme_idx,
            word_list,
        }
    }
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
    let text_data = lex_texts();
    let vocab = create_vocab(&text_data);
    let word_text_matrix = create_word_text_matrix(&vocab, &text_data);
    let word_word_matrix = create_word_word_matrix(&text_data);

    pretty_print_text_theme_matrix();
    pretty_print_word_text_matrix(&word_text_matrix);
    pretty_print_word_word_matrix(&word_word_matrix);
    println!("");
}

fn lex_texts() -> Vec<TextData> {
    let cwd: std::path::PathBuf = std::env::current_dir().expect("failed to get cwd");
    let mut text_data = Vec::new();

    for text_idx in 0..TEXT_COUNT {
        let theme_idx = TEXT_THEME_IDX[text_idx];
        let filename = format!("text/{}_{}.txt", theme_idx + 1, text_idx + 1);
        let filepath = cwd.join(filename);
        let source = std::fs::read_to_string(filepath).expect("file read failed");

        let words = lexer::lex(&source, false);
        pretty_print_words(theme_idx, text_idx, &source, &words);
        text_data.push(TextData::new(theme_idx, words));
    }
    text_data
}

fn create_vocab(text_data: &[TextData]) -> Vocab {
    let mut vocab = Vocab::new();

    for data in text_data {
        vocab.extend(&data.word_list);
    }
    vocab
}

fn create_word_text_matrix(vocab: &Vocab, text_data: &[TextData]) -> WordTextMatrix {
    let mut entries = Vec::new();

    for unique in vocab.unique_ordered.iter() {
        let mut text_word_counts = [0; TEXT_COUNT];
        for (text_idx, data) in text_data.iter().enumerate() {
            let count: u8 = data
                .word_list
                .iter()
                .map(|word| if unique == word { 1 } else { 0 })
                .sum();
            text_word_counts[text_idx] = count;
        }
        entries.push((unique.clone(), text_word_counts));
    }

    WordTextMatrix { entries }
}

fn create_word_word_matrix(text_data: &[TextData]) -> WordWordMatrix {
    let mut entries = HashMap::new();

    for data in text_data {
        let words = &data.word_list;
        for i in 0..words.len() - 1 {
            let word_1 = &words[i];
            let word_2 = &words[i + 1];
            *entries.entry((word_1.clone(), word_2.clone())).or_insert(0) += 1;
        }
    }

    WordWordMatrix { entries }
}

fn print_separator() {
    println!("\n---------------------------------------------------");
}

fn pretty_print_words(theme_idx: usize, text_idx: usize, source: &str, words: &[String]) {
    print_separator();
    println!("GROUP: `{}`, TEXT: `{text_idx}`", THEME_NAMES[theme_idx]);
    println!("\nORIGINAL TEXT:");
    println!("{source}\n");
    println!("PROCESSED TEXT:");

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

fn print_text_index_row() {
    print!("{:14}", "text index");
    for text_idx in 0..TEXT_COUNT {
        print!("{:2} ", text_idx);
    }
    print!("\n");
}

fn pretty_print_text_theme_matrix() {
    print_separator();
    println!("THEME x TEXT MATRIX:");
    print_text_index_row();

    for theme_idx in 0..THEME_COUNT {
        let theme_name = THEME_NAMES[theme_idx];
        println!("{:14}{:?}", theme_name, THEME_TEXT_MATRIX[theme_idx]);
    }
}

fn pretty_print_word_text_matrix(matrix: &WordTextMatrix) {
    print_separator();
    println!("WORD x TEXT MATRIX:");
    print_text_index_row();

    for (word, counts) in matrix.entries.iter() {
        println!("{:14}{:?}", word, counts);
    }
}

fn pretty_print_word_word_matrix(matrix: &WordWordMatrix) {
    print_separator();
    println!("WORD x WORD PAIR MATRIX:");

    for ((word_1, word_2), freq) in matrix.entries.iter() {
        println!("{:14} | {:14} | {:?}", word_1, word_2, *freq);
    }
}
