use std::env;
use std::fs;

mod rake;
mod stopwords;
use rake::Rake;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--filepath".to_string()) && args.len() > 2 {
        let filepath = args.get(2).unwrap();

        let content = fs::read_to_string(filepath).ok().unwrap();

        let mut rake = Rake::new(content);

        if args.contains(&"--lang".to_string()) && args.len() > 4 {
            let lang = args.get(4).unwrap();
            rake.stopwords(lang);
        } else {
            println!("cargo run -- --filepath [] --lang [Optional]");
            return;
        }

        rake = rake.process();
    } else {
        println!("cargo run -- --filepath [] --lang [Optional]");
    }
}
