
/// An enum to represent all characters in the Soyombo block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Soyombo {
    /// \u{11a50}: '𑩐'
    LetterA,
    /// \u{11a51}: '𑩑'
    VowelSignI,
    /// \u{11a52}: '𑩒'
    VowelSignUe,
    /// \u{11a53}: '𑩓'
    VowelSignU,
    /// \u{11a54}: '𑩔'
    VowelSignE,
    /// \u{11a55}: '𑩕'
    VowelSignO,
    /// \u{11a56}: '𑩖'
    VowelSignOe,
    /// \u{11a57}: '𑩗'
    VowelSignAi,
    /// \u{11a58}: '𑩘'
    VowelSignAu,
    /// \u{11a59}: '𑩙'
    VowelSignVocalicR,
    /// \u{11a5a}: '𑩚'
    VowelSignVocalicL,
    /// \u{11a5b}: '𑩛'
    VowelLengthMark,
    /// \u{11a5c}: '𑩜'
    LetterKa,
    /// \u{11a5d}: '𑩝'
    LetterKha,
    /// \u{11a5e}: '𑩞'
    LetterGa,
    /// \u{11a5f}: '𑩟'
    LetterGha,
    /// \u{11a60}: '𑩠'
    LetterNga,
    /// \u{11a61}: '𑩡'
    LetterCa,
    /// \u{11a62}: '𑩢'
    LetterCha,
    /// \u{11a63}: '𑩣'
    LetterJa,
    /// \u{11a64}: '𑩤'
    LetterJha,
    /// \u{11a65}: '𑩥'
    LetterNya,
    /// \u{11a66}: '𑩦'
    LetterTta,
    /// \u{11a67}: '𑩧'
    LetterTtha,
    /// \u{11a68}: '𑩨'
    LetterDda,
    /// \u{11a69}: '𑩩'
    LetterDdha,
    /// \u{11a6a}: '𑩪'
    LetterNna,
    /// \u{11a6b}: '𑩫'
    LetterTa,
    /// \u{11a6c}: '𑩬'
    LetterTha,
    /// \u{11a6d}: '𑩭'
    LetterDa,
    /// \u{11a6e}: '𑩮'
    LetterDha,
    /// \u{11a6f}: '𑩯'
    LetterNa,
    /// \u{11a70}: '𑩰'
    LetterPa,
    /// \u{11a71}: '𑩱'
    LetterPha,
    /// \u{11a72}: '𑩲'
    LetterBa,
    /// \u{11a73}: '𑩳'
    LetterBha,
    /// \u{11a74}: '𑩴'
    LetterMa,
    /// \u{11a75}: '𑩵'
    LetterTsa,
    /// \u{11a76}: '𑩶'
    LetterTsha,
    /// \u{11a77}: '𑩷'
    LetterDza,
    /// \u{11a78}: '𑩸'
    LetterZha,
    /// \u{11a79}: '𑩹'
    LetterZa,
    /// \u{11a7a}: '𑩺'
    LetterDashA,
    /// \u{11a7b}: '𑩻'
    LetterYa,
    /// \u{11a7c}: '𑩼'
    LetterRa,
    /// \u{11a7d}: '𑩽'
    LetterLa,
    /// \u{11a7e}: '𑩾'
    LetterVa,
    /// \u{11a7f}: '𑩿'
    LetterSha,
    /// \u{11a80}: '𑪀'
    LetterSsa,
    /// \u{11a81}: '𑪁'
    LetterSa,
    /// \u{11a82}: '𑪂'
    LetterHa,
    /// \u{11a83}: '𑪃'
    LetterKssa,
    /// \u{11a84}: '𑪄'
    SignJihvamuliya,
    /// \u{11a85}: '𑪅'
    SignUpadhmaniya,
    /// \u{11a86}: '𑪆'
    ClusterDashInitialLetterRa,
    /// \u{11a87}: '𑪇'
    ClusterDashInitialLetterLa,
    /// \u{11a88}: '𑪈'
    ClusterDashInitialLetterSha,
    /// \u{11a89}: '𑪉'
    ClusterDashInitialLetterSa,
    /// \u{11a8a}: '𑪊'
    FinalConsonantSignG,
    /// \u{11a8b}: '𑪋'
    FinalConsonantSignK,
    /// \u{11a8c}: '𑪌'
    FinalConsonantSignNg,
    /// \u{11a8d}: '𑪍'
    FinalConsonantSignD,
    /// \u{11a8e}: '𑪎'
    FinalConsonantSignN,
    /// \u{11a8f}: '𑪏'
    FinalConsonantSignB,
    /// \u{11a90}: '𑪐'
    FinalConsonantSignM,
    /// \u{11a91}: '𑪑'
    FinalConsonantSignR,
    /// \u{11a92}: '𑪒'
    FinalConsonantSignL,
    /// \u{11a93}: '𑪓'
    FinalConsonantSignSh,
    /// \u{11a94}: '𑪔'
    FinalConsonantSignS,
    /// \u{11a95}: '𑪕'
    FinalConsonantSignDashA,
    /// \u{11a96}: '𑪖'
    SignAnusvara,
    /// \u{11a97}: '𑪗'
    SignVisarga,
    /// \u{11a98}: '𑪘'
    GeminationMark,
    /// \u{11a99}: '𑪙'
    Subjoiner,
    /// \u{11a9a}: '𑪚'
    MarkTsheg,
    /// \u{11a9b}: '𑪛'
    MarkShad,
    /// \u{11a9c}: '𑪜'
    MarkDoubleShad,
    /// \u{11a9d}: '𑪝'
    MarkPluta,
    /// \u{11a9e}: '𑪞'
    HeadMarkWithMoonAndSunAndTripleFlame,
    /// \u{11a9f}: '𑪟'
    HeadMarkWithMoonAndSunAndFlame,
    /// \u{11aa0}: '𑪠'
    HeadMarkWithMoonAndSun,
    /// \u{11aa1}: '𑪡'
    TerminalMarkDash1,
    /// \u{11aa2}: '𑪢'
    TerminalMarkDash2,
}

