
/// An enum to represent all characters in the GunjalaGondi block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum GunjalaGondi {
    /// \u{11d60}: '𑵠'
    LetterA,
    /// \u{11d61}: '𑵡'
    LetterAa,
    /// \u{11d62}: '𑵢'
    LetterI,
    /// \u{11d63}: '𑵣'
    LetterIi,
    /// \u{11d64}: '𑵤'
    LetterU,
    /// \u{11d65}: '𑵥'
    LetterUu,
    /// \u{11d67}: '𑵧'
    LetterEe,
    /// \u{11d68}: '𑵨'
    LetterAi,
    /// \u{11d6a}: '𑵪'
    LetterOo,
    /// \u{11d6b}: '𑵫'
    LetterAu,
    /// \u{11d6c}: '𑵬'
    LetterYa,
    /// \u{11d6d}: '𑵭'
    LetterVa,
    /// \u{11d6e}: '𑵮'
    LetterBa,
    /// \u{11d6f}: '𑵯'
    LetterBha,
    /// \u{11d70}: '𑵰'
    LetterMa,
    /// \u{11d71}: '𑵱'
    LetterKa,
    /// \u{11d72}: '𑵲'
    LetterKha,
    /// \u{11d73}: '𑵳'
    LetterTa,
    /// \u{11d74}: '𑵴'
    LetterTha,
    /// \u{11d75}: '𑵵'
    LetterLa,
    /// \u{11d76}: '𑵶'
    LetterGa,
    /// \u{11d77}: '𑵷'
    LetterGha,
    /// \u{11d78}: '𑵸'
    LetterDa,
    /// \u{11d79}: '𑵹'
    LetterDha,
    /// \u{11d7a}: '𑵺'
    LetterNa,
    /// \u{11d7b}: '𑵻'
    LetterCa,
    /// \u{11d7c}: '𑵼'
    LetterCha,
    /// \u{11d7d}: '𑵽'
    LetterTta,
    /// \u{11d7e}: '𑵾'
    LetterTtha,
    /// \u{11d7f}: '𑵿'
    LetterLla,
    /// \u{11d80}: '𑶀'
    LetterJa,
    /// \u{11d81}: '𑶁'
    LetterJha,
    /// \u{11d82}: '𑶂'
    LetterDda,
    /// \u{11d83}: '𑶃'
    LetterDdha,
    /// \u{11d84}: '𑶄'
    LetterNga,
    /// \u{11d85}: '𑶅'
    LetterPa,
    /// \u{11d86}: '𑶆'
    LetterPha,
    /// \u{11d87}: '𑶇'
    LetterHa,
    /// \u{11d88}: '𑶈'
    LetterRa,
    /// \u{11d89}: '𑶉'
    LetterSa,
    /// \u{11d8a}: '𑶊'
    VowelSignAa,
    /// \u{11d8b}: '𑶋'
    VowelSignI,
    /// \u{11d8c}: '𑶌'
    VowelSignIi,
    /// \u{11d8d}: '𑶍'
    VowelSignU,
    /// \u{11d8e}: '𑶎'
    VowelSignUu,
    /// \u{11d90}: '𑶐'
    VowelSignEe,
    /// \u{11d91}: '𑶑'
    VowelSignAi,
    /// \u{11d93}: '𑶓'
    VowelSignOo,
    /// \u{11d94}: '𑶔'
    VowelSignAu,
    /// \u{11d95}: '𑶕'
    SignAnusvara,
    /// \u{11d96}: '𑶖'
    SignVisarga,
    /// \u{11d97}: '𑶗'
    Virama,
    /// \u{11d98}: '𑶘'
    Om,
    /// \u{11da0}: '𑶠'
    DigitZero,
    /// \u{11da1}: '𑶡'
    DigitOne,
    /// \u{11da2}: '𑶢'
    DigitTwo,
    /// \u{11da3}: '𑶣'
    DigitThree,
    /// \u{11da4}: '𑶤'
    DigitFour,
    /// \u{11da5}: '𑶥'
    DigitFive,
    /// \u{11da6}: '𑶦'
    DigitSix,
    /// \u{11da7}: '𑶧'
    DigitSeven,
    /// \u{11da8}: '𑶨'
    DigitEight,
    /// \u{11da9}: '𑶩'
    DigitNine,
}

