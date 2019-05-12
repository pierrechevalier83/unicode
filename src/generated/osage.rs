
/// An enum to represent all characters in the Osage block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Osage {
    /// \u{104b0}: '𐒰'
    CapitalLetterA,
    /// \u{104b1}: '𐒱'
    CapitalLetterAi,
    /// \u{104b2}: '𐒲'
    CapitalLetterAin,
    /// \u{104b3}: '𐒳'
    CapitalLetterAh,
    /// \u{104b4}: '𐒴'
    CapitalLetterBra,
    /// \u{104b5}: '𐒵'
    CapitalLetterCha,
    /// \u{104b6}: '𐒶'
    CapitalLetterEhcha,
    /// \u{104b7}: '𐒷'
    CapitalLetterE,
    /// \u{104b8}: '𐒸'
    CapitalLetterEin,
    /// \u{104b9}: '𐒹'
    CapitalLetterHa,
    /// \u{104ba}: '𐒺'
    CapitalLetterHya,
    /// \u{104bb}: '𐒻'
    CapitalLetterI,
    /// \u{104bc}: '𐒼'
    CapitalLetterKa,
    /// \u{104bd}: '𐒽'
    CapitalLetterEhka,
    /// \u{104be}: '𐒾'
    CapitalLetterKya,
    /// \u{104bf}: '𐒿'
    CapitalLetterLa,
    /// \u{104c0}: '𐓀'
    CapitalLetterMa,
    /// \u{104c1}: '𐓁'
    CapitalLetterNa,
    /// \u{104c2}: '𐓂'
    CapitalLetterO,
    /// \u{104c3}: '𐓃'
    CapitalLetterOin,
    /// \u{104c4}: '𐓄'
    CapitalLetterPa,
    /// \u{104c5}: '𐓅'
    CapitalLetterEhpa,
    /// \u{104c6}: '𐓆'
    CapitalLetterSa,
    /// \u{104c7}: '𐓇'
    CapitalLetterSha,
    /// \u{104c8}: '𐓈'
    CapitalLetterTa,
    /// \u{104c9}: '𐓉'
    CapitalLetterEhta,
    /// \u{104ca}: '𐓊'
    CapitalLetterTsa,
    /// \u{104cb}: '𐓋'
    CapitalLetterEhtsa,
    /// \u{104cc}: '𐓌'
    CapitalLetterTsha,
    /// \u{104cd}: '𐓍'
    CapitalLetterDha,
    /// \u{104ce}: '𐓎'
    CapitalLetterU,
    /// \u{104cf}: '𐓏'
    CapitalLetterWa,
    /// \u{104d0}: '𐓐'
    CapitalLetterKha,
    /// \u{104d1}: '𐓑'
    CapitalLetterGha,
    /// \u{104d2}: '𐓒'
    CapitalLetterZa,
    /// \u{104d3}: '𐓓'
    CapitalLetterZha,
    /// \u{104d8}: '𐓘'
    SmallLetterA,
    /// \u{104d9}: '𐓙'
    SmallLetterAi,
    /// \u{104da}: '𐓚'
    SmallLetterAin,
    /// \u{104db}: '𐓛'
    SmallLetterAh,
    /// \u{104dc}: '𐓜'
    SmallLetterBra,
    /// \u{104dd}: '𐓝'
    SmallLetterCha,
    /// \u{104de}: '𐓞'
    SmallLetterEhcha,
    /// \u{104df}: '𐓟'
    SmallLetterE,
    /// \u{104e0}: '𐓠'
    SmallLetterEin,
    /// \u{104e1}: '𐓡'
    SmallLetterHa,
    /// \u{104e2}: '𐓢'
    SmallLetterHya,
    /// \u{104e3}: '𐓣'
    SmallLetterI,
    /// \u{104e4}: '𐓤'
    SmallLetterKa,
    /// \u{104e5}: '𐓥'
    SmallLetterEhka,
    /// \u{104e6}: '𐓦'
    SmallLetterKya,
    /// \u{104e7}: '𐓧'
    SmallLetterLa,
    /// \u{104e8}: '𐓨'
    SmallLetterMa,
    /// \u{104e9}: '𐓩'
    SmallLetterNa,
    /// \u{104ea}: '𐓪'
    SmallLetterO,
    /// \u{104eb}: '𐓫'
    SmallLetterOin,
    /// \u{104ec}: '𐓬'
    SmallLetterPa,
    /// \u{104ed}: '𐓭'
    SmallLetterEhpa,
    /// \u{104ee}: '𐓮'
    SmallLetterSa,
    /// \u{104ef}: '𐓯'
    SmallLetterSha,
    /// \u{104f0}: '𐓰'
    SmallLetterTa,
    /// \u{104f1}: '𐓱'
    SmallLetterEhta,
    /// \u{104f2}: '𐓲'
    SmallLetterTsa,
    /// \u{104f3}: '𐓳'
    SmallLetterEhtsa,
    /// \u{104f4}: '𐓴'
    SmallLetterTsha,
    /// \u{104f5}: '𐓵'
    SmallLetterDha,
    /// \u{104f6}: '𐓶'
    SmallLetterU,
    /// \u{104f7}: '𐓷'
    SmallLetterWa,
    /// \u{104f8}: '𐓸'
    SmallLetterKha,
    /// \u{104f9}: '𐓹'
    SmallLetterGha,
    /// \u{104fa}: '𐓺'
    SmallLetterZa,
    /// \u{104fb}: '𐓻'
    SmallLetterZha,
}

