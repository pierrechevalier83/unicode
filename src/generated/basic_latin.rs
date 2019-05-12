
/// An enum to represent all characters in the BasicLatin block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BasicLatin {
    /// \u{0}: ' '
    Control0000,
    /// \u{1}: ''
    Control0001,
    /// \u{2}: ''
    Control0002,
    /// \u{3}: ''
    Control0003,
    /// \u{4}: ''
    Control0004,
    /// \u{5}: ''
    Control0005,
    /// \u{6}: ''
    Control0006,
    /// \u{7}: ''
    Control0007,
    /// \u{8}: ''
    Control0008,
    /// \u{9}: '\t'
    Control0009,
    /// \u{a}: '\n'
    Control000a,
    /// \u{b}: ''
    Control000b,
    /// \u{c}: ''
    Control000c,
    /// \u{d}: '\r'
    Control000d,
    /// \u{e}: ''
    Control000e,
    /// \u{f}: ''
    Control000f,
    /// \u{10}: ''
    Control0010,
    /// \u{11}: ''
    Control0011,
    /// \u{12}: ''
    Control0012,
    /// \u{13}: ''
    Control0013,
    /// \u{14}: ''
    Control0014,
    /// \u{15}: ''
    Control0015,
    /// \u{16}: ''
    Control0016,
    /// \u{17}: ''
    Control0017,
    /// \u{18}: ''
    Control0018,
    /// \u{19}: ''
    Control0019,
    /// \u{1a}: ''
    Control001a,
    /// \u{1b}: ''
    Control001b,
    /// \u{1c}: ''
    Control001c,
    /// \u{1d}: ''
    Control001d,
    /// \u{1e}: ''
    Control001e,
    /// \u{1f}: ''
    Control001f,
    /// \u{20}: ' '
    Space,
    /// \u{21}: '!'
    ExclamationMark,
    /// \u{22}: '"'
    QuotationMark,
    /// \u{23}: '#'
    NumberSign,
    /// \u{24}: '$'
    DollarSign,
    /// \u{25}: '%'
    PercentSign,
    /// \u{26}: '&'
    Ampersand,
    /// \u{27}: '\''
    Apostrophe,
    /// \u{28}: '('
    LeftParenthesis,
    /// \u{29}: ')'
    RightParenthesis,
    /// \u{2a}: '*'
    Asterisk,
    /// \u{2b}: '+'
    PlusSign,
    /// \u{2c}: ','
    Comma,
    /// \u{2d}: '-'
    HyphenDashMinus,
    /// \u{2e}: '.'
    FullStop,
    /// \u{2f}: '/'
    Solidus,
    /// \u{30}: '0'
    DigitZero,
    /// \u{31}: '1'
    DigitOne,
    /// \u{32}: '2'
    DigitTwo,
    /// \u{33}: '3'
    DigitThree,
    /// \u{34}: '4'
    DigitFour,
    /// \u{35}: '5'
    DigitFive,
    /// \u{36}: '6'
    DigitSix,
    /// \u{37}: '7'
    DigitSeven,
    /// \u{38}: '8'
    DigitEight,
    /// \u{39}: '9'
    DigitNine,
    /// \u{3a}: ':'
    Colon,
    /// \u{3b}: ';'
    Semicolon,
    /// \u{3c}: '<'
    LessDashThanSign,
    /// \u{3d}: '='
    EqualsSign,
    /// \u{3e}: '>'
    GreaterDashThanSign,
    /// \u{3f}: '?'
    QuestionMark,
    /// \u{40}: '@'
    CommercialAt,
    /// \u{41}: 'A'
    LatinCapitalLetterA,
    /// \u{42}: 'B'
    LatinCapitalLetterB,
    /// \u{43}: 'C'
    LatinCapitalLetterC,
    /// \u{44}: 'D'
    LatinCapitalLetterD,
    /// \u{45}: 'E'
    LatinCapitalLetterE,
    /// \u{46}: 'F'
    LatinCapitalLetterF,
    /// \u{47}: 'G'
    LatinCapitalLetterG,
    /// \u{48}: 'H'
    LatinCapitalLetterH,
    /// \u{49}: 'I'
    LatinCapitalLetterI,
    /// \u{4a}: 'J'
    LatinCapitalLetterJ,
    /// \u{4b}: 'K'
    LatinCapitalLetterK,
    /// \u{4c}: 'L'
    LatinCapitalLetterL,
    /// \u{4d}: 'M'
    LatinCapitalLetterM,
    /// \u{4e}: 'N'
    LatinCapitalLetterN,
    /// \u{4f}: 'O'
    LatinCapitalLetterO,
    /// \u{50}: 'P'
    LatinCapitalLetterP,
    /// \u{51}: 'Q'
    LatinCapitalLetterQ,
    /// \u{52}: 'R'
    LatinCapitalLetterR,
    /// \u{53}: 'S'
    LatinCapitalLetterS,
    /// \u{54}: 'T'
    LatinCapitalLetterT,
    /// \u{55}: 'U'
    LatinCapitalLetterU,
    /// \u{56}: 'V'
    LatinCapitalLetterV,
    /// \u{57}: 'W'
    LatinCapitalLetterW,
    /// \u{58}: 'X'
    LatinCapitalLetterX,
    /// \u{59}: 'Y'
    LatinCapitalLetterY,
    /// \u{5a}: 'Z'
    LatinCapitalLetterZ,
    /// \u{5b}: '['
    LeftSquareBracket,
    /// \u{5c}: '\\'
    ReverseSolidus,
    /// \u{5d}: ']'
    RightSquareBracket,
    /// \u{5e}: '^'
    CircumflexAccent,
    /// \u{5f}: '_'
    LowLine,
    /// \u{60}: '`'
    GraveAccent,
    /// \u{61}: 'a'
    LatinSmallLetterA,
    /// \u{62}: 'b'
    LatinSmallLetterB,
    /// \u{63}: 'c'
    LatinSmallLetterC,
    /// \u{64}: 'd'
    LatinSmallLetterD,
    /// \u{65}: 'e'
    LatinSmallLetterE,
    /// \u{66}: 'f'
    LatinSmallLetterF,
    /// \u{67}: 'g'
    LatinSmallLetterG,
    /// \u{68}: 'h'
    LatinSmallLetterH,
    /// \u{69}: 'i'
    LatinSmallLetterI,
    /// \u{6a}: 'j'
    LatinSmallLetterJ,
    /// \u{6b}: 'k'
    LatinSmallLetterK,
    /// \u{6c}: 'l'
    LatinSmallLetterL,
    /// \u{6d}: 'm'
    LatinSmallLetterM,
    /// \u{6e}: 'n'
    LatinSmallLetterN,
    /// \u{6f}: 'o'
    LatinSmallLetterO,
    /// \u{70}: 'p'
    LatinSmallLetterP,
    /// \u{71}: 'q'
    LatinSmallLetterQ,
    /// \u{72}: 'r'
    LatinSmallLetterR,
    /// \u{73}: 's'
    LatinSmallLetterS,
    /// \u{74}: 't'
    LatinSmallLetterT,
    /// \u{75}: 'u'
    LatinSmallLetterU,
    /// \u{76}: 'v'
    LatinSmallLetterV,
    /// \u{77}: 'w'
    LatinSmallLetterW,
    /// \u{78}: 'x'
    LatinSmallLetterX,
    /// \u{79}: 'y'
    LatinSmallLetterY,
    /// \u{7a}: 'z'
    LatinSmallLetterZ,
    /// \u{7b}: '{'
    LeftCurlyBracket,
    /// \u{7c}: '|'
    VerticalLine,
    /// \u{7d}: '}'
    RightCurlyBracket,
    /// \u{7e}: '~'
    Tilde,
}

