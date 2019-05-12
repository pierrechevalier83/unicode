
/// An enum to represent all characters in the Sogdian block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Sogdian {
    /// \u{10f30}: '𐼰'
    LetterAleph,
    /// \u{10f31}: '𐼱'
    LetterBeth,
    /// \u{10f32}: '𐼲'
    LetterGimel,
    /// \u{10f33}: '𐼳'
    LetterHe,
    /// \u{10f34}: '𐼴'
    LetterWaw,
    /// \u{10f35}: '𐼵'
    LetterZayin,
    /// \u{10f36}: '𐼶'
    LetterHeth,
    /// \u{10f37}: '𐼷'
    LetterYodh,
    /// \u{10f38}: '𐼸'
    LetterKaph,
    /// \u{10f39}: '𐼹'
    LetterLamedh,
    /// \u{10f3a}: '𐼺'
    LetterMem,
    /// \u{10f3b}: '𐼻'
    LetterNun,
    /// \u{10f3c}: '𐼼'
    LetterSamekh,
    /// \u{10f3d}: '𐼽'
    LetterAyin,
    /// \u{10f3e}: '𐼾'
    LetterPe,
    /// \u{10f3f}: '𐼿'
    LetterSadhe,
    /// \u{10f40}: '𐽀'
    LetterReshDashAyin,
    /// \u{10f41}: '𐽁'
    LetterShin,
    /// \u{10f42}: '𐽂'
    LetterTaw,
    /// \u{10f43}: '𐽃'
    LetterFeth,
    /// \u{10f44}: '𐽄'
    LetterLesh,
    /// \u{10f45}: '𐽅'
    IndependentShin,
    /// \u{10f46}: '𐽆'
    CombiningDotBelow,
    /// \u{10f47}: '𐽇'
    CombiningTwoDotsBelow,
    /// \u{10f48}: '𐽈'
    CombiningDotAbove,
    /// \u{10f49}: '𐽉'
    CombiningTwoDotsAbove,
    /// \u{10f4a}: '𐽊'
    CombiningCurveAbove,
    /// \u{10f4b}: '𐽋'
    CombiningCurveBelow,
    /// \u{10f4c}: '𐽌'
    CombiningHookAbove,
    /// \u{10f4d}: '𐽍'
    CombiningHookBelow,
    /// \u{10f4e}: '𐽎'
    CombiningLongHookBelow,
    /// \u{10f4f}: '𐽏'
    CombiningReshBelow,
    /// \u{10f50}: '𐽐'
    CombiningStrokeBelow,
    /// \u{10f51}: '𐽑'
    NumberOne,
    /// \u{10f52}: '𐽒'
    NumberTen,
    /// \u{10f53}: '𐽓'
    NumberTwenty,
    /// \u{10f54}: '𐽔'
    NumberOneHundred,
    /// \u{10f55}: '𐽕'
    PunctuationTwoVerticalBars,
    /// \u{10f56}: '𐽖'
    PunctuationTwoVerticalBarsWithDots,
    /// \u{10f57}: '𐽗'
    PunctuationCircleWithDot,
    /// \u{10f58}: '𐽘'
    PunctuationTwoCirclesWithDots,
    /// \u{10f59}: '𐽙'
    PunctuationHalfCircleWithDot,
}

