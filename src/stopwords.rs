use stop_words::LANGUAGE;

pub fn get_stopwords(lang: &str) -> Option<Vec<String>> {
    match lang {
        "id" | "indonesian" | "indonesia" => Some(stop_words::get(LANGUAGE::Indonesian)),
        "my" | "malaysian" | "malaysia" => Some(stop_words::get(LANGUAGE::Malay)),
        _ => Some(stop_words::get(LANGUAGE::English)),
    }
}
