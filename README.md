# SRAKE

Implementation of Rapid Automatic Keywords Extracted algoritm for rust.

## How to use

- Append modules to dependencies of Cargo.toml

```toml
srake = "0.1.1"
```

or

```fish
cargo add srake
```

- Import modules

```rust
use srake::*;
```

- Create a new instance of `Rake`

```rust
// Create a string of text to be processed
let text = String::from("Natural Language Processing is amazing!");

// Initialize a Rake object with the text to be analyzed
let mut rake = Rake::new(text);

// Process the text to find key phrases using the RAKE algorithm
rake.process();

// Retrieve and display key phrases sorted in ascending order by score
println!("{:?}", rake.keyphrase_scores_ascending());
```

- Set stopwords

```rust
// Sets the stopwords list based on a predefined language code
rake.set_stopwords("id");

// Loads a custom stopwords list from a file 
rake.set_stopwords_from_file("./stopwords.txt");
```
