
/// An enum to represent all characters in the TaiViet block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TaiViet {
    /// \u{aa80}: 'ꪀ'
    LetterLowKo,
    /// \u{aa81}: 'ꪁ'
    LetterHighKo,
    /// \u{aa82}: 'ꪂ'
    LetterLowKho,
    /// \u{aa83}: 'ꪃ'
    LetterHighKho,
    /// \u{aa84}: 'ꪄ'
    LetterLowKhho,
    /// \u{aa85}: 'ꪅ'
    LetterHighKhho,
    /// \u{aa86}: 'ꪆ'
    LetterLowGo,
    /// \u{aa87}: 'ꪇ'
    LetterHighGo,
    /// \u{aa88}: 'ꪈ'
    LetterLowNgo,
    /// \u{aa89}: 'ꪉ'
    LetterHighNgo,
    /// \u{aa8a}: 'ꪊ'
    LetterLowCo,
    /// \u{aa8b}: 'ꪋ'
    LetterHighCo,
    /// \u{aa8c}: 'ꪌ'
    LetterLowCho,
    /// \u{aa8d}: 'ꪍ'
    LetterHighCho,
    /// \u{aa8e}: 'ꪎ'
    LetterLowSo,
    /// \u{aa8f}: 'ꪏ'
    LetterHighSo,
    /// \u{aa90}: 'ꪐ'
    LetterLowNyo,
    /// \u{aa91}: 'ꪑ'
    LetterHighNyo,
    /// \u{aa92}: 'ꪒ'
    LetterLowDo,
    /// \u{aa93}: 'ꪓ'
    LetterHighDo,
    /// \u{aa94}: 'ꪔ'
    LetterLowTo,
    /// \u{aa95}: 'ꪕ'
    LetterHighTo,
    /// \u{aa96}: 'ꪖ'
    LetterLowTho,
    /// \u{aa97}: 'ꪗ'
    LetterHighTho,
    /// \u{aa98}: 'ꪘ'
    LetterLowNo,
    /// \u{aa99}: 'ꪙ'
    LetterHighNo,
    /// \u{aa9a}: 'ꪚ'
    LetterLowBo,
    /// \u{aa9b}: 'ꪛ'
    LetterHighBo,
    /// \u{aa9c}: 'ꪜ'
    LetterLowPo,
    /// \u{aa9d}: 'ꪝ'
    LetterHighPo,
    /// \u{aa9e}: 'ꪞ'
    LetterLowPho,
    /// \u{aa9f}: 'ꪟ'
    LetterHighPho,
    /// \u{aaa0}: 'ꪠ'
    LetterLowFo,
    /// \u{aaa1}: 'ꪡ'
    LetterHighFo,
    /// \u{aaa2}: 'ꪢ'
    LetterLowMo,
    /// \u{aaa3}: 'ꪣ'
    LetterHighMo,
    /// \u{aaa4}: 'ꪤ'
    LetterLowYo,
    /// \u{aaa5}: 'ꪥ'
    LetterHighYo,
    /// \u{aaa6}: 'ꪦ'
    LetterLowRo,
    /// \u{aaa7}: 'ꪧ'
    LetterHighRo,
    /// \u{aaa8}: 'ꪨ'
    LetterLowLo,
    /// \u{aaa9}: 'ꪩ'
    LetterHighLo,
    /// \u{aaaa}: 'ꪪ'
    LetterLowVo,
    /// \u{aaab}: 'ꪫ'
    LetterHighVo,
    /// \u{aaac}: 'ꪬ'
    LetterLowHo,
    /// \u{aaad}: 'ꪭ'
    LetterHighHo,
    /// \u{aaae}: 'ꪮ'
    LetterLowO,
    /// \u{aaaf}: 'ꪯ'
    LetterHighO,
    /// \u{aab0}: 'ꪰ'
    MaiKang,
    /// \u{aab1}: 'ꪱ'
    VowelAa,
    /// \u{aab2}: 'ꪲ'
    VowelI,
    /// \u{aab3}: 'ꪳ'
    VowelUe,
    /// \u{aab4}: 'ꪴ'
    VowelU,
    /// \u{aab5}: 'ꪵ'
    VowelE,
    /// \u{aab6}: 'ꪶ'
    VowelO,
    /// \u{aab7}: 'ꪷ'
    MaiKhit,
    /// \u{aab8}: 'ꪸ'
    VowelIa,
    /// \u{aab9}: 'ꪹ'
    VowelUea,
    /// \u{aaba}: 'ꪺ'
    VowelUa,
    /// \u{aabb}: 'ꪻ'
    VowelAue,
    /// \u{aabc}: 'ꪼ'
    VowelAy,
    /// \u{aabd}: 'ꪽ'
    VowelAn,
    /// \u{aabe}: 'ꪾ'
    VowelAm,
    /// \u{aabf}: '꪿'
    ToneMaiEk,
    /// \u{aac0}: 'ꫀ'
    ToneMaiNueng,
    /// \u{aac1}: '꫁'
    ToneMaiTho,
    /// \u{aac2}: 'ꫂ'
    ToneMaiSong,
    /// \u{aadb}: 'ꫛ'
    SymbolKon,
    /// \u{aadc}: 'ꫜ'
    SymbolNueng,
    /// \u{aadd}: 'ꫝ'
    SymbolSam,
    /// \u{aade}: '꫞'
    SymbolHoHoi,
}

