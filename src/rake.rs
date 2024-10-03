use crate::stopwords::get_stopwords;
use std::collections::HashMap;

/// Rake (Rapid Automatic Keyword Extracted) by Rust
pub struct Rake {
    text: String,
    stopwords: Vec<String>,
    sentences: Vec<String>,
    candidate_keyphrases: Vec<String>,
    word_scores: HashMap<String, f64>,
    keyphrase_scores: HashMap<String, f64>,
    keyphrases: Vec<String>,
}

impl Rake {
    /// Create a new 'Rake' object with a given text.
    ///
    /// Automatic load stopwords for the English Language.
    ///
    /// ## Arguments
    ///
    /// - `text`: The text to extract key phrases from
    ///
    /// ## Example
    ///
    /// ```rust
    /// let rake = Rake::new(String::from("Natural Language Processing is amazing!"));
    /// ```
    pub fn new(text: String) -> Self {
        Self {
            text,
            stopwords: get_stopwords("en").unwrap(),
            sentences: Vec::new(),
            candidate_keyphrases: Vec::new(),
            word_scores: HashMap::new(),
            keyphrase_scores: HashMap::new(),
            keyphrases: Vec::new(),
        }
    }

    /// Process the text and extract key phrases
    ///
    /// ## Example
    ///
    /// ```rust
    /// let rake = Rake::new(String::from("AI is a branch of Computer Science."));
    /// rake.process();
    /// ```
    pub fn process(&mut self) {
        self.extract_sentences();
        self.extract_candidate_keyphrases();
        self.calculate_word_scores();
        self.calculate_keyphrase_scores();

        self.load_keyphrases();
    }

    /// Set stopwords from the specific language
    ///
    /// ## Example
    ///
    /// ```rust
    /// let rake = Rake::new(String::from("RAKE adalah algoritma untuk mengekstraksi kata kunci dari teks"));
    /// rake.set_stopwords("id"); // set stopwords to Indonesian
    /// ```
    pub fn set_stopwords(&mut self, lang: &str) {
        self.stopwords = get_stopwords(lang).unwrap();
    }

    /// Set stopwords from a file
    ///
    /// ## Example
    ///
    /// ```rust
    /// let mut rake = Rake::new(String::from("Text with custom stopwords."));
    /// rake.set_stopwords_from_file("./stopwords.txt");
    /// ```
    pub fn set_stopwords_from_file(&mut self, filepath: &str) {
        let raw_stopwords = fs::read_to_string(filepath).unwrap();
        self.stopwords = raw_stopwords
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect();
    }

    // pub fn get_stopwords_list(&mut self) -> &Vec<String> {
    //     return &self.stopwords;
    // }

    fn extract_sentences(&mut self) {
        self.sentences = self
            .text
            .to_lowercase()
            .split(&['.', ','])
            .map(str::to_string)
            .collect();
    }

    fn extract_candidate_keyphrases(&mut self) {
        for sentence in &self.sentences {
            let words: Vec<&str> = sentence.split_whitespace().collect();

            let mut keyphrase: Vec<String> = Vec::new();

            for word in &words {
                if !self.stopwords.contains(&word.to_string()) {
                    keyphrase.push(word.to_string());
                } else {
                    if keyphrase.len() > 0 {
                        self.candidate_keyphrases.push(keyphrase.join(" "));
                        keyphrase.clear()
                    }
                }
            }

            if keyphrase.len() > 0 {
                self.candidate_keyphrases.push(keyphrase.join(" "));
                keyphrase.clear()
            }
        }
    }

    fn calculate_word_scores(&mut self) {
        let mut word_freq: HashMap<String, i32> = HashMap::new();
        let mut word_degree: HashMap<String, i32> = HashMap::new();

        for phrase in &self.candidate_keyphrases {
            let words: Vec<_> = phrase.split_whitespace().collect();
            let word_count = words.len();

            for word in &words {
                *word_freq.entry(word.to_string()).or_insert(0) += 1;
                *word_degree.entry(word.to_string()).or_insert(0) += word_count as i32;
            }
        }

        for word in word_freq.keys() {
            self.word_scores.insert(
                word.clone(),
                (*word_degree.get(word).unwrap() as f64) / (*word_freq.get(word).unwrap() as f64),
            );
        }
    }

    fn calculate_keyphrase_scores(&mut self) {
        for phrase in &self.candidate_keyphrases {
            let words: Vec<_> = phrase.split_whitespace().collect();

            let mut score = 0.0;

            for word in &words {
                if let Some(value) = self.word_scores.get(&word.to_string()) {
                    score += value;
                }
            }

            self.keyphrase_scores.insert(phrase.clone(), score);
        }
    }

    /// Get keyphrase scores in descending order.
    ///
    /// ## Returns
    ///
    /// A sorted vector of keyphrase-score pairs.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let keyphrases = rake.keyphrase_scores_descending();
    /// ```
    pub fn keyphrase_scores_descending(&self) -> Vec<(String, f64)> {
        let mut sorted: Vec<_> = self.keyphrase_scores.clone().into_iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        return sorted;
    }

    /// Get keyphrase scores in ascending order.
    ///
    /// ## Returns
    ///
    /// A sorted vector of keyphrase-score pairs.
    ///
    /// ## Example
    ///
    /// ```rust
    /// let keyphrases = rake.keyphrase_scores_ascending();
    /// ```
    pub fn keyphrase_scores_ascending(&self) -> Vec<(String, f64)> {
        let mut sorted: Vec<_> = self.keyphrase_scores.clone().into_iter().collect();
        sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        return sorted;
    }

    fn load_keyphrases(&mut self) {
        self.keyphrases = self.keyphrase_scores.keys().cloned().collect();
    }
}
