/*

    https://www.codewars.com/kata/517abf86da9663f1d2000003

    Instructions:

    Complete the method/function so that it converts dash/underscore delimited words into camel casing. 
    The first word within the output should be capitalized only if the original word was capitalized
    (known as Upper Camel Case, also often referred to as Pascal case). 
    The next words should be always capitalized.

    Examples:

    "the-stealth-warrior" gets converted to "theStealthWarrior"

    "The_Stealth_Warrior" gets converted to "TheStealthWarrior"

    "The_Stealth-Warrior" gets converted to "TheStealthWarrior"

*/

fn to_camel_case(text: &str) -> String {
    text.split(|c: char| c == '_' || c == '-')
        .enumerate()
        .map(|(i, word)| {
            if i == 0 {
                word.to_string()
            } else {
                let mut s = String::with_capacity(word.len());
                s.push_str(&word.chars().next().unwrap().to_uppercase().to_string());
                s.push_str(&word[1..]);
                s
            }
        }).collect()
}
