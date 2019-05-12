
/// An enum to represent all characters in the KanaExtendedA block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum KanaExtendedA {
    /// \u{1b100}: '𛄀'
    HentaiganaLetterReDash3,
    /// \u{1b101}: '𛄁'
    HentaiganaLetterReDash4,
    /// \u{1b102}: '𛄂'
    HentaiganaLetterRoDash1,
    /// \u{1b103}: '𛄃'
    HentaiganaLetterRoDash2,
    /// \u{1b104}: '𛄄'
    HentaiganaLetterRoDash3,
    /// \u{1b105}: '𛄅'
    HentaiganaLetterRoDash4,
    /// \u{1b106}: '𛄆'
    HentaiganaLetterRoDash5,
    /// \u{1b107}: '𛄇'
    HentaiganaLetterRoDash6,
    /// \u{1b108}: '𛄈'
    HentaiganaLetterWaDash1,
    /// \u{1b109}: '𛄉'
    HentaiganaLetterWaDash2,
    /// \u{1b10a}: '𛄊'
    HentaiganaLetterWaDash3,
    /// \u{1b10b}: '𛄋'
    HentaiganaLetterWaDash4,
    /// \u{1b10c}: '𛄌'
    HentaiganaLetterWaDash5,
    /// \u{1b10d}: '𛄍'
    HentaiganaLetterWiDash1,
    /// \u{1b10e}: '𛄎'
    HentaiganaLetterWiDash2,
    /// \u{1b10f}: '𛄏'
    HentaiganaLetterWiDash3,
    /// \u{1b110}: '𛄐'
    HentaiganaLetterWiDash4,
    /// \u{1b111}: '𛄑'
    HentaiganaLetterWiDash5,
    /// \u{1b112}: '𛄒'
    HentaiganaLetterWeDash1,
    /// \u{1b113}: '𛄓'
    HentaiganaLetterWeDash2,
    /// \u{1b114}: '𛄔'
    HentaiganaLetterWeDash3,
    /// \u{1b115}: '𛄕'
    HentaiganaLetterWeDash4,
    /// \u{1b116}: '𛄖'
    HentaiganaLetterWoDash1,
    /// \u{1b117}: '𛄗'
    HentaiganaLetterWoDash2,
    /// \u{1b118}: '𛄘'
    HentaiganaLetterWoDash3,
    /// \u{1b119}: '𛄙'
    HentaiganaLetterWoDash4,
    /// \u{1b11a}: '𛄚'
    HentaiganaLetterWoDash5,
    /// \u{1b11b}: '𛄛'
    HentaiganaLetterWoDash6,
    /// \u{1b11c}: '𛄜'
    HentaiganaLetterWoDash7,
    /// \u{1b11d}: '𛄝'
    HentaiganaLetterNDashMuDashMoDash1,
    /// \u{1b11e}: '𛄞'
    HentaiganaLetterNDashMuDashMoDash2,
}

