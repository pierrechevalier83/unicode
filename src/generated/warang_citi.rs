
/// An enum to represent all characters in the WarangCiti block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WarangCiti {
    /// \u{118a0}: '𑢠'
    CapitalLetterNgaa,
    /// \u{118a1}: '𑢡'
    CapitalLetterA,
    /// \u{118a2}: '𑢢'
    CapitalLetterWi,
    /// \u{118a3}: '𑢣'
    CapitalLetterYu,
    /// \u{118a4}: '𑢤'
    CapitalLetterYa,
    /// \u{118a5}: '𑢥'
    CapitalLetterYo,
    /// \u{118a6}: '𑢦'
    CapitalLetterIi,
    /// \u{118a7}: '𑢧'
    CapitalLetterUu,
    /// \u{118a8}: '𑢨'
    CapitalLetterE,
    /// \u{118a9}: '𑢩'
    CapitalLetterO,
    /// \u{118aa}: '𑢪'
    CapitalLetterAng,
    /// \u{118ab}: '𑢫'
    CapitalLetterGa,
    /// \u{118ac}: '𑢬'
    CapitalLetterKo,
    /// \u{118ad}: '𑢭'
    CapitalLetterEny,
    /// \u{118ae}: '𑢮'
    CapitalLetterYuj,
    /// \u{118af}: '𑢯'
    CapitalLetterUc,
    /// \u{118b0}: '𑢰'
    CapitalLetterEnn,
    /// \u{118b1}: '𑢱'
    CapitalLetterOdd,
    /// \u{118b2}: '𑢲'
    CapitalLetterTte,
    /// \u{118b3}: '𑢳'
    CapitalLetterNung,
    /// \u{118b4}: '𑢴'
    CapitalLetterDa,
    /// \u{118b5}: '𑢵'
    CapitalLetterAt,
    /// \u{118b6}: '𑢶'
    CapitalLetterAm,
    /// \u{118b7}: '𑢷'
    CapitalLetterBu,
    /// \u{118b8}: '𑢸'
    CapitalLetterPu,
    /// \u{118b9}: '𑢹'
    CapitalLetterHiyo,
    /// \u{118ba}: '𑢺'
    CapitalLetterHolo,
    /// \u{118bb}: '𑢻'
    CapitalLetterHorr,
    /// \u{118bc}: '𑢼'
    CapitalLetterHar,
    /// \u{118bd}: '𑢽'
    CapitalLetterSsuu,
    /// \u{118be}: '𑢾'
    CapitalLetterSii,
    /// \u{118bf}: '𑢿'
    CapitalLetterViyo,
    /// \u{118c0}: '𑣀'
    SmallLetterNgaa,
    /// \u{118c1}: '𑣁'
    SmallLetterA,
    /// \u{118c2}: '𑣂'
    SmallLetterWi,
    /// \u{118c3}: '𑣃'
    SmallLetterYu,
    /// \u{118c4}: '𑣄'
    SmallLetterYa,
    /// \u{118c5}: '𑣅'
    SmallLetterYo,
    /// \u{118c6}: '𑣆'
    SmallLetterIi,
    /// \u{118c7}: '𑣇'
    SmallLetterUu,
    /// \u{118c8}: '𑣈'
    SmallLetterE,
    /// \u{118c9}: '𑣉'
    SmallLetterO,
    /// \u{118ca}: '𑣊'
    SmallLetterAng,
    /// \u{118cb}: '𑣋'
    SmallLetterGa,
    /// \u{118cc}: '𑣌'
    SmallLetterKo,
    /// \u{118cd}: '𑣍'
    SmallLetterEny,
    /// \u{118ce}: '𑣎'
    SmallLetterYuj,
    /// \u{118cf}: '𑣏'
    SmallLetterUc,
    /// \u{118d0}: '𑣐'
    SmallLetterEnn,
    /// \u{118d1}: '𑣑'
    SmallLetterOdd,
    /// \u{118d2}: '𑣒'
    SmallLetterTte,
    /// \u{118d3}: '𑣓'
    SmallLetterNung,
    /// \u{118d4}: '𑣔'
    SmallLetterDa,
    /// \u{118d5}: '𑣕'
    SmallLetterAt,
    /// \u{118d6}: '𑣖'
    SmallLetterAm,
    /// \u{118d7}: '𑣗'
    SmallLetterBu,
    /// \u{118d8}: '𑣘'
    SmallLetterPu,
    /// \u{118d9}: '𑣙'
    SmallLetterHiyo,
    /// \u{118da}: '𑣚'
    SmallLetterHolo,
    /// \u{118db}: '𑣛'
    SmallLetterHorr,
    /// \u{118dc}: '𑣜'
    SmallLetterHar,
    /// \u{118dd}: '𑣝'
    SmallLetterSsuu,
    /// \u{118de}: '𑣞'
    SmallLetterSii,
    /// \u{118df}: '𑣟'
    SmallLetterViyo,
    /// \u{118e0}: '𑣠'
    DigitZero,
    /// \u{118e1}: '𑣡'
    DigitOne,
    /// \u{118e2}: '𑣢'
    DigitTwo,
    /// \u{118e3}: '𑣣'
    DigitThree,
    /// \u{118e4}: '𑣤'
    DigitFour,
    /// \u{118e5}: '𑣥'
    DigitFive,
    /// \u{118e6}: '𑣦'
    DigitSix,
    /// \u{118e7}: '𑣧'
    DigitSeven,
    /// \u{118e8}: '𑣨'
    DigitEight,
    /// \u{118e9}: '𑣩'
    DigitNine,
    /// \u{118ea}: '𑣪'
    NumberTen,
    /// \u{118eb}: '𑣫'
    NumberTwenty,
    /// \u{118ec}: '𑣬'
    NumberThirty,
    /// \u{118ed}: '𑣭'
    NumberForty,
    /// \u{118ee}: '𑣮'
    NumberFifty,
    /// \u{118ef}: '𑣯'
    NumberSixty,
    /// \u{118f0}: '𑣰'
    NumberSeventy,
    /// \u{118f1}: '𑣱'
    NumberEighty,
    /// \u{118f2}: '𑣲'
    NumberNinety,
}

