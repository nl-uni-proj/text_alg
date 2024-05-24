mod ansi;
mod lexer;

fn main() {
    let rust_docs = ThemeGroup::new("rust_docs", 1, 3);
    let lex_wiki = ThemeGroup::new("lex_wiki", 2, 4);
    let hobbit_book = ThemeGroup::new("hobbit_book", 3, 3);

    analyze_group(rust_docs);
    analyze_group(lex_wiki);
    analyze_group(hobbit_book);
}

struct ThemeGroup {
    name: &'static str,
    group_id: usize,
    text_count: usize,
}

impl ThemeGroup {
    fn new(name: &'static str, group_id: usize, text_count: usize) -> ThemeGroup {
        ThemeGroup {
            name,
            group_id,
            text_count,
        }
    }
}

fn analyze_group(group: ThemeGroup) {
    let cwd = std::env::current_dir().expect("failed to get cwd");

    for file_idx in 1..=group.text_count {
        let filename = format!("text/{}_{file_idx}.txt", group.group_id);
        let filepath = cwd.join(filename);
        let words = lexer::lex_stemmed(filepath);

        pretty_print_words(&group, file_idx, &words);
    }
}

fn pretty_print_words(group: &ThemeGroup, file_idx: usize, words: &[String]) {
    let g = ansi::GREEN_BOLD;
    let r = ansi::RESET;
    println!("\n{g}GROUP: `{}`, FILE: `{file_idx}`{r}", group.name);

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