impl Into<char> for Soyombo {
    fn into(self) -> char {
        match self {
            Soyombo::LetterA => '𑩐',
            Soyombo::VowelSignI => '𑩑',
            Soyombo::VowelSignUe => '𑩒',
            Soyombo::VowelSignU => '𑩓',
            Soyombo::VowelSignE => '𑩔',
            Soyombo::VowelSignO => '𑩕',
            Soyombo::VowelSignOe => '𑩖',
            Soyombo::VowelSignAi => '𑩗',
            Soyombo::VowelSignAu => '𑩘',
            Soyombo::VowelSignVocalicR => '𑩙',
            Soyombo::VowelSignVocalicL => '𑩚',
            Soyombo::VowelLengthMark => '𑩛',
            Soyombo::LetterKa => '𑩜',
            Soyombo::LetterKha => '𑩝',
            Soyombo::LetterGa => '𑩞',
            Soyombo::LetterGha => '𑩟',
            Soyombo::LetterNga => '𑩠',
            Soyombo::LetterCa => '𑩡',
            Soyombo::LetterCha => '𑩢',
            Soyombo::LetterJa => '𑩣',
            Soyombo::LetterJha => '𑩤',
            Soyombo::LetterNya => '𑩥',
            Soyombo::LetterTta => '𑩦',
            Soyombo::LetterTtha => '𑩧',
            Soyombo::LetterDda => '𑩨',
            Soyombo::LetterDdha => '𑩩',
            Soyombo::LetterNna => '𑩪',
            Soyombo::LetterTa => '𑩫',
            Soyombo::LetterTha => '𑩬',
            Soyombo::LetterDa => '𑩭',
            Soyombo::LetterDha => '𑩮',
            Soyombo::LetterNa => '𑩯',
            Soyombo::LetterPa => '𑩰',
            Soyombo::LetterPha => '𑩱',
            Soyombo::LetterBa => '𑩲',
            Soyombo::LetterBha => '𑩳',
            Soyombo::LetterMa => '𑩴',
            Soyombo::LetterTsa => '𑩵',
            Soyombo::LetterTsha => '𑩶',
            Soyombo::LetterDza => '𑩷',
            Soyombo::LetterZha => '𑩸',
            Soyombo::LetterZa => '𑩹',
            Soyombo::LetterDashA => '𑩺',
            Soyombo::LetterYa => '𑩻',
            Soyombo::LetterRa => '𑩼',
            Soyombo::LetterLa => '𑩽',
            Soyombo::LetterVa => '𑩾',
            Soyombo::LetterSha => '𑩿',
            Soyombo::LetterSsa => '𑪀',
            Soyombo::LetterSa => '𑪁',
            Soyombo::LetterHa => '𑪂',
            Soyombo::LetterKssa => '𑪃',
            Soyombo::SignJihvamuliya => '𑪄',
            Soyombo::SignUpadhmaniya => '𑪅',
            Soyombo::ClusterDashInitialLetterRa => '𑪆',
            Soyombo::ClusterDashInitialLetterLa => '𑪇',
            Soyombo::ClusterDashInitialLetterSha => '𑪈',
            Soyombo::ClusterDashInitialLetterSa => '𑪉',
            Soyombo::FinalConsonantSignG => '𑪊',
            Soyombo::FinalConsonantSignK => '𑪋',
            Soyombo::FinalConsonantSignNg => '𑪌',
            Soyombo::FinalConsonantSignD => '𑪍',
            Soyombo::FinalConsonantSignN => '𑪎',
            Soyombo::FinalConsonantSignB => '𑪏',
            Soyombo::FinalConsonantSignM => '𑪐',
            Soyombo::FinalConsonantSignR => '𑪑',
            Soyombo::FinalConsonantSignL => '𑪒',
            Soyombo::FinalConsonantSignSh => '𑪓',
            Soyombo::FinalConsonantSignS => '𑪔',
            Soyombo::FinalConsonantSignDashA => '𑪕',
            Soyombo::SignAnusvara => '𑪖',
            Soyombo::SignVisarga => '𑪗',
            Soyombo::GeminationMark => '𑪘',
            Soyombo::Subjoiner => '𑪙',
            Soyombo::MarkTsheg => '𑪚',
            Soyombo::MarkShad => '𑪛',
            Soyombo::MarkDoubleShad => '𑪜',
            Soyombo::MarkPluta => '𑪝',
            Soyombo::HeadMarkWithMoonAndSunAndTripleFlame => '𑪞',
            Soyombo::HeadMarkWithMoonAndSunAndFlame => '𑪟',
            Soyombo::HeadMarkWithMoonAndSun => '𑪠',
            Soyombo::TerminalMarkDash1 => '𑪡',
            Soyombo::TerminalMarkDash2 => '𑪢',
        }
    }
}