impl Into<char> for Osage {
    fn into(self) -> char {
        match self {
            Osage::CapitalLetterA => '𐒰',
            Osage::CapitalLetterAi => '𐒱',
            Osage::CapitalLetterAin => '𐒲',
            Osage::CapitalLetterAh => '𐒳',
            Osage::CapitalLetterBra => '𐒴',
            Osage::CapitalLetterCha => '𐒵',
            Osage::CapitalLetterEhcha => '𐒶',
            Osage::CapitalLetterE => '𐒷',
            Osage::CapitalLetterEin => '𐒸',
            Osage::CapitalLetterHa => '𐒹',
            Osage::CapitalLetterHya => '𐒺',
            Osage::CapitalLetterI => '𐒻',
            Osage::CapitalLetterKa => '𐒼',
            Osage::CapitalLetterEhka => '𐒽',
            Osage::CapitalLetterKya => '𐒾',
            Osage::CapitalLetterLa => '𐒿',
            Osage::CapitalLetterMa => '𐓀',
            Osage::CapitalLetterNa => '𐓁',
            Osage::CapitalLetterO => '𐓂',
            Osage::CapitalLetterOin => '𐓃',
            Osage::CapitalLetterPa => '𐓄',
            Osage::CapitalLetterEhpa => '𐓅',
            Osage::CapitalLetterSa => '𐓆',
            Osage::CapitalLetterSha => '𐓇',
            Osage::CapitalLetterTa => '𐓈',
            Osage::CapitalLetterEhta => '𐓉',
            Osage::CapitalLetterTsa => '𐓊',
            Osage::CapitalLetterEhtsa => '𐓋',
            Osage::CapitalLetterTsha => '𐓌',
            Osage::CapitalLetterDha => '𐓍',
            Osage::CapitalLetterU => '𐓎',
            Osage::CapitalLetterWa => '𐓏',
            Osage::CapitalLetterKha => '𐓐',
            Osage::CapitalLetterGha => '𐓑',
            Osage::CapitalLetterZa => '𐓒',
            Osage::CapitalLetterZha => '𐓓',
            Osage::SmallLetterA => '𐓘',
            Osage::SmallLetterAi => '𐓙',
            Osage::SmallLetterAin => '𐓚',
            Osage::SmallLetterAh => '𐓛',
            Osage::SmallLetterBra => '𐓜',
            Osage::SmallLetterCha => '𐓝',
            Osage::SmallLetterEhcha => '𐓞',
            Osage::SmallLetterE => '𐓟',
            Osage::SmallLetterEin => '𐓠',
            Osage::SmallLetterHa => '𐓡',
            Osage::SmallLetterHya => '𐓢',
            Osage::SmallLetterI => '𐓣',
            Osage::SmallLetterKa => '𐓤',
            Osage::SmallLetterEhka => '𐓥',
            Osage::SmallLetterKya => '𐓦',
            Osage::SmallLetterLa => '𐓧',
            Osage::SmallLetterMa => '𐓨',
            Osage::SmallLetterNa => '𐓩',
            Osage::SmallLetterO => '𐓪',
            Osage::SmallLetterOin => '𐓫',
            Osage::SmallLetterPa => '𐓬',
            Osage::SmallLetterEhpa => '𐓭',
            Osage::SmallLetterSa => '𐓮',
            Osage::SmallLetterSha => '𐓯',
            Osage::SmallLetterTa => '𐓰',
            Osage::SmallLetterEhta => '𐓱',
            Osage::SmallLetterTsa => '𐓲',
            Osage::SmallLetterEhtsa => '𐓳',
            Osage::SmallLetterTsha => '𐓴',
            Osage::SmallLetterDha => '𐓵',
            Osage::SmallLetterU => '𐓶',
            Osage::SmallLetterWa => '𐓷',
            Osage::SmallLetterKha => '𐓸',
            Osage::SmallLetterGha => '𐓹',
            Osage::SmallLetterZa => '𐓺',
            Osage::SmallLetterZha => '𐓻',
        }
    }
}

