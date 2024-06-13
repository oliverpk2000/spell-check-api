fn get_bigram(word: String) -> Vec<String> {
    let mut result = vec![];
    for i in 0..(word.len() - 1) {
        let bigram_slice_one = &word[i..i + 1];
        let bigram_slice_two = &word[i + 1..i + 2];
        result.push(bigram_slice_one.to_owned() + bigram_slice_two);
    }
    result
}

pub fn get_similarity(mut word1: String, mut word2: String) -> f64 {
    word1 = word1.to_ascii_lowercase();
    word2 = word2.to_ascii_lowercase();

    let bigram1 = get_bigram(word1);
    let bigram2 = get_bigram(word2);

    let mut similar = 0.0;

    for i in 0..bigram1.len() {
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
