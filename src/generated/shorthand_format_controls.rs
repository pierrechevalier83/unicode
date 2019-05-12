
/// An enum to represent all characters in the ShorthandFormatControls block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ShorthandFormatControls {
    /// \u{1bca0}: '𛲠'
    ShorthandFormatLetterOverlap,
    /// \u{1bca1}: '𛲡'
    ShorthandFormatContinuingOverlap,
    /// \u{1bca2}: '𛲢'
    ShorthandFormatDownStep,
    /// \u{1bca3}: '𛲣'
    ShorthandFormatUpStep,
}

impl Into<char> for ShorthandFormatControls {
    fn into(self) -> char {
        match self {
            ShorthandFormatControls::ShorthandFormatLetterOverlap => '𛲠',
            ShorthandFormatControls::ShorthandFormatContinuingOverlap => '𛲡',
            ShorthandFormatControls::ShorthandFormatDownStep => '𛲢',
            ShorthandFormatControls::ShorthandFormatUpStep => '𛲣',
        }
    }
}

impl std::convert::TryFrom<char> for ShorthandFormatControls {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𛲠' => Ok(ShorthandFormatControls::ShorthandFormatLetterOverlap),
            '𛲡' => Ok(ShorthandFormatControls::ShorthandFormatContinuingOverlap),
            '𛲢' => Ok(ShorthandFormatControls::ShorthandFormatDownStep),
            '𛲣' => Ok(ShorthandFormatControls::ShorthandFormatUpStep),
            _ => Err(()),
        }
    }
}

impl Into<u32> for ShorthandFormatControls {
    fn into(self) -> u32 {
        let c: char = self.into();
        let hex = c
            .escape_unicode()
            .to_string()
            .replace("\\u{", "")
            .replace("}", "");
        u32::from_str_radix(&hex, 16).unwrap()
    }
}

impl std::convert::TryFrom<u32> for ShorthandFormatControls {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for ShorthandFormatControls {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl ShorthandFormatControls {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        ShorthandFormatControls::ShorthandFormatLetterOverlap
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("ShorthandFormatControls{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
