/// Author: users/hakt
/// 
/// Kata:
/// Complete the solution so that the function will break up camel casing, using a space between words.

#[allow(dead_code)]
pub fn solution(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        if ch.is_uppercase() {
            result.push(' ')
        }
        result.push(ch)
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution("camelCasing"), "camel Casing");
        assert_eq!(solution("camelCasingTest"), "camel Casing Test");
    }
}
