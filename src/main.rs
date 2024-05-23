mod lexer;

fn main() {
    let words = lexer::lex("some 4  q -  -1 cool \n text");
    for word in words {
        println!("{word}");
    }
}