impl Into<char> for BasicLatin {
    fn into(self) -> char {
        match self {
            BasicLatin::Control0000 => ' ',
            BasicLatin::Control0001 => '',
            BasicLatin::Control0002 => '',
            BasicLatin::Control0003 => '',
            BasicLatin::Control0004 => '',
            BasicLatin::Control0005 => '',
            BasicLatin::Control0006 => '',
            BasicLatin::Control0007 => '',
            BasicLatin::Control0008 => '',
            BasicLatin::Control0009 => '\t',
            BasicLatin::Control000a => '\n',
            BasicLatin::Control000b => '',
            BasicLatin::Control000c => '',
            BasicLatin::Control000d => '\r',
            BasicLatin::Control000e => '',
            BasicLatin::Control000f => '',
            BasicLatin::Control0010 => '',
            BasicLatin::Control0011 => '',
            BasicLatin::Control0012 => '',
            BasicLatin::Control0013 => '',
            BasicLatin::Control0014 => '',
            BasicLatin::Control0015 => '',
            BasicLatin::Control0016 => '',
            BasicLatin::Control0017 => '',
            BasicLatin::Control0018 => '',
            BasicLatin::Control0019 => '',
            BasicLatin::Control001a => '',
            BasicLatin::Control001b => '',
            BasicLatin::Control001c => '',
            BasicLatin::Control001d => '',
            BasicLatin::Control001e => '',
            BasicLatin::Control001f => '',
            BasicLatin::Space => ' ',
            BasicLatin::ExclamationMark => '!',
            BasicLatin::QuotationMark => '"',
            BasicLatin::NumberSign => '#',
            BasicLatin::DollarSign => '$',
            BasicLatin::PercentSign => '%',
            BasicLatin::Ampersand => '&',
            BasicLatin::Apostrophe => '\'',
            BasicLatin::LeftParenthesis => '(',
            BasicLatin::RightParenthesis => ')',
            BasicLatin::Asterisk => '*',
            BasicLatin::PlusSign => '+',
            BasicLatin::Comma => ',',
            BasicLatin::HyphenDashMinus => '-',
            BasicLatin::FullStop => '.',
            BasicLatin::Solidus => '/',
            BasicLatin::DigitZero => '0',
            BasicLatin::DigitOne => '1',
            BasicLatin::DigitTwo => '2',
            BasicLatin::DigitThree => '3',
            BasicLatin::DigitFour => '4',
            BasicLatin::DigitFive => '5',
            BasicLatin::DigitSix => '6',
            BasicLatin::DigitSeven => '7',
            BasicLatin::DigitEight => '8',
            BasicLatin::DigitNine => '9',
            BasicLatin::Colon => ':',
            BasicLatin::Semicolon => ';',
            BasicLatin::LessDashThanSign => '<',
            BasicLatin::EqualsSign => '=',
            BasicLatin::GreaterDashThanSign => '>',
            BasicLatin::QuestionMark => '?',
            BasicLatin::CommercialAt => '@',
            BasicLatin::LatinCapitalLetterA => 'A',
            BasicLatin::LatinCapitalLetterB => 'B',
            BasicLatin::LatinCapitalLetterC => 'C',
            BasicLatin::LatinCapitalLetterD => 'D',
            BasicLatin::LatinCapitalLetterE => 'E',
            BasicLatin::LatinCapitalLetterF => 'F',
            BasicLatin::LatinCapitalLetterG => 'G',
            BasicLatin::LatinCapitalLetterH => 'H',
            BasicLatin::LatinCapitalLetterI => 'I',
            BasicLatin::LatinCapitalLetterJ => 'J',
            BasicLatin::LatinCapitalLetterK => 'K',
            BasicLatin::LatinCapitalLetterL => 'L',
            BasicLatin::LatinCapitalLetterM => 'M',
            BasicLatin::LatinCapitalLetterN => 'N',
            BasicLatin::LatinCapitalLetterO => 'O',
            BasicLatin::LatinCapitalLetterP => 'P',
            BasicLatin::LatinCapitalLetterQ => 'Q',
            BasicLatin::LatinCapitalLetterR => 'R',
            BasicLatin::LatinCapitalLetterS => 'S',
            BasicLatin::LatinCapitalLetterT => 'T',
            BasicLatin::LatinCapitalLetterU => 'U',
            BasicLatin::LatinCapitalLetterV => 'V',
            BasicLatin::LatinCapitalLetterW => 'W',
            BasicLatin::LatinCapitalLetterX => 'X',
            BasicLatin::LatinCapitalLetterY => 'Y',
            BasicLatin::LatinCapitalLetterZ => 'Z',
            BasicLatin::LeftSquareBracket => '[',
            BasicLatin::ReverseSolidus => '\\',
            BasicLatin::RightSquareBracket => ']',
            BasicLatin::CircumflexAccent => '^',
            BasicLatin::LowLine => '_',
            BasicLatin::GraveAccent => '`',
            BasicLatin::LatinSmallLetterA => 'a',
            BasicLatin::LatinSmallLetterB => 'b',
            BasicLatin::LatinSmallLetterC => 'c',
            BasicLatin::LatinSmallLetterD => 'd',
            BasicLatin::LatinSmallLetterE => 'e',
            BasicLatin::LatinSmallLetterF => 'f',
            BasicLatin::LatinSmallLetterG => 'g',
            BasicLatin::LatinSmallLetterH => 'h',
            BasicLatin::LatinSmallLetterI => 'i',
            BasicLatin::LatinSmallLetterJ => 'j',
            BasicLatin::LatinSmallLetterK => 'k',
            BasicLatin::LatinSmallLetterL => 'l',
            BasicLatin::LatinSmallLetterM => 'm',
            BasicLatin::LatinSmallLetterN => 'n',
            BasicLatin::LatinSmallLetterO => 'o',
            BasicLatin::LatinSmallLetterP => 'p',
            BasicLatin::LatinSmallLetterQ => 'q',
            BasicLatin::LatinSmallLetterR => 'r',
            BasicLatin::LatinSmallLetterS => 's',
            BasicLatin::LatinSmallLetterT => 't',
            BasicLatin::LatinSmallLetterU => 'u',
            BasicLatin::LatinSmallLetterV => 'v',
            BasicLatin::LatinSmallLetterW => 'w',
            BasicLatin::LatinSmallLetterX => 'x',
            BasicLatin::LatinSmallLetterY => 'y',
            BasicLatin::LatinSmallLetterZ => 'z',
            BasicLatin::LeftCurlyBracket => '{',
            BasicLatin::VerticalLine => '|',
            BasicLatin::RightCurlyBracket => '}',
            BasicLatin::Tilde => '~',
        }
    }
}