impl Into<char> for KanaExtendedA {
    fn into(self) -> char {
        match self {
            KanaExtendedA::HentaiganaLetterReDash3 => '𛄀',
            KanaExtendedA::HentaiganaLetterReDash4 => '𛄁',
            KanaExtendedA::HentaiganaLetterRoDash1 => '𛄂',
            KanaExtendedA::HentaiganaLetterRoDash2 => '𛄃',
            KanaExtendedA::HentaiganaLetterRoDash3 => '𛄄',
            KanaExtendedA::HentaiganaLetterRoDash4 => '𛄅',
            KanaExtendedA::HentaiganaLetterRoDash5 => '𛄆',
            KanaExtendedA::HentaiganaLetterRoDash6 => '𛄇',
            KanaExtendedA::HentaiganaLetterWaDash1 => '𛄈',
            KanaExtendedA::HentaiganaLetterWaDash2 => '𛄉',
            KanaExtendedA::HentaiganaLetterWaDash3 => '𛄊',
            KanaExtendedA::HentaiganaLetterWaDash4 => '𛄋',
            KanaExtendedA::HentaiganaLetterWaDash5 => '𛄌',
            KanaExtendedA::HentaiganaLetterWiDash1 => '𛄍',
            KanaExtendedA::HentaiganaLetterWiDash2 => '𛄎',
            KanaExtendedA::HentaiganaLetterWiDash3 => '𛄏',
            KanaExtendedA::HentaiganaLetterWiDash4 => '𛄐',
            KanaExtendedA::HentaiganaLetterWiDash5 => '𛄑',
            KanaExtendedA::HentaiganaLetterWeDash1 => '𛄒',
            KanaExtendedA::HentaiganaLetterWeDash2 => '𛄓',
            KanaExtendedA::HentaiganaLetterWeDash3 => '𛄔',
            KanaExtendedA::HentaiganaLetterWeDash4 => '𛄕',
            KanaExtendedA::HentaiganaLetterWoDash1 => '𛄖',
            KanaExtendedA::HentaiganaLetterWoDash2 => '𛄗',
            KanaExtendedA::HentaiganaLetterWoDash3 => '𛄘',
            KanaExtendedA::HentaiganaLetterWoDash4 => '𛄙',
            KanaExtendedA::HentaiganaLetterWoDash5 => '𛄚',
            KanaExtendedA::HentaiganaLetterWoDash6 => '𛄛',
            KanaExtendedA::HentaiganaLetterWoDash7 => '𛄜',
            KanaExtendedA::HentaiganaLetterNDashMuDashMoDash1 => '𛄝',
            KanaExtendedA::HentaiganaLetterNDashMuDashMoDash2 => '𛄞',
        }
    }
}

impl std::convert::TryFrom<char> for KanaExtendedA {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𛄀' => Ok(KanaExtendedA::HentaiganaLetterReDash3),
            '𛄁' => Ok(KanaExtendedA::HentaiganaLetterReDash4),
            '𛄂' => Ok(KanaExtendedA::HentaiganaLetterRoDash1),
            '𛄃' => Ok(KanaExtendedA::HentaiganaLetterRoDash2),
            '𛄄' => Ok(KanaExtendedA::HentaiganaLetterRoDash3),
            '𛄅' => Ok(KanaExtendedA::HentaiganaLetterRoDash4),
            '𛄆' => Ok(KanaExtendedA::HentaiganaLetterRoDash5),
            '𛄇' => Ok(KanaExtendedA::HentaiganaLetterRoDash6),
            '𛄈' => Ok(KanaExtendedA::HentaiganaLetterWaDash1),
            '𛄉' => Ok(KanaExtendedA::HentaiganaLetterWaDash2),
            '𛄊' => Ok(KanaExtendedA::HentaiganaLetterWaDash3),
            '𛄋' => Ok(KanaExtendedA::HentaiganaLetterWaDash4),
            '𛄌' => Ok(KanaExtendedA::HentaiganaLetterWaDash5),
            '𛄍' => Ok(KanaExtendedA::HentaiganaLetterWiDash1),
            '𛄎' => Ok(KanaExtendedA::HentaiganaLetterWiDash2),
            '𛄏' => Ok(KanaExtendedA::HentaiganaLetterWiDash3),
            '𛄐' => Ok(KanaExtendedA::HentaiganaLetterWiDash4),
            '𛄑' => Ok(KanaExtendedA::HentaiganaLetterWiDash5),
            '𛄒' => Ok(KanaExtendedA::HentaiganaLetterWeDash1),
            '𛄓' => Ok(KanaExtendedA::HentaiganaLetterWeDash2),
            '𛄔' => Ok(KanaExtendedA::HentaiganaLetterWeDash3),
            '𛄕' => Ok(KanaExtendedA::HentaiganaLetterWeDash4),
            '𛄖' => Ok(KanaExtendedA::HentaiganaLetterWoDash1),
            '𛄗' => Ok(KanaExtendedA::HentaiganaLetterWoDash2),
            '𛄘' => Ok(KanaExtendedA::HentaiganaLetterWoDash3),
            '𛄙' => Ok(KanaExtendedA::HentaiganaLetterWoDash4),
            '𛄚' => Ok(KanaExtendedA::HentaiganaLetterWoDash5),
            '𛄛' => Ok(KanaExtendedA::HentaiganaLetterWoDash6),
            '𛄜' => Ok(KanaExtendedA::HentaiganaLetterWoDash7),
            '𛄝' => Ok(KanaExtendedA::HentaiganaLetterNDashMuDashMoDash1),
            '𛄞' => Ok(KanaExtendedA::HentaiganaLetterNDashMuDashMoDash2),
            _ => Err(()),
        }
    }
}

