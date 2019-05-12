
/// An enum to represent all characters in the OrnamentalDingbats block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OrnamentalDingbats {
    /// \u{1f650}: '🙐'
    NorthWestPointingLeaf,
    /// \u{1f651}: '🙑'
    SouthWestPointingLeaf,
    /// \u{1f652}: '🙒'
    NorthEastPointingLeaf,
    /// \u{1f653}: '🙓'
    SouthEastPointingLeaf,
    /// \u{1f654}: '🙔'
    TurnedNorthWestPointingLeaf,
    /// \u{1f655}: '🙕'
    TurnedSouthWestPointingLeaf,
    /// \u{1f656}: '🙖'
    TurnedNorthEastPointingLeaf,
    /// \u{1f657}: '🙗'
    TurnedSouthEastPointingLeaf,
    /// \u{1f658}: '🙘'
    NorthWestPointingVineLeaf,
    /// \u{1f659}: '🙙'
    SouthWestPointingVineLeaf,
    /// \u{1f65a}: '🙚'
    NorthEastPointingVineLeaf,
    /// \u{1f65b}: '🙛'
    SouthEastPointingVineLeaf,
    /// \u{1f65c}: '🙜'
    HeavyNorthWestPointingVineLeaf,
    /// \u{1f65d}: '🙝'
    HeavySouthWestPointingVineLeaf,
    /// \u{1f65e}: '🙞'
    HeavyNorthEastPointingVineLeaf,
    /// \u{1f65f}: '🙟'
    HeavySouthEastPointingVineLeaf,
    /// \u{1f660}: '🙠'
    NorthWestPointingBud,
    /// \u{1f661}: '🙡'
    SouthWestPointingBud,
    /// \u{1f662}: '🙢'
    NorthEastPointingBud,
    /// \u{1f663}: '🙣'
    SouthEastPointingBud,
    /// \u{1f664}: '🙤'
    HeavyNorthWestPointingBud,
    /// \u{1f665}: '🙥'
    HeavySouthWestPointingBud,
    /// \u{1f666}: '🙦'
    HeavyNorthEastPointingBud,
    /// \u{1f667}: '🙧'
    HeavySouthEastPointingBud,
    /// \u{1f668}: '🙨'
    HollowQuiltSquareOrnament,
    /// \u{1f669}: '🙩'
    HollowQuiltSquareOrnamentInBlackSquare,
    /// \u{1f66a}: '🙪'
    SolidQuiltSquareOrnament,
    /// \u{1f66b}: '🙫'
    SolidQuiltSquareOrnamentInBlackSquare,
    /// \u{1f66c}: '🙬'
    LeftwardsRocket,
    /// \u{1f66d}: '🙭'
    UpwardsRocket,
    /// \u{1f66e}: '🙮'
    RightwardsRocket,
    /// \u{1f66f}: '🙯'
    DownwardsRocket,
    /// \u{1f670}: '🙰'
    ScriptLigatureEtOrnament,
    /// \u{1f671}: '🙱'
    HeavyScriptLigatureEtOrnament,
    /// \u{1f672}: '🙲'
    LigatureOpenEtOrnament,
    /// \u{1f673}: '🙳'
    HeavyLigatureOpenEtOrnament,
    /// \u{1f674}: '🙴'
    HeavyAmpersandOrnament,
    /// \u{1f675}: '🙵'
    SwashAmpersandOrnament,
    /// \u{1f676}: '🙶'
    SansDashSerifHeavyDoubleTurnedCommaQuotationMarkOrnament,
    /// \u{1f677}: '🙷'
    SansDashSerifHeavyDoubleCommaQuotationMarkOrnament,
    /// \u{1f678}: '🙸'
    SansDashSerifHeavyLowDoubleCommaQuotationMarkOrnament,
    /// \u{1f679}: '🙹'
    HeavyInterrobangOrnament,
    /// \u{1f67a}: '🙺'
    SansDashSerifInterrobangOrnament,
    /// \u{1f67b}: '🙻'
    HeavySansDashSerifInterrobangOrnament,
    /// \u{1f67c}: '🙼'
    VeryHeavySolidus,
    /// \u{1f67d}: '🙽'
    VeryHeavyReverseSolidus,
    /// \u{1f67e}: '🙾'
    CheckerBoard,
}

