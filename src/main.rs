use std::collections::{HashMap};

fn extract_sentences(text: String) -> Vec<String> {

    let sentences: Vec<String> = text.to_lowercase().split(&['.', ',']).map(str::to_string).collect();

    sentences
}

fn extract_candidate_keyphrases(sentences: Vec<String>) -> Vec<String> {
    let stopwords = ["is", "not", "that", "there", "are", "many", "that", "can", "you", "with", "is", "one", "of", "those"];

    let mut candidate_keyphrases: Vec<String> = Vec::new();

    for sentence in sentences {

        let words: Vec<&str> = sentence.split_whitespace().collect();

        let mut keyphrase: Vec<String> = Vec::new();

        for word in &words {

            if !stopwords.contains(word) {
                keyphrase.push(word.to_string());
            } else {
                if keyphrase.len() > 0 {
                    candidate_keyphrases.push(keyphrase.join(" "));
                    keyphrase.clear()
                }
            }

        }

        if keyphrase.len() > 0 {
            candidate_keyphrases.push(keyphrase.join(" "));
            keyphrase.clear()
        }

    }


    candidate_keyphrases

}

fn calculate_word_scores(keyphrases: Vec<String> ) -> HashMap<String, f64> {

    let mut word_freq: HashMap<String, i32> = HashMap::new();
    let mut word_degree: HashMap<String, i32> = HashMap::new();

    for phrase in keyphrases {
        let words: Vec<_> = phrase.split_whitespace().collect();
        let word_count = words.len();

        for word in &words {
            *word_freq.entry(word.to_string()).or_insert(0) += 1;
            *word_degree.entry(word.to_string()).or_insert(0) += word_count as i32;
        }
    }

    let mut word_scores: HashMap<String, f64> = HashMap::new();

    for word in word_freq.keys() {
        word_scores.insert(word.clone(), (*word_degree.get(word).unwrap() as f64) / (*word_freq.get(word).unwrap() as f64));
    }

    word_scores

}

fn calculate_keyphrase_scores(keyphrases: Vec<String>, word_scores: HashMap<String, f64>) -> HashMap<String, f64> {

    let mut keyphrases_scores: HashMap<String, f64> = HashMap::new();

    for phrase in keyphrases {
        let words: Vec<_> = phrase.split_whitespace().collect();

        let mut score = 0.0;

        for word in &words {

            if let Some(value) = word_scores.get(word.clone()) {
                score += value;
            }

        }

        keyphrases_scores.insert(phrase, score);

    }

    keyphrases_scores
}

fn main() {
    let text = String::from("Feature extraction is not that complex. There are many algorithms available that can help you with feature extraction. Rapid Automatic Keyword Extraction is one of those.");

    let sentences = extract_sentences(text);
    let keyphrases = extract_candidate_keyphrases(sentences);
    let word_scores = calculate_word_scores(keyphrases.clone());
    let keyphrase_scores = calculate_keyphrase_scores(keyphrases, word_scores);

    let mut sorted_keyphrase_scores: Vec<_> = keyphrase_scores.iter().collect();
    sorted_keyphrase_scores.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    dbg!(sorted_keyphrase_scores);

}