impl Into<u32> for KanaExtendedA {
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

impl std::convert::TryFrom<u32> for KanaExtendedA {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for KanaExtendedA {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl KanaExtendedA {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        KanaExtendedA::HentaiganaLetterReDash3
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            KanaExtendedA::HentaiganaLetterReDash3 => "hentaigana letter re-3",
            KanaExtendedA::HentaiganaLetterReDash4 => "hentaigana letter re-4",
            KanaExtendedA::HentaiganaLetterRoDash1 => "hentaigana letter ro-1",
            KanaExtendedA::HentaiganaLetterRoDash2 => "hentaigana letter ro-2",
            KanaExtendedA::HentaiganaLetterRoDash3 => "hentaigana letter ro-3",
            KanaExtendedA::HentaiganaLetterRoDash4 => "hentaigana letter ro-4",
            KanaExtendedA::HentaiganaLetterRoDash5 => "hentaigana letter ro-5",
            KanaExtendedA::HentaiganaLetterRoDash6 => "hentaigana letter ro-6",
            KanaExtendedA::HentaiganaLetterWaDash1 => "hentaigana letter wa-1",
            KanaExtendedA::HentaiganaLetterWaDash2 => "hentaigana letter wa-2",
            KanaExtendedA::HentaiganaLetterWaDash3 => "hentaigana letter wa-3",
            KanaExtendedA::HentaiganaLetterWaDash4 => "hentaigana letter wa-4",
            KanaExtendedA::HentaiganaLetterWaDash5 => "hentaigana letter wa-5",
            KanaExtendedA::HentaiganaLetterWiDash1 => "hentaigana letter wi-1",
            KanaExtendedA::HentaiganaLetterWiDash2 => "hentaigana letter wi-2",
            KanaExtendedA::HentaiganaLetterWiDash3 => "hentaigana letter wi-3",
            KanaExtendedA::HentaiganaLetterWiDash4 => "hentaigana letter wi-4",
            KanaExtendedA::HentaiganaLetterWiDash5 => "hentaigana letter wi-5",
            KanaExtendedA::HentaiganaLetterWeDash1 => "hentaigana letter we-1",
            KanaExtendedA::HentaiganaLetterWeDash2 => "hentaigana letter we-2",
            KanaExtendedA::HentaiganaLetterWeDash3 => "hentaigana letter we-3",
            KanaExtendedA::HentaiganaLetterWeDash4 => "hentaigana letter we-4",
            KanaExtendedA::HentaiganaLetterWoDash1 => "hentaigana letter wo-1",
            KanaExtendedA::HentaiganaLetterWoDash2 => "hentaigana letter wo-2",
            KanaExtendedA::HentaiganaLetterWoDash3 => "hentaigana letter wo-3",
            KanaExtendedA::HentaiganaLetterWoDash4 => "hentaigana letter wo-4",
            KanaExtendedA::HentaiganaLetterWoDash5 => "hentaigana letter wo-5",
            KanaExtendedA::HentaiganaLetterWoDash6 => "hentaigana letter wo-6",
            KanaExtendedA::HentaiganaLetterWoDash7 => "hentaigana letter wo-7",
            KanaExtendedA::HentaiganaLetterNDashMuDashMoDash1 => "hentaigana letter n-mu-mo-1",
            KanaExtendedA::HentaiganaLetterNDashMuDashMoDash2 => "hentaigana letter n-mu-mo-2",
        }
    }
}