impl Into<char> for WarangCiti {
    fn into(self) -> char {
        match self {
            WarangCiti::CapitalLetterNgaa => '𑢠',
            WarangCiti::CapitalLetterA => '𑢡',
            WarangCiti::CapitalLetterWi => '𑢢',
            WarangCiti::CapitalLetterYu => '𑢣',
            WarangCiti::CapitalLetterYa => '𑢤',
            WarangCiti::CapitalLetterYo => '𑢥',
            WarangCiti::CapitalLetterIi => '𑢦',
            WarangCiti::CapitalLetterUu => '𑢧',
            WarangCiti::CapitalLetterE => '𑢨',
            WarangCiti::CapitalLetterO => '𑢩',
            WarangCiti::CapitalLetterAng => '𑢪',
            WarangCiti::CapitalLetterGa => '𑢫',
            WarangCiti::CapitalLetterKo => '𑢬',
            WarangCiti::CapitalLetterEny => '𑢭',
            WarangCiti::CapitalLetterYuj => '𑢮',
            WarangCiti::CapitalLetterUc => '𑢯',
            WarangCiti::CapitalLetterEnn => '𑢰',
            WarangCiti::CapitalLetterOdd => '𑢱',
            WarangCiti::CapitalLetterTte => '𑢲',
            WarangCiti::CapitalLetterNung => '𑢳',
            WarangCiti::CapitalLetterDa => '𑢴',
            WarangCiti::CapitalLetterAt => '𑢵',
            WarangCiti::CapitalLetterAm => '𑢶',
            WarangCiti::CapitalLetterBu => '𑢷',
            WarangCiti::CapitalLetterPu => '𑢸',
            WarangCiti::CapitalLetterHiyo => '𑢹',
            WarangCiti::CapitalLetterHolo => '𑢺',
            WarangCiti::CapitalLetterHorr => '𑢻',
            WarangCiti::CapitalLetterHar => '𑢼',
            WarangCiti::CapitalLetterSsuu => '𑢽',
            WarangCiti::CapitalLetterSii => '𑢾',
            WarangCiti::CapitalLetterViyo => '𑢿',
            WarangCiti::SmallLetterNgaa => '𑣀',
            WarangCiti::SmallLetterA => '𑣁',
            WarangCiti::SmallLetterWi => '𑣂',
            WarangCiti::SmallLetterYu => '𑣃',
            WarangCiti::SmallLetterYa => '𑣄',
            WarangCiti::SmallLetterYo => '𑣅',
            WarangCiti::SmallLetterIi => '𑣆',
            WarangCiti::SmallLetterUu => '𑣇',
            WarangCiti::SmallLetterE => '𑣈',
            WarangCiti::SmallLetterO => '𑣉',
            WarangCiti::SmallLetterAng => '𑣊',
            WarangCiti::SmallLetterGa => '𑣋',
            WarangCiti::SmallLetterKo => '𑣌',
            WarangCiti::SmallLetterEny => '𑣍',
            WarangCiti::SmallLetterYuj => '𑣎',
            WarangCiti::SmallLetterUc => '𑣏',
            WarangCiti::SmallLetterEnn => '𑣐',
            WarangCiti::SmallLetterOdd => '𑣑',
            WarangCiti::SmallLetterTte => '𑣒',
            WarangCiti::SmallLetterNung => '𑣓',
            WarangCiti::SmallLetterDa => '𑣔',
            WarangCiti::SmallLetterAt => '𑣕',
            WarangCiti::SmallLetterAm => '𑣖',
            WarangCiti::SmallLetterBu => '𑣗',
            WarangCiti::SmallLetterPu => '𑣘',
            WarangCiti::SmallLetterHiyo => '𑣙',
            WarangCiti::SmallLetterHolo => '𑣚',
            WarangCiti::SmallLetterHorr => '𑣛',
            WarangCiti::SmallLetterHar => '𑣜',
            WarangCiti::SmallLetterSsuu => '𑣝',
            WarangCiti::SmallLetterSii => '𑣞',
            WarangCiti::SmallLetterViyo => '𑣟',
            WarangCiti::DigitZero => '𑣠',
            WarangCiti::DigitOne => '𑣡',
            WarangCiti::DigitTwo => '𑣢',
            WarangCiti::DigitThree => '𑣣',
            WarangCiti::DigitFour => '𑣤',
            WarangCiti::DigitFive => '𑣥',
            WarangCiti::DigitSix => '𑣦',
            WarangCiti::DigitSeven => '𑣧',
            WarangCiti::DigitEight => '𑣨',
            WarangCiti::DigitNine => '𑣩',
            WarangCiti::NumberTen => '𑣪',
            WarangCiti::NumberTwenty => '𑣫',
            WarangCiti::NumberThirty => '𑣬',
            WarangCiti::NumberForty => '𑣭',
            WarangCiti::NumberFifty => '𑣮',
            WarangCiti::NumberSixty => '𑣯',
            WarangCiti::NumberSeventy => '𑣰',
            WarangCiti::NumberEighty => '𑣱',
            WarangCiti::NumberNinety => '𑣲',
        }
    }
}