impl std::convert::TryFrom<char> for Osage {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐒰' => Ok(Osage::CapitalLetterA),
            '𐒱' => Ok(Osage::CapitalLetterAi),
            '𐒲' => Ok(Osage::CapitalLetterAin),
            '𐒳' => Ok(Osage::CapitalLetterAh),
            '𐒴' => Ok(Osage::CapitalLetterBra),
            '𐒵' => Ok(Osage::CapitalLetterCha),
            '𐒶' => Ok(Osage::CapitalLetterEhcha),
            '𐒷' => Ok(Osage::CapitalLetterE),
            '𐒸' => Ok(Osage::CapitalLetterEin),
            '𐒹' => Ok(Osage::CapitalLetterHa),
            '𐒺' => Ok(Osage::CapitalLetterHya),
            '𐒻' => Ok(Osage::CapitalLetterI),
            '𐒼' => Ok(Osage::CapitalLetterKa),
            '𐒽' => Ok(Osage::CapitalLetterEhka),
            '𐒾' => Ok(Osage::CapitalLetterKya),
            '𐒿' => Ok(Osage::CapitalLetterLa),
            '𐓀' => Ok(Osage::CapitalLetterMa),
            '𐓁' => Ok(Osage::CapitalLetterNa),
            '𐓂' => Ok(Osage::CapitalLetterO),
            '𐓃' => Ok(Osage::CapitalLetterOin),
            '𐓄' => Ok(Osage::CapitalLetterPa),
            '𐓅' => Ok(Osage::CapitalLetterEhpa),
            '𐓆' => Ok(Osage::CapitalLetterSa),
            '𐓇' => Ok(Osage::CapitalLetterSha),
            '𐓈' => Ok(Osage::CapitalLetterTa),
            '𐓉' => Ok(Osage::CapitalLetterEhta),
            '𐓊' => Ok(Osage::CapitalLetterTsa),
            '𐓋' => Ok(Osage::CapitalLetterEhtsa),
            '𐓌' => Ok(Osage::CapitalLetterTsha),
            '𐓍' => Ok(Osage::CapitalLetterDha),
            '𐓎' => Ok(Osage::CapitalLetterU),
            '𐓏' => Ok(Osage::CapitalLetterWa),
            '𐓐' => Ok(Osage::CapitalLetterKha),
            '𐓑' => Ok(Osage::CapitalLetterGha),
            '𐓒' => Ok(Osage::CapitalLetterZa),
            '𐓓' => Ok(Osage::CapitalLetterZha),
            '𐓘' => Ok(Osage::SmallLetterA),
            '𐓙' => Ok(Osage::SmallLetterAi),
            '𐓚' => Ok(Osage::SmallLetterAin),
            '𐓛' => Ok(Osage::SmallLetterAh),
            '𐓜' => Ok(Osage::SmallLetterBra),
            '𐓝' => Ok(Osage::SmallLetterCha),
            '𐓞' => Ok(Osage::SmallLetterEhcha),
            '𐓟' => Ok(Osage::SmallLetterE),
            '𐓠' => Ok(Osage::SmallLetterEin),
            '𐓡' => Ok(Osage::SmallLetterHa),
            '𐓢' => Ok(Osage::SmallLetterHya),
            '𐓣' => Ok(Osage::SmallLetterI),
            '𐓤' => Ok(Osage::SmallLetterKa),
            '𐓥' => Ok(Osage::SmallLetterEhka),
            '𐓦' => Ok(Osage::SmallLetterKya),
            '𐓧' => Ok(Osage::SmallLetterLa),
            '𐓨' => Ok(Osage::SmallLetterMa),
            '𐓩' => Ok(Osage::SmallLetterNa),
            '𐓪' => Ok(Osage::SmallLetterO),
            '𐓫' => Ok(Osage::SmallLetterOin),
            '𐓬' => Ok(Osage::SmallLetterPa),
            '𐓭' => Ok(Osage::SmallLetterEhpa),
            '𐓮' => Ok(Osage::SmallLetterSa),
            '𐓯' => Ok(Osage::SmallLetterSha),
            '𐓰' => Ok(Osage::SmallLetterTa),
            '𐓱' => Ok(Osage::SmallLetterEhta),
            '𐓲' => Ok(Osage::SmallLetterTsa),
            '𐓳' => Ok(Osage::SmallLetterEhtsa),
            '𐓴' => Ok(Osage::SmallLetterTsha),
            '𐓵' => Ok(Osage::SmallLetterDha),
            '𐓶' => Ok(Osage::SmallLetterU),
            '𐓷' => Ok(Osage::SmallLetterWa),
            '𐓸' => Ok(Osage::SmallLetterKha),
            '𐓹' => Ok(Osage::SmallLetterGha),
            '𐓺' => Ok(Osage::SmallLetterZa),
            '𐓻' => Ok(Osage::SmallLetterZha),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Osage {
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

impl std::convert::TryFrom<u32> for Osage {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Osage {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Osage {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Osage::CapitalLetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Osage{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
