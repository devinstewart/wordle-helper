use fancy_regex::Regex;
use crate::{KnownState, State, Suggestion};
use crate::data::Word;
use crate::data::{all_words, alphabet};


pub fn make_guess(mut known_status: KnownState) -> KnownState {
    let suggestion = &known_status.suggestions[known_status.suggestions.len() - 1];
    let mut correct_count: usize = 0;
    suggestion.iter().enumerate().for_each(|(i, _value)| {
        match suggestion[i].state {
            State::WrongPlace => {
                if !known_status.word_contains.contains(&suggestion[i].letter) {
                    known_status.word_contains.push(suggestion[i].letter);
                }
                if !known_status.position_is_not[i].contains(&suggestion[i].letter) {
                    known_status.position_is_not[i].push(suggestion[i].letter);
                }
                let mut found_elsewere_count: usize = 0;
                suggestion.iter().enumerate().for_each(|(j, _value)| {
                    if suggestion[i].letter == suggestion[j].letter && (suggestion[j].state == State::Correct  || suggestion[j].state == State::WrongPlace) {
                        found_elsewere_count += 1;
                    }
                });
                if found_elsewere_count > 1 && !known_status.word_contains_multiple.contains(&suggestion[i].letter) {
                    known_status.word_contains_multiple.push(suggestion[i].letter);
                }
            }
            State::Wrong => {
                let mut found_elsewere_count: usize = 0;
                if !known_status.position_is_not[i].contains(&suggestion[i].letter) {
                    known_status.position_is_not[i].push(suggestion[i].letter);
                }

                suggestion.iter().enumerate().for_each(|(j, _value)| {
                    if suggestion[i].letter == suggestion[j].letter && (suggestion[j].state == State::Correct || suggestion[j].state == State::WrongPlace) {
                        found_elsewere_count += 1;
                    }
                });

                if found_elsewere_count > 1 && !known_status.word_contains_multiple.contains(&suggestion[i].letter) {
                    known_status.word_contains_multiple.push(suggestion[i].letter);
                } else if found_elsewere_count == 1 && !known_status.word_contains_only_one.contains(&suggestion[i].letter) {
                    known_status.word_contains_only_one.push(suggestion[i].letter);
                } else if !known_status.word_does_not_contain.contains(&suggestion[i].letter) {
                    known_status.word_does_not_contain.push(suggestion[i].letter);
                }
            }
            State::Correct => {
                correct_count += 1;
                if !known_status.word_contains.contains(&suggestion[i].letter) {
                    known_status.word_contains.push(suggestion[i].letter);
                }

                known_status.position_is[i] = Some(suggestion[i].letter);
                if correct_count == suggestion.len() {
                    known_status.answer_found = true;
                }

            }
            State::Unknown => {}
        }
    });

    if known_status.answer_found {
        return known_status;
    }

    known_status.position_regex.iter_mut().enumerate().for_each(|(i, regex)| {
        *regex = "[".to_string();
        let alphabet = alphabet();
        alphabet.iter().for_each(|letter| {
            if known_status.position_is[i].is_some() {
                if known_status.position_is[i].unwrap() == *letter {
                    regex.push(*letter);
                }
            } else if !(known_status.position_is_not[i].contains(letter) || known_status.word_does_not_contain.contains(letter)) {
                regex.push(*letter);
            }
        });
        regex.push_str("]");
    });

    let mut final_regex = "".to_string();

    known_status.word_contains.iter().enumerate().for_each(|(_i, letter)| {
       final_regex.push_str("(?=.*");
       final_regex.push(*letter);
       final_regex.push_str(")");
    });

    known_status.word_contains_multiple.iter().enumerate().for_each(|(_i, letter)| {
        final_regex.push_str("(?=.*");
        final_regex.push(*letter);
        final_regex.push_str(".*");
        final_regex.push(*letter);
        final_regex.push_str(")");
    });


    final_regex.push_str("(");
    final_regex.push_str(&known_status.position_regex.join(""));
    final_regex.push_str(")$");

    let words = all_words();

    let re = Regex::new(&final_regex);
    let mut all_possible_words: Vec<Word> = words.into_iter().filter(|word| {
        match &re {
            Ok(re) => {
                re.is_match(&word.word).unwrap()
            },
            Err(_) => false
        }
    }).map(|word| {
        word
    }).collect();

    known_status.word_contains_only_one.iter().enumerate().for_each(|(_i, letter)| {
        let re = Regex::new(&format!("{}", letter)).unwrap();
        all_possible_words = all_possible_words.clone().into_iter().filter(|word| {
            re.find_iter(&word.word).count() == 1
        }).map(|word| {
            word
        }).collect();
    });

    let final_suggestion: Word = all_possible_words.into_iter().fold(Word { word: "I give up", value: 0 }, |acc, word| {
        if word.value > acc.value {
            word
        } else {
            acc
        }
    });

    if final_suggestion.word == "I give up" {
        known_status.game_lost = true;
        return known_status;
    }

    let letters: Vec<char> = final_suggestion.word.chars().collect();

    let new_suggestion: Vec<Suggestion> = letters.into_iter().map(|letter| {
        Suggestion {
            letter,
            state: State::Unknown
        }
    }).collect();

    known_status.suggestions.push(new_suggestion);

    known_status
}
