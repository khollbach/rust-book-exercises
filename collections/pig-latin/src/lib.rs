pub fn to_pig(s: &str) -> String {
    let mut buf = String::new();

    let (first, rest) = {
        let mut chars = s.chars();
        match chars.next() {
            Some(c) => (c, chars),
            None => return buf,
        }
    };

    if is_vowel(first) {
        buf.push(first);
    }
    buf.extend(rest);
    buf.push('-');
    buf.push(if is_vowel(first) { 'h' } else { first });
    buf.push_str("ay");

    buf
}

fn is_vowel(ch: char) -> bool {
    match ch.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("irst-fay", to_pig("first"));
        assert_eq!("apple-hay", to_pig("apple"));
        assert_eq!("ou-yay", to_pig("you"));
        assert_eq!("", to_pig(""));
    }
}
