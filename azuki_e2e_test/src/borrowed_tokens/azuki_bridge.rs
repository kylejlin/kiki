pub type Text<'src> = &'src str;
pub type Words<'src> = Vec<Text<'src>>;

pub fn empty<'src>() -> Words<'src> {
    Vec::new()
}

pub fn push<'src>(mut words: Words<'src>, word: Text<'src>) -> Words<'src> {
    words.push(word);
    words
}
