#[cfg(not(feature = "pcre"))]
pub type Error = oniguruma::Error;
#[cfg(feature = "pcre")]
pub type Error = pcre2::Error;

pub struct Regex {
    #[cfg(not(feature = "pcre"))]
    inner: oniguruma::Regex,

    #[cfg(feature = "pcre")]
    inner: pcre2::bytes::Regex,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, Error> {
        #[cfg(not(feature = "pcre"))]
        let regex = oniguruma::Regex::new(pattern);
        #[cfg(feature = "pcre")]
        let regex = pcre2::bytes::RegexBuilder::new().utf(true).build(pattern);

        regex.map(|regex| Regex { inner: regex })
    }

    pub fn captures<'t>(&self, text: &'t str) -> Option<Captures<'t>> {
        #[cfg(not(feature = "pcre"))]
        let captures = self.inner.captures(text);
        #[cfg(feature = "pcre")]
        let captures = self.inner.captures(text.as_bytes()).ok().and_then(|c| c);

        captures.map(|c| Captures { inner: c })
    }
}

pub struct Captures<'a> {
    #[cfg(not(feature = "pcre"))]
    inner: oniguruma::Captures<'a>,

    #[cfg(feature = "pcre")]
    inner: pcre2::bytes::Captures<'a>,
}

impl<'t> Captures<'t> {
    pub fn pos(&self, pos: usize) -> Option<(usize, usize)> {
        #[cfg(not(feature = "pcre"))]
        return self.inner.pos(pos);

        #[cfg(feature = "pcre")]
        return self.inner.get(pos).map(|m| (m.start(), m.end()));
    }

    pub fn at(&self, pos: usize) -> Option<&'t str> {
        #[cfg(not(feature = "pcre"))]
        return self.inner.at(pos);

        #[cfg(feature = "pcre")]
        return self
            .inner
            .get(pos)
            .map(|m| unsafe { std::str::from_utf8_unchecked(m.as_bytes()) });
        // utf8 must be valid for the regex object to be constructed
    }
}
