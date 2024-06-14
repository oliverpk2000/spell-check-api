fn get_bigram(word: &str) -> Vec<String> {
    let mut result = vec![];
    for i in 0..(word.len() - 1) {
        let bigram_slice_one = &word[i..i + 1];
        let bigram_slice_two = &word[i + 1..i + 2];
        result.push(bigram_slice_one.to_owned() + bigram_slice_two);
    }
    result
}

fn get_similarity(word1: &str, word2: &str) -> f64 {
    let word1_lc = word1.to_ascii_lowercase();
    let word2_lc = word2.to_ascii_lowercase();

    let bigram1 = get_bigram(&word1_lc);
    let bigram2 = get_bigram(&word2_lc);

    let mut similar = 0.0;

    for i in 0..(bigram1.len() - 1) {
        let bigram_bit_1 = bigram1.get(i).unwrap();

        let index_optional = bigram2
            .iter()
            .position(|bigram_bit_2| bigram_bit_2 == bigram_bit_1);

        match index_optional {
            None => {}
            Some(_) => {
                similar += 1.0;
            }
        }
    }
    let similarity = similar / (std::cmp::max(bigram1.len() as i32, bigram2.len() as i32) as f64);

    return similarity;
}

pub fn auto_correct(word: &str, word_list: Vec<String>) -> Vec<String> {

    let mut best_match = SimilarWord{word: word.to_string(), similarity_score: 0.0};

    for i in 0..word_list.len() {
        let similarity = get_similarity(&word, &word_list[i]);

        if similarity > best_match.similarity_score {
            best_match = SimilarWord {
                similarity_score: similarity,
                word: word_list[i].to_string(),
            }
        }
    }

    if best_match.similarity_score < 0.5 {
        return vec![word.to_string()];
    }

    println!("{}: {}", best_match.word, best_match.similarity_score);

    return vec![best_match.word];
}

struct SimilarWord {
    similarity_score: f64,
    word: String,
}
