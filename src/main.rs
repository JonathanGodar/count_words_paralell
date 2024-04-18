use rayon::prelude::*;
use std::time::Instant;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let text = read_to_string("loremipsum.txt").unwrap();

    // Count the words in parallell
    let start = Instant::now();
    for _ in 0..100 {
        count_words_par(&text);
    }

    let total_time = start.elapsed();
    dbg!(total_time);

    // Count the words on a single thread
    let start = Instant::now();
    for _ in 0..100 {
        count_words(&text);
    }

    let total_time = start.elapsed();
    dbg!(total_time);
}

fn count_words_par(text: &str) -> HashMap<String, i32> {
    let words = text.par_split_ascii_whitespace();

    words
        .fold(HashMap::<String, i32>::new, add_word_to_count)
        .reduce(HashMap::new, combine_hashmaps)
}

fn count_words(text: &str) -> HashMap<String, i32> {
    let words = text.split_whitespace();
    let result = words.fold(HashMap::<String, i32>::new(), |map, val| {
        add_word_to_count(map, val)
    });

    result
}

fn add_word_to_count(mut map: HashMap<String, i32>, word: &str) -> HashMap<String, i32> {
    map.entry(clean_word(word))
        .and_modify(|count| *count += 1)
        .or_insert(1);
    map
}

fn combine_hashmaps(mut a: HashMap<String, i32>, b: HashMap<String, i32>) -> HashMap<String, i32> {
    for (key, value) in b.into_iter() {
        a.entry(key)
            .and_modify(|val| *val += value)
            .or_insert(value);
    }

    a
}

fn clean_word(word: &str) -> String {
    let res = word.strip_suffix('.').unwrap_or(word);
    let res = res.strip_suffix(',').unwrap_or(res);

    res.to_lowercase()
}
