/// means remainder and delimter now have same lifetime
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimeter: &'a str,
}

impl<'a> StrSplit<'a> {
    /// * means remainder and haystack now have same lifetime
    /// * And therefore hastack will live as long as remainder
    ///   which will live as long as delimeter live
    /// * In Short haystack, remainder & delimeter will have lifetime of largest of them.
    pub fn new(haystack: &'a str, delimeter: &'a str) -> Self {
        Self {
            remainder: haystack,
            delimeter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimeter) {
            let until_delimeter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[(next_delim + self.delimeter.len())..];
            return Some(until_delimeter);
        }
        if self.remainder.len() > 0 {
            let last = &self.remainder[..];
            self.remainder = "";
            return Some(last);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::StrSplit;

    #[test]
    fn it_works() {
        let haystack = "a b c d ";
        let letters = StrSplit::new(haystack, " ");
        assert_eq!(
            // letters.into_iter().collect::<Vec<&str>>(),
            letters.into_iter().collect::<Vec<_>>(),
            vec!["a", "b", "c", "d"]
        );
    }

    #[test]
    fn it_works2() {
        let haystack = "ab c d";
        let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
        assert_eq!(letters, vec!["ab", "c", "d"]);
    }

    #[test]
    fn it_works3() {
        let haystack = "a:b c: ddd:";
        let letters = StrSplit::new(haystack, ":");
        assert!(letters.eq(vec!["a", "b c", " ddd"]));
    }

    #[test]
    fn it_works4() {
        let haystack = "Hello world";
        let letters = StrSplit::new(haystack, "o");
        assert!(letters.eq(vec!["Hell", " w", "rld"]))
    }

    #[test]
    fn it_works5() {
        let haystack = "Hello world";
        let letters = StrSplit::new(haystack, " wo");
        assert!(letters.eq(vec!["Hello", "rld"]));
    }
}
