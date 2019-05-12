
/// An enum to represent all characters in the Coptic block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Coptic {
    /// \u{2c80}: 'Ⲁ'
    CapitalLetterAlfa,
    /// \u{2c81}: 'ⲁ'
    SmallLetterAlfa,
    /// \u{2c82}: 'Ⲃ'
    CapitalLetterVida,
    /// \u{2c83}: 'ⲃ'
    SmallLetterVida,
    /// \u{2c84}: 'Ⲅ'
    CapitalLetterGamma,
    /// \u{2c85}: 'ⲅ'
    SmallLetterGamma,
    /// \u{2c86}: 'Ⲇ'
    CapitalLetterDalda,
    /// \u{2c87}: 'ⲇ'
    SmallLetterDalda,
    /// \u{2c88}: 'Ⲉ'
    CapitalLetterEie,
    /// \u{2c89}: 'ⲉ'
    SmallLetterEie,
    /// \u{2c8a}: 'Ⲋ'
    CapitalLetterSou,
    /// \u{2c8b}: 'ⲋ'
    SmallLetterSou,
    /// \u{2c8c}: 'Ⲍ'
    CapitalLetterZata,
    /// \u{2c8d}: 'ⲍ'
    SmallLetterZata,
    /// \u{2c8e}: 'Ⲏ'
    CapitalLetterHate,
    /// \u{2c8f}: 'ⲏ'
    SmallLetterHate,
    /// \u{2c90}: 'Ⲑ'
    CapitalLetterThethe,
    /// \u{2c91}: 'ⲑ'
    SmallLetterThethe,
    /// \u{2c92}: 'Ⲓ'
    CapitalLetterIauda,
    /// \u{2c93}: 'ⲓ'
    SmallLetterIauda,
    /// \u{2c94}: 'Ⲕ'
    CapitalLetterKapa,
    /// \u{2c95}: 'ⲕ'
    SmallLetterKapa,
    /// \u{2c96}: 'Ⲗ'
    CapitalLetterLaula,
    /// \u{2c97}: 'ⲗ'
    SmallLetterLaula,
    /// \u{2c98}: 'Ⲙ'
    CapitalLetterMi,
    /// \u{2c99}: 'ⲙ'
    SmallLetterMi,
    /// \u{2c9a}: 'Ⲛ'
    CapitalLetterNi,
    /// \u{2c9b}: 'ⲛ'
    SmallLetterNi,
    /// \u{2c9c}: 'Ⲝ'
    CapitalLetterKsi,
    /// \u{2c9d}: 'ⲝ'
    SmallLetterKsi,
    /// \u{2c9e}: 'Ⲟ'
    CapitalLetterO,
    /// \u{2c9f}: 'ⲟ'
    SmallLetterO,
    /// \u{2ca0}: 'Ⲡ'
    CapitalLetterPi,
    /// \u{2ca1}: 'ⲡ'
    SmallLetterPi,
    /// \u{2ca2}: 'Ⲣ'
    CapitalLetterRo,
    /// \u{2ca3}: 'ⲣ'
    SmallLetterRo,
    /// \u{2ca4}: 'Ⲥ'
    CapitalLetterSima,
    /// \u{2ca5}: 'ⲥ'
    SmallLetterSima,
    /// \u{2ca6}: 'Ⲧ'
    CapitalLetterTau,
    /// \u{2ca7}: 'ⲧ'
    SmallLetterTau,
    /// \u{2ca8}: 'Ⲩ'
    CapitalLetterUa,
    /// \u{2ca9}: 'ⲩ'
    SmallLetterUa,
    /// \u{2caa}: 'Ⲫ'
    CapitalLetterFi,
    /// \u{2cab}: 'ⲫ'
    SmallLetterFi,
    /// \u{2cac}: 'Ⲭ'
    CapitalLetterKhi,
    /// \u{2cad}: 'ⲭ'
    SmallLetterKhi,
    /// \u{2cae}: 'Ⲯ'
    CapitalLetterPsi,
    /// \u{2caf}: 'ⲯ'
    SmallLetterPsi,
    /// \u{2cb0}: 'Ⲱ'
    CapitalLetterOou,
    /// \u{2cb1}: 'ⲱ'
    SmallLetterOou,
    /// \u{2cb2}: 'Ⲳ'
    CapitalLetterDialectDashPAlef,
    /// \u{2cb3}: 'ⲳ'
    SmallLetterDialectDashPAlef,
    /// \u{2cb4}: 'Ⲵ'
    CapitalLetterOldAin,
    /// \u{2cb5}: 'ⲵ'
    SmallLetterOldAin,
    /// \u{2cb6}: 'Ⲷ'
    CapitalLetterCryptogrammicEie,
    /// \u{2cb7}: 'ⲷ'
    SmallLetterCryptogrammicEie,
    /// \u{2cb8}: 'Ⲹ'
    CapitalLetterDialectDashPKapa,
    /// \u{2cb9}: 'ⲹ'
    SmallLetterDialectDashPKapa,
    /// \u{2cba}: 'Ⲻ'
    CapitalLetterDialectDashPNi,
    /// \u{2cbb}: 'ⲻ'
    SmallLetterDialectDashPNi,
    /// \u{2cbc}: 'Ⲽ'
    CapitalLetterCryptogrammicNi,
    /// \u{2cbd}: 'ⲽ'
    SmallLetterCryptogrammicNi,
    /// \u{2cbe}: 'Ⲿ'
    CapitalLetterOldOou,
    /// \u{2cbf}: 'ⲿ'
    SmallLetterOldOou,
    /// \u{2cc0}: 'Ⳁ'
    CapitalLetterSampi,
    /// \u{2cc1}: 'ⳁ'
    SmallLetterSampi,
    /// \u{2cc2}: 'Ⳃ'
    CapitalLetterCrossedShei,
    /// \u{2cc3}: 'ⳃ'
    SmallLetterCrossedShei,
    /// \u{2cc4}: 'Ⳅ'
    CapitalLetterOldShei,
    /// \u{2cc5}: 'ⳅ'
    SmallLetterOldShei,
    /// \u{2cc6}: 'Ⳇ'
    CapitalLetterOldEsh,
    /// \u{2cc7}: 'ⳇ'
    SmallLetterOldEsh,
    /// \u{2cc8}: 'Ⳉ'
    CapitalLetterAkhmimicKhei,
    /// \u{2cc9}: 'ⳉ'
    SmallLetterAkhmimicKhei,
    /// \u{2cca}: 'Ⳋ'
    CapitalLetterDialectDashPHori,
    /// \u{2ccb}: 'ⳋ'
    SmallLetterDialectDashPHori,
    /// \u{2ccc}: 'Ⳍ'
    CapitalLetterOldHori,
    /// \u{2ccd}: 'ⳍ'
    SmallLetterOldHori,
    /// \u{2cce}: 'Ⳏ'
    CapitalLetterOldHa,
    /// \u{2ccf}: 'ⳏ'
    SmallLetterOldHa,
    /// \u{2cd0}: 'Ⳑ'
    CapitalLetterLDashShapedHa,
    /// \u{2cd1}: 'ⳑ'
    SmallLetterLDashShapedHa,
    /// \u{2cd2}: 'Ⳓ'
    CapitalLetterOldHei,
    /// \u{2cd3}: 'ⳓ'
    SmallLetterOldHei,
    /// \u{2cd4}: 'Ⳕ'
    CapitalLetterOldHat,
    /// \u{2cd5}: 'ⳕ'
    SmallLetterOldHat,
    /// \u{2cd6}: 'Ⳗ'
    CapitalLetterOldGangia,
    /// \u{2cd7}: 'ⳗ'
    SmallLetterOldGangia,
    /// \u{2cd8}: 'Ⳙ'
    CapitalLetterOldDja,
    /// \u{2cd9}: 'ⳙ'
    SmallLetterOldDja,
    /// \u{2cda}: 'Ⳛ'
    CapitalLetterOldShima,
    /// \u{2cdb}: 'ⳛ'
    SmallLetterOldShima,
    /// \u{2cdc}: 'Ⳝ'
    CapitalLetterOldNubianShima,
    /// \u{2cdd}: 'ⳝ'
    SmallLetterOldNubianShima,
    /// \u{2cde}: 'Ⳟ'
    CapitalLetterOldNubianNgi,
    /// \u{2cdf}: 'ⳟ'
    SmallLetterOldNubianNgi,
    /// \u{2ce0}: 'Ⳡ'
    CapitalLetterOldNubianNyi,
    /// \u{2ce1}: 'ⳡ'
    SmallLetterOldNubianNyi,
    /// \u{2ce2}: 'Ⳣ'
    CapitalLetterOldNubianWau,
    /// \u{2ce3}: 'ⳣ'
    SmallLetterOldNubianWau,
    /// \u{2ce4}: 'ⳤ'
    SymbolKai,
    /// \u{2ce5}: '⳥'
    SymbolMiRo,
    /// \u{2ce6}: '⳦'
    SymbolPiRo,
    /// \u{2ce7}: '⳧'
    SymbolStauros,
    /// \u{2ce8}: '⳨'
    SymbolTauRo,
    /// \u{2ce9}: '⳩'
    SymbolKhiRo,
    /// \u{2cea}: '⳪'
    SymbolShimaSima,
    /// \u{2ceb}: 'Ⳬ'
    CapitalLetterCryptogrammicShei,
    /// \u{2cec}: 'ⳬ'
    SmallLetterCryptogrammicShei,
    /// \u{2ced}: 'Ⳮ'
    CapitalLetterCryptogrammicGangia,
    /// \u{2cee}: 'ⳮ'
    SmallLetterCryptogrammicGangia,
    /// \u{2cef}: '⳯'
    CombiningNiAbove,
    /// \u{2cf0}: '⳰'
    CombiningSpiritusAsper,
    /// \u{2cf1}: '⳱'
    CombiningSpiritusLenis,
    /// \u{2cf2}: 'Ⳳ'
    CapitalLetterBohairicKhei,
    /// \u{2cf3}: 'ⳳ'
    SmallLetterBohairicKhei,
    /// \u{2cf9}: '⳹'
    OldNubianFullStop,
    /// \u{2cfa}: '⳺'
    OldNubianDirectQuestionMark,
    /// \u{2cfb}: '⳻'
    OldNubianIndirectQuestionMark,
    /// \u{2cfc}: '⳼'
    OldNubianVerseDivider,
    /// \u{2cfd}: '⳽'
    FractionOneHalf,
    /// \u{2cfe}: '⳾'
    FullStop,
}

