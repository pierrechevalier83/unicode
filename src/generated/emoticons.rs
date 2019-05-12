
/// An enum to represent all characters in the Emoticons block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Emoticons {
    /// \u{1f600}: '😀'
    GrinningFace,
    /// \u{1f601}: '😁'
    GrinningFaceWithSmilingEyes,
    /// \u{1f602}: '😂'
    FaceWithTearsOfJoy,
    /// \u{1f603}: '😃'
    SmilingFaceWithOpenMouth,
    /// \u{1f604}: '😄'
    SmilingFaceWithOpenMouthAndSmilingEyes,
    /// \u{1f605}: '😅'
    SmilingFaceWithOpenMouthAndColdSweat,
    /// \u{1f606}: '😆'
    SmilingFaceWithOpenMouthAndTightlyDashClosedEyes,
    /// \u{1f607}: '😇'
    SmilingFaceWithHalo,
    /// \u{1f608}: '😈'
    SmilingFaceWithHorns,
    /// \u{1f609}: '😉'
    WinkingFace,
    /// \u{1f60a}: '😊'
    SmilingFaceWithSmilingEyes,
    /// \u{1f60b}: '😋'
    FaceSavouringDeliciousFood,
    /// \u{1f60c}: '😌'
    RelievedFace,
    /// \u{1f60d}: '😍'
    SmilingFaceWithHeartDashShapedEyes,
    /// \u{1f60e}: '😎'
    SmilingFaceWithSunglasses,
    /// \u{1f60f}: '😏'
    SmirkingFace,
    /// \u{1f610}: '😐'
    NeutralFace,
    /// \u{1f611}: '😑'
    ExpressionlessFace,
    /// \u{1f612}: '😒'
    UnamusedFace,
    /// \u{1f613}: '😓'
    FaceWithColdSweat,
    /// \u{1f614}: '😔'
    PensiveFace,
    /// \u{1f615}: '😕'
    ConfusedFace,
    /// \u{1f616}: '😖'
    ConfoundedFace,
    /// \u{1f617}: '😗'
    KissingFace,
    /// \u{1f618}: '😘'
    FaceThrowingAKiss,
    /// \u{1f619}: '😙'
    KissingFaceWithSmilingEyes,
    /// \u{1f61a}: '😚'
    KissingFaceWithClosedEyes,
    /// \u{1f61b}: '😛'
    FaceWithStuckDashOutTongue,
    /// \u{1f61c}: '😜'
    FaceWithStuckDashOutTongueAndWinkingEye,
    /// \u{1f61d}: '😝'
    FaceWithStuckDashOutTongueAndTightlyDashClosedEyes,
    /// \u{1f61e}: '😞'
    DisappointedFace,
    /// \u{1f61f}: '😟'
    WorriedFace,
    /// \u{1f620}: '😠'
    AngryFace,
    /// \u{1f621}: '😡'
    PoutingFace,
    /// \u{1f622}: '😢'
    CryingFace,
    /// \u{1f623}: '😣'
    PerseveringFace,
    /// \u{1f624}: '😤'
    FaceWithLookOfTriumph,
    /// \u{1f625}: '😥'
    DisappointedButRelievedFace,
    /// \u{1f626}: '😦'
    FrowningFaceWithOpenMouth,
    /// \u{1f627}: '😧'
    AnguishedFace,
    /// \u{1f628}: '😨'
    FearfulFace,
    /// \u{1f629}: '😩'
    WearyFace,
    /// \u{1f62a}: '😪'
    SleepyFace,
    /// \u{1f62b}: '😫'
    TiredFace,
    /// \u{1f62c}: '😬'
    GrimacingFace,
    /// \u{1f62d}: '😭'
    LoudlyCryingFace,
    /// \u{1f62e}: '😮'
    FaceWithOpenMouth,
    /// \u{1f62f}: '😯'
    HushedFace,
    /// \u{1f630}: '😰'
    FaceWithOpenMouthAndColdSweat,
    /// \u{1f631}: '😱'
    FaceScreamingInFear,
    /// \u{1f632}: '😲'
    AstonishedFace,
    /// \u{1f633}: '😳'
    FlushedFace,
    /// \u{1f634}: '😴'
    SleepingFace,
    /// \u{1f635}: '😵'
    DizzyFace,
    /// \u{1f636}: '😶'
    FaceWithoutMouth,
    /// \u{1f637}: '😷'
    FaceWithMedicalMask,
    /// \u{1f638}: '😸'
    GrinningCatFaceWithSmilingEyes,
    /// \u{1f639}: '😹'
    CatFaceWithTearsOfJoy,
    /// \u{1f63a}: '😺'
    SmilingCatFaceWithOpenMouth,
    /// \u{1f63b}: '😻'
    SmilingCatFaceWithHeartDashShapedEyes,
    /// \u{1f63c}: '😼'
    CatFaceWithWrySmile,
    /// \u{1f63d}: '😽'
    KissingCatFaceWithClosedEyes,
    /// \u{1f63e}: '😾'
    PoutingCatFace,
    /// \u{1f63f}: '😿'
    CryingCatFace,
    /// \u{1f640}: '🙀'
    WearyCatFace,
    /// \u{1f641}: '🙁'
    SlightlyFrowningFace,
    /// \u{1f642}: '🙂'
    SlightlySmilingFace,
    /// \u{1f643}: '🙃'
    UpsideDashDownFace,
    /// \u{1f644}: '🙄'
    FaceWithRollingEyes,
    /// \u{1f645}: '🙅'
    FaceWithNoGoodGesture,
    /// \u{1f646}: '🙆'
    FaceWithOkGesture,
    /// \u{1f647}: '🙇'
    PersonBowingDeeply,
    /// \u{1f648}: '🙈'
    SeeDashNoDashEvilMonkey,
    /// \u{1f649}: '🙉'
    HearDashNoDashEvilMonkey,
    /// \u{1f64a}: '🙊'
    SpeakDashNoDashEvilMonkey,
    /// \u{1f64b}: '🙋'
    HappyPersonRaisingOneHand,
    /// \u{1f64c}: '🙌'
    PersonRaisingBothHandsInCelebration,
    /// \u{1f64d}: '🙍'
    PersonFrowning,
    /// \u{1f64e}: '🙎'
    PersonWithPoutingFace,
}

