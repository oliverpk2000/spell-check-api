mod spell_check;

use axum::extract::Path;
use axum::{routing::get, Json, Router};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[tokio::main]
async fn main() {
    println!("running on 0.0.0.0:3000/");

    let app = Router::new()
        .route("/", get(|| async { "the api is at /words/:word" }))
        .route("/words/:word", get(get_corrected_words));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_corrected_words(Path(arg): Path<String>) -> Json<Vec<String>> {
    let words = load_word_vec("src/wordlists/english.txt");
    let best_match = spell_check::auto_correct(&arg, words);
    let correct_words = best_match;
    Json(correct_words)
}

fn load_word_vec(filename: impl AsRef<std::path::Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