impl std::convert::TryFrom<char> for WarangCiti {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑢠' => Ok(WarangCiti::CapitalLetterNgaa),
            '𑢡' => Ok(WarangCiti::CapitalLetterA),
            '𑢢' => Ok(WarangCiti::CapitalLetterWi),
            '𑢣' => Ok(WarangCiti::CapitalLetterYu),
            '𑢤' => Ok(WarangCiti::CapitalLetterYa),
            '𑢥' => Ok(WarangCiti::CapitalLetterYo),
            '𑢦' => Ok(WarangCiti::CapitalLetterIi),
            '𑢧' => Ok(WarangCiti::CapitalLetterUu),
            '𑢨' => Ok(WarangCiti::CapitalLetterE),
            '𑢩' => Ok(WarangCiti::CapitalLetterO),
            '𑢪' => Ok(WarangCiti::CapitalLetterAng),
            '𑢫' => Ok(WarangCiti::CapitalLetterGa),
            '𑢬' => Ok(WarangCiti::CapitalLetterKo),
            '𑢭' => Ok(WarangCiti::CapitalLetterEny),
            '𑢮' => Ok(WarangCiti::CapitalLetterYuj),
            '𑢯' => Ok(WarangCiti::CapitalLetterUc),
            '𑢰' => Ok(WarangCiti::CapitalLetterEnn),
            '𑢱' => Ok(WarangCiti::CapitalLetterOdd),
            '𑢲' => Ok(WarangCiti::CapitalLetterTte),
            '𑢳' => Ok(WarangCiti::CapitalLetterNung),
            '𑢴' => Ok(WarangCiti::CapitalLetterDa),
            '𑢵' => Ok(WarangCiti::CapitalLetterAt),
            '𑢶' => Ok(WarangCiti::CapitalLetterAm),
            '𑢷' => Ok(WarangCiti::CapitalLetterBu),
            '𑢸' => Ok(WarangCiti::CapitalLetterPu),
            '𑢹' => Ok(WarangCiti::CapitalLetterHiyo),
            '𑢺' => Ok(WarangCiti::CapitalLetterHolo),
            '𑢻' => Ok(WarangCiti::CapitalLetterHorr),
            '𑢼' => Ok(WarangCiti::CapitalLetterHar),
            '𑢽' => Ok(WarangCiti::CapitalLetterSsuu),
            '𑢾' => Ok(WarangCiti::CapitalLetterSii),
            '𑢿' => Ok(WarangCiti::CapitalLetterViyo),
            '𑣀' => Ok(WarangCiti::SmallLetterNgaa),
            '𑣁' => Ok(WarangCiti::SmallLetterA),
            '𑣂' => Ok(WarangCiti::SmallLetterWi),
            '𑣃' => Ok(WarangCiti::SmallLetterYu),
            '𑣄' => Ok(WarangCiti::SmallLetterYa),
            '𑣅' => Ok(WarangCiti::SmallLetterYo),
            '𑣆' => Ok(WarangCiti::SmallLetterIi),
            '𑣇' => Ok(WarangCiti::SmallLetterUu),
            '𑣈' => Ok(WarangCiti::SmallLetterE),
            '𑣉' => Ok(WarangCiti::SmallLetterO),
            '𑣊' => Ok(WarangCiti::SmallLetterAng),
            '𑣋' => Ok(WarangCiti::SmallLetterGa),
            '𑣌' => Ok(WarangCiti::SmallLetterKo),
            '𑣍' => Ok(WarangCiti::SmallLetterEny),
            '𑣎' => Ok(WarangCiti::SmallLetterYuj),
            '𑣏' => Ok(WarangCiti::SmallLetterUc),
            '𑣐' => Ok(WarangCiti::SmallLetterEnn),
            '𑣑' => Ok(WarangCiti::SmallLetterOdd),
            '𑣒' => Ok(WarangCiti::SmallLetterTte),
            '𑣓' => Ok(WarangCiti::SmallLetterNung),
            '𑣔' => Ok(WarangCiti::SmallLetterDa),
            '𑣕' => Ok(WarangCiti::SmallLetterAt),
            '𑣖' => Ok(WarangCiti::SmallLetterAm),
            '𑣗' => Ok(WarangCiti::SmallLetterBu),
            '𑣘' => Ok(WarangCiti::SmallLetterPu),
            '𑣙' => Ok(WarangCiti::SmallLetterHiyo),
            '𑣚' => Ok(WarangCiti::SmallLetterHolo),
            '𑣛' => Ok(WarangCiti::SmallLetterHorr),
            '𑣜' => Ok(WarangCiti::SmallLetterHar),
            '𑣝' => Ok(WarangCiti::SmallLetterSsuu),
            '𑣞' => Ok(WarangCiti::SmallLetterSii),
            '𑣟' => Ok(WarangCiti::SmallLetterViyo),
            '𑣠' => Ok(WarangCiti::DigitZero),
            '𑣡' => Ok(WarangCiti::DigitOne),
            '𑣢' => Ok(WarangCiti::DigitTwo),
            '𑣣' => Ok(WarangCiti::DigitThree),
            '𑣤' => Ok(WarangCiti::DigitFour),
            '𑣥' => Ok(WarangCiti::DigitFive),
            '𑣦' => Ok(WarangCiti::DigitSix),
            '𑣧' => Ok(WarangCiti::DigitSeven),
            '𑣨' => Ok(WarangCiti::DigitEight),
            '𑣩' => Ok(WarangCiti::DigitNine),
            '𑣪' => Ok(WarangCiti::NumberTen),
            '𑣫' => Ok(WarangCiti::NumberTwenty),
            '𑣬' => Ok(WarangCiti::NumberThirty),
            '𑣭' => Ok(WarangCiti::NumberForty),
            '𑣮' => Ok(WarangCiti::NumberFifty),
            '𑣯' => Ok(WarangCiti::NumberSixty),
            '𑣰' => Ok(WarangCiti::NumberSeventy),
            '𑣱' => Ok(WarangCiti::NumberEighty),
            '𑣲' => Ok(WarangCiti::NumberNinety),
            _ => Err(()),
        }
    }
}

impl Into<u32> for WarangCiti {
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

impl std::convert::TryFrom<u32> for WarangCiti {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for WarangCiti {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl WarangCiti {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        WarangCiti::CapitalLetterNgaa
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("WarangCiti{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