impl Into<char> for Emoticons {
    fn into(self) -> char {
        match self {
            Emoticons::GrinningFace => '😀',
            Emoticons::GrinningFaceWithSmilingEyes => '😁',
            Emoticons::FaceWithTearsOfJoy => '😂',
            Emoticons::SmilingFaceWithOpenMouth => '😃',
            Emoticons::SmilingFaceWithOpenMouthAndSmilingEyes => '😄',
            Emoticons::SmilingFaceWithOpenMouthAndColdSweat => '😅',
            Emoticons::SmilingFaceWithOpenMouthAndTightlyDashClosedEyes => '😆',
            Emoticons::SmilingFaceWithHalo => '😇',
            Emoticons::SmilingFaceWithHorns => '😈',
            Emoticons::WinkingFace => '😉',
            Emoticons::SmilingFaceWithSmilingEyes => '😊',
            Emoticons::FaceSavouringDeliciousFood => '😋',
            Emoticons::RelievedFace => '😌',
            Emoticons::SmilingFaceWithHeartDashShapedEyes => '😍',
            Emoticons::SmilingFaceWithSunglasses => '😎',
            Emoticons::SmirkingFace => '😏',
            Emoticons::NeutralFace => '😐',
            Emoticons::ExpressionlessFace => '😑',
            Emoticons::UnamusedFace => '😒',
            Emoticons::FaceWithColdSweat => '😓',
            Emoticons::PensiveFace => '😔',
            Emoticons::ConfusedFace => '😕',
            Emoticons::ConfoundedFace => '😖',
            Emoticons::KissingFace => '😗',
            Emoticons::FaceThrowingAKiss => '😘',
            Emoticons::KissingFaceWithSmilingEyes => '😙',
            Emoticons::KissingFaceWithClosedEyes => '😚',
            Emoticons::FaceWithStuckDashOutTongue => '😛',
            Emoticons::FaceWithStuckDashOutTongueAndWinkingEye => '😜',
            Emoticons::FaceWithStuckDashOutTongueAndTightlyDashClosedEyes => '😝',
            Emoticons::DisappointedFace => '😞',
            Emoticons::WorriedFace => '😟',
            Emoticons::AngryFace => '😠',
            Emoticons::PoutingFace => '😡',
            Emoticons::CryingFace => '😢',
            Emoticons::PerseveringFace => '😣',
            Emoticons::FaceWithLookOfTriumph => '😤',
            Emoticons::DisappointedButRelievedFace => '😥',
            Emoticons::FrowningFaceWithOpenMouth => '😦',
            Emoticons::AnguishedFace => '😧',
            Emoticons::FearfulFace => '😨',
            Emoticons::WearyFace => '😩',
            Emoticons::SleepyFace => '😪',
            Emoticons::TiredFace => '😫',
            Emoticons::GrimacingFace => '😬',
            Emoticons::LoudlyCryingFace => '😭',
            Emoticons::FaceWithOpenMouth => '😮',
            Emoticons::HushedFace => '😯',
            Emoticons::FaceWithOpenMouthAndColdSweat => '😰',
            Emoticons::FaceScreamingInFear => '😱',
            Emoticons::AstonishedFace => '😲',
            Emoticons::FlushedFace => '😳',
            Emoticons::SleepingFace => '😴',
            Emoticons::DizzyFace => '😵',
            Emoticons::FaceWithoutMouth => '😶',
            Emoticons::FaceWithMedicalMask => '😷',
            Emoticons::GrinningCatFaceWithSmilingEyes => '😸',
            Emoticons::CatFaceWithTearsOfJoy => '😹',
            Emoticons::SmilingCatFaceWithOpenMouth => '😺',
            Emoticons::SmilingCatFaceWithHeartDashShapedEyes => '😻',
            Emoticons::CatFaceWithWrySmile => '😼',
            Emoticons::KissingCatFaceWithClosedEyes => '😽',
            Emoticons::PoutingCatFace => '😾',
            Emoticons::CryingCatFace => '😿',
            Emoticons::WearyCatFace => '🙀',
            Emoticons::SlightlyFrowningFace => '🙁',
            Emoticons::SlightlySmilingFace => '🙂',
            Emoticons::UpsideDashDownFace => '🙃',
            Emoticons::FaceWithRollingEyes => '🙄',
            Emoticons::FaceWithNoGoodGesture => '🙅',
            Emoticons::FaceWithOkGesture => '🙆',
            Emoticons::PersonBowingDeeply => '🙇',
            Emoticons::SeeDashNoDashEvilMonkey => '🙈',
            Emoticons::HearDashNoDashEvilMonkey => '🙉',
            Emoticons::SpeakDashNoDashEvilMonkey => '🙊',
            Emoticons::HappyPersonRaisingOneHand => '🙋',
            Emoticons::PersonRaisingBothHandsInCelebration => '🙌',
            Emoticons::PersonFrowning => '🙍',
            Emoticons::PersonWithPoutingFace => '🙎',
        }
    }
}

