use crate::{types::index::IndexEntry, USER_AGENT};

use super::consts::LETTER_BASE;

pub fn get_letter_string() -> Vec<String> {
    let mut letters = vec![String::from("#")];

    for x in (b'A'..=b'Z').map(char::from) {
        letters.append(&mut vec![x.to_string()])
    }

    return letters;
}

pub fn get_letter_results(letter: String) -> Vec<IndexEntry> {
    let mut entrties: Vec<IndexEntry> = vec![];

    let client = reqwest::blocking::Client::new();

    let res = client
        .get(format!("{}/{}", LETTER_BASE, letter))
        .header("User-Agent", USER_AGENT)
        .send();

    let res_body = match res {
        Ok(body) => body,
        Err(error) => panic!("Error while fetching letter.\nDetails: {error}"),
    };

    //TODO: Search good library for souping
    //TODO: Continue with https://github.com/marcus-crane/khinsider/blob/v2/pkg/scrape/scrape.go#L49

    return entrties;
}
