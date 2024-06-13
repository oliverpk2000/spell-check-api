fn get_bigram(word:String) -> Vec<String>{
    let mut result = vec![];
    for i in 0..(word.len()-1){
        let bigram_slice_one = &word[i..i+1];
        let bigram_slice_two = &word[i+1..i+2];
        result.push(bigram_slice_one.to_owned() + bigram_slice_two);
    }
    result
}

