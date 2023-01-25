/// means remainder and delimter now have same lifetime
/// as delmeter doesn't need to exit as long as remainder we can give delimeter its own lifetime
/// now we say lifetime of delimeter is only tide to StrSplit not to remainder.
pub struct StrSplit<'h, 'd> {
    remainder: Option<&'h str>,
    delimeter: &'d str,
}

impl<'h, 'd> StrSplit<'h, 'd> {
    /// * means remainder and haystack now have same lifetime
    /// * And therefore hastack will live as long as remainder
    ///   which will live as long as delimeter live
    /// * In Short haystack, remainder & delimeter will have lifetime of largest of them.
    pub fn new(haystack: &'h str, delimeter: &'d str) -> Self {
        Self {
            remainder: Some(haystack),
            delimeter,
        }
    }
}

impl<'h> Iterator for StrSplit<'h, '_> {
    type Item = &'h str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some(next_delim) = remainder.find(self.delimeter) {
            let until_delimeter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimeter.len())..];
            Some(until_delimeter)
        } else {
            self.remainder.take()
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, &format!("{}", c))
        .next()
        .expect("Strsplit atleast return one result")
}

#[cfg(test)]
mod tests {
    use super::{until_char, StrSplit};

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello_world", 'o'), "hell");
    }

    #[test]
    fn it_works() {
        let haystack = "a b c d ";
        let letters = StrSplit::new(haystack, " ");
        assert_eq!(
            // letters.into_iter().collect::<Vec<&str>>(),
            letters.into_iter().collect::<Vec<_>>(),
            vec!["a", "b", "c", "d", ""]
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
        assert!(letters.eq(vec!["a", "b c", " ddd", ""]));
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

    #[test]
    fn it_works6() {
        let haystack = "Hello world";
        let letters: Vec<_> = StrSplit::new(haystack, "_").collect();
        // println!("{:?}", letters); assert_eq!(letter.len(), 0);

        assert_eq!(letters, vec!["Hello world"]);
    }
}
