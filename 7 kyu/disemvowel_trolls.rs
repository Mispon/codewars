/// Author: users/osuushi
/// 
/// Kata: 
/// Trolls are attacking your comment section!
/// A common way to deal with this situation is to remove all of the vowels from the trolls' comments, neutralizing the threat.
/// Your task is to write a function that takes a string and return a new string with all vowels removed.
/// For example, the string "This website is for losers LOL!" would become "Ths wbst s fr lsrs LL!".
/// Note: for this kata y isn't considered a vowel.

// My solution
#[allow(dead_code)]
fn disemvowel(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let mut result = String::new();
    for ch in s.chars() {
        if !vowels.contains(&ch.to_ascii_lowercase()) {
            result.push(ch)
        }
    }

    result
}

// Best practice
#[allow(dead_code)]
fn best_disemvowel(s: &str) -> String {
    s.chars()
        .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::disemvowel;
    
    #[test]
    fn example_test() {
        assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
    }
}