impl Into<char> for Coptic {
    fn into(self) -> char {
        match self {
            Coptic::CapitalLetterAlfa => 'Ⲁ',
            Coptic::SmallLetterAlfa => 'ⲁ',
            Coptic::CapitalLetterVida => 'Ⲃ',
            Coptic::SmallLetterVida => 'ⲃ',
            Coptic::CapitalLetterGamma => 'Ⲅ',
            Coptic::SmallLetterGamma => 'ⲅ',
            Coptic::CapitalLetterDalda => 'Ⲇ',
            Coptic::SmallLetterDalda => 'ⲇ',
            Coptic::CapitalLetterEie => 'Ⲉ',
            Coptic::SmallLetterEie => 'ⲉ',
            Coptic::CapitalLetterSou => 'Ⲋ',
            Coptic::SmallLetterSou => 'ⲋ',
            Coptic::CapitalLetterZata => 'Ⲍ',
            Coptic::SmallLetterZata => 'ⲍ',
            Coptic::CapitalLetterHate => 'Ⲏ',
            Coptic::SmallLetterHate => 'ⲏ',
            Coptic::CapitalLetterThethe => 'Ⲑ',
            Coptic::SmallLetterThethe => 'ⲑ',
            Coptic::CapitalLetterIauda => 'Ⲓ',
            Coptic::SmallLetterIauda => 'ⲓ',
            Coptic::CapitalLetterKapa => 'Ⲕ',
            Coptic::SmallLetterKapa => 'ⲕ',
            Coptic::CapitalLetterLaula => 'Ⲗ',
            Coptic::SmallLetterLaula => 'ⲗ',
            Coptic::CapitalLetterMi => 'Ⲙ',
            Coptic::SmallLetterMi => 'ⲙ',
            Coptic::CapitalLetterNi => 'Ⲛ',
            Coptic::SmallLetterNi => 'ⲛ',
            Coptic::CapitalLetterKsi => 'Ⲝ',
            Coptic::SmallLetterKsi => 'ⲝ',
            Coptic::CapitalLetterO => 'Ⲟ',
            Coptic::SmallLetterO => 'ⲟ',
            Coptic::CapitalLetterPi => 'Ⲡ',
            Coptic::SmallLetterPi => 'ⲡ',
            Coptic::CapitalLetterRo => 'Ⲣ',
            Coptic::SmallLetterRo => 'ⲣ',
            Coptic::CapitalLetterSima => 'Ⲥ',
            Coptic::SmallLetterSima => 'ⲥ',
            Coptic::CapitalLetterTau => 'Ⲧ',
            Coptic::SmallLetterTau => 'ⲧ',
            Coptic::CapitalLetterUa => 'Ⲩ',
            Coptic::SmallLetterUa => 'ⲩ',
            Coptic::CapitalLetterFi => 'Ⲫ',
            Coptic::SmallLetterFi => 'ⲫ',
            Coptic::CapitalLetterKhi => 'Ⲭ',
            Coptic::SmallLetterKhi => 'ⲭ',
            Coptic::CapitalLetterPsi => 'Ⲯ',
            Coptic::SmallLetterPsi => 'ⲯ',
            Coptic::CapitalLetterOou => 'Ⲱ',
            Coptic::SmallLetterOou => 'ⲱ',
            Coptic::CapitalLetterDialectDashPAlef => 'Ⲳ',
            Coptic::SmallLetterDialectDashPAlef => 'ⲳ',
            Coptic::CapitalLetterOldAin => 'Ⲵ',
            Coptic::SmallLetterOldAin => 'ⲵ',
            Coptic::CapitalLetterCryptogrammicEie => 'Ⲷ',
            Coptic::SmallLetterCryptogrammicEie => 'ⲷ',
            Coptic::CapitalLetterDialectDashPKapa => 'Ⲹ',
            Coptic::SmallLetterDialectDashPKapa => 'ⲹ',
            Coptic::CapitalLetterDialectDashPNi => 'Ⲻ',
            Coptic::SmallLetterDialectDashPNi => 'ⲻ',
            Coptic::CapitalLetterCryptogrammicNi => 'Ⲽ',
            Coptic::SmallLetterCryptogrammicNi => 'ⲽ',
            Coptic::CapitalLetterOldOou => 'Ⲿ',
            Coptic::SmallLetterOldOou => 'ⲿ',
            Coptic::CapitalLetterSampi => 'Ⳁ',
            Coptic::SmallLetterSampi => 'ⳁ',
            Coptic::CapitalLetterCrossedShei => 'Ⳃ',
            Coptic::SmallLetterCrossedShei => 'ⳃ',
            Coptic::CapitalLetterOldShei => 'Ⳅ',
            Coptic::SmallLetterOldShei => 'ⳅ',
            Coptic::CapitalLetterOldEsh => 'Ⳇ',
            Coptic::SmallLetterOldEsh => 'ⳇ',
            Coptic::CapitalLetterAkhmimicKhei => 'Ⳉ',
            Coptic::SmallLetterAkhmimicKhei => 'ⳉ',
            Coptic::CapitalLetterDialectDashPHori => 'Ⳋ',
            Coptic::SmallLetterDialectDashPHori => 'ⳋ',
            Coptic::CapitalLetterOldHori => 'Ⳍ',
            Coptic::SmallLetterOldHori => 'ⳍ',
            Coptic::CapitalLetterOldHa => 'Ⳏ',
            Coptic::SmallLetterOldHa => 'ⳏ',
            Coptic::CapitalLetterLDashShapedHa => 'Ⳑ',
            Coptic::SmallLetterLDashShapedHa => 'ⳑ',
            Coptic::CapitalLetterOldHei => 'Ⳓ',
            Coptic::SmallLetterOldHei => 'ⳓ',
            Coptic::CapitalLetterOldHat => 'Ⳕ',
            Coptic::SmallLetterOldHat => 'ⳕ',
            Coptic::CapitalLetterOldGangia => 'Ⳗ',
            Coptic::SmallLetterOldGangia => 'ⳗ',
            Coptic::CapitalLetterOldDja => 'Ⳙ',
            Coptic::SmallLetterOldDja => 'ⳙ',
            Coptic::CapitalLetterOldShima => 'Ⳛ',
            Coptic::SmallLetterOldShima => 'ⳛ',
            Coptic::CapitalLetterOldNubianShima => 'Ⳝ',
            Coptic::SmallLetterOldNubianShima => 'ⳝ',
            Coptic::CapitalLetterOldNubianNgi => 'Ⳟ',
            Coptic::SmallLetterOldNubianNgi => 'ⳟ',
            Coptic::CapitalLetterOldNubianNyi => 'Ⳡ',
            Coptic::SmallLetterOldNubianNyi => 'ⳡ',
            Coptic::CapitalLetterOldNubianWau => 'Ⳣ',
            Coptic::SmallLetterOldNubianWau => 'ⳣ',
            Coptic::SymbolKai => 'ⳤ',
            Coptic::SymbolMiRo => '⳥',
            Coptic::SymbolPiRo => '⳦',
            Coptic::SymbolStauros => '⳧',
            Coptic::SymbolTauRo => '⳨',
            Coptic::SymbolKhiRo => '⳩',
            Coptic::SymbolShimaSima => '⳪',
            Coptic::CapitalLetterCryptogrammicShei => 'Ⳬ',
            Coptic::SmallLetterCryptogrammicShei => 'ⳬ',
            Coptic::CapitalLetterCryptogrammicGangia => 'Ⳮ',
            Coptic::SmallLetterCryptogrammicGangia => 'ⳮ',
            Coptic::CombiningNiAbove => '⳯',
            Coptic::CombiningSpiritusAsper => '⳰',
            Coptic::CombiningSpiritusLenis => '⳱',
            Coptic::CapitalLetterBohairicKhei => 'Ⳳ',
            Coptic::SmallLetterBohairicKhei => 'ⳳ',
            Coptic::OldNubianFullStop => '⳹',
            Coptic::OldNubianDirectQuestionMark => '⳺',
            Coptic::OldNubianIndirectQuestionMark => '⳻',
            Coptic::OldNubianVerseDivider => '⳼',
            Coptic::FractionOneHalf => '⳽',
            Coptic::FullStop => '⳾',
        }
    }
}

