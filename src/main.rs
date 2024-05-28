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

struct TextData {
    theme_idx: usize,
    word_list: Vec<String>,
    vocab: Vocab,
    unique_tf: Vec<f32>,
}

struct Vocab {
    word_freq_set: HashMap<String, u32>,
    unique_words: Vec<String>,
}

impl TextData {
    fn new(theme_idx: usize, word_list: Vec<String>) -> TextData {
        let vocab = Vocab::new_from_words(&word_list);
        let unique_len = vocab.unique_words.len();

        let mut unique_tf = Vec::with_capacity(unique_len);

        for word in vocab.unique_words.iter() {
            let freq = vocab.word_freq(&word) as f32;
            let total = word_list.len() as f32;
            unique_tf.push(freq / total);
        }

        TextData {
            theme_idx,
            word_list,
            vocab,
            unique_tf,
        }
    }
}

impl Vocab {
    fn new() -> Vocab {
        Vocab {
            word_freq_set: HashMap::new(),
            unique_words: Vec::new(),
        }
    }

    fn new_from_words(words: &[String]) -> Vocab {
        let mut vocab = Vocab::new();
        vocab.extend(words);
        vocab
    }

    fn new_from_text_data(text_data: &[TextData]) -> Vocab {
        let mut vocab = Vocab::new();
        for data in text_data {
            vocab.extend(&data.word_list);
        }
        vocab
    }

    fn word_freq(&self, string: &str) -> u32 {
        *self.word_freq_set.get(string).unwrap()
    }

    fn extend(&mut self, words: &[String]) {
        for word in words {
            let new_word = !self.word_freq_set.contains_key(word);
            let entry = self.word_freq_set.entry(word.clone()).or_insert(0);
            *entry += 1;

            if new_word {
                self.unique_words.push(word.clone());
            }
        }
    }
}

fn main() {
    let text_data = lex_texts();
    let vocab = Vocab::new_from_text_data(&text_data);
    let word_text_matrix = create_word_text_matrix(&vocab, &text_data);
    let idf_scores = create_idf_scores(&vocab, &word_text_matrix);

    pretty_print_text_theme_matrix();
    pretty_print_word_text_matrix(&word_text_matrix);
    pretty_print_idf_scores(&vocab.unique_words, &idf_scores);
    pretty_print_word_frequency_table(&vocab, true);
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
        let data = TextData::new(theme_idx, words);

        pretty_print_words(theme_idx, text_idx, &source, &data.word_list);
        pretty_print_word_frequency_table(&data.vocab, false);
        pretty_print_tf_scores(&data.vocab.unique_words, &data.unique_tf);

        text_data.push(data);
    }
    text_data
}

fn create_word_text_matrix(vocab: &Vocab, text_data: &[TextData]) -> WordTextMatrix {
    let mut entries = Vec::new();

    for unique in vocab.unique_words.iter() {
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

fn create_idf_scores(vocab: &Vocab, word_text_matrix: &WordTextMatrix) -> Vec<f32> {
    let mut idf_scores = Vec::with_capacity(vocab.unique_words.len());

    for (_, text_freqs) in word_text_matrix.entries.iter() {
        let text_occurences: u8 = text_freqs
            .iter()
            .map(|freq| if *freq != 0 { 1 } else { 0 })
            .sum();
        let idf = TEXT_COUNT as f32 / text_occurences as f32;
        idf_scores.push(idf);
    }

    idf_scores
}

fn print_separator() {
    println!("\n---------------------------------------------------");
}

fn pretty_print_words(theme_idx: usize, text_idx: usize, source: &str, words: &[String]) {
    print_separator();
    println!("THEME: `{}`, TEXT: `{text_idx}`", THEME_NAMES[theme_idx]);
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

fn pretty_print_word_frequency_table(vocab: &Vocab, sep: bool) {
    if sep {
        print_separator();
    } else {
        println!("");
    }
    println!("WORD FREQUENCY TABLE:");

    for (word, freq) in vocab.word_freq_set.iter() {
        println!("{:14} | {}", word, freq);
    }
}

fn pretty_print_tf_scores(unique_words: &[String], unique_tf: &[f32]) {
    println!("");
    println!("TF SCORE:");

    for i in 0..unique_words.len() {
        let word = &unique_words[i];
        let tf = unique_tf[i];
        println!("{:14} | {}", word, tf);
    }
}

fn pretty_print_idf_scores(unique_words: &[String], idf_scores: &[f32]) {
    print_separator();
    println!("IDF SCORE:");

    for i in 0..unique_words.len() {
        let word = &unique_words[i];
        let idf = idf_scores[i];
        println!("{:14} | {}", word, idf);
    }
}