impl Into<char> for TaiViet {
    fn into(self) -> char {
        match self {
            TaiViet::LetterLowKo => 'ꪀ',
            TaiViet::LetterHighKo => 'ꪁ',
            TaiViet::LetterLowKho => 'ꪂ',
            TaiViet::LetterHighKho => 'ꪃ',
            TaiViet::LetterLowKhho => 'ꪄ',
            TaiViet::LetterHighKhho => 'ꪅ',
            TaiViet::LetterLowGo => 'ꪆ',
            TaiViet::LetterHighGo => 'ꪇ',
            TaiViet::LetterLowNgo => 'ꪈ',
            TaiViet::LetterHighNgo => 'ꪉ',
            TaiViet::LetterLowCo => 'ꪊ',
            TaiViet::LetterHighCo => 'ꪋ',
            TaiViet::LetterLowCho => 'ꪌ',
            TaiViet::LetterHighCho => 'ꪍ',
            TaiViet::LetterLowSo => 'ꪎ',
            TaiViet::LetterHighSo => 'ꪏ',
            TaiViet::LetterLowNyo => 'ꪐ',
            TaiViet::LetterHighNyo => 'ꪑ',
            TaiViet::LetterLowDo => 'ꪒ',
            TaiViet::LetterHighDo => 'ꪓ',
            TaiViet::LetterLowTo => 'ꪔ',
            TaiViet::LetterHighTo => 'ꪕ',
            TaiViet::LetterLowTho => 'ꪖ',
            TaiViet::LetterHighTho => 'ꪗ',
            TaiViet::LetterLowNo => 'ꪘ',
            TaiViet::LetterHighNo => 'ꪙ',
            TaiViet::LetterLowBo => 'ꪚ',
            TaiViet::LetterHighBo => 'ꪛ',
            TaiViet::LetterLowPo => 'ꪜ',
            TaiViet::LetterHighPo => 'ꪝ',
            TaiViet::LetterLowPho => 'ꪞ',
            TaiViet::LetterHighPho => 'ꪟ',
            TaiViet::LetterLowFo => 'ꪠ',
            TaiViet::LetterHighFo => 'ꪡ',
            TaiViet::LetterLowMo => 'ꪢ',
            TaiViet::LetterHighMo => 'ꪣ',
            TaiViet::LetterLowYo => 'ꪤ',
            TaiViet::LetterHighYo => 'ꪥ',
            TaiViet::LetterLowRo => 'ꪦ',
            TaiViet::LetterHighRo => 'ꪧ',
            TaiViet::LetterLowLo => 'ꪨ',
            TaiViet::LetterHighLo => 'ꪩ',
            TaiViet::LetterLowVo => 'ꪪ',
            TaiViet::LetterHighVo => 'ꪫ',
            TaiViet::LetterLowHo => 'ꪬ',
            TaiViet::LetterHighHo => 'ꪭ',
            TaiViet::LetterLowO => 'ꪮ',
            TaiViet::LetterHighO => 'ꪯ',
            TaiViet::MaiKang => 'ꪰ',
            TaiViet::VowelAa => 'ꪱ',
            TaiViet::VowelI => 'ꪲ',
            TaiViet::VowelUe => 'ꪳ',
            TaiViet::VowelU => 'ꪴ',
            TaiViet::VowelE => 'ꪵ',
            TaiViet::VowelO => 'ꪶ',
            TaiViet::MaiKhit => 'ꪷ',
            TaiViet::VowelIa => 'ꪸ',
            TaiViet::VowelUea => 'ꪹ',
            TaiViet::VowelUa => 'ꪺ',
            TaiViet::VowelAue => 'ꪻ',
            TaiViet::VowelAy => 'ꪼ',
            TaiViet::VowelAn => 'ꪽ',
            TaiViet::VowelAm => 'ꪾ',
            TaiViet::ToneMaiEk => '꪿',
            TaiViet::ToneMaiNueng => 'ꫀ',
            TaiViet::ToneMaiTho => '꫁',
            TaiViet::ToneMaiSong => 'ꫂ',
            TaiViet::SymbolKon => 'ꫛ',
            TaiViet::SymbolNueng => 'ꫜ',
            TaiViet::SymbolSam => 'ꫝ',
            TaiViet::SymbolHoHoi => '꫞',
        }
    }
}