impl std::convert::TryFrom<char> for Coptic {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'Ⲁ' => Ok(Coptic::CapitalLetterAlfa),
            'ⲁ' => Ok(Coptic::SmallLetterAlfa),
            'Ⲃ' => Ok(Coptic::CapitalLetterVida),
            'ⲃ' => Ok(Coptic::SmallLetterVida),
            'Ⲅ' => Ok(Coptic::CapitalLetterGamma),
            'ⲅ' => Ok(Coptic::SmallLetterGamma),
            'Ⲇ' => Ok(Coptic::CapitalLetterDalda),
            'ⲇ' => Ok(Coptic::SmallLetterDalda),
            'Ⲉ' => Ok(Coptic::CapitalLetterEie),
            'ⲉ' => Ok(Coptic::SmallLetterEie),
            'Ⲋ' => Ok(Coptic::CapitalLetterSou),
            'ⲋ' => Ok(Coptic::SmallLetterSou),
            'Ⲍ' => Ok(Coptic::CapitalLetterZata),
            'ⲍ' => Ok(Coptic::SmallLetterZata),
            'Ⲏ' => Ok(Coptic::CapitalLetterHate),
            'ⲏ' => Ok(Coptic::SmallLetterHate),
            'Ⲑ' => Ok(Coptic::CapitalLetterThethe),
            'ⲑ' => Ok(Coptic::SmallLetterThethe),
            'Ⲓ' => Ok(Coptic::CapitalLetterIauda),
            'ⲓ' => Ok(Coptic::SmallLetterIauda),
            'Ⲕ' => Ok(Coptic::CapitalLetterKapa),
            'ⲕ' => Ok(Coptic::SmallLetterKapa),
            'Ⲗ' => Ok(Coptic::CapitalLetterLaula),
            'ⲗ' => Ok(Coptic::SmallLetterLaula),
            'Ⲙ' => Ok(Coptic::CapitalLetterMi),
            'ⲙ' => Ok(Coptic::SmallLetterMi),
            'Ⲛ' => Ok(Coptic::CapitalLetterNi),
            'ⲛ' => Ok(Coptic::SmallLetterNi),
            'Ⲝ' => Ok(Coptic::CapitalLetterKsi),
            'ⲝ' => Ok(Coptic::SmallLetterKsi),
            'Ⲟ' => Ok(Coptic::CapitalLetterO),
            'ⲟ' => Ok(Coptic::SmallLetterO),
            'Ⲡ' => Ok(Coptic::CapitalLetterPi),
            'ⲡ' => Ok(Coptic::SmallLetterPi),
            'Ⲣ' => Ok(Coptic::CapitalLetterRo),
            'ⲣ' => Ok(Coptic::SmallLetterRo),
            'Ⲥ' => Ok(Coptic::CapitalLetterSima),
            'ⲥ' => Ok(Coptic::SmallLetterSima),
            'Ⲧ' => Ok(Coptic::CapitalLetterTau),
            'ⲧ' => Ok(Coptic::SmallLetterTau),
            'Ⲩ' => Ok(Coptic::CapitalLetterUa),
            'ⲩ' => Ok(Coptic::SmallLetterUa),
            'Ⲫ' => Ok(Coptic::CapitalLetterFi),
            'ⲫ' => Ok(Coptic::SmallLetterFi),
            'Ⲭ' => Ok(Coptic::CapitalLetterKhi),
            'ⲭ' => Ok(Coptic::SmallLetterKhi),
            'Ⲯ' => Ok(Coptic::CapitalLetterPsi),
            'ⲯ' => Ok(Coptic::SmallLetterPsi),
            'Ⲱ' => Ok(Coptic::CapitalLetterOou),
            'ⲱ' => Ok(Coptic::SmallLetterOou),
            'Ⲳ' => Ok(Coptic::CapitalLetterDialectDashPAlef),
            'ⲳ' => Ok(Coptic::SmallLetterDialectDashPAlef),
            'Ⲵ' => Ok(Coptic::CapitalLetterOldAin),
            'ⲵ' => Ok(Coptic::SmallLetterOldAin),
            'Ⲷ' => Ok(Coptic::CapitalLetterCryptogrammicEie),
            'ⲷ' => Ok(Coptic::SmallLetterCryptogrammicEie),
            'Ⲹ' => Ok(Coptic::CapitalLetterDialectDashPKapa),
            'ⲹ' => Ok(Coptic::SmallLetterDialectDashPKapa),
            'Ⲻ' => Ok(Coptic::CapitalLetterDialectDashPNi),
            'ⲻ' => Ok(Coptic::SmallLetterDialectDashPNi),
            'Ⲽ' => Ok(Coptic::CapitalLetterCryptogrammicNi),
            'ⲽ' => Ok(Coptic::SmallLetterCryptogrammicNi),
            'Ⲿ' => Ok(Coptic::CapitalLetterOldOou),
            'ⲿ' => Ok(Coptic::SmallLetterOldOou),
            'Ⳁ' => Ok(Coptic::CapitalLetterSampi),
            'ⳁ' => Ok(Coptic::SmallLetterSampi),
            'Ⳃ' => Ok(Coptic::CapitalLetterCrossedShei),
            'ⳃ' => Ok(Coptic::SmallLetterCrossedShei),
            'Ⳅ' => Ok(Coptic::CapitalLetterOldShei),
            'ⳅ' => Ok(Coptic::SmallLetterOldShei),
            'Ⳇ' => Ok(Coptic::CapitalLetterOldEsh),
            'ⳇ' => Ok(Coptic::SmallLetterOldEsh),
            'Ⳉ' => Ok(Coptic::CapitalLetterAkhmimicKhei),
            'ⳉ' => Ok(Coptic::SmallLetterAkhmimicKhei),
            'Ⳋ' => Ok(Coptic::CapitalLetterDialectDashPHori),
            'ⳋ' => Ok(Coptic::SmallLetterDialectDashPHori),
            'Ⳍ' => Ok(Coptic::CapitalLetterOldHori),
            'ⳍ' => Ok(Coptic::SmallLetterOldHori),
            'Ⳏ' => Ok(Coptic::CapitalLetterOldHa),
            'ⳏ' => Ok(Coptic::SmallLetterOldHa),
            'Ⳑ' => Ok(Coptic::CapitalLetterLDashShapedHa),
            'ⳑ' => Ok(Coptic::SmallLetterLDashShapedHa),
            'Ⳓ' => Ok(Coptic::CapitalLetterOldHei),
            'ⳓ' => Ok(Coptic::SmallLetterOldHei),
            'Ⳕ' => Ok(Coptic::CapitalLetterOldHat),
            'ⳕ' => Ok(Coptic::SmallLetterOldHat),
            'Ⳗ' => Ok(Coptic::CapitalLetterOldGangia),
            'ⳗ' => Ok(Coptic::SmallLetterOldGangia),
            'Ⳙ' => Ok(Coptic::CapitalLetterOldDja),
            'ⳙ' => Ok(Coptic::SmallLetterOldDja),
            'Ⳛ' => Ok(Coptic::CapitalLetterOldShima),
            'ⳛ' => Ok(Coptic::SmallLetterOldShima),
            'Ⳝ' => Ok(Coptic::CapitalLetterOldNubianShima),
            'ⳝ' => Ok(Coptic::SmallLetterOldNubianShima),
            'Ⳟ' => Ok(Coptic::CapitalLetterOldNubianNgi),
            'ⳟ' => Ok(Coptic::SmallLetterOldNubianNgi),
            'Ⳡ' => Ok(Coptic::CapitalLetterOldNubianNyi),
            'ⳡ' => Ok(Coptic::SmallLetterOldNubianNyi),
            'Ⳣ' => Ok(Coptic::CapitalLetterOldNubianWau),
            'ⳣ' => Ok(Coptic::SmallLetterOldNubianWau),
            'ⳤ' => Ok(Coptic::SymbolKai),
            '⳥' => Ok(Coptic::SymbolMiRo),
            '⳦' => Ok(Coptic::SymbolPiRo),
            '⳧' => Ok(Coptic::SymbolStauros),
            '⳨' => Ok(Coptic::SymbolTauRo),
            '⳩' => Ok(Coptic::SymbolKhiRo),
            '⳪' => Ok(Coptic::SymbolShimaSima),
            'Ⳬ' => Ok(Coptic::CapitalLetterCryptogrammicShei),
            'ⳬ' => Ok(Coptic::SmallLetterCryptogrammicShei),
            'Ⳮ' => Ok(Coptic::CapitalLetterCryptogrammicGangia),
            'ⳮ' => Ok(Coptic::SmallLetterCryptogrammicGangia),
            '⳯' => Ok(Coptic::CombiningNiAbove),
            '⳰' => Ok(Coptic::CombiningSpiritusAsper),
            '⳱' => Ok(Coptic::CombiningSpiritusLenis),
            'Ⳳ' => Ok(Coptic::CapitalLetterBohairicKhei),
            'ⳳ' => Ok(Coptic::SmallLetterBohairicKhei),
            '⳹' => Ok(Coptic::OldNubianFullStop),
            '⳺' => Ok(Coptic::OldNubianDirectQuestionMark),
            '⳻' => Ok(Coptic::OldNubianIndirectQuestionMark),
            '⳼' => Ok(Coptic::OldNubianVerseDivider),
            '⳽' => Ok(Coptic::FractionOneHalf),
            '⳾' => Ok(Coptic::FullStop),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Coptic {
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

impl std::convert::TryFrom<u32> for Coptic {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Coptic {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Coptic {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Coptic::CapitalLetterAlfa
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            Coptic::CapitalLetterAlfa => "coptic capital letter alfa",
            Coptic::SmallLetterAlfa => "coptic small letter alfa",
            Coptic::CapitalLetterVida => "coptic capital letter vida",
            Coptic::SmallLetterVida => "coptic small letter vida",
            Coptic::CapitalLetterGamma => "coptic capital letter gamma",
            Coptic::SmallLetterGamma => "coptic small letter gamma",
            Coptic::CapitalLetterDalda => "coptic capital letter dalda",
            Coptic::SmallLetterDalda => "coptic small letter dalda",
            Coptic::CapitalLetterEie => "coptic capital letter eie",
            Coptic::SmallLetterEie => "coptic small letter eie",
            Coptic::CapitalLetterSou => "coptic capital letter sou",
            Coptic::SmallLetterSou => "coptic small letter sou",
            Coptic::CapitalLetterZata => "coptic capital letter zata",
            Coptic::SmallLetterZata => "coptic small letter zata",
            Coptic::CapitalLetterHate => "coptic capital letter hate",
            Coptic::SmallLetterHate => "coptic small letter hate",
            Coptic::CapitalLetterThethe => "coptic capital letter thethe",
            Coptic::SmallLetterThethe => "coptic small letter thethe",
            Coptic::CapitalLetterIauda => "coptic capital letter iauda",
            Coptic::SmallLetterIauda => "coptic small letter iauda",
            Coptic::CapitalLetterKapa => "coptic capital letter kapa",
            Coptic::SmallLetterKapa => "coptic small letter kapa",
            Coptic::CapitalLetterLaula => "coptic capital letter laula",
            Coptic::SmallLetterLaula => "coptic small letter laula",
            Coptic::CapitalLetterMi => "coptic capital letter mi",
            Coptic::SmallLetterMi => "coptic small letter mi",
            Coptic::CapitalLetterNi => "coptic capital letter ni",
            Coptic::SmallLetterNi => "coptic small letter ni",
            Coptic::CapitalLetterKsi => "coptic capital letter ksi",
            Coptic::SmallLetterKsi => "coptic small letter ksi",
            Coptic::CapitalLetterO => "coptic capital letter o",
            Coptic::SmallLetterO => "coptic small letter o",
            Coptic::CapitalLetterPi => "coptic capital letter pi",
            Coptic::SmallLetterPi => "coptic small letter pi",
            Coptic::CapitalLetterRo => "coptic capital letter ro",
            Coptic::SmallLetterRo => "coptic small letter ro",
            Coptic::CapitalLetterSima => "coptic capital letter sima",
            Coptic::SmallLetterSima => "coptic small letter sima",
            Coptic::CapitalLetterTau => "coptic capital letter tau",
            Coptic::SmallLetterTau => "coptic small letter tau",
            Coptic::CapitalLetterUa => "coptic capital letter ua",
            Coptic::SmallLetterUa => "coptic small letter ua",
            Coptic::CapitalLetterFi => "coptic capital letter fi",
            Coptic::SmallLetterFi => "coptic small letter fi",
            Coptic::CapitalLetterKhi => "coptic capital letter khi",
            Coptic::SmallLetterKhi => "coptic small letter khi",
            Coptic::CapitalLetterPsi => "coptic capital letter psi",
            Coptic::SmallLetterPsi => "coptic small letter psi",
            Coptic::CapitalLetterOou => "coptic capital letter oou",
            Coptic::SmallLetterOou => "coptic small letter oou",
            Coptic::CapitalLetterDialectDashPAlef => "coptic capital letter dialect-p alef",
            Coptic::SmallLetterDialectDashPAlef => "coptic small letter dialect-p alef",
            Coptic::CapitalLetterOldAin => "coptic capital letter old coptic ain",
            Coptic::SmallLetterOldAin => "coptic small letter old coptic ain",
            Coptic::CapitalLetterCryptogrammicEie => "coptic capital letter cryptogrammic eie",
            Coptic::SmallLetterCryptogrammicEie => "coptic small letter cryptogrammic eie",
            Coptic::CapitalLetterDialectDashPKapa => "coptic capital letter dialect-p kapa",
            Coptic::SmallLetterDialectDashPKapa => "coptic small letter dialect-p kapa",
            Coptic::CapitalLetterDialectDashPNi => "coptic capital letter dialect-p ni",
            Coptic::SmallLetterDialectDashPNi => "coptic small letter dialect-p ni",
            Coptic::CapitalLetterCryptogrammicNi => "coptic capital letter cryptogrammic ni",
            Coptic::SmallLetterCryptogrammicNi => "coptic small letter cryptogrammic ni",
            Coptic::CapitalLetterOldOou => "coptic capital letter old coptic oou",
            Coptic::SmallLetterOldOou => "coptic small letter old coptic oou",
            Coptic::CapitalLetterSampi => "coptic capital letter sampi",
            Coptic::SmallLetterSampi => "coptic small letter sampi",
            Coptic::CapitalLetterCrossedShei => "coptic capital letter crossed shei",
            Coptic::SmallLetterCrossedShei => "coptic small letter crossed shei",
            Coptic::CapitalLetterOldShei => "coptic capital letter old coptic shei",
            Coptic::SmallLetterOldShei => "coptic small letter old coptic shei",
            Coptic::CapitalLetterOldEsh => "coptic capital letter old coptic esh",
            Coptic::SmallLetterOldEsh => "coptic small letter old coptic esh",
            Coptic::CapitalLetterAkhmimicKhei => "coptic capital letter akhmimic khei",
            Coptic::SmallLetterAkhmimicKhei => "coptic small letter akhmimic khei",
            Coptic::CapitalLetterDialectDashPHori => "coptic capital letter dialect-p hori",
            Coptic::SmallLetterDialectDashPHori => "coptic small letter dialect-p hori",
            Coptic::CapitalLetterOldHori => "coptic capital letter old coptic hori",
            Coptic::SmallLetterOldHori => "coptic small letter old coptic hori",
            Coptic::CapitalLetterOldHa => "coptic capital letter old coptic ha",
            Coptic::SmallLetterOldHa => "coptic small letter old coptic ha",
            Coptic::CapitalLetterLDashShapedHa => "coptic capital letter l-shaped ha",
            Coptic::SmallLetterLDashShapedHa => "coptic small letter l-shaped ha",
            Coptic::CapitalLetterOldHei => "coptic capital letter old coptic hei",
            Coptic::SmallLetterOldHei => "coptic small letter old coptic hei",
            Coptic::CapitalLetterOldHat => "coptic capital letter old coptic hat",
            Coptic::SmallLetterOldHat => "coptic small letter old coptic hat",
            Coptic::CapitalLetterOldGangia => "coptic capital letter old coptic gangia",
            Coptic::SmallLetterOldGangia => "coptic small letter old coptic gangia",
            Coptic::CapitalLetterOldDja => "coptic capital letter old coptic dja",
            Coptic::SmallLetterOldDja => "coptic small letter old coptic dja",
            Coptic::CapitalLetterOldShima => "coptic capital letter old coptic shima",
            Coptic::SmallLetterOldShima => "coptic small letter old coptic shima",
            Coptic::CapitalLetterOldNubianShima => "coptic capital letter old nubian shima",
            Coptic::SmallLetterOldNubianShima => "coptic small letter old nubian shima",
            Coptic::CapitalLetterOldNubianNgi => "coptic capital letter old nubian ngi",
            Coptic::SmallLetterOldNubianNgi => "coptic small letter old nubian ngi",
            Coptic::CapitalLetterOldNubianNyi => "coptic capital letter old nubian nyi",
            Coptic::SmallLetterOldNubianNyi => "coptic small letter old nubian nyi",
            Coptic::CapitalLetterOldNubianWau => "coptic capital letter old nubian wau",
            Coptic::SmallLetterOldNubianWau => "coptic small letter old nubian wau",
            Coptic::SymbolKai => "coptic symbol kai",
            Coptic::SymbolMiRo => "coptic symbol mi ro",
            Coptic::SymbolPiRo => "coptic symbol pi ro",
            Coptic::SymbolStauros => "coptic symbol stauros",
            Coptic::SymbolTauRo => "coptic symbol tau ro",
            Coptic::SymbolKhiRo => "coptic symbol khi ro",
            Coptic::SymbolShimaSima => "coptic symbol shima sima",
            Coptic::CapitalLetterCryptogrammicShei => "coptic capital letter cryptogrammic shei",
            Coptic::SmallLetterCryptogrammicShei => "coptic small letter cryptogrammic shei",
            Coptic::CapitalLetterCryptogrammicGangia => "coptic capital letter cryptogrammic gangia",
            Coptic::SmallLetterCryptogrammicGangia => "coptic small letter cryptogrammic gangia",
            Coptic::CombiningNiAbove => "coptic combining ni above",
            Coptic::CombiningSpiritusAsper => "coptic combining spiritus asper",
            Coptic::CombiningSpiritusLenis => "coptic combining spiritus lenis",
            Coptic::CapitalLetterBohairicKhei => "coptic capital letter bohairic khei",
            Coptic::SmallLetterBohairicKhei => "coptic small letter bohairic khei",
            Coptic::OldNubianFullStop => "coptic old nubian full stop",
            Coptic::OldNubianDirectQuestionMark => "coptic old nubian direct question mark",
            Coptic::OldNubianIndirectQuestionMark => "coptic old nubian indirect question mark",
            Coptic::OldNubianVerseDivider => "coptic old nubian verse divider",
            Coptic::FractionOneHalf => "coptic fraction one half",
            Coptic::FullStop => "coptic full stop",
        }
    }
}
