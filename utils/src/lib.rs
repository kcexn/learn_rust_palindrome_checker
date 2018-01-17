use std::collections::VecDeque;

pub fn is_palindrome(text: &str) -> bool {
    let palindrome_string = String::from(text.to_lowercase());
    let mut palindrome_deque: VecDeque<char> = palindrome_string.chars().collect(); 
    while palindrome_deque.len() > 1 {
        let first_char = match palindrome_deque.pop_front() {
            Some(c) => c,
            None => panic!(),
        };

        let last_char = match palindrome_deque.pop_back() {
            Some(c) => c,
            None => panic!(),
        };

        if first_char != last_char {
            return false;
        }
    }
    return true
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