impl std::convert::TryFrom<char> for TaiViet {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'ꪀ' => Ok(TaiViet::LetterLowKo),
            'ꪁ' => Ok(TaiViet::LetterHighKo),
            'ꪂ' => Ok(TaiViet::LetterLowKho),
            'ꪃ' => Ok(TaiViet::LetterHighKho),
            'ꪄ' => Ok(TaiViet::LetterLowKhho),
            'ꪅ' => Ok(TaiViet::LetterHighKhho),
            'ꪆ' => Ok(TaiViet::LetterLowGo),
            'ꪇ' => Ok(TaiViet::LetterHighGo),
            'ꪈ' => Ok(TaiViet::LetterLowNgo),
            'ꪉ' => Ok(TaiViet::LetterHighNgo),
            'ꪊ' => Ok(TaiViet::LetterLowCo),
            'ꪋ' => Ok(TaiViet::LetterHighCo),
            'ꪌ' => Ok(TaiViet::LetterLowCho),
            'ꪍ' => Ok(TaiViet::LetterHighCho),
            'ꪎ' => Ok(TaiViet::LetterLowSo),
            'ꪏ' => Ok(TaiViet::LetterHighSo),
            'ꪐ' => Ok(TaiViet::LetterLowNyo),
            'ꪑ' => Ok(TaiViet::LetterHighNyo),
            'ꪒ' => Ok(TaiViet::LetterLowDo),
            'ꪓ' => Ok(TaiViet::LetterHighDo),
            'ꪔ' => Ok(TaiViet::LetterLowTo),
            'ꪕ' => Ok(TaiViet::LetterHighTo),
            'ꪖ' => Ok(TaiViet::LetterLowTho),
            'ꪗ' => Ok(TaiViet::LetterHighTho),
            'ꪘ' => Ok(TaiViet::LetterLowNo),
            'ꪙ' => Ok(TaiViet::LetterHighNo),
            'ꪚ' => Ok(TaiViet::LetterLowBo),
            'ꪛ' => Ok(TaiViet::LetterHighBo),
            'ꪜ' => Ok(TaiViet::LetterLowPo),
            'ꪝ' => Ok(TaiViet::LetterHighPo),
            'ꪞ' => Ok(TaiViet::LetterLowPho),
            'ꪟ' => Ok(TaiViet::LetterHighPho),
            'ꪠ' => Ok(TaiViet::LetterLowFo),
            'ꪡ' => Ok(TaiViet::LetterHighFo),
            'ꪢ' => Ok(TaiViet::LetterLowMo),
            'ꪣ' => Ok(TaiViet::LetterHighMo),
            'ꪤ' => Ok(TaiViet::LetterLowYo),
            'ꪥ' => Ok(TaiViet::LetterHighYo),
            'ꪦ' => Ok(TaiViet::LetterLowRo),
            'ꪧ' => Ok(TaiViet::LetterHighRo),
            'ꪨ' => Ok(TaiViet::LetterLowLo),
            'ꪩ' => Ok(TaiViet::LetterHighLo),
            'ꪪ' => Ok(TaiViet::LetterLowVo),
            'ꪫ' => Ok(TaiViet::LetterHighVo),
            'ꪬ' => Ok(TaiViet::LetterLowHo),
            'ꪭ' => Ok(TaiViet::LetterHighHo),
            'ꪮ' => Ok(TaiViet::LetterLowO),
            'ꪯ' => Ok(TaiViet::LetterHighO),
            'ꪰ' => Ok(TaiViet::MaiKang),
            'ꪱ' => Ok(TaiViet::VowelAa),
            'ꪲ' => Ok(TaiViet::VowelI),
            'ꪳ' => Ok(TaiViet::VowelUe),
            'ꪴ' => Ok(TaiViet::VowelU),
            'ꪵ' => Ok(TaiViet::VowelE),
            'ꪶ' => Ok(TaiViet::VowelO),
            'ꪷ' => Ok(TaiViet::MaiKhit),
            'ꪸ' => Ok(TaiViet::VowelIa),
            'ꪹ' => Ok(TaiViet::VowelUea),
            'ꪺ' => Ok(TaiViet::VowelUa),
            'ꪻ' => Ok(TaiViet::VowelAue),
            'ꪼ' => Ok(TaiViet::VowelAy),
            'ꪽ' => Ok(TaiViet::VowelAn),
            'ꪾ' => Ok(TaiViet::VowelAm),
            '꪿' => Ok(TaiViet::ToneMaiEk),
            'ꫀ' => Ok(TaiViet::ToneMaiNueng),
            '꫁' => Ok(TaiViet::ToneMaiTho),
            'ꫂ' => Ok(TaiViet::ToneMaiSong),
            'ꫛ' => Ok(TaiViet::SymbolKon),
            'ꫜ' => Ok(TaiViet::SymbolNueng),
            'ꫝ' => Ok(TaiViet::SymbolSam),
            '꫞' => Ok(TaiViet::SymbolHoHoi),
            _ => Err(()),
        }
    }
}

