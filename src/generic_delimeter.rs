pub struct StrSplit<'h, D> {
    remainder: Option<&'h str>,
    delimeter: D,
}

impl<'h, D> StrSplit<'h, D> {
    pub fn new(haystack: &'h str, delimeter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimeter,
        }
    }
}

pub trait Delimeter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Delimeter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(*self).map(|start| (start, start + self.len()))
    }
}

impl Delimeter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(*self).map(|start| (start, start + self.len_utf8()))
    }
}

// here Strsplit is generic over D
// only thing StrSplit needs is someting that implements find_next()
impl<'h, D> Iterator for StrSplit<'h, D>
where
    D: Delimeter,
{
    type Item = &'h str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some((delim_start, delim_end)) = self.delimeter.find_next(&remainder) {
            let until_delimeter = &remainder[..delim_start];
            *remainder = &remainder[delim_end..];
            Some(until_delimeter)
        } else {
            self.remainder.take()
        }
    }
}

fn until_char(s: &str, c: char) -> &str {
    StrSplit::new(s, c)
        .next()
        .expect("Strsplit atleast return one result")
}

fn until_str<'a>(s: &'a str, find: &str) -> &'a str {
    StrSplit::new(s, find)
        .next()
        .expect("Strsplit atleast return one result")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn until_char_test() {
        assert_eq!(until_char("hello_world", 'o'), "hell");
    }

    #[test]
    fn until_str_test() {
        assert_eq!(until_str("hello_world", "orl"), "hello_w");
    }

    #[test]
    fn it_works() {
        let haystack = "a b c d ";
        let letters = StrSplit::new(haystack, ' ');
        assert_eq!(
            // letters.into_iter().collect::<Vec<&str>>(),
            letters.into_iter().collect::<Vec<_>>(),
            vec!["a", "b", "c", "d", ""]
        );
    }

    #[test]
    fn it_works_2() {
        let haystack = "Hello world";
        let letters = StrSplit::new(haystack, "o");
        assert!(letters.eq(vec!["Hell", " w", "rld"]))
    }

    #[test]
    fn it_works_3() {
        let haystack = "Hello world";
        let letters: Vec<_> = StrSplit::new(haystack, "_").collect();
        // println!("{:?}", letters); assert_eq!(letter.len(), 0);

        assert_eq!(letters, vec!["Hello world"]);
    }
}