impl std::convert::TryFrom<char> for Soyombo {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𑩐' => Ok(Soyombo::LetterA),
            '𑩑' => Ok(Soyombo::VowelSignI),
            '𑩒' => Ok(Soyombo::VowelSignUe),
            '𑩓' => Ok(Soyombo::VowelSignU),
            '𑩔' => Ok(Soyombo::VowelSignE),
            '𑩕' => Ok(Soyombo::VowelSignO),
            '𑩖' => Ok(Soyombo::VowelSignOe),
            '𑩗' => Ok(Soyombo::VowelSignAi),
            '𑩘' => Ok(Soyombo::VowelSignAu),
            '𑩙' => Ok(Soyombo::VowelSignVocalicR),
            '𑩚' => Ok(Soyombo::VowelSignVocalicL),
            '𑩛' => Ok(Soyombo::VowelLengthMark),
            '𑩜' => Ok(Soyombo::LetterKa),
            '𑩝' => Ok(Soyombo::LetterKha),
            '𑩞' => Ok(Soyombo::LetterGa),
            '𑩟' => Ok(Soyombo::LetterGha),
            '𑩠' => Ok(Soyombo::LetterNga),
            '𑩡' => Ok(Soyombo::LetterCa),
            '𑩢' => Ok(Soyombo::LetterCha),
            '𑩣' => Ok(Soyombo::LetterJa),
            '𑩤' => Ok(Soyombo::LetterJha),
            '𑩥' => Ok(Soyombo::LetterNya),
            '𑩦' => Ok(Soyombo::LetterTta),
            '𑩧' => Ok(Soyombo::LetterTtha),
            '𑩨' => Ok(Soyombo::LetterDda),
            '𑩩' => Ok(Soyombo::LetterDdha),
            '𑩪' => Ok(Soyombo::LetterNna),
            '𑩫' => Ok(Soyombo::LetterTa),
            '𑩬' => Ok(Soyombo::LetterTha),
            '𑩭' => Ok(Soyombo::LetterDa),
            '𑩮' => Ok(Soyombo::LetterDha),
            '𑩯' => Ok(Soyombo::LetterNa),
            '𑩰' => Ok(Soyombo::LetterPa),
            '𑩱' => Ok(Soyombo::LetterPha),
            '𑩲' => Ok(Soyombo::LetterBa),
            '𑩳' => Ok(Soyombo::LetterBha),
            '𑩴' => Ok(Soyombo::LetterMa),
            '𑩵' => Ok(Soyombo::LetterTsa),
            '𑩶' => Ok(Soyombo::LetterTsha),
            '𑩷' => Ok(Soyombo::LetterDza),
            '𑩸' => Ok(Soyombo::LetterZha),
            '𑩹' => Ok(Soyombo::LetterZa),
            '𑩺' => Ok(Soyombo::LetterDashA),
            '𑩻' => Ok(Soyombo::LetterYa),
            '𑩼' => Ok(Soyombo::LetterRa),
            '𑩽' => Ok(Soyombo::LetterLa),
            '𑩾' => Ok(Soyombo::LetterVa),
            '𑩿' => Ok(Soyombo::LetterSha),
            '𑪀' => Ok(Soyombo::LetterSsa),
            '𑪁' => Ok(Soyombo::LetterSa),
            '𑪂' => Ok(Soyombo::LetterHa),
            '𑪃' => Ok(Soyombo::LetterKssa),
            '𑪄' => Ok(Soyombo::SignJihvamuliya),
            '𑪅' => Ok(Soyombo::SignUpadhmaniya),
            '𑪆' => Ok(Soyombo::ClusterDashInitialLetterRa),
            '𑪇' => Ok(Soyombo::ClusterDashInitialLetterLa),
            '𑪈' => Ok(Soyombo::ClusterDashInitialLetterSha),
            '𑪉' => Ok(Soyombo::ClusterDashInitialLetterSa),
            '𑪊' => Ok(Soyombo::FinalConsonantSignG),
            '𑪋' => Ok(Soyombo::FinalConsonantSignK),
            '𑪌' => Ok(Soyombo::FinalConsonantSignNg),
            '𑪍' => Ok(Soyombo::FinalConsonantSignD),
            '𑪎' => Ok(Soyombo::FinalConsonantSignN),
            '𑪏' => Ok(Soyombo::FinalConsonantSignB),
            '𑪐' => Ok(Soyombo::FinalConsonantSignM),
            '𑪑' => Ok(Soyombo::FinalConsonantSignR),
            '𑪒' => Ok(Soyombo::FinalConsonantSignL),
            '𑪓' => Ok(Soyombo::FinalConsonantSignSh),
            '𑪔' => Ok(Soyombo::FinalConsonantSignS),
            '𑪕' => Ok(Soyombo::FinalConsonantSignDashA),
            '𑪖' => Ok(Soyombo::SignAnusvara),
            '𑪗' => Ok(Soyombo::SignVisarga),
            '𑪘' => Ok(Soyombo::GeminationMark),
            '𑪙' => Ok(Soyombo::Subjoiner),
            '𑪚' => Ok(Soyombo::MarkTsheg),
            '𑪛' => Ok(Soyombo::MarkShad),
            '𑪜' => Ok(Soyombo::MarkDoubleShad),
            '𑪝' => Ok(Soyombo::MarkPluta),
            '𑪞' => Ok(Soyombo::HeadMarkWithMoonAndSunAndTripleFlame),
            '𑪟' => Ok(Soyombo::HeadMarkWithMoonAndSunAndFlame),
            '𑪠' => Ok(Soyombo::HeadMarkWithMoonAndSun),
            '𑪡' => Ok(Soyombo::TerminalMarkDash1),
            '𑪢' => Ok(Soyombo::TerminalMarkDash2),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Soyombo {
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

impl std::convert::TryFrom<u32> for Soyombo {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Soyombo {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Soyombo {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Soyombo::LetterA
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Soyombo{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