impl Into<char> for GunjalaGondi {
    fn into(self) -> char {
        match self {
            GunjalaGondi::LetterA => '𑵠',
            GunjalaGondi::LetterAa => '𑵡',
            GunjalaGondi::LetterI => '𑵢',
            GunjalaGondi::LetterIi => '𑵣',
            GunjalaGondi::LetterU => '𑵤',
            GunjalaGondi::LetterUu => '𑵥',
            GunjalaGondi::LetterEe => '𑵧',
            GunjalaGondi::LetterAi => '𑵨',
            GunjalaGondi::LetterOo => '𑵪',
            GunjalaGondi::LetterAu => '𑵫',
            GunjalaGondi::LetterYa => '𑵬',
            GunjalaGondi::LetterVa => '𑵭',
            GunjalaGondi::LetterBa => '𑵮',
            GunjalaGondi::LetterBha => '𑵯',
            GunjalaGondi::LetterMa => '𑵰',
            GunjalaGondi::LetterKa => '𑵱',
            GunjalaGondi::LetterKha => '𑵲',
            GunjalaGondi::LetterTa => '𑵳',
            GunjalaGondi::LetterTha => '𑵴',
            GunjalaGondi::LetterLa => '𑵵',
            GunjalaGondi::LetterGa => '𑵶',
            GunjalaGondi::LetterGha => '𑵷',
            GunjalaGondi::LetterDa => '𑵸',
            GunjalaGondi::LetterDha => '𑵹',
            GunjalaGondi::LetterNa => '𑵺',
            GunjalaGondi::LetterCa => '𑵻',
            GunjalaGondi::LetterCha => '𑵼',
            GunjalaGondi::LetterTta => '𑵽',
            GunjalaGondi::LetterTtha => '𑵾',
            GunjalaGondi::LetterLla => '𑵿',
            GunjalaGondi::LetterJa => '𑶀',
            GunjalaGondi::LetterJha => '𑶁',
            GunjalaGondi::LetterDda => '𑶂',
            GunjalaGondi::LetterDdha => '𑶃',
            GunjalaGondi::LetterNga => '𑶄',
            GunjalaGondi::LetterPa => '𑶅',
            GunjalaGondi::LetterPha => '𑶆',
            GunjalaGondi::LetterHa => '𑶇',
            GunjalaGondi::LetterRa => '𑶈',
            GunjalaGondi::LetterSa => '𑶉',
            GunjalaGondi::VowelSignAa => '𑶊',
            GunjalaGondi::VowelSignI => '𑶋',
            GunjalaGondi::VowelSignIi => '𑶌',
            GunjalaGondi::VowelSignU => '𑶍',
            GunjalaGondi::VowelSignUu => '𑶎',
            GunjalaGondi::VowelSignEe => '𑶐',
            GunjalaGondi::VowelSignAi => '𑶑',
            GunjalaGondi::VowelSignOo => '𑶓',
            GunjalaGondi::VowelSignAu => '𑶔',
            GunjalaGondi::SignAnusvara => '𑶕',
            GunjalaGondi::SignVisarga => '𑶖',
            GunjalaGondi::Virama => '𑶗',
            GunjalaGondi::Om => '𑶘',
            GunjalaGondi::DigitZero => '𑶠',
            GunjalaGondi::DigitOne => '𑶡',
            GunjalaGondi::DigitTwo => '𑶢',
            GunjalaGondi::DigitThree => '𑶣',
            GunjalaGondi::DigitFour => '𑶤',
            GunjalaGondi::DigitFive => '𑶥',
            GunjalaGondi::DigitSix => '𑶦',
            GunjalaGondi::DigitSeven => '𑶧',
            GunjalaGondi::DigitEight => '𑶨',
            GunjalaGondi::DigitNine => '𑶩',
        }
    }
}