impl std::convert::TryFrom<char> for BasicLatin {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(BasicLatin::Control0000),
            '' => Ok(BasicLatin::Control0001),
            '' => Ok(BasicLatin::Control0002),
            '' => Ok(BasicLatin::Control0003),
            '' => Ok(BasicLatin::Control0004),
            '' => Ok(BasicLatin::Control0005),
            '' => Ok(BasicLatin::Control0006),
            '' => Ok(BasicLatin::Control0007),
            '' => Ok(BasicLatin::Control0008),
            '\t' => Ok(BasicLatin::Control0009),
            '\n' => Ok(BasicLatin::Control000a),
            '' => Ok(BasicLatin::Control000b),
            '' => Ok(BasicLatin::Control000c),
            '\r' => Ok(BasicLatin::Control000d),
            '' => Ok(BasicLatin::Control000e),
            '' => Ok(BasicLatin::Control000f),
            '' => Ok(BasicLatin::Control0010),
            '' => Ok(BasicLatin::Control0011),
            '' => Ok(BasicLatin::Control0012),
            '' => Ok(BasicLatin::Control0013),
            '' => Ok(BasicLatin::Control0014),
            '' => Ok(BasicLatin::Control0015),
            '' => Ok(BasicLatin::Control0016),
            '' => Ok(BasicLatin::Control0017),
            '' => Ok(BasicLatin::Control0018),
            '' => Ok(BasicLatin::Control0019),
            '' => Ok(BasicLatin::Control001a),
            '' => Ok(BasicLatin::Control001b),
            '' => Ok(BasicLatin::Control001c),
            '' => Ok(BasicLatin::Control001d),
            '' => Ok(BasicLatin::Control001e),
            '' => Ok(BasicLatin::Control001f),
            ' ' => Ok(BasicLatin::Space),
            '!' => Ok(BasicLatin::ExclamationMark),
            '"' => Ok(BasicLatin::QuotationMark),
            '#' => Ok(BasicLatin::NumberSign),
            '$' => Ok(BasicLatin::DollarSign),
            '%' => Ok(BasicLatin::PercentSign),
            '&' => Ok(BasicLatin::Ampersand),
            '\'' => Ok(BasicLatin::Apostrophe),
            '(' => Ok(BasicLatin::LeftParenthesis),
            ')' => Ok(BasicLatin::RightParenthesis),
            '*' => Ok(BasicLatin::Asterisk),
            '+' => Ok(BasicLatin::PlusSign),
            ',' => Ok(BasicLatin::Comma),
            '-' => Ok(BasicLatin::HyphenDashMinus),
            '.' => Ok(BasicLatin::FullStop),
            '/' => Ok(BasicLatin::Solidus),
            '0' => Ok(BasicLatin::DigitZero),
            '1' => Ok(BasicLatin::DigitOne),
            '2' => Ok(BasicLatin::DigitTwo),
            '3' => Ok(BasicLatin::DigitThree),
            '4' => Ok(BasicLatin::DigitFour),
            '5' => Ok(BasicLatin::DigitFive),
            '6' => Ok(BasicLatin::DigitSix),
            '7' => Ok(BasicLatin::DigitSeven),
            '8' => Ok(BasicLatin::DigitEight),
            '9' => Ok(BasicLatin::DigitNine),
            ':' => Ok(BasicLatin::Colon),
            ';' => Ok(BasicLatin::Semicolon),
            '<' => Ok(BasicLatin::LessDashThanSign),
            '=' => Ok(BasicLatin::EqualsSign),
            '>' => Ok(BasicLatin::GreaterDashThanSign),
            '?' => Ok(BasicLatin::QuestionMark),
            '@' => Ok(BasicLatin::CommercialAt),
            'A' => Ok(BasicLatin::LatinCapitalLetterA),
            'B' => Ok(BasicLatin::LatinCapitalLetterB),
            'C' => Ok(BasicLatin::LatinCapitalLetterC),
            'D' => Ok(BasicLatin::LatinCapitalLetterD),
            'E' => Ok(BasicLatin::LatinCapitalLetterE),
            'F' => Ok(BasicLatin::LatinCapitalLetterF),
            'G' => Ok(BasicLatin::LatinCapitalLetterG),
            'H' => Ok(BasicLatin::LatinCapitalLetterH),
            'I' => Ok(BasicLatin::LatinCapitalLetterI),
            'J' => Ok(BasicLatin::LatinCapitalLetterJ),
            'K' => Ok(BasicLatin::LatinCapitalLetterK),
            'L' => Ok(BasicLatin::LatinCapitalLetterL),
            'M' => Ok(BasicLatin::LatinCapitalLetterM),
            'N' => Ok(BasicLatin::LatinCapitalLetterN),
            'O' => Ok(BasicLatin::LatinCapitalLetterO),
            'P' => Ok(BasicLatin::LatinCapitalLetterP),
            'Q' => Ok(BasicLatin::LatinCapitalLetterQ),
            'R' => Ok(BasicLatin::LatinCapitalLetterR),
            'S' => Ok(BasicLatin::LatinCapitalLetterS),
            'T' => Ok(BasicLatin::LatinCapitalLetterT),
            'U' => Ok(BasicLatin::LatinCapitalLetterU),
            'V' => Ok(BasicLatin::LatinCapitalLetterV),
            'W' => Ok(BasicLatin::LatinCapitalLetterW),
            'X' => Ok(BasicLatin::LatinCapitalLetterX),
            'Y' => Ok(BasicLatin::LatinCapitalLetterY),
            'Z' => Ok(BasicLatin::LatinCapitalLetterZ),
            '[' => Ok(BasicLatin::LeftSquareBracket),
            '\\' => Ok(BasicLatin::ReverseSolidus),
            ']' => Ok(BasicLatin::RightSquareBracket),
            '^' => Ok(BasicLatin::CircumflexAccent),
            '_' => Ok(BasicLatin::LowLine),
            '`' => Ok(BasicLatin::GraveAccent),
            'a' => Ok(BasicLatin::LatinSmallLetterA),
            'b' => Ok(BasicLatin::LatinSmallLetterB),
            'c' => Ok(BasicLatin::LatinSmallLetterC),
            'd' => Ok(BasicLatin::LatinSmallLetterD),
            'e' => Ok(BasicLatin::LatinSmallLetterE),
            'f' => Ok(BasicLatin::LatinSmallLetterF),
            'g' => Ok(BasicLatin::LatinSmallLetterG),
            'h' => Ok(BasicLatin::LatinSmallLetterH),
            'i' => Ok(BasicLatin::LatinSmallLetterI),
            'j' => Ok(BasicLatin::LatinSmallLetterJ),
            'k' => Ok(BasicLatin::LatinSmallLetterK),
            'l' => Ok(BasicLatin::LatinSmallLetterL),
            'm' => Ok(BasicLatin::LatinSmallLetterM),
            'n' => Ok(BasicLatin::LatinSmallLetterN),
            'o' => Ok(BasicLatin::LatinSmallLetterO),
            'p' => Ok(BasicLatin::LatinSmallLetterP),
            'q' => Ok(BasicLatin::LatinSmallLetterQ),
            'r' => Ok(BasicLatin::LatinSmallLetterR),
            's' => Ok(BasicLatin::LatinSmallLetterS),
            't' => Ok(BasicLatin::LatinSmallLetterT),
            'u' => Ok(BasicLatin::LatinSmallLetterU),
            'v' => Ok(BasicLatin::LatinSmallLetterV),
            'w' => Ok(BasicLatin::LatinSmallLetterW),
            'x' => Ok(BasicLatin::LatinSmallLetterX),
            'y' => Ok(BasicLatin::LatinSmallLetterY),
            'z' => Ok(BasicLatin::LatinSmallLetterZ),
            '{' => Ok(BasicLatin::LeftCurlyBracket),
            '|' => Ok(BasicLatin::VerticalLine),
            '}' => Ok(BasicLatin::RightCurlyBracket),
            '~' => Ok(BasicLatin::Tilde),
            _ => Err(()),
        }
    }
}