impl Into<char> for OrnamentalDingbats {
    fn into(self) -> char {
        match self {
            OrnamentalDingbats::NorthWestPointingLeaf => '🙐',
            OrnamentalDingbats::SouthWestPointingLeaf => '🙑',
            OrnamentalDingbats::NorthEastPointingLeaf => '🙒',
            OrnamentalDingbats::SouthEastPointingLeaf => '🙓',
            OrnamentalDingbats::TurnedNorthWestPointingLeaf => '🙔',
            OrnamentalDingbats::TurnedSouthWestPointingLeaf => '🙕',
            OrnamentalDingbats::TurnedNorthEastPointingLeaf => '🙖',
            OrnamentalDingbats::TurnedSouthEastPointingLeaf => '🙗',
            OrnamentalDingbats::NorthWestPointingVineLeaf => '🙘',
            OrnamentalDingbats::SouthWestPointingVineLeaf => '🙙',
            OrnamentalDingbats::NorthEastPointingVineLeaf => '🙚',
            OrnamentalDingbats::SouthEastPointingVineLeaf => '🙛',
            OrnamentalDingbats::HeavyNorthWestPointingVineLeaf => '🙜',
            OrnamentalDingbats::HeavySouthWestPointingVineLeaf => '🙝',
            OrnamentalDingbats::HeavyNorthEastPointingVineLeaf => '🙞',
            OrnamentalDingbats::HeavySouthEastPointingVineLeaf => '🙟',
            OrnamentalDingbats::NorthWestPointingBud => '🙠',
            OrnamentalDingbats::SouthWestPointingBud => '🙡',
            OrnamentalDingbats::NorthEastPointingBud => '🙢',
            OrnamentalDingbats::SouthEastPointingBud => '🙣',
            OrnamentalDingbats::HeavyNorthWestPointingBud => '🙤',
            OrnamentalDingbats::HeavySouthWestPointingBud => '🙥',
            OrnamentalDingbats::HeavyNorthEastPointingBud => '🙦',
            OrnamentalDingbats::HeavySouthEastPointingBud => '🙧',
            OrnamentalDingbats::HollowQuiltSquareOrnament => '🙨',
            OrnamentalDingbats::HollowQuiltSquareOrnamentInBlackSquare => '🙩',
            OrnamentalDingbats::SolidQuiltSquareOrnament => '🙪',
            OrnamentalDingbats::SolidQuiltSquareOrnamentInBlackSquare => '🙫',
            OrnamentalDingbats::LeftwardsRocket => '🙬',
            OrnamentalDingbats::UpwardsRocket => '🙭',
            OrnamentalDingbats::RightwardsRocket => '🙮',
            OrnamentalDingbats::DownwardsRocket => '🙯',
            OrnamentalDingbats::ScriptLigatureEtOrnament => '🙰',
            OrnamentalDingbats::HeavyScriptLigatureEtOrnament => '🙱',
            OrnamentalDingbats::LigatureOpenEtOrnament => '🙲',
            OrnamentalDingbats::HeavyLigatureOpenEtOrnament => '🙳',
            OrnamentalDingbats::HeavyAmpersandOrnament => '🙴',
            OrnamentalDingbats::SwashAmpersandOrnament => '🙵',
            OrnamentalDingbats::SansDashSerifHeavyDoubleTurnedCommaQuotationMarkOrnament => '🙶',
            OrnamentalDingbats::SansDashSerifHeavyDoubleCommaQuotationMarkOrnament => '🙷',
            OrnamentalDingbats::SansDashSerifHeavyLowDoubleCommaQuotationMarkOrnament => '🙸',
            OrnamentalDingbats::HeavyInterrobangOrnament => '🙹',
            OrnamentalDingbats::SansDashSerifInterrobangOrnament => '🙺',
            OrnamentalDingbats::HeavySansDashSerifInterrobangOrnament => '🙻',
            OrnamentalDingbats::VeryHeavySolidus => '🙼',
            OrnamentalDingbats::VeryHeavyReverseSolidus => '🙽',
            OrnamentalDingbats::CheckerBoard => '🙾',
        }
    }
}

