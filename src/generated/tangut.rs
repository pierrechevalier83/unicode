
/// An enum to represent all characters in the Tangut block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Tangut {
    /// \u{17000}: '𗀀'
    IdeographFirst,
    /// \u{187f7}: '𘟷'
    IdeographLast,
}

impl Into<char> for Tangut {
    fn into(self) -> char {
        match self {
            Tangut::IdeographFirst => '𗀀',
            Tangut::IdeographLast => '𘟷',
        }
    }
}

impl std::convert::TryFrom<char> for Tangut {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𗀀' => Ok(Tangut::IdeographFirst),
            '𘟷' => Ok(Tangut::IdeographLast),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Tangut {
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

impl std::convert::TryFrom<u32> for Tangut {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Tangut {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Tangut {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Tangut::IdeographFirst
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Tangut::IdeographFirst => "tangut ideograph first",
            Tangut::IdeographLast => "tangut ideograph last",
        }
    }
}
