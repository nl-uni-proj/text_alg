mod ansi;
mod lexer;

const TEXT_COUNT: usize = 10;
const THEME_COUNT: usize = 3;

const THEME_NAMES: [&'static str; THEME_COUNT] = ["rust_docs", "lex_wiki", "hobbit_book"];
const THEME_TEXT_COUNTS: [usize; THEME_COUNT] = [3, 4, 3];

const THEME_TEXT_MATRIX: [[usize; TEXT_COUNT]; THEME_COUNT] = [
    [1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 1, 1, 1, 1, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
];

fn main() {
    for theme_idx in 0..THEME_COUNT {
        analyze_group_theme(theme_idx);
    }
    pretty_print_theme_text_matrix();
    println!("");
}

fn analyze_group_theme(theme_idx: usize) {
    let cwd: std::path::PathBuf = std::env::current_dir().expect("failed to get cwd");

    for text_idx in 0..THEME_TEXT_COUNTS[theme_idx] {
        let filename = format!("text/{}_{}.txt", theme_idx + 1, text_idx + 1);
        let filepath = cwd.join(filename);
        let words = lexer::lex_stemmed(filepath);

        pretty_print_words(theme_idx, text_idx, &words);
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
