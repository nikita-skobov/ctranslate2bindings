use ctranslate2bindings::*;
use sentencepiece::SentencePieceProcessor;
use std::io;


fn main() {
    println!("Enter what you want to be translated:");
    let stdin = io::stdin();
    let mut input: String = "".into();
    stdin.read_line(&mut input).expect("Failed to read from stdin?");
    let spp = SentencePieceProcessor::open("sentencepiece.model").unwrap();
    let pieces = spp.encode(&input).unwrap()
        .into_iter().map(|p| p.piece).collect::<Vec<_>>();
    let mut pieces_ref = vec![];
    for piece in &pieces {
        pieces_ref.push(piece.as_str());
    }
    // println!("{:?}", pieces_ref);

    let mytranslator = Translator::new("./model").unwrap();
    // let data = vec!["▁H", "ello"];
    // let data = vec!["▁Hola", "."];
    println!("Translating:\n{:#?}", pieces_ref);
    let result = mytranslator.translate(pieces_ref);
    println!("To:\n{:#?}", result);
}