impl std::convert::TryFrom<char> for GunjalaGondi {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑵠' => Ok(GunjalaGondi::LetterA),
            '𑵡' => Ok(GunjalaGondi::LetterAa),
            '𑵢' => Ok(GunjalaGondi::LetterI),
            '𑵣' => Ok(GunjalaGondi::LetterIi),
            '𑵤' => Ok(GunjalaGondi::LetterU),
            '𑵥' => Ok(GunjalaGondi::LetterUu),
            '𑵧' => Ok(GunjalaGondi::LetterEe),
            '𑵨' => Ok(GunjalaGondi::LetterAi),
            '𑵪' => Ok(GunjalaGondi::LetterOo),
            '𑵫' => Ok(GunjalaGondi::LetterAu),
            '𑵬' => Ok(GunjalaGondi::LetterYa),
            '𑵭' => Ok(GunjalaGondi::LetterVa),
            '𑵮' => Ok(GunjalaGondi::LetterBa),
            '𑵯' => Ok(GunjalaGondi::LetterBha),
            '𑵰' => Ok(GunjalaGondi::LetterMa),
            '𑵱' => Ok(GunjalaGondi::LetterKa),
            '𑵲' => Ok(GunjalaGondi::LetterKha),
            '𑵳' => Ok(GunjalaGondi::LetterTa),
            '𑵴' => Ok(GunjalaGondi::LetterTha),
            '𑵵' => Ok(GunjalaGondi::LetterLa),
            '𑵶' => Ok(GunjalaGondi::LetterGa),
            '𑵷' => Ok(GunjalaGondi::LetterGha),
            '𑵸' => Ok(GunjalaGondi::LetterDa),
            '𑵹' => Ok(GunjalaGondi::LetterDha),
            '𑵺' => Ok(GunjalaGondi::LetterNa),
            '𑵻' => Ok(GunjalaGondi::LetterCa),
            '𑵼' => Ok(GunjalaGondi::LetterCha),
            '𑵽' => Ok(GunjalaGondi::LetterTta),
            '𑵾' => Ok(GunjalaGondi::LetterTtha),
            '𑵿' => Ok(GunjalaGondi::LetterLla),
            '𑶀' => Ok(GunjalaGondi::LetterJa),
            '𑶁' => Ok(GunjalaGondi::LetterJha),
            '𑶂' => Ok(GunjalaGondi::LetterDda),
            '𑶃' => Ok(GunjalaGondi::LetterDdha),
            '𑶄' => Ok(GunjalaGondi::LetterNga),
            '𑶅' => Ok(GunjalaGondi::LetterPa),
            '𑶆' => Ok(GunjalaGondi::LetterPha),
            '𑶇' => Ok(GunjalaGondi::LetterHa),
            '𑶈' => Ok(GunjalaGondi::LetterRa),
            '𑶉' => Ok(GunjalaGondi::LetterSa),
            '𑶊' => Ok(GunjalaGondi::VowelSignAa),
            '𑶋' => Ok(GunjalaGondi::VowelSignI),
            '𑶌' => Ok(GunjalaGondi::VowelSignIi),
            '𑶍' => Ok(GunjalaGondi::VowelSignU),
            '𑶎' => Ok(GunjalaGondi::VowelSignUu),
            '𑶐' => Ok(GunjalaGondi::VowelSignEe),
            '𑶑' => Ok(GunjalaGondi::VowelSignAi),
            '𑶓' => Ok(GunjalaGondi::VowelSignOo),
            '𑶔' => Ok(GunjalaGondi::VowelSignAu),
            '𑶕' => Ok(GunjalaGondi::SignAnusvara),
            '𑶖' => Ok(GunjalaGondi::SignVisarga),
            '𑶗' => Ok(GunjalaGondi::Virama),
            '𑶘' => Ok(GunjalaGondi::Om),
            '𑶠' => Ok(GunjalaGondi::DigitZero),
            '𑶡' => Ok(GunjalaGondi::DigitOne),
            '𑶢' => Ok(GunjalaGondi::DigitTwo),
            '𑶣' => Ok(GunjalaGondi::DigitThree),
            '𑶤' => Ok(GunjalaGondi::DigitFour),
            '𑶥' => Ok(GunjalaGondi::DigitFive),
            '𑶦' => Ok(GunjalaGondi::DigitSix),
            '𑶧' => Ok(GunjalaGondi::DigitSeven),
            '𑶨' => Ok(GunjalaGondi::DigitEight),
            '𑶩' => Ok(GunjalaGondi::DigitNine),
            _ => Err(()),
        }
    }
}

impl Into<u32> for GunjalaGondi {
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

impl std::convert::TryFrom<u32> for GunjalaGondi {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for GunjalaGondi {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl GunjalaGondi {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        GunjalaGondi::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("GunjalaGondi{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
