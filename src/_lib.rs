pub struct StrSplit<'h, 'd> {
    remainder: Option<&'h str>,
    delimeter: &'d str,
}

impl<'h, 'd> StrSplit<'h, 'd> {
    pub fn new(haystack: &'h str, delimeter: &'d str) -> Self {
        Self {
            remainder: Some(haystack),
            delimeter,
        }
    }
}

impl<'h, 'd> Iterator for StrSplit<'h, 'd> {
    type Item = &'h str;
    fn next(&mut self) -> Option<Self::Item> {
        // matching remainder of type(&'a str) to self.remainder (&'a str)
        // here we need mutable reference to self.remainder hence we use "ref mut"
        // to convert remainder to &mut &'a str
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimeter) {
                let result = &remainder[..next_delim];
                // remainder is of type (&mut &'a str) hence * to dereferance to get &'a str
                *remainder = &remainder[(next_delim + self.delimeter.len())..];
                Some(result)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

/// mutiple lifetimes
// pub fn until_char<'a> (s: &'a str, c: char) -> &'a str { // this is behind
pub fn until_char(s: &str, c: char) -> &str {
    // here compiler automatically infers lifetimes
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("Str split aleast return one thing.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_works() {
        let haystack = "a b c d";

        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn next_works_2() {
        let haystack = "a b c d ";

        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
    }

    #[test]
    fn until_char_works() {
        assert_eq!(until_char("Hello World", 'o'), "Hell");
    }
}