impl Into<u32> for BasicLatin {
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

impl std::convert::TryFrom<u32> for BasicLatin {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for BasicLatin {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl BasicLatin {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        BasicLatin::Control0000
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            BasicLatin::Control0000 => "control 0000",
            BasicLatin::Control0001 => "control 0001",
            BasicLatin::Control0002 => "control 0002",
            BasicLatin::Control0003 => "control 0003",
            BasicLatin::Control0004 => "control 0004",
            BasicLatin::Control0005 => "control 0005",
            BasicLatin::Control0006 => "control 0006",
            BasicLatin::Control0007 => "control 0007",
            BasicLatin::Control0008 => "control 0008",
            BasicLatin::Control0009 => "control 0009",
            BasicLatin::Control000a => "control 000a",
            BasicLatin::Control000b => "control 000b",
            BasicLatin::Control000c => "control 000c",
            BasicLatin::Control000d => "control 000d",
            BasicLatin::Control000e => "control 000e",
            BasicLatin::Control000f => "control 000f",
            BasicLatin::Control0010 => "control 0010",
            BasicLatin::Control0011 => "control 0011",
            BasicLatin::Control0012 => "control 0012",
            BasicLatin::Control0013 => "control 0013",
            BasicLatin::Control0014 => "control 0014",
            BasicLatin::Control0015 => "control 0015",
            BasicLatin::Control0016 => "control 0016",
            BasicLatin::Control0017 => "control 0017",
            BasicLatin::Control0018 => "control 0018",
            BasicLatin::Control0019 => "control 0019",
            BasicLatin::Control001a => "control 001a",
            BasicLatin::Control001b => "control 001b",
            BasicLatin::Control001c => "control 001c",
            BasicLatin::Control001d => "control 001d",
            BasicLatin::Control001e => "control 001e",
            BasicLatin::Control001f => "control 001f",
            BasicLatin::Space => "space",
            BasicLatin::ExclamationMark => "exclamation mark",
            BasicLatin::QuotationMark => "quotation mark",
            BasicLatin::NumberSign => "number sign",
            BasicLatin::DollarSign => "dollar sign",
            BasicLatin::PercentSign => "percent sign",
            BasicLatin::Ampersand => "ampersand",
            BasicLatin::Apostrophe => "apostrophe",
            BasicLatin::LeftParenthesis => "left parenthesis",
            BasicLatin::RightParenthesis => "right parenthesis",
            BasicLatin::Asterisk => "asterisk",
            BasicLatin::PlusSign => "plus sign",
            BasicLatin::Comma => "comma",
            BasicLatin::HyphenDashMinus => "hyphen-minus",
            BasicLatin::FullStop => "full stop",
            BasicLatin::Solidus => "solidus",
            BasicLatin::DigitZero => "digit zero",
            BasicLatin::DigitOne => "digit one",
            BasicLatin::DigitTwo => "digit two",
            BasicLatin::DigitThree => "digit three",
            BasicLatin::DigitFour => "digit four",
            BasicLatin::DigitFive => "digit five",
            BasicLatin::DigitSix => "digit six",
            BasicLatin::DigitSeven => "digit seven",
            BasicLatin::DigitEight => "digit eight",
            BasicLatin::DigitNine => "digit nine",
            BasicLatin::Colon => "colon",
            BasicLatin::Semicolon => "semicolon",
            BasicLatin::LessDashThanSign => "less-than sign",
            BasicLatin::EqualsSign => "equals sign",
            BasicLatin::GreaterDashThanSign => "greater-than sign",
            BasicLatin::QuestionMark => "question mark",
            BasicLatin::CommercialAt => "commercial at",
            BasicLatin::LatinCapitalLetterA => "latin capital letter a",
            BasicLatin::LatinCapitalLetterB => "latin capital letter b",
            BasicLatin::LatinCapitalLetterC => "latin capital letter c",
            BasicLatin::LatinCapitalLetterD => "latin capital letter d",
            BasicLatin::LatinCapitalLetterE => "latin capital letter e",
            BasicLatin::LatinCapitalLetterF => "latin capital letter f",
            BasicLatin::LatinCapitalLetterG => "latin capital letter g",
            BasicLatin::LatinCapitalLetterH => "latin capital letter h",
            BasicLatin::LatinCapitalLetterI => "latin capital letter i",
            BasicLatin::LatinCapitalLetterJ => "latin capital letter j",
            BasicLatin::LatinCapitalLetterK => "latin capital letter k",
            BasicLatin::LatinCapitalLetterL => "latin capital letter l",
            BasicLatin::LatinCapitalLetterM => "latin capital letter m",
            BasicLatin::LatinCapitalLetterN => "latin capital letter n",
            BasicLatin::LatinCapitalLetterO => "latin capital letter o",
            BasicLatin::LatinCapitalLetterP => "latin capital letter p",
            BasicLatin::LatinCapitalLetterQ => "latin capital letter q",
            BasicLatin::LatinCapitalLetterR => "latin capital letter r",
            BasicLatin::LatinCapitalLetterS => "latin capital letter s",
            BasicLatin::LatinCapitalLetterT => "latin capital letter t",
            BasicLatin::LatinCapitalLetterU => "latin capital letter u",
            BasicLatin::LatinCapitalLetterV => "latin capital letter v",
            BasicLatin::LatinCapitalLetterW => "latin capital letter w",
            BasicLatin::LatinCapitalLetterX => "latin capital letter x",
            BasicLatin::LatinCapitalLetterY => "latin capital letter y",
            BasicLatin::LatinCapitalLetterZ => "latin capital letter z",
            BasicLatin::LeftSquareBracket => "left square bracket",
            BasicLatin::ReverseSolidus => "reverse solidus",
            BasicLatin::RightSquareBracket => "right square bracket",
            BasicLatin::CircumflexAccent => "circumflex accent",
            BasicLatin::LowLine => "low line",
            BasicLatin::GraveAccent => "grave accent",
            BasicLatin::LatinSmallLetterA => "latin small letter a",
            BasicLatin::LatinSmallLetterB => "latin small letter b",
            BasicLatin::LatinSmallLetterC => "latin small letter c",
            BasicLatin::LatinSmallLetterD => "latin small letter d",
            BasicLatin::LatinSmallLetterE => "latin small letter e",
            BasicLatin::LatinSmallLetterF => "latin small letter f",
            BasicLatin::LatinSmallLetterG => "latin small letter g",
            BasicLatin::LatinSmallLetterH => "latin small letter h",
            BasicLatin::LatinSmallLetterI => "latin small letter i",
            BasicLatin::LatinSmallLetterJ => "latin small letter j",
            BasicLatin::LatinSmallLetterK => "latin small letter k",
            BasicLatin::LatinSmallLetterL => "latin small letter l",
            BasicLatin::LatinSmallLetterM => "latin small letter m",
            BasicLatin::LatinSmallLetterN => "latin small letter n",
            BasicLatin::LatinSmallLetterO => "latin small letter o",
            BasicLatin::LatinSmallLetterP => "latin small letter p",
            BasicLatin::LatinSmallLetterQ => "latin small letter q",
            BasicLatin::LatinSmallLetterR => "latin small letter r",
            BasicLatin::LatinSmallLetterS => "latin small letter s",
            BasicLatin::LatinSmallLetterT => "latin small letter t",
            BasicLatin::LatinSmallLetterU => "latin small letter u",
            BasicLatin::LatinSmallLetterV => "latin small letter v",
            BasicLatin::LatinSmallLetterW => "latin small letter w",
            BasicLatin::LatinSmallLetterX => "latin small letter x",
            BasicLatin::LatinSmallLetterY => "latin small letter y",
            BasicLatin::LatinSmallLetterZ => "latin small letter z",
            BasicLatin::LeftCurlyBracket => "left curly bracket",
            BasicLatin::VerticalLine => "vertical line",
            BasicLatin::RightCurlyBracket => "right curly bracket",
            BasicLatin::Tilde => "tilde",
        }
    }
}