impl std::convert::TryFrom<char> for OrnamentalDingbats {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🙐' => Ok(OrnamentalDingbats::NorthWestPointingLeaf),
            '🙑' => Ok(OrnamentalDingbats::SouthWestPointingLeaf),
            '🙒' => Ok(OrnamentalDingbats::NorthEastPointingLeaf),
            '🙓' => Ok(OrnamentalDingbats::SouthEastPointingLeaf),
            '🙔' => Ok(OrnamentalDingbats::TurnedNorthWestPointingLeaf),
            '🙕' => Ok(OrnamentalDingbats::TurnedSouthWestPointingLeaf),
            '🙖' => Ok(OrnamentalDingbats::TurnedNorthEastPointingLeaf),
            '🙗' => Ok(OrnamentalDingbats::TurnedSouthEastPointingLeaf),
            '🙘' => Ok(OrnamentalDingbats::NorthWestPointingVineLeaf),
            '🙙' => Ok(OrnamentalDingbats::SouthWestPointingVineLeaf),
            '🙚' => Ok(OrnamentalDingbats::NorthEastPointingVineLeaf),
            '🙛' => Ok(OrnamentalDingbats::SouthEastPointingVineLeaf),
            '🙜' => Ok(OrnamentalDingbats::HeavyNorthWestPointingVineLeaf),
            '🙝' => Ok(OrnamentalDingbats::HeavySouthWestPointingVineLeaf),
            '🙞' => Ok(OrnamentalDingbats::HeavyNorthEastPointingVineLeaf),
            '🙟' => Ok(OrnamentalDingbats::HeavySouthEastPointingVineLeaf),
            '🙠' => Ok(OrnamentalDingbats::NorthWestPointingBud),
            '🙡' => Ok(OrnamentalDingbats::SouthWestPointingBud),
            '🙢' => Ok(OrnamentalDingbats::NorthEastPointingBud),
            '🙣' => Ok(OrnamentalDingbats::SouthEastPointingBud),
            '🙤' => Ok(OrnamentalDingbats::HeavyNorthWestPointingBud),
            '🙥' => Ok(OrnamentalDingbats::HeavySouthWestPointingBud),
            '🙦' => Ok(OrnamentalDingbats::HeavyNorthEastPointingBud),
            '🙧' => Ok(OrnamentalDingbats::HeavySouthEastPointingBud),
            '🙨' => Ok(OrnamentalDingbats::HollowQuiltSquareOrnament),
            '🙩' => Ok(OrnamentalDingbats::HollowQuiltSquareOrnamentInBlackSquare),
            '🙪' => Ok(OrnamentalDingbats::SolidQuiltSquareOrnament),
            '🙫' => Ok(OrnamentalDingbats::SolidQuiltSquareOrnamentInBlackSquare),
            '🙬' => Ok(OrnamentalDingbats::LeftwardsRocket),
            '🙭' => Ok(OrnamentalDingbats::UpwardsRocket),
            '🙮' => Ok(OrnamentalDingbats::RightwardsRocket),
            '🙯' => Ok(OrnamentalDingbats::DownwardsRocket),
            '🙰' => Ok(OrnamentalDingbats::ScriptLigatureEtOrnament),
            '🙱' => Ok(OrnamentalDingbats::HeavyScriptLigatureEtOrnament),
            '🙲' => Ok(OrnamentalDingbats::LigatureOpenEtOrnament),
            '🙳' => Ok(OrnamentalDingbats::HeavyLigatureOpenEtOrnament),
            '🙴' => Ok(OrnamentalDingbats::HeavyAmpersandOrnament),
            '🙵' => Ok(OrnamentalDingbats::SwashAmpersandOrnament),
            '🙶' => Ok(OrnamentalDingbats::SansDashSerifHeavyDoubleTurnedCommaQuotationMarkOrnament),
            '🙷' => Ok(OrnamentalDingbats::SansDashSerifHeavyDoubleCommaQuotationMarkOrnament),
            '🙸' => Ok(OrnamentalDingbats::SansDashSerifHeavyLowDoubleCommaQuotationMarkOrnament),
            '🙹' => Ok(OrnamentalDingbats::HeavyInterrobangOrnament),
            '🙺' => Ok(OrnamentalDingbats::SansDashSerifInterrobangOrnament),
            '🙻' => Ok(OrnamentalDingbats::HeavySansDashSerifInterrobangOrnament),
            '🙼' => Ok(OrnamentalDingbats::VeryHeavySolidus),
            '🙽' => Ok(OrnamentalDingbats::VeryHeavyReverseSolidus),
            '🙾' => Ok(OrnamentalDingbats::CheckerBoard),
            _ => Err(()),
        }
    }
}