impl Into<u32> for TaiViet {
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

impl std::convert::TryFrom<u32> for TaiViet {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for TaiViet {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl TaiViet {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        TaiViet::LetterLowKo
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            TaiViet::LetterLowKo => "tai viet letter low ko",
            TaiViet::LetterHighKo => "tai viet letter high ko",
            TaiViet::LetterLowKho => "tai viet letter low kho",
            TaiViet::LetterHighKho => "tai viet letter high kho",
            TaiViet::LetterLowKhho => "tai viet letter low khho",
            TaiViet::LetterHighKhho => "tai viet letter high khho",
            TaiViet::LetterLowGo => "tai viet letter low go",
            TaiViet::LetterHighGo => "tai viet letter high go",
            TaiViet::LetterLowNgo => "tai viet letter low ngo",
            TaiViet::LetterHighNgo => "tai viet letter high ngo",
            TaiViet::LetterLowCo => "tai viet letter low co",
            TaiViet::LetterHighCo => "tai viet letter high co",
            TaiViet::LetterLowCho => "tai viet letter low cho",
            TaiViet::LetterHighCho => "tai viet letter high cho",
            TaiViet::LetterLowSo => "tai viet letter low so",
            TaiViet::LetterHighSo => "tai viet letter high so",
            TaiViet::LetterLowNyo => "tai viet letter low nyo",
            TaiViet::LetterHighNyo => "tai viet letter high nyo",
            TaiViet::LetterLowDo => "tai viet letter low do",
            TaiViet::LetterHighDo => "tai viet letter high do",
            TaiViet::LetterLowTo => "tai viet letter low to",
            TaiViet::LetterHighTo => "tai viet letter high to",
            TaiViet::LetterLowTho => "tai viet letter low tho",
            TaiViet::LetterHighTho => "tai viet letter high tho",
            TaiViet::LetterLowNo => "tai viet letter low no",
            TaiViet::LetterHighNo => "tai viet letter high no",
            TaiViet::LetterLowBo => "tai viet letter low bo",
            TaiViet::LetterHighBo => "tai viet letter high bo",
            TaiViet::LetterLowPo => "tai viet letter low po",
            TaiViet::LetterHighPo => "tai viet letter high po",
            TaiViet::LetterLowPho => "tai viet letter low pho",
            TaiViet::LetterHighPho => "tai viet letter high pho",
            TaiViet::LetterLowFo => "tai viet letter low fo",
            TaiViet::LetterHighFo => "tai viet letter high fo",
            TaiViet::LetterLowMo => "tai viet letter low mo",
            TaiViet::LetterHighMo => "tai viet letter high mo",
            TaiViet::LetterLowYo => "tai viet letter low yo",
            TaiViet::LetterHighYo => "tai viet letter high yo",
            TaiViet::LetterLowRo => "tai viet letter low ro",
            TaiViet::LetterHighRo => "tai viet letter high ro",
            TaiViet::LetterLowLo => "tai viet letter low lo",
            TaiViet::LetterHighLo => "tai viet letter high lo",
            TaiViet::LetterLowVo => "tai viet letter low vo",
            TaiViet::LetterHighVo => "tai viet letter high vo",
            TaiViet::LetterLowHo => "tai viet letter low ho",
            TaiViet::LetterHighHo => "tai viet letter high ho",
            TaiViet::LetterLowO => "tai viet letter low o",
            TaiViet::LetterHighO => "tai viet letter high o",
            TaiViet::MaiKang => "tai viet mai kang",
            TaiViet::VowelAa => "tai viet vowel aa",
            TaiViet::VowelI => "tai viet vowel i",
            TaiViet::VowelUe => "tai viet vowel ue",
            TaiViet::VowelU => "tai viet vowel u",
            TaiViet::VowelE => "tai viet vowel e",
            TaiViet::VowelO => "tai viet vowel o",
            TaiViet::MaiKhit => "tai viet mai khit",
            TaiViet::VowelIa => "tai viet vowel ia",
            TaiViet::VowelUea => "tai viet vowel uea",
            TaiViet::VowelUa => "tai viet vowel ua",
            TaiViet::VowelAue => "tai viet vowel aue",
            TaiViet::VowelAy => "tai viet vowel ay",
            TaiViet::VowelAn => "tai viet vowel an",
            TaiViet::VowelAm => "tai viet vowel am",
            TaiViet::ToneMaiEk => "tai viet tone mai ek",
            TaiViet::ToneMaiNueng => "tai viet tone mai nueng",
            TaiViet::ToneMaiTho => "tai viet tone mai tho",
            TaiViet::ToneMaiSong => "tai viet tone mai song",
            TaiViet::SymbolKon => "tai viet symbol kon",
            TaiViet::SymbolNueng => "tai viet symbol nueng",
            TaiViet::SymbolSam => "tai viet symbol sam",
            TaiViet::SymbolHoHoi => "tai viet symbol ho hoi",
        }
    }
}