impl std::convert::TryFrom<char> for Emoticons {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '😀' => Ok(Emoticons::GrinningFace),
            '😁' => Ok(Emoticons::GrinningFaceWithSmilingEyes),
            '😂' => Ok(Emoticons::FaceWithTearsOfJoy),
            '😃' => Ok(Emoticons::SmilingFaceWithOpenMouth),
            '😄' => Ok(Emoticons::SmilingFaceWithOpenMouthAndSmilingEyes),
            '😅' => Ok(Emoticons::SmilingFaceWithOpenMouthAndColdSweat),
            '😆' => Ok(Emoticons::SmilingFaceWithOpenMouthAndTightlyDashClosedEyes),
            '😇' => Ok(Emoticons::SmilingFaceWithHalo),
            '😈' => Ok(Emoticons::SmilingFaceWithHorns),
            '😉' => Ok(Emoticons::WinkingFace),
            '😊' => Ok(Emoticons::SmilingFaceWithSmilingEyes),
            '😋' => Ok(Emoticons::FaceSavouringDeliciousFood),
            '😌' => Ok(Emoticons::RelievedFace),
            '😍' => Ok(Emoticons::SmilingFaceWithHeartDashShapedEyes),
            '😎' => Ok(Emoticons::SmilingFaceWithSunglasses),
            '😏' => Ok(Emoticons::SmirkingFace),
            '😐' => Ok(Emoticons::NeutralFace),
            '😑' => Ok(Emoticons::ExpressionlessFace),
            '😒' => Ok(Emoticons::UnamusedFace),
            '😓' => Ok(Emoticons::FaceWithColdSweat),
            '😔' => Ok(Emoticons::PensiveFace),
            '😕' => Ok(Emoticons::ConfusedFace),
            '😖' => Ok(Emoticons::ConfoundedFace),
            '😗' => Ok(Emoticons::KissingFace),
            '😘' => Ok(Emoticons::FaceThrowingAKiss),
            '😙' => Ok(Emoticons::KissingFaceWithSmilingEyes),
            '😚' => Ok(Emoticons::KissingFaceWithClosedEyes),
            '😛' => Ok(Emoticons::FaceWithStuckDashOutTongue),
            '😜' => Ok(Emoticons::FaceWithStuckDashOutTongueAndWinkingEye),
            '😝' => Ok(Emoticons::FaceWithStuckDashOutTongueAndTightlyDashClosedEyes),
            '😞' => Ok(Emoticons::DisappointedFace),
            '😟' => Ok(Emoticons::WorriedFace),
            '😠' => Ok(Emoticons::AngryFace),
            '😡' => Ok(Emoticons::PoutingFace),
            '😢' => Ok(Emoticons::CryingFace),
            '😣' => Ok(Emoticons::PerseveringFace),
            '😤' => Ok(Emoticons::FaceWithLookOfTriumph),
            '😥' => Ok(Emoticons::DisappointedButRelievedFace),
            '😦' => Ok(Emoticons::FrowningFaceWithOpenMouth),
            '😧' => Ok(Emoticons::AnguishedFace),
            '😨' => Ok(Emoticons::FearfulFace),
            '😩' => Ok(Emoticons::WearyFace),
            '😪' => Ok(Emoticons::SleepyFace),
            '😫' => Ok(Emoticons::TiredFace),
            '😬' => Ok(Emoticons::GrimacingFace),
            '😭' => Ok(Emoticons::LoudlyCryingFace),
            '😮' => Ok(Emoticons::FaceWithOpenMouth),
            '😯' => Ok(Emoticons::HushedFace),
            '😰' => Ok(Emoticons::FaceWithOpenMouthAndColdSweat),
            '😱' => Ok(Emoticons::FaceScreamingInFear),
            '😲' => Ok(Emoticons::AstonishedFace),
            '😳' => Ok(Emoticons::FlushedFace),
            '😴' => Ok(Emoticons::SleepingFace),
            '😵' => Ok(Emoticons::DizzyFace),
            '😶' => Ok(Emoticons::FaceWithoutMouth),
            '😷' => Ok(Emoticons::FaceWithMedicalMask),
            '😸' => Ok(Emoticons::GrinningCatFaceWithSmilingEyes),
            '😹' => Ok(Emoticons::CatFaceWithTearsOfJoy),
            '😺' => Ok(Emoticons::SmilingCatFaceWithOpenMouth),
            '😻' => Ok(Emoticons::SmilingCatFaceWithHeartDashShapedEyes),
            '😼' => Ok(Emoticons::CatFaceWithWrySmile),
            '😽' => Ok(Emoticons::KissingCatFaceWithClosedEyes),
            '😾' => Ok(Emoticons::PoutingCatFace),
            '😿' => Ok(Emoticons::CryingCatFace),
            '🙀' => Ok(Emoticons::WearyCatFace),
            '🙁' => Ok(Emoticons::SlightlyFrowningFace),
            '🙂' => Ok(Emoticons::SlightlySmilingFace),
            '🙃' => Ok(Emoticons::UpsideDashDownFace),
            '🙄' => Ok(Emoticons::FaceWithRollingEyes),
            '🙅' => Ok(Emoticons::FaceWithNoGoodGesture),
            '🙆' => Ok(Emoticons::FaceWithOkGesture),
            '🙇' => Ok(Emoticons::PersonBowingDeeply),
            '🙈' => Ok(Emoticons::SeeDashNoDashEvilMonkey),
            '🙉' => Ok(Emoticons::HearDashNoDashEvilMonkey),
            '🙊' => Ok(Emoticons::SpeakDashNoDashEvilMonkey),
            '🙋' => Ok(Emoticons::HappyPersonRaisingOneHand),
            '🙌' => Ok(Emoticons::PersonRaisingBothHandsInCelebration),
            '🙍' => Ok(Emoticons::PersonFrowning),
            '🙎' => Ok(Emoticons::PersonWithPoutingFace),
            _ => Err(()),
        }
    }
}

impl Into<u32> for Emoticons {
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

impl std::convert::TryFrom<u32> for Emoticons {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for Emoticons {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl Emoticons {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        Emoticons::GrinningFace
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("Emoticons{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