impl Into<char> for Sogdian {
    fn into(self) -> char {
        match self {
            Sogdian::LetterAleph => '𐼰',
            Sogdian::LetterBeth => '𐼱',
            Sogdian::LetterGimel => '𐼲',
            Sogdian::LetterHe => '𐼳',
            Sogdian::LetterWaw => '𐼴',
            Sogdian::LetterZayin => '𐼵',
            Sogdian::LetterHeth => '𐼶',
            Sogdian::LetterYodh => '𐼷',
            Sogdian::LetterKaph => '𐼸',
            Sogdian::LetterLamedh => '𐼹',
            Sogdian::LetterMem => '𐼺',
            Sogdian::LetterNun => '𐼻',
            Sogdian::LetterSamekh => '𐼼',
            Sogdian::LetterAyin => '𐼽',
            Sogdian::LetterPe => '𐼾',
            Sogdian::LetterSadhe => '𐼿',
            Sogdian::LetterReshDashAyin => '𐽀',
            Sogdian::LetterShin => '𐽁',
            Sogdian::LetterTaw => '𐽂',
            Sogdian::LetterFeth => '𐽃',
            Sogdian::LetterLesh => '𐽄',
            Sogdian::IndependentShin => '𐽅',
            Sogdian::CombiningDotBelow => '𐽆',
            Sogdian::CombiningTwoDotsBelow => '𐽇',
            Sogdian::CombiningDotAbove => '𐽈',
            Sogdian::CombiningTwoDotsAbove => '𐽉',
            Sogdian::CombiningCurveAbove => '𐽊',
            Sogdian::CombiningCurveBelow => '𐽋',
            Sogdian::CombiningHookAbove => '𐽌',
            Sogdian::CombiningHookBelow => '𐽍',
            Sogdian::CombiningLongHookBelow => '𐽎',
            Sogdian::CombiningReshBelow => '𐽏',
            Sogdian::CombiningStrokeBelow => '𐽐',
            Sogdian::NumberOne => '𐽑',
            Sogdian::NumberTen => '𐽒',
            Sogdian::NumberTwenty => '𐽓',
            Sogdian::NumberOneHundred => '𐽔',
            Sogdian::PunctuationTwoVerticalBars => '𐽕',
            Sogdian::PunctuationTwoVerticalBarsWithDots => '𐽖',
            Sogdian::PunctuationCircleWithDot => '𐽗',
            Sogdian::PunctuationTwoCirclesWithDots => '𐽘',
            Sogdian::PunctuationHalfCircleWithDot => '𐽙',
        }
    }
}

impl std::convert::TryFrom<char> for Sogdian {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𐼰' => Ok(Sogdian::LetterAleph),
            '𐼱' => Ok(Sogdian::LetterBeth),
            '𐼲' => Ok(Sogdian::LetterGimel),
            '𐼳' => Ok(Sogdian::LetterHe),
            '𐼴' => Ok(Sogdian::LetterWaw),
            '𐼵' => Ok(Sogdian::LetterZayin),
            '𐼶' => Ok(Sogdian::LetterHeth),
            '𐼷' => Ok(Sogdian::LetterYodh),
            '𐼸' => Ok(Sogdian::LetterKaph),
            '𐼹' => Ok(Sogdian::LetterLamedh),
            '𐼺' => Ok(Sogdian::LetterMem),
            '𐼻' => Ok(Sogdian::LetterNun),
            '𐼼' => Ok(Sogdian::LetterSamekh),
            '𐼽' => Ok(Sogdian::LetterAyin),
            '𐼾' => Ok(Sogdian::LetterPe),
            '𐼿' => Ok(Sogdian::LetterSadhe),
            '𐽀' => Ok(Sogdian::LetterReshDashAyin),
            '𐽁' => Ok(Sogdian::LetterShin),
            '𐽂' => Ok(Sogdian::LetterTaw),
            '𐽃' => Ok(Sogdian::LetterFeth),
            '𐽄' => Ok(Sogdian::LetterLesh),
            '𐽅' => Ok(Sogdian::IndependentShin),
            '𐽆' => Ok(Sogdian::CombiningDotBelow),
            '𐽇' => Ok(Sogdian::CombiningTwoDotsBelow),
            '𐽈' => Ok(Sogdian::CombiningDotAbove),
            '𐽉' => Ok(Sogdian::CombiningTwoDotsAbove),
            '𐽊' => Ok(Sogdian::CombiningCurveAbove),
            '𐽋' => Ok(Sogdian::CombiningCurveBelow),
            '𐽌' => Ok(Sogdian::CombiningHookAbove),
            '𐽍' => Ok(Sogdian::CombiningHookBelow),
            '𐽎' => Ok(Sogdian::CombiningLongHookBelow),
            '𐽏' => Ok(Sogdian::CombiningReshBelow),
            '𐽐' => Ok(Sogdian::CombiningStrokeBelow),
            '𐽑' => Ok(Sogdian::NumberOne),
            '𐽒' => Ok(Sogdian::NumberTen),
            '𐽓' => Ok(Sogdian::NumberTwenty),
            '𐽔' => Ok(Sogdian::NumberOneHundred),
            '𐽕' => Ok(Sogdian::PunctuationTwoVerticalBars),
            '𐽖' => Ok(Sogdian::PunctuationTwoVerticalBarsWithDots),
            '𐽗' => Ok(Sogdian::PunctuationCircleWithDot),
            '𐽘' => Ok(Sogdian::PunctuationTwoCirclesWithDots),
            '𐽙' => Ok(Sogdian::PunctuationHalfCircleWithDot),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Sogdian {
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

impl std::convert::TryFrom<u32> for Sogdian {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Sogdian {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Sogdian {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Sogdian::LetterAleph
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Sogdian{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