impl Into<u32> for OrnamentalDingbats {
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

impl std::convert::TryFrom<u32> for OrnamentalDingbats {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for OrnamentalDingbats {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl OrnamentalDingbats {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        OrnamentalDingbats::NorthWestPointingLeaf
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            OrnamentalDingbats::NorthWestPointingLeaf => "north west pointing leaf",
            OrnamentalDingbats::SouthWestPointingLeaf => "south west pointing leaf",
            OrnamentalDingbats::NorthEastPointingLeaf => "north east pointing leaf",
            OrnamentalDingbats::SouthEastPointingLeaf => "south east pointing leaf",
            OrnamentalDingbats::TurnedNorthWestPointingLeaf => "turned north west pointing leaf",
            OrnamentalDingbats::TurnedSouthWestPointingLeaf => "turned south west pointing leaf",
            OrnamentalDingbats::TurnedNorthEastPointingLeaf => "turned north east pointing leaf",
            OrnamentalDingbats::TurnedSouthEastPointingLeaf => "turned south east pointing leaf",
            OrnamentalDingbats::NorthWestPointingVineLeaf => "north west pointing vine leaf",
            OrnamentalDingbats::SouthWestPointingVineLeaf => "south west pointing vine leaf",
            OrnamentalDingbats::NorthEastPointingVineLeaf => "north east pointing vine leaf",
            OrnamentalDingbats::SouthEastPointingVineLeaf => "south east pointing vine leaf",
            OrnamentalDingbats::HeavyNorthWestPointingVineLeaf => "heavy north west pointing vine leaf",
            OrnamentalDingbats::HeavySouthWestPointingVineLeaf => "heavy south west pointing vine leaf",
            OrnamentalDingbats::HeavyNorthEastPointingVineLeaf => "heavy north east pointing vine leaf",
            OrnamentalDingbats::HeavySouthEastPointingVineLeaf => "heavy south east pointing vine leaf",
            OrnamentalDingbats::NorthWestPointingBud => "north west pointing bud",
            OrnamentalDingbats::SouthWestPointingBud => "south west pointing bud",
            OrnamentalDingbats::NorthEastPointingBud => "north east pointing bud",
            OrnamentalDingbats::SouthEastPointingBud => "south east pointing bud",
            OrnamentalDingbats::HeavyNorthWestPointingBud => "heavy north west pointing bud",
            OrnamentalDingbats::HeavySouthWestPointingBud => "heavy south west pointing bud",
            OrnamentalDingbats::HeavyNorthEastPointingBud => "heavy north east pointing bud",
            OrnamentalDingbats::HeavySouthEastPointingBud => "heavy south east pointing bud",
            OrnamentalDingbats::HollowQuiltSquareOrnament => "hollow quilt square ornament",
            OrnamentalDingbats::HollowQuiltSquareOrnamentInBlackSquare => "hollow quilt square ornament in black square",
            OrnamentalDingbats::SolidQuiltSquareOrnament => "solid quilt square ornament",
            OrnamentalDingbats::SolidQuiltSquareOrnamentInBlackSquare => "solid quilt square ornament in black square",
            OrnamentalDingbats::LeftwardsRocket => "leftwards rocket",
            OrnamentalDingbats::UpwardsRocket => "upwards rocket",
            OrnamentalDingbats::RightwardsRocket => "rightwards rocket",
            OrnamentalDingbats::DownwardsRocket => "downwards rocket",
            OrnamentalDingbats::ScriptLigatureEtOrnament => "script ligature et ornament",
            OrnamentalDingbats::HeavyScriptLigatureEtOrnament => "heavy script ligature et ornament",
            OrnamentalDingbats::LigatureOpenEtOrnament => "ligature open et ornament",
            OrnamentalDingbats::HeavyLigatureOpenEtOrnament => "heavy ligature open et ornament",
            OrnamentalDingbats::HeavyAmpersandOrnament => "heavy ampersand ornament",
            OrnamentalDingbats::SwashAmpersandOrnament => "swash ampersand ornament",
            OrnamentalDingbats::SansDashSerifHeavyDoubleTurnedCommaQuotationMarkOrnament => "sans-serif heavy double turned comma quotation mark ornament",
            OrnamentalDingbats::SansDashSerifHeavyDoubleCommaQuotationMarkOrnament => "sans-serif heavy double comma quotation mark ornament",
            OrnamentalDingbats::SansDashSerifHeavyLowDoubleCommaQuotationMarkOrnament => "sans-serif heavy low double comma quotation mark ornament",
            OrnamentalDingbats::HeavyInterrobangOrnament => "heavy interrobang ornament",
            OrnamentalDingbats::SansDashSerifInterrobangOrnament => "sans-serif interrobang ornament",
            OrnamentalDingbats::HeavySansDashSerifInterrobangOrnament => "heavy sans-serif interrobang ornament",
            OrnamentalDingbats::VeryHeavySolidus => "very heavy solidus",
            OrnamentalDingbats::VeryHeavyReverseSolidus => "very heavy reverse solidus",
            OrnamentalDingbats::CheckerBoard => "checker board",
        }
    }
}
