
/// An enum to represent all characters in the SuttonSignWriting block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SuttonSignWriting {
    /// \u{1d800}: '𝠀'
    SignwritingHandDashFistIndex,
    /// \u{1d801}: '𝠁'
    SignwritingHandDashCircleIndex,
    /// \u{1d802}: '𝠂'
    SignwritingHandDashCupIndex,
    /// \u{1d803}: '𝠃'
    SignwritingHandDashOvalIndex,
    /// \u{1d804}: '𝠄'
    SignwritingHandDashHingeIndex,
    /// \u{1d805}: '𝠅'
    SignwritingHandDashAngleIndex,
    /// \u{1d806}: '𝠆'
    SignwritingHandDashFistIndexBent,
    /// \u{1d807}: '𝠇'
    SignwritingHandDashCircleIndexBent,
    /// \u{1d808}: '𝠈'
    SignwritingHandDashFistThumbUnderIndexBent,
    /// \u{1d809}: '𝠉'
    SignwritingHandDashFistIndexRaisedKnuckle,
    /// \u{1d80a}: '𝠊'
    SignwritingHandDashFistIndexCupped,
    /// \u{1d80b}: '𝠋'
    SignwritingHandDashFistIndexHinged,
    /// \u{1d80c}: '𝠌'
    SignwritingHandDashFistIndexHingedLow,
    /// \u{1d80d}: '𝠍'
    SignwritingHandDashCircleIndexHinge,
    /// \u{1d80e}: '𝠎'
    SignwritingHandDashFistIndexMiddle,
    /// \u{1d80f}: '𝠏'
    SignwritingHandDashCircleIndexMiddle,
    /// \u{1d810}: '𝠐'
    SignwritingHandDashFistIndexMiddleBent,
    /// \u{1d811}: '𝠑'
    SignwritingHandDashFistIndexMiddleRaisedKnuckles,
    /// \u{1d812}: '𝠒'
    SignwritingHandDashFistIndexMiddleHinged,
    /// \u{1d813}: '𝠓'
    SignwritingHandDashFistIndexUpMiddleHinged,
    /// \u{1d814}: '𝠔'
    SignwritingHandDashFistIndexHingedMiddleUp,
    /// \u{1d815}: '𝠕'
    SignwritingHandDashFistIndexMiddleConjoined,
    /// \u{1d816}: '𝠖'
    SignwritingHandDashFistIndexMiddleConjoinedIndexBent,
    /// \u{1d817}: '𝠗'
    SignwritingHandDashFistIndexMiddleConjoinedMiddleBent,
    /// \u{1d818}: '𝠘'
    SignwritingHandDashFistIndexMiddleConjoinedCupped,
    /// \u{1d819}: '𝠙'
    SignwritingHandDashFistIndexMiddleConjoinedHinged,
    /// \u{1d81a}: '𝠚'
    SignwritingHandDashFistIndexMiddleCrossed,
    /// \u{1d81b}: '𝠛'
    SignwritingHandDashCircleIndexMiddleCrossed,
    /// \u{1d81c}: '𝠜'
    SignwritingHandDashFistMiddleBentOverIndex,
    /// \u{1d81d}: '𝠝'
    SignwritingHandDashFistIndexBentOverMiddle,
    /// \u{1d81e}: '𝠞'
    SignwritingHandDashFistIndexMiddleThumb,
    /// \u{1d81f}: '𝠟'
    SignwritingHandDashCircleIndexMiddleThumb,
    /// \u{1d820}: '𝠠'
    SignwritingHandDashFistIndexMiddleStraightThumbBent,
    /// \u{1d821}: '𝠡'
    SignwritingHandDashFistIndexMiddleBentThumbStraight,
    /// \u{1d822}: '𝠢'
    SignwritingHandDashFistIndexMiddleThumbBent,
    /// \u{1d823}: '𝠣'
    SignwritingHandDashFistIndexMiddleHingedSpreadThumbSide,
    /// \u{1d824}: '𝠤'
    SignwritingHandDashFistIndexUpMiddleHingedThumbSide,
    /// \u{1d825}: '𝠥'
    SignwritingHandDashFistIndexUpMiddleHingedThumbConjoined,
    /// \u{1d826}: '𝠦'
    SignwritingHandDashFistIndexHingedMiddleUpThumbSide,
    /// \u{1d827}: '𝠧'
    SignwritingHandDashFistIndexMiddleUpSpreadThumbForward,
    /// \u{1d828}: '𝠨'
    SignwritingHandDashFistIndexMiddleThumbCupped,
    /// \u{1d829}: '𝠩'
    SignwritingHandDashFistIndexMiddleThumbCircled,
    /// \u{1d82a}: '𝠪'
    SignwritingHandDashFistIndexMiddleThumbHooked,
    /// \u{1d82b}: '𝠫'
    SignwritingHandDashFistIndexMiddleThumbHinged,
    /// \u{1d82c}: '𝠬'
    SignwritingHandDashFistThumbBetweenIndexMiddleStraight,
    /// \u{1d82d}: '𝠭'
    SignwritingHandDashFistIndexMiddleConjoinedThumbSide,
    /// \u{1d82e}: '𝠮'
    SignwritingHandDashFistIndexMiddleConjoinedThumbSideConjoined,
    /// \u{1d82f}: '𝠯'
    SignwritingHandDashFistIndexMiddleConjoinedThumbSideBent,
    /// \u{1d830}: '𝠰'
    SignwritingHandDashFistMiddleThumbHookedIndexUp,
    /// \u{1d831}: '𝠱'
    SignwritingHandDashFistIndexThumbHookedMiddleUp,
    /// \u{1d832}: '𝠲'
    SignwritingHandDashFistIndexMiddleConjoinedHingedThumbSide,
    /// \u{1d833}: '𝠳'
    SignwritingHandDashFistIndexMiddleCrossedThumbSide,
    /// \u{1d834}: '𝠴'
    SignwritingHandDashFistIndexMiddleConjoinedThumbForward,
    /// \u{1d835}: '𝠵'
    SignwritingHandDashFistIndexMiddleConjoinedCuppedThumbForward,
    /// \u{1d836}: '𝠶'
    SignwritingHandDashFistMiddleThumbCuppedIndexUp,
    /// \u{1d837}: '𝠷'
    SignwritingHandDashFistIndexThumbCuppedMiddleUp,
    /// \u{1d838}: '𝠸'
    SignwritingHandDashFistMiddleThumbCircledIndexUp,
    /// \u{1d839}: '𝠹'
    SignwritingHandDashFistMiddleThumbCircledIndexHinged,
    /// \u{1d83a}: '𝠺'
    SignwritingHandDashFistIndexThumbAngledOutMiddleUp,
    /// \u{1d83b}: '𝠻'
    SignwritingHandDashFistIndexThumbAngledInMiddleUp,
    /// \u{1d83c}: '𝠼'
    SignwritingHandDashFistIndexThumbCircledMiddleUp,
    /// \u{1d83d}: '𝠽'
    SignwritingHandDashFistIndexMiddleThumbConjoinedHinged,
    /// \u{1d83e}: '𝠾'
    SignwritingHandDashFistIndexMiddleThumbAngledOut,
    /// \u{1d83f}: '𝠿'
    SignwritingHandDashFistIndexMiddleThumbAngled,
    /// \u{1d840}: '𝡀'
    SignwritingHandDashFistMiddleThumbAngledOutIndexUp,
    /// \u{1d841}: '𝡁'
    SignwritingHandDashFistMiddleThumbAngledOutIndexCrossed,
    /// \u{1d842}: '𝡂'
    SignwritingHandDashFistMiddleThumbAngledIndexUp,
    /// \u{1d843}: '𝡃'
    SignwritingHandDashFistIndexThumbHookedMiddleHinged,
    /// \u{1d844}: '𝡄'
    SignwritingHandDashFlatFourFingers,
    /// \u{1d845}: '𝡅'
    SignwritingHandDashFlatFourFingersBent,
    /// \u{1d846}: '𝡆'
    SignwritingHandDashFlatFourFingersHinged,
    /// \u{1d847}: '𝡇'
    SignwritingHandDashFlatFourFingersConjoined,
    /// \u{1d848}: '𝡈'
    SignwritingHandDashFlatFourFingersConjoinedSplit,
    /// \u{1d849}: '𝡉'
    SignwritingHandDashClawFourFingersConjoined,
    /// \u{1d84a}: '𝡊'
    SignwritingHandDashFistFourFingersConjoinedBent,
    /// \u{1d84b}: '𝡋'
    SignwritingHandDashHingeFourFingersConjoined,
    /// \u{1d84c}: '𝡌'
    SignwritingHandDashFlatFiveFingersSpread,
    /// \u{1d84d}: '𝡍'
    SignwritingHandDashFlatHeelFiveFingersSpread,
    /// \u{1d84e}: '𝡎'
    SignwritingHandDashFlatFiveFingersSpreadFourBent,
    /// \u{1d84f}: '𝡏'
    SignwritingHandDashFlatHeelFiveFingersSpreadFourBent,
    /// \u{1d850}: '𝡐'
    SignwritingHandDashFlatFiveFingersSpreadBent,
    /// \u{1d851}: '𝡑'
    SignwritingHandDashFlatHeelFiveFingersSpreadBent,
    /// \u{1d852}: '𝡒'
    SignwritingHandDashFlatFiveFingersSpreadThumbForward,
    /// \u{1d853}: '𝡓'
    SignwritingHandDashCupFiveFingersSpread,
    /// \u{1d854}: '𝡔'
    SignwritingHandDashCupFiveFingersSpreadOpen,
    /// \u{1d855}: '𝡕'
    SignwritingHandDashHingeFiveFingersSpreadOpen,
    /// \u{1d856}: '𝡖'
    SignwritingHandDashOvalFiveFingersSpread,
    /// \u{1d857}: '𝡗'
    SignwritingHandDashFlatFiveFingersSpreadHinged,
    /// \u{1d858}: '𝡘'
    SignwritingHandDashFlatFiveFingersSpreadHingedThumbSide,
    /// \u{1d859}: '𝡙'
    SignwritingHandDashFlatFiveFingersSpreadHingedNoThumb,
    /// \u{1d85a}: '𝡚'
    SignwritingHandDashFlat,
    /// \u{1d85b}: '𝡛'
    SignwritingHandDashFlatBetweenPalmFacings,
    /// \u{1d85c}: '𝡜'
    SignwritingHandDashFlatHeel,
    /// \u{1d85d}: '𝡝'
    SignwritingHandDashFlatThumbSide,
    /// \u{1d85e}: '𝡞'
    SignwritingHandDashFlatHeelThumbSide,
    /// \u{1d85f}: '𝡟'
    SignwritingHandDashFlatThumbBent,
    /// \u{1d860}: '𝡠'
    SignwritingHandDashFlatThumbForward,
    /// \u{1d861}: '𝡡'
    SignwritingHandDashFlatSplitIndexThumbSide,
    /// \u{1d862}: '𝡢'
    SignwritingHandDashFlatSplitCentre,
    /// \u{1d863}: '𝡣'
    SignwritingHandDashFlatSplitCentreThumbSide,
    /// \u{1d864}: '𝡤'
    SignwritingHandDashFlatSplitCentreThumbSideBent,
    /// \u{1d865}: '𝡥'
    SignwritingHandDashFlatSplitLittle,
    /// \u{1d866}: '𝡦'
    SignwritingHandDashClaw,
    /// \u{1d867}: '𝡧'
    SignwritingHandDashClawThumbSide,
    /// \u{1d868}: '𝡨'
    SignwritingHandDashClawNoThumb,
    /// \u{1d869}: '𝡩'
    SignwritingHandDashClawThumbForward,
    /// \u{1d86a}: '𝡪'
    SignwritingHandDashHookCurlicue,
    /// \u{1d86b}: '𝡫'
    SignwritingHandDashHook,
    /// \u{1d86c}: '𝡬'
    SignwritingHandDashCupOpen,
    /// \u{1d86d}: '𝡭'
    SignwritingHandDashCup,
    /// \u{1d86e}: '𝡮'
    SignwritingHandDashCupOpenThumbSide,
    /// \u{1d86f}: '𝡯'
    SignwritingHandDashCupThumbSide,
    /// \u{1d870}: '𝡰'
    SignwritingHandDashCupOpenNoThumb,
    /// \u{1d871}: '𝡱'
    SignwritingHandDashCupNoThumb,
    /// \u{1d872}: '𝡲'
    SignwritingHandDashCupOpenThumbForward,
    /// \u{1d873}: '𝡳'
    SignwritingHandDashCupThumbForward,
    /// \u{1d874}: '𝡴'
    SignwritingHandDashCurlicueOpen,
    /// \u{1d875}: '𝡵'
    SignwritingHandDashCurlicue,
    /// \u{1d876}: '𝡶'
    SignwritingHandDashCircle,
    /// \u{1d877}: '𝡷'
    SignwritingHandDashOval,
    /// \u{1d878}: '𝡸'
    SignwritingHandDashOvalThumbSide,
    /// \u{1d879}: '𝡹'
    SignwritingHandDashOvalNoThumb,
    /// \u{1d87a}: '𝡺'
    SignwritingHandDashOvalThumbForward,
    /// \u{1d87b}: '𝡻'
    SignwritingHandDashHingeOpen,
    /// \u{1d87c}: '𝡼'
    SignwritingHandDashHingeOpenThumbForward,
    /// \u{1d87d}: '𝡽'
    SignwritingHandDashHinge,
    /// \u{1d87e}: '𝡾'
    SignwritingHandDashHingeSmall,
    /// \u{1d87f}: '𝡿'
    SignwritingHandDashHingeOpenThumbSide,
    /// \u{1d880}: '𝢀'
    SignwritingHandDashHingeThumbSide,
    /// \u{1d881}: '𝢁'
    SignwritingHandDashHingeOpenNoThumb,
    /// \u{1d882}: '𝢂'
    SignwritingHandDashHingeNoThumb,
    /// \u{1d883}: '𝢃'
    SignwritingHandDashHingeThumbSideTouchingIndex,
    /// \u{1d884}: '𝢄'
    SignwritingHandDashHingeThumbBetweenMiddleRing,
    /// \u{1d885}: '𝢅'
    SignwritingHandDashAngle,
    /// \u{1d886}: '𝢆'
    SignwritingHandDashFistIndexMiddleRing,
    /// \u{1d887}: '𝢇'
    SignwritingHandDashCircleIndexMiddleRing,
    /// \u{1d888}: '𝢈'
    SignwritingHandDashHingeIndexMiddleRing,
    /// \u{1d889}: '𝢉'
    SignwritingHandDashAngleIndexMiddleRing,
    /// \u{1d88a}: '𝢊'
    SignwritingHandDashHingeLittle,
    /// \u{1d88b}: '𝢋'
    SignwritingHandDashFistIndexMiddleRingBent,
    /// \u{1d88c}: '𝢌'
    SignwritingHandDashFistIndexMiddleRingConjoined,
    /// \u{1d88d}: '𝢍'
    SignwritingHandDashHingeIndexMiddleRingConjoined,
    /// \u{1d88e}: '𝢎'
    SignwritingHandDashFistLittleDown,
    /// \u{1d88f}: '𝢏'
    SignwritingHandDashFistLittleDownRippleStraight,
    /// \u{1d890}: '𝢐'
    SignwritingHandDashFistLittleDownRippleCurved,
    /// \u{1d891}: '𝢑'
    SignwritingHandDashFistLittleDownOthersCircled,
    /// \u{1d892}: '𝢒'
    SignwritingHandDashFistLittleUp,
    /// \u{1d893}: '𝢓'
    SignwritingHandDashFistThumbUnderLittleUp,
    /// \u{1d894}: '𝢔'
    SignwritingHandDashCircleLittleUp,
    /// \u{1d895}: '𝢕'
    SignwritingHandDashOvalLittleUp,
    /// \u{1d896}: '𝢖'
    SignwritingHandDashAngleLittleUp,
    /// \u{1d897}: '𝢗'
    SignwritingHandDashFistLittleRaisedKnuckle,
    /// \u{1d898}: '𝢘'
    SignwritingHandDashFistLittleBent,
    /// \u{1d899}: '𝢙'
    SignwritingHandDashFistLittleTouchesThumb,
    /// \u{1d89a}: '𝢚'
    SignwritingHandDashFistLittleThumb,
    /// \u{1d89b}: '𝢛'
    SignwritingHandDashHingeLittleThumb,
    /// \u{1d89c}: '𝢜'
    SignwritingHandDashFistLittleIndexThumb,
    /// \u{1d89d}: '𝢝'
    SignwritingHandDashHingeLittleIndexThumb,
    /// \u{1d89e}: '𝢞'
    SignwritingHandDashAngleLittleIndexThumbIndexThumbOut,
    /// \u{1d89f}: '𝢟'
    SignwritingHandDashAngleLittleIndexThumbIndexThumb,
    /// \u{1d8a0}: '𝢠'
    SignwritingHandDashFistLittleIndex,
    /// \u{1d8a1}: '𝢡'
    SignwritingHandDashCircleLittleIndex,
    /// \u{1d8a2}: '𝢢'
    SignwritingHandDashHingeLittleIndex,
    /// \u{1d8a3}: '𝢣'
    SignwritingHandDashAngleLittleIndex,
    /// \u{1d8a4}: '𝢤'
    SignwritingHandDashFistIndexMiddleLittle,
    /// \u{1d8a5}: '𝢥'
    SignwritingHandDashCircleIndexMiddleLittle,
    /// \u{1d8a6}: '𝢦'
    SignwritingHandDashHingeIndexMiddleLittle,
    /// \u{1d8a7}: '𝢧'
    SignwritingHandDashHingeRing,
    /// \u{1d8a8}: '𝢨'
    SignwritingHandDashAngleIndexMiddleLittle,
    /// \u{1d8a9}: '𝢩'
    SignwritingHandDashFistIndexMiddleCrossLittle,
    /// \u{1d8aa}: '𝢪'
    SignwritingHandDashCircleIndexMiddleCrossLittle,
    /// \u{1d8ab}: '𝢫'
    SignwritingHandDashFistRingDown,
    /// \u{1d8ac}: '𝢬'
    SignwritingHandDashHingeRingDownIndexThumbHookMiddle,
    /// \u{1d8ad}: '𝢭'
    SignwritingHandDashAngleRingDownMiddleThumbIndexCross,
    /// \u{1d8ae}: '𝢮'
    SignwritingHandDashFistRingUp,
    /// \u{1d8af}: '𝢯'
    SignwritingHandDashFistRingRaisedKnuckle,
    /// \u{1d8b0}: '𝢰'
    SignwritingHandDashFistRingLittle,
    /// \u{1d8b1}: '𝢱'
    SignwritingHandDashCircleRingLittle,
    /// \u{1d8b2}: '𝢲'
    SignwritingHandDashOvalRingLittle,
    /// \u{1d8b3}: '𝢳'
    SignwritingHandDashAngleRingLittle,
    /// \u{1d8b4}: '𝢴'
    SignwritingHandDashFistRingMiddle,
    /// \u{1d8b5}: '𝢵'
    SignwritingHandDashFistRingMiddleConjoined,
    /// \u{1d8b6}: '𝢶'
    SignwritingHandDashFistRingMiddleRaisedKnuckles,
    /// \u{1d8b7}: '𝢷'
    SignwritingHandDashFistRingIndex,
    /// \u{1d8b8}: '𝢸'
    SignwritingHandDashFistRingThumb,
    /// \u{1d8b9}: '𝢹'
    SignwritingHandDashHookRingThumb,
    /// \u{1d8ba}: '𝢺'
    SignwritingHandDashFistIndexRingLittle,
    /// \u{1d8bb}: '𝢻'
    SignwritingHandDashCircleIndexRingLittle,
    /// \u{1d8bc}: '𝢼'
    SignwritingHandDashCurlicueIndexRingLittleOn,
    /// \u{1d8bd}: '𝢽'
    SignwritingHandDashHookIndexRingLittleOut,
    /// \u{1d8be}: '𝢾'
    SignwritingHandDashHookIndexRingLittleIn,
    /// \u{1d8bf}: '𝢿'
    SignwritingHandDashHookIndexRingLittleUnder,
    /// \u{1d8c0}: '𝣀'
    SignwritingHandDashCupIndexRingLittle,
    /// \u{1d8c1}: '𝣁'
    SignwritingHandDashHingeIndexRingLittle,
    /// \u{1d8c2}: '𝣂'
    SignwritingHandDashAngleIndexRingLittleOut,
    /// \u{1d8c3}: '𝣃'
    SignwritingHandDashAngleIndexRingLittle,
    /// \u{1d8c4}: '𝣄'
    SignwritingHandDashFistMiddleDown,
    /// \u{1d8c5}: '𝣅'
    SignwritingHandDashHingeMiddle,
    /// \u{1d8c6}: '𝣆'
    SignwritingHandDashFistMiddleUp,
    /// \u{1d8c7}: '𝣇'
    SignwritingHandDashCircleMiddleUp,
    /// \u{1d8c8}: '𝣈'
    SignwritingHandDashFistMiddleRaisedKnuckle,
    /// \u{1d8c9}: '𝣉'
    SignwritingHandDashFistMiddleUpThumbSide,
    /// \u{1d8ca}: '𝣊'
    SignwritingHandDashHookMiddleThumb,
    /// \u{1d8cb}: '𝣋'
    SignwritingHandDashFistMiddleThumbLittle,
    /// \u{1d8cc}: '𝣌'
    SignwritingHandDashFistMiddleLittle,
    /// \u{1d8cd}: '𝣍'
    SignwritingHandDashFistMiddleRingLittle,
    /// \u{1d8ce}: '𝣎'
    SignwritingHandDashCircleMiddleRingLittle,
    /// \u{1d8cf}: '𝣏'
    SignwritingHandDashCurlicueMiddleRingLittleOn,
    /// \u{1d8d0}: '𝣐'
    SignwritingHandDashCupMiddleRingLittle,
    /// \u{1d8d1}: '𝣑'
    SignwritingHandDashHingeMiddleRingLittle,
    /// \u{1d8d2}: '𝣒'
    SignwritingHandDashAngleMiddleRingLittleOut,
    /// \u{1d8d3}: '𝣓'
    SignwritingHandDashAngleMiddleRingLittleIn,
    /// \u{1d8d4}: '𝣔'
    SignwritingHandDashAngleMiddleRingLittle,
    /// \u{1d8d5}: '𝣕'
    SignwritingHandDashCircleMiddleRingLittleBent,
    /// \u{1d8d6}: '𝣖'
    SignwritingHandDashClawMiddleRingLittleConjoined,
    /// \u{1d8d7}: '𝣗'
    SignwritingHandDashClawMiddleRingLittleConjoinedSide,
    /// \u{1d8d8}: '𝣘'
    SignwritingHandDashHookMiddleRingLittleConjoinedOut,
    /// \u{1d8d9}: '𝣙'
    SignwritingHandDashHookMiddleRingLittleConjoinedIn,
    /// \u{1d8da}: '𝣚'
    SignwritingHandDashHookMiddleRingLittleConjoined,
    /// \u{1d8db}: '𝣛'
    SignwritingHandDashHingeIndexHinged,
    /// \u{1d8dc}: '𝣜'
    SignwritingHandDashFistIndexThumbSide,
    /// \u{1d8dd}: '𝣝'
    SignwritingHandDashHingeIndexThumbSide,
    /// \u{1d8de}: '𝣞'
    SignwritingHandDashFistIndexThumbSideThumbDiagonal,
    /// \u{1d8df}: '𝣟'
    SignwritingHandDashFistIndexThumbSideThumbConjoined,
    /// \u{1d8e0}: '𝣠'
    SignwritingHandDashFistIndexThumbSideThumbBent,
    /// \u{1d8e1}: '𝣡'
    SignwritingHandDashFistIndexThumbSideIndexBent,
    /// \u{1d8e2}: '𝣢'
    SignwritingHandDashFistIndexThumbSideBothBent,
    /// \u{1d8e3}: '𝣣'
    SignwritingHandDashFistIndexThumbSideIndexHinge,
    /// \u{1d8e4}: '𝣤'
    SignwritingHandDashFistIndexThumbForwardIndexStraight,
    /// \u{1d8e5}: '𝣥'
    SignwritingHandDashFistIndexThumbForwardIndexBent,
    /// \u{1d8e6}: '𝣦'
    SignwritingHandDashFistIndexThumbHook,
    /// \u{1d8e7}: '𝣧'
    SignwritingHandDashFistIndexThumbCurlicue,
    /// \u{1d8e8}: '𝣨'
    SignwritingHandDashFistIndexThumbCurveThumbInside,
    /// \u{1d8e9}: '𝣩'
    SignwritingHandDashClawIndexThumbCurveThumbInside,
    /// \u{1d8ea}: '𝣪'
    SignwritingHandDashFistIndexThumbCurveThumbUnder,
    /// \u{1d8eb}: '𝣫'
    SignwritingHandDashFistIndexThumbCircle,
    /// \u{1d8ec}: '𝣬'
    SignwritingHandDashCupIndexThumb,
    /// \u{1d8ed}: '𝣭'
    SignwritingHandDashCupIndexThumbOpen,
    /// \u{1d8ee}: '𝣮'
    SignwritingHandDashHingeIndexThumbOpen,
    /// \u{1d8ef}: '𝣯'
    SignwritingHandDashHingeIndexThumbLarge,
    /// \u{1d8f0}: '𝣰'
    SignwritingHandDashHingeIndexThumb,
    /// \u{1d8f1}: '𝣱'
    SignwritingHandDashHingeIndexThumbSmall,
    /// \u{1d8f2}: '𝣲'
    SignwritingHandDashAngleIndexThumbOut,
    /// \u{1d8f3}: '𝣳'
    SignwritingHandDashAngleIndexThumbIn,
    /// \u{1d8f4}: '𝣴'
    SignwritingHandDashAngleIndexThumb,
    /// \u{1d8f5}: '𝣵'
    SignwritingHandDashFistThumb,
    /// \u{1d8f6}: '𝣶'
    SignwritingHandDashFistThumbHeel,
    /// \u{1d8f7}: '𝣷'
    SignwritingHandDashFistThumbSideDiagonal,
    /// \u{1d8f8}: '𝣸'
    SignwritingHandDashFistThumbSideConjoined,
    /// \u{1d8f9}: '𝣹'
    SignwritingHandDashFistThumbSideBent,
    /// \u{1d8fa}: '𝣺'
    SignwritingHandDashFistThumbForward,
    /// \u{1d8fb}: '𝣻'
    SignwritingHandDashFistThumbBetweenIndexMiddle,
    /// \u{1d8fc}: '𝣼'
    SignwritingHandDashFistThumbBetweenMiddleRing,
    /// \u{1d8fd}: '𝣽'
    SignwritingHandDashFistThumbBetweenRingLittle,
    /// \u{1d8fe}: '𝣾'
    SignwritingHandDashFistThumbUnderTwoFingers,
    /// \u{1d8ff}: '𝣿'
    SignwritingHandDashFistThumbOverTwoFingers,
    /// \u{1d900}: '𝤀'
    SignwritingHandDashFistThumbUnderThreeFingers,
    /// \u{1d901}: '𝤁'
    SignwritingHandDashFistThumbUnderFourFingers,
    /// \u{1d902}: '𝤂'
    SignwritingHandDashFistThumbOverFourRaisedKnuckles,
    /// \u{1d903}: '𝤃'
    SignwritingHandDashFist,
    /// \u{1d904}: '𝤄'
    SignwritingHandDashFistHeel,
    /// \u{1d905}: '𝤅'
    SignwritingTouchSingle,
    /// \u{1d906}: '𝤆'
    SignwritingTouchMultiple,
    /// \u{1d907}: '𝤇'
    SignwritingTouchBetween,
    /// \u{1d908}: '𝤈'
    SignwritingGraspSingle,
    /// \u{1d909}: '𝤉'
    SignwritingGraspMultiple,
    /// \u{1d90a}: '𝤊'
    SignwritingGraspBetween,
    /// \u{1d90b}: '𝤋'
    SignwritingStrikeSingle,
    /// \u{1d90c}: '𝤌'
    SignwritingStrikeMultiple,
    /// \u{1d90d}: '𝤍'
    SignwritingStrikeBetween,
    /// \u{1d90e}: '𝤎'
    SignwritingBrushSingle,
    /// \u{1d90f}: '𝤏'
    SignwritingBrushMultiple,
    /// \u{1d910}: '𝤐'
    SignwritingBrushBetween,
    /// \u{1d911}: '𝤑'
    SignwritingRubSingle,
    /// \u{1d912}: '𝤒'
    SignwritingRubMultiple,
    /// \u{1d913}: '𝤓'
    SignwritingRubBetween,
    /// \u{1d914}: '𝤔'
    SignwritingSurfaceSymbols,
    /// \u{1d915}: '𝤕'
    SignwritingSurfaceBetween,
    /// \u{1d916}: '𝤖'
    SignwritingSqueezeLargeSingle,
    /// \u{1d917}: '𝤗'
    SignwritingSqueezeSmallSingle,
    /// \u{1d918}: '𝤘'
    SignwritingSqueezeLargeMultiple,
    /// \u{1d919}: '𝤙'
    SignwritingSqueezeSmallMultiple,
    /// \u{1d91a}: '𝤚'
    SignwritingSqueezeSequential,
    /// \u{1d91b}: '𝤛'
    SignwritingFlickLargeSingle,
    /// \u{1d91c}: '𝤜'
    SignwritingFlickSmallSingle,
    /// \u{1d91d}: '𝤝'
    SignwritingFlickLargeMultiple,
    /// \u{1d91e}: '𝤞'
    SignwritingFlickSmallMultiple,
    /// \u{1d91f}: '𝤟'
    SignwritingFlickSequential,
    /// \u{1d920}: '𝤠'
    SignwritingSqueezeFlickAlternating,
    /// \u{1d921}: '𝤡'
    SignwritingMovementDashHingeUpDownLarge,
    /// \u{1d922}: '𝤢'
    SignwritingMovementDashHingeUpDownSmall,
    /// \u{1d923}: '𝤣'
    SignwritingMovementDashHingeUpSequential,
    /// \u{1d924}: '𝤤'
    SignwritingMovementDashHingeDownSequential,
    /// \u{1d925}: '𝤥'
    SignwritingMovementDashHingeUpDownAlternatingLarge,
    /// \u{1d926}: '𝤦'
    SignwritingMovementDashHingeUpDownAlternatingSmall,
    /// \u{1d927}: '𝤧'
    SignwritingMovementDashHingeSideToSideScissors,
    /// \u{1d928}: '𝤨'
    SignwritingMovementDashWallplaneFingerContact,
    /// \u{1d929}: '𝤩'
    SignwritingMovementDashFloorplaneFingerContact,
    /// \u{1d92a}: '𝤪'
    SignwritingMovementDashWallplaneSingleStraightSmall,
    /// \u{1d92b}: '𝤫'
    SignwritingMovementDashWallplaneSingleStraightMedium,
    /// \u{1d92c}: '𝤬'
    SignwritingMovementDashWallplaneSingleStraightLarge,
    /// \u{1d92d}: '𝤭'
    SignwritingMovementDashWallplaneSingleStraightLargest,
    /// \u{1d92e}: '𝤮'
    SignwritingMovementDashWallplaneSingleWristFlex,
    /// \u{1d92f}: '𝤯'
    SignwritingMovementDashWallplaneDoubleStraight,
    /// \u{1d930}: '𝤰'
    SignwritingMovementDashWallplaneDoubleWristFlex,
    /// \u{1d931}: '𝤱'
    SignwritingMovementDashWallplaneDoubleAlternating,
    /// \u{1d932}: '𝤲'
    SignwritingMovementDashWallplaneDoubleAlternatingWristFlex,
    /// \u{1d933}: '𝤳'
    SignwritingMovementDashWallplaneCross,
    /// \u{1d934}: '𝤴'
    SignwritingMovementDashWallplaneTripleStraightMovement,
    /// \u{1d935}: '𝤵'
    SignwritingMovementDashWallplaneTripleWristFlex,
    /// \u{1d936}: '𝤶'
    SignwritingMovementDashWallplaneTripleAlternating,
    /// \u{1d937}: '𝤷'
    SignwritingMovementDashWallplaneTripleAlternatingWristFlex,
    /// \u{1d938}: '𝤸'
    SignwritingMovementDashWallplaneBendSmall,
    /// \u{1d939}: '𝤹'
    SignwritingMovementDashWallplaneBendMedium,
    /// \u{1d93a}: '𝤺'
    SignwritingMovementDashWallplaneBendLarge,
    /// \u{1d93b}: '𝤻'
    SignwritingMovementDashWallplaneCornerSmall,
    /// \u{1d93c}: '𝤼'
    SignwritingMovementDashWallplaneCornerMedium,
    /// \u{1d93d}: '𝤽'
    SignwritingMovementDashWallplaneCornerLarge,
    /// \u{1d93e}: '𝤾'
    SignwritingMovementDashWallplaneCornerRotation,
    /// \u{1d93f}: '𝤿'
    SignwritingMovementDashWallplaneCheckSmall,
    /// \u{1d940}: '𝥀'
    SignwritingMovementDashWallplaneCheckMedium,
    /// \u{1d941}: '𝥁'
    SignwritingMovementDashWallplaneCheckLarge,
    /// \u{1d942}: '𝥂'
    SignwritingMovementDashWallplaneBoxSmall,
    /// \u{1d943}: '𝥃'
    SignwritingMovementDashWallplaneBoxMedium,
    /// \u{1d944}: '𝥄'
    SignwritingMovementDashWallplaneBoxLarge,
    /// \u{1d945}: '𝥅'
    SignwritingMovementDashWallplaneZigzagSmall,
    /// \u{1d946}: '𝥆'
    SignwritingMovementDashWallplaneZigzagMedium,
    /// \u{1d947}: '𝥇'
    SignwritingMovementDashWallplaneZigzagLarge,
    /// \u{1d948}: '𝥈'
    SignwritingMovementDashWallplanePeaksSmall,
    /// \u{1d949}: '𝥉'
    SignwritingMovementDashWallplanePeaksMedium,
    /// \u{1d94a}: '𝥊'
    SignwritingMovementDashWallplanePeaksLarge,
    /// \u{1d94b}: '𝥋'
    SignwritingTravelDashWallplaneRotationDashWallplaneSingle,
    /// \u{1d94c}: '𝥌'
    SignwritingTravelDashWallplaneRotationDashWallplaneDouble,
    /// \u{1d94d}: '𝥍'
    SignwritingTravelDashWallplaneRotationDashWallplaneAlternating,
    /// \u{1d94e}: '𝥎'
    SignwritingTravelDashWallplaneRotationDashFloorplaneSingle,
    /// \u{1d94f}: '𝥏'
    SignwritingTravelDashWallplaneRotationDashFloorplaneDouble,
    /// \u{1d950}: '𝥐'
    SignwritingTravelDashWallplaneRotationDashFloorplaneAlternating,
    /// \u{1d951}: '𝥑'
    SignwritingTravelDashWallplaneShaking,
    /// \u{1d952}: '𝥒'
    SignwritingTravelDashWallplaneArmSpiralSingle,
    /// \u{1d953}: '𝥓'
    SignwritingTravelDashWallplaneArmSpiralDouble,
    /// \u{1d954}: '𝥔'
    SignwritingTravelDashWallplaneArmSpiralTriple,
    /// \u{1d955}: '𝥕'
    SignwritingMovementDashDiagonalAwaySmall,
    /// \u{1d956}: '𝥖'
    SignwritingMovementDashDiagonalAwayMedium,
    /// \u{1d957}: '𝥗'
    SignwritingMovementDashDiagonalAwayLarge,
    /// \u{1d958}: '𝥘'
    SignwritingMovementDashDiagonalAwayLargest,
    /// \u{1d959}: '𝥙'
    SignwritingMovementDashDiagonalTowardsSmall,
    /// \u{1d95a}: '𝥚'
    SignwritingMovementDashDiagonalTowardsMedium,
    /// \u{1d95b}: '𝥛'
    SignwritingMovementDashDiagonalTowardsLarge,
    /// \u{1d95c}: '𝥜'
    SignwritingMovementDashDiagonalTowardsLargest,
    /// \u{1d95d}: '𝥝'
    SignwritingMovementDashDiagonalBetweenAwaySmall,
    /// \u{1d95e}: '𝥞'
    SignwritingMovementDashDiagonalBetweenAwayMedium,
    /// \u{1d95f}: '𝥟'
    SignwritingMovementDashDiagonalBetweenAwayLarge,
    /// \u{1d960}: '𝥠'
    SignwritingMovementDashDiagonalBetweenAwayLargest,
    /// \u{1d961}: '𝥡'
    SignwritingMovementDashDiagonalBetweenTowardsSmall,
    /// \u{1d962}: '𝥢'
    SignwritingMovementDashDiagonalBetweenTowardsMedium,
    /// \u{1d963}: '𝥣'
    SignwritingMovementDashDiagonalBetweenTowardsLarge,
    /// \u{1d964}: '𝥤'
    SignwritingMovementDashDiagonalBetweenTowardsLargest,
    /// \u{1d965}: '𝥥'
    SignwritingMovementDashFloorplaneSingleStraightSmall,
    /// \u{1d966}: '𝥦'
    SignwritingMovementDashFloorplaneSingleStraightMedium,
    /// \u{1d967}: '𝥧'
    SignwritingMovementDashFloorplaneSingleStraightLarge,
    /// \u{1d968}: '𝥨'
    SignwritingMovementDashFloorplaneSingleStraightLargest,
    /// \u{1d969}: '𝥩'
    SignwritingMovementDashFloorplaneSingleWristFlex,
    /// \u{1d96a}: '𝥪'
    SignwritingMovementDashFloorplaneDoubleStraight,
    /// \u{1d96b}: '𝥫'
    SignwritingMovementDashFloorplaneDoubleWristFlex,
    /// \u{1d96c}: '𝥬'
    SignwritingMovementDashFloorplaneDoubleAlternating,
    /// \u{1d96d}: '𝥭'
    SignwritingMovementDashFloorplaneDoubleAlternatingWristFlex,
    /// \u{1d96e}: '𝥮'
    SignwritingMovementDashFloorplaneCross,
    /// \u{1d96f}: '𝥯'
    SignwritingMovementDashFloorplaneTripleStraightMovement,
    /// \u{1d970}: '𝥰'
    SignwritingMovementDashFloorplaneTripleWristFlex,
    /// \u{1d971}: '𝥱'
    SignwritingMovementDashFloorplaneTripleAlternatingMovement,
    /// \u{1d972}: '𝥲'
    SignwritingMovementDashFloorplaneTripleAlternatingWristFlex,
    /// \u{1d973}: '𝥳'
    SignwritingMovementDashFloorplaneBend,
    /// \u{1d974}: '𝥴'
    SignwritingMovementDashFloorplaneCornerSmall,
    /// \u{1d975}: '𝥵'
    SignwritingMovementDashFloorplaneCornerMedium,
    /// \u{1d976}: '𝥶'
    SignwritingMovementDashFloorplaneCornerLarge,
    /// \u{1d977}: '𝥷'
    SignwritingMovementDashFloorplaneCheck,
    /// \u{1d978}: '𝥸'
    SignwritingMovementDashFloorplaneBoxSmall,
    /// \u{1d979}: '𝥹'
    SignwritingMovementDashFloorplaneBoxMedium,
    /// \u{1d97a}: '𝥺'
    SignwritingMovementDashFloorplaneBoxLarge,
    /// \u{1d97b}: '𝥻'
    SignwritingMovementDashFloorplaneZigzagSmall,
    /// \u{1d97c}: '𝥼'
    SignwritingMovementDashFloorplaneZigzagMedium,
    /// \u{1d97d}: '𝥽'
    SignwritingMovementDashFloorplaneZigzagLarge,
    /// \u{1d97e}: '𝥾'
    SignwritingMovementDashFloorplanePeaksSmall,
    /// \u{1d97f}: '𝥿'
    SignwritingMovementDashFloorplanePeaksMedium,
    /// \u{1d980}: '𝦀'
    SignwritingMovementDashFloorplanePeaksLarge,
    /// \u{1d981}: '𝦁'
    SignwritingTravelDashFloorplaneRotationDashFloorplaneSingle,
    /// \u{1d982}: '𝦂'
    SignwritingTravelDashFloorplaneRotationDashFloorplaneDouble,
    /// \u{1d983}: '𝦃'
    SignwritingTravelDashFloorplaneRotationDashFloorplaneAlternating,
    /// \u{1d984}: '𝦄'
    SignwritingTravelDashFloorplaneRotationDashWallplaneSingle,
    /// \u{1d985}: '𝦅'
    SignwritingTravelDashFloorplaneRotationDashWallplaneDouble,
    /// \u{1d986}: '𝦆'
    SignwritingTravelDashFloorplaneRotationDashWallplaneAlternating,
    /// \u{1d987}: '𝦇'
    SignwritingTravelDashFloorplaneShaking,
    /// \u{1d988}: '𝦈'
    SignwritingMovementDashWallplaneCurveQuarterSmall,
    /// \u{1d989}: '𝦉'
    SignwritingMovementDashWallplaneCurveQuarterMedium,
    /// \u{1d98a}: '𝦊'
    SignwritingMovementDashWallplaneCurveQuarterLarge,
    /// \u{1d98b}: '𝦋'
    SignwritingMovementDashWallplaneCurveQuarterLargest,
    /// \u{1d98c}: '𝦌'
    SignwritingMovementDashWallplaneCurveHalfDashCircleSmall,
    /// \u{1d98d}: '𝦍'
    SignwritingMovementDashWallplaneCurveHalfDashCircleMedium,
    /// \u{1d98e}: '𝦎'
    SignwritingMovementDashWallplaneCurveHalfDashCircleLarge,
    /// \u{1d98f}: '𝦏'
    SignwritingMovementDashWallplaneCurveHalfDashCircleLargest,
    /// \u{1d990}: '𝦐'
    SignwritingMovementDashWallplaneCurveThreeDashQuarterCircleSmall,
    /// \u{1d991}: '𝦑'
    SignwritingMovementDashWallplaneCurveThreeDashQuarterCircleMedium,
    /// \u{1d992}: '𝦒'
    SignwritingMovementDashWallplaneHumpSmall,
    /// \u{1d993}: '𝦓'
    SignwritingMovementDashWallplaneHumpMedium,
    /// \u{1d994}: '𝦔'
    SignwritingMovementDashWallplaneHumpLarge,
    /// \u{1d995}: '𝦕'
    SignwritingMovementDashWallplaneLoopSmall,
    /// \u{1d996}: '𝦖'
    SignwritingMovementDashWallplaneLoopMedium,
    /// \u{1d997}: '𝦗'
    SignwritingMovementDashWallplaneLoopLarge,
    /// \u{1d998}: '𝦘'
    SignwritingMovementDashWallplaneLoopSmallDouble,
    /// \u{1d999}: '𝦙'
    SignwritingMovementDashWallplaneWaveCurveDoubleSmall,
    /// \u{1d99a}: '𝦚'
    SignwritingMovementDashWallplaneWaveCurveDoubleMedium,
    /// \u{1d99b}: '𝦛'
    SignwritingMovementDashWallplaneWaveCurveDoubleLarge,
    /// \u{1d99c}: '𝦜'
    SignwritingMovementDashWallplaneWaveCurveTripleSmall,
    /// \u{1d99d}: '𝦝'
    SignwritingMovementDashWallplaneWaveCurveTripleMedium,
    /// \u{1d99e}: '𝦞'
    SignwritingMovementDashWallplaneWaveCurveTripleLarge,
    /// \u{1d99f}: '𝦟'
    SignwritingMovementDashWallplaneCurveThenStraight,
    /// \u{1d9a0}: '𝦠'
    SignwritingMovementDashWallplaneCurvedCrossSmall,
    /// \u{1d9a1}: '𝦡'
    SignwritingMovementDashWallplaneCurvedCrossMedium,
    /// \u{1d9a2}: '𝦢'
    SignwritingRotationDashWallplaneSingle,
    /// \u{1d9a3}: '𝦣'
    SignwritingRotationDashWallplaneDouble,
    /// \u{1d9a4}: '𝦤'
    SignwritingRotationDashWallplaneAlternate,
    /// \u{1d9a5}: '𝦥'
    SignwritingMovementDashWallplaneShaking,
    /// \u{1d9a6}: '𝦦'
    SignwritingMovementDashWallplaneCurveHittingFrontWall,
    /// \u{1d9a7}: '𝦧'
    SignwritingMovementDashWallplaneHumpHittingFrontWall,
    /// \u{1d9a8}: '𝦨'
    SignwritingMovementDashWallplaneLoopHittingFrontWall,
    /// \u{1d9a9}: '𝦩'
    SignwritingMovementDashWallplaneWaveHittingFrontWall,
    /// \u{1d9aa}: '𝦪'
    SignwritingRotationDashWallplaneSingleHittingFrontWall,
    /// \u{1d9ab}: '𝦫'
    SignwritingRotationDashWallplaneDoubleHittingFrontWall,
    /// \u{1d9ac}: '𝦬'
    SignwritingRotationDashWallplaneAlternatingHittingFrontWall,
    /// \u{1d9ad}: '𝦭'
    SignwritingMovementDashWallplaneCurveHittingChest,
    /// \u{1d9ae}: '𝦮'
    SignwritingMovementDashWallplaneHumpHittingChest,
    /// \u{1d9af}: '𝦯'
    SignwritingMovementDashWallplaneLoopHittingChest,
    /// \u{1d9b0}: '𝦰'
    SignwritingMovementDashWallplaneWaveHittingChest,
    /// \u{1d9b1}: '𝦱'
    SignwritingRotationDashWallplaneSingleHittingChest,
    /// \u{1d9b2}: '𝦲'
    SignwritingRotationDashWallplaneDoubleHittingChest,
    /// \u{1d9b3}: '𝦳'
    SignwritingRotationDashWallplaneAlternatingHittingChest,
    /// \u{1d9b4}: '𝦴'
    SignwritingMovementDashWallplaneWaveDiagonalPathSmall,
    /// \u{1d9b5}: '𝦵'
    SignwritingMovementDashWallplaneWaveDiagonalPathMedium,
    /// \u{1d9b6}: '𝦶'
    SignwritingMovementDashWallplaneWaveDiagonalPathLarge,
    /// \u{1d9b7}: '𝦷'
    SignwritingMovementDashFloorplaneCurveHittingCeilingSmall,
    /// \u{1d9b8}: '𝦸'
    SignwritingMovementDashFloorplaneCurveHittingCeilingLarge,
    /// \u{1d9b9}: '𝦹'
    SignwritingMovementDashFloorplaneHumpHittingCeilingSmallDouble,
    /// \u{1d9ba}: '𝦺'
    SignwritingMovementDashFloorplaneHumpHittingCeilingLargeDouble,
    /// \u{1d9bb}: '𝦻'
    SignwritingMovementDashFloorplaneHumpHittingCeilingSmallTriple,
    /// \u{1d9bc}: '𝦼'
    SignwritingMovementDashFloorplaneHumpHittingCeilingLargeTriple,
    /// \u{1d9bd}: '𝦽'
    SignwritingMovementDashFloorplaneLoopHittingCeilingSmallSingle,
    /// \u{1d9be}: '𝦾'
    SignwritingMovementDashFloorplaneLoopHittingCeilingLargeSingle,
    /// \u{1d9bf}: '𝦿'
    SignwritingMovementDashFloorplaneLoopHittingCeilingSmallDouble,
    /// \u{1d9c0}: '𝧀'
    SignwritingMovementDashFloorplaneLoopHittingCeilingLargeDouble,
    /// \u{1d9c1}: '𝧁'
    SignwritingMovementDashFloorplaneWaveHittingCeilingSmall,
    /// \u{1d9c2}: '𝧂'
    SignwritingMovementDashFloorplaneWaveHittingCeilingLarge,
    /// \u{1d9c3}: '𝧃'
    SignwritingRotationDashFloorplaneSingleHittingCeiling,
    /// \u{1d9c4}: '𝧄'
    SignwritingRotationDashFloorplaneDoubleHittingCeiling,
    /// \u{1d9c5}: '𝧅'
    SignwritingRotationDashFloorplaneAlternatingHittingCeiling,
    /// \u{1d9c6}: '𝧆'
    SignwritingMovementDashFloorplaneCurveHittingFloorSmall,
    /// \u{1d9c7}: '𝧇'
    SignwritingMovementDashFloorplaneCurveHittingFloorLarge,
    /// \u{1d9c8}: '𝧈'
    SignwritingMovementDashFloorplaneHumpHittingFloorSmallDouble,
    /// \u{1d9c9}: '𝧉'
    SignwritingMovementDashFloorplaneHumpHittingFloorLargeDouble,
    /// \u{1d9ca}: '𝧊'
    SignwritingMovementDashFloorplaneHumpHittingFloorTripleSmallTriple,
    /// \u{1d9cb}: '𝧋'
    SignwritingMovementDashFloorplaneHumpHittingFloorTripleLargeTriple,
    /// \u{1d9cc}: '𝧌'
    SignwritingMovementDashFloorplaneLoopHittingFloorSmallSingle,
    /// \u{1d9cd}: '𝧍'
    SignwritingMovementDashFloorplaneLoopHittingFloorLargeSingle,
    /// \u{1d9ce}: '𝧎'
    SignwritingMovementDashFloorplaneLoopHittingFloorSmallDouble,
    /// \u{1d9cf}: '𝧏'
    SignwritingMovementDashFloorplaneLoopHittingFloorLargeDouble,
    /// \u{1d9d0}: '𝧐'
    SignwritingMovementDashFloorplaneWaveHittingFloorSmall,
    /// \u{1d9d1}: '𝧑'
    SignwritingMovementDashFloorplaneWaveHittingFloorLarge,
    /// \u{1d9d2}: '𝧒'
    SignwritingRotationDashFloorplaneSingleHittingFloor,
    /// \u{1d9d3}: '𝧓'
    SignwritingRotationDashFloorplaneDoubleHittingFloor,
    /// \u{1d9d4}: '𝧔'
    SignwritingRotationDashFloorplaneAlternatingHittingFloor,
    /// \u{1d9d5}: '𝧕'
    SignwritingMovementDashFloorplaneCurveSmall,
    /// \u{1d9d6}: '𝧖'
    SignwritingMovementDashFloorplaneCurveMedium,
    /// \u{1d9d7}: '𝧗'
    SignwritingMovementDashFloorplaneCurveLarge,
    /// \u{1d9d8}: '𝧘'
    SignwritingMovementDashFloorplaneCurveLargest,
    /// \u{1d9d9}: '𝧙'
    SignwritingMovementDashFloorplaneCurveCombined,
    /// \u{1d9da}: '𝧚'
    SignwritingMovementDashFloorplaneHumpSmall,
    /// \u{1d9db}: '𝧛'
    SignwritingMovementDashFloorplaneLoopSmall,
    /// \u{1d9dc}: '𝧜'
    SignwritingMovementDashFloorplaneWaveSnake,
    /// \u{1d9dd}: '𝧝'
    SignwritingMovementDashFloorplaneWaveSmall,
    /// \u{1d9de}: '𝧞'
    SignwritingMovementDashFloorplaneWaveLarge,
    /// \u{1d9df}: '𝧟'
    SignwritingRotationDashFloorplaneSingle,
    /// \u{1d9e0}: '𝧠'
    SignwritingRotationDashFloorplaneDouble,
    /// \u{1d9e1}: '𝧡'
    SignwritingRotationDashFloorplaneAlternating,
    /// \u{1d9e2}: '𝧢'
    SignwritingMovementDashFloorplaneShakingParallel,
    /// \u{1d9e3}: '𝧣'
    SignwritingMovementDashWallplaneArmCircleSmallSingle,
    /// \u{1d9e4}: '𝧤'
    SignwritingMovementDashWallplaneArmCircleMediumSingle,
    /// \u{1d9e5}: '𝧥'
    SignwritingMovementDashWallplaneArmCircleSmallDouble,
    /// \u{1d9e6}: '𝧦'
    SignwritingMovementDashWallplaneArmCircleMediumDouble,
    /// \u{1d9e7}: '𝧧'
    SignwritingMovementDashFloorplaneArmCircleHittingWallSmallSingle,
    /// \u{1d9e8}: '𝧨'
    SignwritingMovementDashFloorplaneArmCircleHittingWallMediumSingle,
    /// \u{1d9e9}: '𝧩'
    SignwritingMovementDashFloorplaneArmCircleHittingWallLargeSingle,
    /// \u{1d9ea}: '𝧪'
    SignwritingMovementDashFloorplaneArmCircleHittingWallSmallDouble,
    /// \u{1d9eb}: '𝧫'
    SignwritingMovementDashFloorplaneArmCircleHittingWallMediumDouble,
    /// \u{1d9ec}: '𝧬'
    SignwritingMovementDashFloorplaneArmCircleHittingWallLargeDouble,
    /// \u{1d9ed}: '𝧭'
    SignwritingMovementDashWallplaneWristCircleFrontSingle,
    /// \u{1d9ee}: '𝧮'
    SignwritingMovementDashWallplaneWristCircleFrontDouble,
    /// \u{1d9ef}: '𝧯'
    SignwritingMovementDashFloorplaneWristCircleHittingWallSingle,
    /// \u{1d9f0}: '𝧰'
    SignwritingMovementDashFloorplaneWristCircleHittingWallDouble,
    /// \u{1d9f1}: '𝧱'
    SignwritingMovementDashWallplaneFingerCirclesSingle,
    /// \u{1d9f2}: '𝧲'
    SignwritingMovementDashWallplaneFingerCirclesDouble,
    /// \u{1d9f3}: '𝧳'
    SignwritingMovementDashFloorplaneFingerCirclesHittingWallSingle,
    /// \u{1d9f4}: '𝧴'
    SignwritingMovementDashFloorplaneFingerCirclesHittingWallDouble,
    /// \u{1d9f5}: '𝧵'
    SignwritingDynamicArrowheadSmall,
    /// \u{1d9f6}: '𝧶'
    SignwritingDynamicArrowheadLarge,
    /// \u{1d9f7}: '𝧷'
    SignwritingDynamicFast,
    /// \u{1d9f8}: '𝧸'
    SignwritingDynamicSlow,
    /// \u{1d9f9}: '𝧹'
    SignwritingDynamicTense,
    /// \u{1d9fa}: '𝧺'
    SignwritingDynamicRelaxed,
    /// \u{1d9fb}: '𝧻'
    SignwritingDynamicSimultaneous,
    /// \u{1d9fc}: '𝧼'
    SignwritingDynamicSimultaneousAlternating,
    /// \u{1d9fd}: '𝧽'
    SignwritingDynamicEveryOtherTime,
    /// \u{1d9fe}: '𝧾'
    SignwritingDynamicGradual,
    /// \u{1d9ff}: '𝧿'
    SignwritingHead,
    /// \u{1da00}: '𝨀'
    SignwritingHeadRim,
    /// \u{1da01}: '𝨁'
    SignwritingHeadMovementDashWallplaneStraight,
    /// \u{1da02}: '𝨂'
    SignwritingHeadMovementDashWallplaneTilt,
    /// \u{1da03}: '𝨃'
    SignwritingHeadMovementDashFloorplaneStraight,
    /// \u{1da04}: '𝨄'
    SignwritingHeadMovementDashWallplaneCurve,
    /// \u{1da05}: '𝨅'
    SignwritingHeadMovementDashFloorplaneCurve,
    /// \u{1da06}: '𝨆'
    SignwritingHeadMovementCircle,
    /// \u{1da07}: '𝨇'
    SignwritingFaceDirectionPositionNoseForwardTilting,
    /// \u{1da08}: '𝨈'
    SignwritingFaceDirectionPositionNoseUpOrDown,
    /// \u{1da09}: '𝨉'
    SignwritingFaceDirectionPositionNoseUpOrDownTilting,
    /// \u{1da0a}: '𝨊'
    SignwritingEyebrowsStraightUp,
    /// \u{1da0b}: '𝨋'
    SignwritingEyebrowsStraightNeutral,
    /// \u{1da0c}: '𝨌'
    SignwritingEyebrowsStraightDown,
    /// \u{1da0d}: '𝨍'
    SignwritingDreamyEyebrowsNeutralDown,
    /// \u{1da0e}: '𝨎'
    SignwritingDreamyEyebrowsDownNeutral,
    /// \u{1da0f}: '𝨏'
    SignwritingDreamyEyebrowsUpNeutral,
    /// \u{1da10}: '𝨐'
    SignwritingDreamyEyebrowsNeutralUp,
    /// \u{1da11}: '𝨑'
    SignwritingForeheadNeutral,
    /// \u{1da12}: '𝨒'
    SignwritingForeheadContact,
    /// \u{1da13}: '𝨓'
    SignwritingForeheadWrinkled,
    /// \u{1da14}: '𝨔'
    SignwritingEyesOpen,
    /// \u{1da15}: '𝨕'
    SignwritingEyesSqueezed,
    /// \u{1da16}: '𝨖'
    SignwritingEyesClosed,
    /// \u{1da17}: '𝨗'
    SignwritingEyeBlinkSingle,
    /// \u{1da18}: '𝨘'
    SignwritingEyeBlinkMultiple,
    /// \u{1da19}: '𝨙'
    SignwritingEyesHalfOpen,
    /// \u{1da1a}: '𝨚'
    SignwritingEyesWideOpen,
    /// \u{1da1b}: '𝨛'
    SignwritingEyesHalfClosed,
    /// \u{1da1c}: '𝨜'
    SignwritingEyesWideningMovement,
    /// \u{1da1d}: '𝨝'
    SignwritingEyeWink,
    /// \u{1da1e}: '𝨞'
    SignwritingEyelashesUp,
    /// \u{1da1f}: '𝨟'
    SignwritingEyelashesDown,
    /// \u{1da20}: '𝨠'
    SignwritingEyelashesFluttering,
    /// \u{1da21}: '𝨡'
    SignwritingEyegazeDashWallplaneStraight,
    /// \u{1da22}: '𝨢'
    SignwritingEyegazeDashWallplaneStraightDouble,
    /// \u{1da23}: '𝨣'
    SignwritingEyegazeDashWallplaneStraightAlternating,
    /// \u{1da24}: '𝨤'
    SignwritingEyegazeDashFloorplaneStraight,
    /// \u{1da25}: '𝨥'
    SignwritingEyegazeDashFloorplaneStraightDouble,
    /// \u{1da26}: '𝨦'
    SignwritingEyegazeDashFloorplaneStraightAlternating,
    /// \u{1da27}: '𝨧'
    SignwritingEyegazeDashWallplaneCurved,
    /// \u{1da28}: '𝨨'
    SignwritingEyegazeDashFloorplaneCurved,
    /// \u{1da29}: '𝨩'
    SignwritingEyegazeDashWallplaneCircling,
    /// \u{1da2a}: '𝨪'
    SignwritingCheeksPuffed,
    /// \u{1da2b}: '𝨫'
    SignwritingCheeksNeutral,
    /// \u{1da2c}: '𝨬'
    SignwritingCheeksSucked,
    /// \u{1da2d}: '𝨭'
    SignwritingTenseCheeksHigh,
    /// \u{1da2e}: '𝨮'
    SignwritingTenseCheeksMiddle,
    /// \u{1da2f}: '𝨯'
    SignwritingTenseCheeksLow,
    /// \u{1da30}: '𝨰'
    SignwritingEars,
    /// \u{1da31}: '𝨱'
    SignwritingNoseNeutral,
    /// \u{1da32}: '𝨲'
    SignwritingNoseContact,
    /// \u{1da33}: '𝨳'
    SignwritingNoseWrinkles,
    /// \u{1da34}: '𝨴'
    SignwritingNoseWiggles,
    /// \u{1da35}: '𝨵'
    SignwritingAirBlowingOut,
    /// \u{1da36}: '𝨶'
    SignwritingAirSuckingIn,
    /// \u{1da37}: '𝨷'
    SignwritingAirBlowSmallRotations,
    /// \u{1da38}: '𝨸'
    SignwritingAirSuckSmallRotations,
    /// \u{1da39}: '𝨹'
    SignwritingBreathInhale,
    /// \u{1da3a}: '𝨺'
    SignwritingBreathExhale,
    /// \u{1da3b}: '𝨻'
    SignwritingMouthClosedNeutral,
    /// \u{1da3c}: '𝨼'
    SignwritingMouthClosedForward,
    /// \u{1da3d}: '𝨽'
    SignwritingMouthClosedContact,
    /// \u{1da3e}: '𝨾'
    SignwritingMouthSmile,
    /// \u{1da3f}: '𝨿'
    SignwritingMouthSmileWrinkled,
    /// \u{1da40}: '𝩀'
    SignwritingMouthSmileOpen,
    /// \u{1da41}: '𝩁'
    SignwritingMouthFrown,
    /// \u{1da42}: '𝩂'
    SignwritingMouthFrownWrinkled,
    /// \u{1da43}: '𝩃'
    SignwritingMouthFrownOpen,
    /// \u{1da44}: '𝩄'
    SignwritingMouthOpenCircle,
    /// \u{1da45}: '𝩅'
    SignwritingMouthOpenForward,
    /// \u{1da46}: '𝩆'
    SignwritingMouthOpenWrinkled,
    /// \u{1da47}: '𝩇'
    SignwritingMouthOpenOval,
    /// \u{1da48}: '𝩈'
    SignwritingMouthOpenOvalWrinkled,
    /// \u{1da49}: '𝩉'
    SignwritingMouthOpenOvalYawn,
    /// \u{1da4a}: '𝩊'
    SignwritingMouthOpenRectangle,
    /// \u{1da4b}: '𝩋'
    SignwritingMouthOpenRectangleWrinkled,
    /// \u{1da4c}: '𝩌'
    SignwritingMouthOpenRectangleYawn,
    /// \u{1da4d}: '𝩍'
    SignwritingMouthKiss,
    /// \u{1da4e}: '𝩎'
    SignwritingMouthKissForward,
    /// \u{1da4f}: '𝩏'
    SignwritingMouthKissWrinkled,
    /// \u{1da50}: '𝩐'
    SignwritingMouthTense,
    /// \u{1da51}: '𝩑'
    SignwritingMouthTenseForward,
    /// \u{1da52}: '𝩒'
    SignwritingMouthTenseSucked,
    /// \u{1da53}: '𝩓'
    SignwritingLipsPressedTogether,
    /// \u{1da54}: '𝩔'
    SignwritingLipLowerOverUpper,
    /// \u{1da55}: '𝩕'
    SignwritingLipUpperOverLower,
    /// \u{1da56}: '𝩖'
    SignwritingMouthCorners,
    /// \u{1da57}: '𝩗'
    SignwritingMouthWrinklesSingle,
    /// \u{1da58}: '𝩘'
    SignwritingMouthWrinklesDouble,
    /// \u{1da59}: '𝩙'
    SignwritingTongueStickingOutFar,
    /// \u{1da5a}: '𝩚'
    SignwritingTongueLickingLips,
    /// \u{1da5b}: '𝩛'
    SignwritingTongueTipBetweenLips,
    /// \u{1da5c}: '𝩜'
    SignwritingTongueTipTouchingInsideMouth,
    /// \u{1da5d}: '𝩝'
    SignwritingTongueInsideMouthRelaxed,
    /// \u{1da5e}: '𝩞'
    SignwritingTongueMovesAgainstCheek,
    /// \u{1da5f}: '𝩟'
    SignwritingTongueCentreStickingOut,
    /// \u{1da60}: '𝩠'
    SignwritingTongueCentreInsideMouth,
    /// \u{1da61}: '𝩡'
    SignwritingTeeth,
    /// \u{1da62}: '𝩢'
    SignwritingTeethMovement,
    /// \u{1da63}: '𝩣'
    SignwritingTeethOnTongue,
    /// \u{1da64}: '𝩤'
    SignwritingTeethOnTongueMovement,
    /// \u{1da65}: '𝩥'
    SignwritingTeethOnLips,
    /// \u{1da66}: '𝩦'
    SignwritingTeethOnLipsMovement,
    /// \u{1da67}: '𝩧'
    SignwritingTeethBiteLips,
    /// \u{1da68}: '𝩨'
    SignwritingMovementDashWallplaneJaw,
    /// \u{1da69}: '𝩩'
    SignwritingMovementDashFloorplaneJaw,
    /// \u{1da6a}: '𝩪'
    SignwritingNeck,
    /// \u{1da6b}: '𝩫'
    SignwritingHair,
    /// \u{1da6c}: '𝩬'
    SignwritingExcitement,
    /// \u{1da6d}: '𝩭'
    SignwritingShoulderHipSpine,
    /// \u{1da6e}: '𝩮'
    SignwritingShoulderHipPositions,
    /// \u{1da6f}: '𝩯'
    SignwritingWallplaneShoulderHipMove,
    /// \u{1da70}: '𝩰'
    SignwritingFloorplaneShoulderHipMove,
    /// \u{1da71}: '𝩱'
    SignwritingShoulderTiltingFromWaist,
    /// \u{1da72}: '𝩲'
    SignwritingTorsoDashWallplaneStraightStretch,
    /// \u{1da73}: '𝩳'
    SignwritingTorsoDashWallplaneCurvedBend,
    /// \u{1da74}: '𝩴'
    SignwritingTorsoDashFloorplaneTwisting,
    /// \u{1da75}: '𝩵'
    SignwritingUpperBodyTiltingFromHipJoints,
    /// \u{1da76}: '𝩶'
    SignwritingLimbCombination,
    /// \u{1da77}: '𝩷'
    SignwritingLimbLengthDash1,
    /// \u{1da78}: '𝩸'
    SignwritingLimbLengthDash2,
    /// \u{1da79}: '𝩹'
    SignwritingLimbLengthDash3,
    /// \u{1da7a}: '𝩺'
    SignwritingLimbLengthDash4,
    /// \u{1da7b}: '𝩻'
    SignwritingLimbLengthDash5,
    /// \u{1da7c}: '𝩼'
    SignwritingLimbLengthDash6,
    /// \u{1da7d}: '𝩽'
    SignwritingLimbLengthDash7,
    /// \u{1da7e}: '𝩾'
    SignwritingFinger,
    /// \u{1da7f}: '𝩿'
    SignwritingLocationDashWallplaneSpace,
    /// \u{1da80}: '𝪀'
    SignwritingLocationDashFloorplaneSpace,
    /// \u{1da81}: '𝪁'
    SignwritingLocationHeight,
    /// \u{1da82}: '𝪂'
    SignwritingLocationWidth,
    /// \u{1da83}: '𝪃'
    SignwritingLocationDepth,
    /// \u{1da84}: '𝪄'
    SignwritingLocationHeadNeck,
    /// \u{1da85}: '𝪅'
    SignwritingLocationTorso,
    /// \u{1da86}: '𝪆'
    SignwritingLocationLimbsDigits,
    /// \u{1da87}: '𝪇'
    SignwritingComma,
    /// \u{1da88}: '𝪈'
    SignwritingFullStop,
    /// \u{1da89}: '𝪉'
    SignwritingSemicolon,
    /// \u{1da8a}: '𝪊'
    SignwritingColon,
    /// \u{1da8b}: '𝪋'
    SignwritingParenthesis,
    /// \u{1da9b}: '𝪛'
    SignwritingFillModifierDash2,
    /// \u{1da9c}: '𝪜'
    SignwritingFillModifierDash3,
    /// \u{1da9d}: '𝪝'
    SignwritingFillModifierDash4,
    /// \u{1da9e}: '𝪞'
    SignwritingFillModifierDash5,
    /// \u{1da9f}: '𝪟'
    SignwritingFillModifierDash6,
    /// \u{1daa1}: '𝪡'
    SignwritingRotationModifierDash2,
    /// \u{1daa2}: '𝪢'
    SignwritingRotationModifierDash3,
    /// \u{1daa3}: '𝪣'
    SignwritingRotationModifierDash4,
    /// \u{1daa4}: '𝪤'
    SignwritingRotationModifierDash5,
    /// \u{1daa5}: '𝪥'
    SignwritingRotationModifierDash6,
    /// \u{1daa6}: '𝪦'
    SignwritingRotationModifierDash7,
    /// \u{1daa7}: '𝪧'
    SignwritingRotationModifierDash8,
    /// \u{1daa8}: '𝪨'
    SignwritingRotationModifierDash9,
    /// \u{1daa9}: '𝪩'
    SignwritingRotationModifierDash10,
    /// \u{1daaa}: '𝪪'
    SignwritingRotationModifierDash11,
    /// \u{1daab}: '𝪫'
    SignwritingRotationModifierDash12,
    /// \u{1daac}: '𝪬'
    SignwritingRotationModifierDash13,
    /// \u{1daad}: '𝪭'
    SignwritingRotationModifierDash14,
    /// \u{1daae}: '𝪮'
    SignwritingRotationModifierDash15,
}

impl Into<char> for SuttonSignWriting {
    fn into(self) -> char {
        match self {
            SuttonSignWriting::SignwritingHandDashFistIndex => '𝠀',
            SuttonSignWriting::SignwritingHandDashCircleIndex => '𝠁',
            SuttonSignWriting::SignwritingHandDashCupIndex => '𝠂',
            SuttonSignWriting::SignwritingHandDashOvalIndex => '𝠃',
            SuttonSignWriting::SignwritingHandDashHingeIndex => '𝠄',
            SuttonSignWriting::SignwritingHandDashAngleIndex => '𝠅',
            SuttonSignWriting::SignwritingHandDashFistIndexBent => '𝠆',
            SuttonSignWriting::SignwritingHandDashCircleIndexBent => '𝠇',
            SuttonSignWriting::SignwritingHandDashFistThumbUnderIndexBent => '𝠈',
            SuttonSignWriting::SignwritingHandDashFistIndexRaisedKnuckle => '𝠉',
            SuttonSignWriting::SignwritingHandDashFistIndexCupped => '𝠊',
            SuttonSignWriting::SignwritingHandDashFistIndexHinged => '𝠋',
            SuttonSignWriting::SignwritingHandDashFistIndexHingedLow => '𝠌',
            SuttonSignWriting::SignwritingHandDashCircleIndexHinge => '𝠍',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddle => '𝠎',
            SuttonSignWriting::SignwritingHandDashCircleIndexMiddle => '𝠏',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleBent => '𝠐',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleRaisedKnuckles => '𝠑',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleHinged => '𝠒',
            SuttonSignWriting::SignwritingHandDashFistIndexUpMiddleHinged => '𝠓',
            SuttonSignWriting::SignwritingHandDashFistIndexHingedMiddleUp => '𝠔',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoined => '𝠕',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedIndexBent => '𝠖',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedMiddleBent => '𝠗',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedCupped => '𝠘',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedHinged => '𝠙',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleCrossed => '𝠚',
            SuttonSignWriting::SignwritingHandDashCircleIndexMiddleCrossed => '𝠛',
            SuttonSignWriting::SignwritingHandDashFistMiddleBentOverIndex => '𝠜',
            SuttonSignWriting::SignwritingHandDashFistIndexBentOverMiddle => '𝠝',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumb => '𝠞',
            SuttonSignWriting::SignwritingHandDashCircleIndexMiddleThumb => '𝠟',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleStraightThumbBent => '𝠠',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleBentThumbStraight => '𝠡',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbBent => '𝠢',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleHingedSpreadThumbSide => '𝠣',
            SuttonSignWriting::SignwritingHandDashFistIndexUpMiddleHingedThumbSide => '𝠤',
            SuttonSignWriting::SignwritingHandDashFistIndexUpMiddleHingedThumbConjoined => '𝠥',
            SuttonSignWriting::SignwritingHandDashFistIndexHingedMiddleUpThumbSide => '𝠦',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleUpSpreadThumbForward => '𝠧',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbCupped => '𝠨',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbCircled => '𝠩',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbHooked => '𝠪',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbHinged => '𝠫',
            SuttonSignWriting::SignwritingHandDashFistThumbBetweenIndexMiddleStraight => '𝠬',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedThumbSide => '𝠭',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedThumbSideConjoined => '𝠮',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedThumbSideBent => '𝠯',
            SuttonSignWriting::SignwritingHandDashFistMiddleThumbHookedIndexUp => '𝠰',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbHookedMiddleUp => '𝠱',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedHingedThumbSide => '𝠲',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleCrossedThumbSide => '𝠳',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedThumbForward => '𝠴',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedCuppedThumbForward => '𝠵',
            SuttonSignWriting::SignwritingHandDashFistMiddleThumbCuppedIndexUp => '𝠶',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbCuppedMiddleUp => '𝠷',
            SuttonSignWriting::SignwritingHandDashFistMiddleThumbCircledIndexUp => '𝠸',
            SuttonSignWriting::SignwritingHandDashFistMiddleThumbCircledIndexHinged => '𝠹',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbAngledOutMiddleUp => '𝠺',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbAngledInMiddleUp => '𝠻',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbCircledMiddleUp => '𝠼',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbConjoinedHinged => '𝠽',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbAngledOut => '𝠾',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbAngled => '𝠿',
            SuttonSignWriting::SignwritingHandDashFistMiddleThumbAngledOutIndexUp => '𝡀',
            SuttonSignWriting::SignwritingHandDashFistMiddleThumbAngledOutIndexCrossed => '𝡁',
            SuttonSignWriting::SignwritingHandDashFistMiddleThumbAngledIndexUp => '𝡂',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbHookedMiddleHinged => '𝡃',
            SuttonSignWriting::SignwritingHandDashFlatFourFingers => '𝡄',
            SuttonSignWriting::SignwritingHandDashFlatFourFingersBent => '𝡅',
            SuttonSignWriting::SignwritingHandDashFlatFourFingersHinged => '𝡆',
            SuttonSignWriting::SignwritingHandDashFlatFourFingersConjoined => '𝡇',
            SuttonSignWriting::SignwritingHandDashFlatFourFingersConjoinedSplit => '𝡈',
            SuttonSignWriting::SignwritingHandDashClawFourFingersConjoined => '𝡉',
            SuttonSignWriting::SignwritingHandDashFistFourFingersConjoinedBent => '𝡊',
            SuttonSignWriting::SignwritingHandDashHingeFourFingersConjoined => '𝡋',
            SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpread => '𝡌',
            SuttonSignWriting::SignwritingHandDashFlatHeelFiveFingersSpread => '𝡍',
            SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadFourBent => '𝡎',
            SuttonSignWriting::SignwritingHandDashFlatHeelFiveFingersSpreadFourBent => '𝡏',
            SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadBent => '𝡐',
            SuttonSignWriting::SignwritingHandDashFlatHeelFiveFingersSpreadBent => '𝡑',
            SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadThumbForward => '𝡒',
            SuttonSignWriting::SignwritingHandDashCupFiveFingersSpread => '𝡓',
            SuttonSignWriting::SignwritingHandDashCupFiveFingersSpreadOpen => '𝡔',
            SuttonSignWriting::SignwritingHandDashHingeFiveFingersSpreadOpen => '𝡕',
            SuttonSignWriting::SignwritingHandDashOvalFiveFingersSpread => '𝡖',
            SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadHinged => '𝡗',
            SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadHingedThumbSide => '𝡘',
            SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadHingedNoThumb => '𝡙',
            SuttonSignWriting::SignwritingHandDashFlat => '𝡚',
            SuttonSignWriting::SignwritingHandDashFlatBetweenPalmFacings => '𝡛',
            SuttonSignWriting::SignwritingHandDashFlatHeel => '𝡜',
            SuttonSignWriting::SignwritingHandDashFlatThumbSide => '𝡝',
            SuttonSignWriting::SignwritingHandDashFlatHeelThumbSide => '𝡞',
            SuttonSignWriting::SignwritingHandDashFlatThumbBent => '𝡟',
            SuttonSignWriting::SignwritingHandDashFlatThumbForward => '𝡠',
            SuttonSignWriting::SignwritingHandDashFlatSplitIndexThumbSide => '𝡡',
            SuttonSignWriting::SignwritingHandDashFlatSplitCentre => '𝡢',
            SuttonSignWriting::SignwritingHandDashFlatSplitCentreThumbSide => '𝡣',
            SuttonSignWriting::SignwritingHandDashFlatSplitCentreThumbSideBent => '𝡤',
            SuttonSignWriting::SignwritingHandDashFlatSplitLittle => '𝡥',
            SuttonSignWriting::SignwritingHandDashClaw => '𝡦',
            SuttonSignWriting::SignwritingHandDashClawThumbSide => '𝡧',
            SuttonSignWriting::SignwritingHandDashClawNoThumb => '𝡨',
            SuttonSignWriting::SignwritingHandDashClawThumbForward => '𝡩',
            SuttonSignWriting::SignwritingHandDashHookCurlicue => '𝡪',
            SuttonSignWriting::SignwritingHandDashHook => '𝡫',
            SuttonSignWriting::SignwritingHandDashCupOpen => '𝡬',
            SuttonSignWriting::SignwritingHandDashCup => '𝡭',
            SuttonSignWriting::SignwritingHandDashCupOpenThumbSide => '𝡮',
            SuttonSignWriting::SignwritingHandDashCupThumbSide => '𝡯',
            SuttonSignWriting::SignwritingHandDashCupOpenNoThumb => '𝡰',
            SuttonSignWriting::SignwritingHandDashCupNoThumb => '𝡱',
            SuttonSignWriting::SignwritingHandDashCupOpenThumbForward => '𝡲',
            SuttonSignWriting::SignwritingHandDashCupThumbForward => '𝡳',
            SuttonSignWriting::SignwritingHandDashCurlicueOpen => '𝡴',
            SuttonSignWriting::SignwritingHandDashCurlicue => '𝡵',
            SuttonSignWriting::SignwritingHandDashCircle => '𝡶',
            SuttonSignWriting::SignwritingHandDashOval => '𝡷',
            SuttonSignWriting::SignwritingHandDashOvalThumbSide => '𝡸',
            SuttonSignWriting::SignwritingHandDashOvalNoThumb => '𝡹',
            SuttonSignWriting::SignwritingHandDashOvalThumbForward => '𝡺',
            SuttonSignWriting::SignwritingHandDashHingeOpen => '𝡻',
            SuttonSignWriting::SignwritingHandDashHingeOpenThumbForward => '𝡼',
            SuttonSignWriting::SignwritingHandDashHinge => '𝡽',
            SuttonSignWriting::SignwritingHandDashHingeSmall => '𝡾',
            SuttonSignWriting::SignwritingHandDashHingeOpenThumbSide => '𝡿',
            SuttonSignWriting::SignwritingHandDashHingeThumbSide => '𝢀',
            SuttonSignWriting::SignwritingHandDashHingeOpenNoThumb => '𝢁',
            SuttonSignWriting::SignwritingHandDashHingeNoThumb => '𝢂',
            SuttonSignWriting::SignwritingHandDashHingeThumbSideTouchingIndex => '𝢃',
            SuttonSignWriting::SignwritingHandDashHingeThumbBetweenMiddleRing => '𝢄',
            SuttonSignWriting::SignwritingHandDashAngle => '𝢅',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleRing => '𝢆',
            SuttonSignWriting::SignwritingHandDashCircleIndexMiddleRing => '𝢇',
            SuttonSignWriting::SignwritingHandDashHingeIndexMiddleRing => '𝢈',
            SuttonSignWriting::SignwritingHandDashAngleIndexMiddleRing => '𝢉',
            SuttonSignWriting::SignwritingHandDashHingeLittle => '𝢊',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleRingBent => '𝢋',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleRingConjoined => '𝢌',
            SuttonSignWriting::SignwritingHandDashHingeIndexMiddleRingConjoined => '𝢍',
            SuttonSignWriting::SignwritingHandDashFistLittleDown => '𝢎',
            SuttonSignWriting::SignwritingHandDashFistLittleDownRippleStraight => '𝢏',
            SuttonSignWriting::SignwritingHandDashFistLittleDownRippleCurved => '𝢐',
            SuttonSignWriting::SignwritingHandDashFistLittleDownOthersCircled => '𝢑',
            SuttonSignWriting::SignwritingHandDashFistLittleUp => '𝢒',
            SuttonSignWriting::SignwritingHandDashFistThumbUnderLittleUp => '𝢓',
            SuttonSignWriting::SignwritingHandDashCircleLittleUp => '𝢔',
            SuttonSignWriting::SignwritingHandDashOvalLittleUp => '𝢕',
            SuttonSignWriting::SignwritingHandDashAngleLittleUp => '𝢖',
            SuttonSignWriting::SignwritingHandDashFistLittleRaisedKnuckle => '𝢗',
            SuttonSignWriting::SignwritingHandDashFistLittleBent => '𝢘',
            SuttonSignWriting::SignwritingHandDashFistLittleTouchesThumb => '𝢙',
            SuttonSignWriting::SignwritingHandDashFistLittleThumb => '𝢚',
            SuttonSignWriting::SignwritingHandDashHingeLittleThumb => '𝢛',
            SuttonSignWriting::SignwritingHandDashFistLittleIndexThumb => '𝢜',
            SuttonSignWriting::SignwritingHandDashHingeLittleIndexThumb => '𝢝',
            SuttonSignWriting::SignwritingHandDashAngleLittleIndexThumbIndexThumbOut => '𝢞',
            SuttonSignWriting::SignwritingHandDashAngleLittleIndexThumbIndexThumb => '𝢟',
            SuttonSignWriting::SignwritingHandDashFistLittleIndex => '𝢠',
            SuttonSignWriting::SignwritingHandDashCircleLittleIndex => '𝢡',
            SuttonSignWriting::SignwritingHandDashHingeLittleIndex => '𝢢',
            SuttonSignWriting::SignwritingHandDashAngleLittleIndex => '𝢣',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleLittle => '𝢤',
            SuttonSignWriting::SignwritingHandDashCircleIndexMiddleLittle => '𝢥',
            SuttonSignWriting::SignwritingHandDashHingeIndexMiddleLittle => '𝢦',
            SuttonSignWriting::SignwritingHandDashHingeRing => '𝢧',
            SuttonSignWriting::SignwritingHandDashAngleIndexMiddleLittle => '𝢨',
            SuttonSignWriting::SignwritingHandDashFistIndexMiddleCrossLittle => '𝢩',
            SuttonSignWriting::SignwritingHandDashCircleIndexMiddleCrossLittle => '𝢪',
            SuttonSignWriting::SignwritingHandDashFistRingDown => '𝢫',
            SuttonSignWriting::SignwritingHandDashHingeRingDownIndexThumbHookMiddle => '𝢬',
            SuttonSignWriting::SignwritingHandDashAngleRingDownMiddleThumbIndexCross => '𝢭',
            SuttonSignWriting::SignwritingHandDashFistRingUp => '𝢮',
            SuttonSignWriting::SignwritingHandDashFistRingRaisedKnuckle => '𝢯',
            SuttonSignWriting::SignwritingHandDashFistRingLittle => '𝢰',
            SuttonSignWriting::SignwritingHandDashCircleRingLittle => '𝢱',
            SuttonSignWriting::SignwritingHandDashOvalRingLittle => '𝢲',
            SuttonSignWriting::SignwritingHandDashAngleRingLittle => '𝢳',
            SuttonSignWriting::SignwritingHandDashFistRingMiddle => '𝢴',
            SuttonSignWriting::SignwritingHandDashFistRingMiddleConjoined => '𝢵',
            SuttonSignWriting::SignwritingHandDashFistRingMiddleRaisedKnuckles => '𝢶',
            SuttonSignWriting::SignwritingHandDashFistRingIndex => '𝢷',
            SuttonSignWriting::SignwritingHandDashFistRingThumb => '𝢸',
            SuttonSignWriting::SignwritingHandDashHookRingThumb => '𝢹',
            SuttonSignWriting::SignwritingHandDashFistIndexRingLittle => '𝢺',
            SuttonSignWriting::SignwritingHandDashCircleIndexRingLittle => '𝢻',
            SuttonSignWriting::SignwritingHandDashCurlicueIndexRingLittleOn => '𝢼',
            SuttonSignWriting::SignwritingHandDashHookIndexRingLittleOut => '𝢽',
            SuttonSignWriting::SignwritingHandDashHookIndexRingLittleIn => '𝢾',
            SuttonSignWriting::SignwritingHandDashHookIndexRingLittleUnder => '𝢿',
            SuttonSignWriting::SignwritingHandDashCupIndexRingLittle => '𝣀',
            SuttonSignWriting::SignwritingHandDashHingeIndexRingLittle => '𝣁',
            SuttonSignWriting::SignwritingHandDashAngleIndexRingLittleOut => '𝣂',
            SuttonSignWriting::SignwritingHandDashAngleIndexRingLittle => '𝣃',
            SuttonSignWriting::SignwritingHandDashFistMiddleDown => '𝣄',
            SuttonSignWriting::SignwritingHandDashHingeMiddle => '𝣅',
            SuttonSignWriting::SignwritingHandDashFistMiddleUp => '𝣆',
            SuttonSignWriting::SignwritingHandDashCircleMiddleUp => '𝣇',
            SuttonSignWriting::SignwritingHandDashFistMiddleRaisedKnuckle => '𝣈',
            SuttonSignWriting::SignwritingHandDashFistMiddleUpThumbSide => '𝣉',
            SuttonSignWriting::SignwritingHandDashHookMiddleThumb => '𝣊',
            SuttonSignWriting::SignwritingHandDashFistMiddleThumbLittle => '𝣋',
            SuttonSignWriting::SignwritingHandDashFistMiddleLittle => '𝣌',
            SuttonSignWriting::SignwritingHandDashFistMiddleRingLittle => '𝣍',
            SuttonSignWriting::SignwritingHandDashCircleMiddleRingLittle => '𝣎',
            SuttonSignWriting::SignwritingHandDashCurlicueMiddleRingLittleOn => '𝣏',
            SuttonSignWriting::SignwritingHandDashCupMiddleRingLittle => '𝣐',
            SuttonSignWriting::SignwritingHandDashHingeMiddleRingLittle => '𝣑',
            SuttonSignWriting::SignwritingHandDashAngleMiddleRingLittleOut => '𝣒',
            SuttonSignWriting::SignwritingHandDashAngleMiddleRingLittleIn => '𝣓',
            SuttonSignWriting::SignwritingHandDashAngleMiddleRingLittle => '𝣔',
            SuttonSignWriting::SignwritingHandDashCircleMiddleRingLittleBent => '𝣕',
            SuttonSignWriting::SignwritingHandDashClawMiddleRingLittleConjoined => '𝣖',
            SuttonSignWriting::SignwritingHandDashClawMiddleRingLittleConjoinedSide => '𝣗',
            SuttonSignWriting::SignwritingHandDashHookMiddleRingLittleConjoinedOut => '𝣘',
            SuttonSignWriting::SignwritingHandDashHookMiddleRingLittleConjoinedIn => '𝣙',
            SuttonSignWriting::SignwritingHandDashHookMiddleRingLittleConjoined => '𝣚',
            SuttonSignWriting::SignwritingHandDashHingeIndexHinged => '𝣛',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbSide => '𝣜',
            SuttonSignWriting::SignwritingHandDashHingeIndexThumbSide => '𝣝',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbSideThumbDiagonal => '𝣞',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbSideThumbConjoined => '𝣟',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbSideThumbBent => '𝣠',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbSideIndexBent => '𝣡',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbSideBothBent => '𝣢',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbSideIndexHinge => '𝣣',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbForwardIndexStraight => '𝣤',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbForwardIndexBent => '𝣥',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbHook => '𝣦',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbCurlicue => '𝣧',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbCurveThumbInside => '𝣨',
            SuttonSignWriting::SignwritingHandDashClawIndexThumbCurveThumbInside => '𝣩',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbCurveThumbUnder => '𝣪',
            SuttonSignWriting::SignwritingHandDashFistIndexThumbCircle => '𝣫',
            SuttonSignWriting::SignwritingHandDashCupIndexThumb => '𝣬',
            SuttonSignWriting::SignwritingHandDashCupIndexThumbOpen => '𝣭',
            SuttonSignWriting::SignwritingHandDashHingeIndexThumbOpen => '𝣮',
            SuttonSignWriting::SignwritingHandDashHingeIndexThumbLarge => '𝣯',
            SuttonSignWriting::SignwritingHandDashHingeIndexThumb => '𝣰',
            SuttonSignWriting::SignwritingHandDashHingeIndexThumbSmall => '𝣱',
            SuttonSignWriting::SignwritingHandDashAngleIndexThumbOut => '𝣲',
            SuttonSignWriting::SignwritingHandDashAngleIndexThumbIn => '𝣳',
            SuttonSignWriting::SignwritingHandDashAngleIndexThumb => '𝣴',
            SuttonSignWriting::SignwritingHandDashFistThumb => '𝣵',
            SuttonSignWriting::SignwritingHandDashFistThumbHeel => '𝣶',
            SuttonSignWriting::SignwritingHandDashFistThumbSideDiagonal => '𝣷',
            SuttonSignWriting::SignwritingHandDashFistThumbSideConjoined => '𝣸',
            SuttonSignWriting::SignwritingHandDashFistThumbSideBent => '𝣹',
            SuttonSignWriting::SignwritingHandDashFistThumbForward => '𝣺',
            SuttonSignWriting::SignwritingHandDashFistThumbBetweenIndexMiddle => '𝣻',
            SuttonSignWriting::SignwritingHandDashFistThumbBetweenMiddleRing => '𝣼',
            SuttonSignWriting::SignwritingHandDashFistThumbBetweenRingLittle => '𝣽',
            SuttonSignWriting::SignwritingHandDashFistThumbUnderTwoFingers => '𝣾',
            SuttonSignWriting::SignwritingHandDashFistThumbOverTwoFingers => '𝣿',
            SuttonSignWriting::SignwritingHandDashFistThumbUnderThreeFingers => '𝤀',
            SuttonSignWriting::SignwritingHandDashFistThumbUnderFourFingers => '𝤁',
            SuttonSignWriting::SignwritingHandDashFistThumbOverFourRaisedKnuckles => '𝤂',
            SuttonSignWriting::SignwritingHandDashFist => '𝤃',
            SuttonSignWriting::SignwritingHandDashFistHeel => '𝤄',
            SuttonSignWriting::SignwritingTouchSingle => '𝤅',
            SuttonSignWriting::SignwritingTouchMultiple => '𝤆',
            SuttonSignWriting::SignwritingTouchBetween => '𝤇',
            SuttonSignWriting::SignwritingGraspSingle => '𝤈',
            SuttonSignWriting::SignwritingGraspMultiple => '𝤉',
            SuttonSignWriting::SignwritingGraspBetween => '𝤊',
            SuttonSignWriting::SignwritingStrikeSingle => '𝤋',
            SuttonSignWriting::SignwritingStrikeMultiple => '𝤌',
            SuttonSignWriting::SignwritingStrikeBetween => '𝤍',
            SuttonSignWriting::SignwritingBrushSingle => '𝤎',
            SuttonSignWriting::SignwritingBrushMultiple => '𝤏',
            SuttonSignWriting::SignwritingBrushBetween => '𝤐',
            SuttonSignWriting::SignwritingRubSingle => '𝤑',
            SuttonSignWriting::SignwritingRubMultiple => '𝤒',
            SuttonSignWriting::SignwritingRubBetween => '𝤓',
            SuttonSignWriting::SignwritingSurfaceSymbols => '𝤔',
            SuttonSignWriting::SignwritingSurfaceBetween => '𝤕',
            SuttonSignWriting::SignwritingSqueezeLargeSingle => '𝤖',
            SuttonSignWriting::SignwritingSqueezeSmallSingle => '𝤗',
            SuttonSignWriting::SignwritingSqueezeLargeMultiple => '𝤘',
            SuttonSignWriting::SignwritingSqueezeSmallMultiple => '𝤙',
            SuttonSignWriting::SignwritingSqueezeSequential => '𝤚',
            SuttonSignWriting::SignwritingFlickLargeSingle => '𝤛',
            SuttonSignWriting::SignwritingFlickSmallSingle => '𝤜',
            SuttonSignWriting::SignwritingFlickLargeMultiple => '𝤝',
            SuttonSignWriting::SignwritingFlickSmallMultiple => '𝤞',
            SuttonSignWriting::SignwritingFlickSequential => '𝤟',
            SuttonSignWriting::SignwritingSqueezeFlickAlternating => '𝤠',
            SuttonSignWriting::SignwritingMovementDashHingeUpDownLarge => '𝤡',
            SuttonSignWriting::SignwritingMovementDashHingeUpDownSmall => '𝤢',
            SuttonSignWriting::SignwritingMovementDashHingeUpSequential => '𝤣',
            SuttonSignWriting::SignwritingMovementDashHingeDownSequential => '𝤤',
            SuttonSignWriting::SignwritingMovementDashHingeUpDownAlternatingLarge => '𝤥',
            SuttonSignWriting::SignwritingMovementDashHingeUpDownAlternatingSmall => '𝤦',
            SuttonSignWriting::SignwritingMovementDashHingeSideToSideScissors => '𝤧',
            SuttonSignWriting::SignwritingMovementDashWallplaneFingerContact => '𝤨',
            SuttonSignWriting::SignwritingMovementDashFloorplaneFingerContact => '𝤩',
            SuttonSignWriting::SignwritingMovementDashWallplaneSingleStraightSmall => '𝤪',
            SuttonSignWriting::SignwritingMovementDashWallplaneSingleStraightMedium => '𝤫',
            SuttonSignWriting::SignwritingMovementDashWallplaneSingleStraightLarge => '𝤬',
            SuttonSignWriting::SignwritingMovementDashWallplaneSingleStraightLargest => '𝤭',
            SuttonSignWriting::SignwritingMovementDashWallplaneSingleWristFlex => '𝤮',
            SuttonSignWriting::SignwritingMovementDashWallplaneDoubleStraight => '𝤯',
            SuttonSignWriting::SignwritingMovementDashWallplaneDoubleWristFlex => '𝤰',
            SuttonSignWriting::SignwritingMovementDashWallplaneDoubleAlternating => '𝤱',
            SuttonSignWriting::SignwritingMovementDashWallplaneDoubleAlternatingWristFlex => '𝤲',
            SuttonSignWriting::SignwritingMovementDashWallplaneCross => '𝤳',
            SuttonSignWriting::SignwritingMovementDashWallplaneTripleStraightMovement => '𝤴',
            SuttonSignWriting::SignwritingMovementDashWallplaneTripleWristFlex => '𝤵',
            SuttonSignWriting::SignwritingMovementDashWallplaneTripleAlternating => '𝤶',
            SuttonSignWriting::SignwritingMovementDashWallplaneTripleAlternatingWristFlex => '𝤷',
            SuttonSignWriting::SignwritingMovementDashWallplaneBendSmall => '𝤸',
            SuttonSignWriting::SignwritingMovementDashWallplaneBendMedium => '𝤹',
            SuttonSignWriting::SignwritingMovementDashWallplaneBendLarge => '𝤺',
            SuttonSignWriting::SignwritingMovementDashWallplaneCornerSmall => '𝤻',
            SuttonSignWriting::SignwritingMovementDashWallplaneCornerMedium => '𝤼',
            SuttonSignWriting::SignwritingMovementDashWallplaneCornerLarge => '𝤽',
            SuttonSignWriting::SignwritingMovementDashWallplaneCornerRotation => '𝤾',
            SuttonSignWriting::SignwritingMovementDashWallplaneCheckSmall => '𝤿',
            SuttonSignWriting::SignwritingMovementDashWallplaneCheckMedium => '𝥀',
            SuttonSignWriting::SignwritingMovementDashWallplaneCheckLarge => '𝥁',
            SuttonSignWriting::SignwritingMovementDashWallplaneBoxSmall => '𝥂',
            SuttonSignWriting::SignwritingMovementDashWallplaneBoxMedium => '𝥃',
            SuttonSignWriting::SignwritingMovementDashWallplaneBoxLarge => '𝥄',
            SuttonSignWriting::SignwritingMovementDashWallplaneZigzagSmall => '𝥅',
            SuttonSignWriting::SignwritingMovementDashWallplaneZigzagMedium => '𝥆',
            SuttonSignWriting::SignwritingMovementDashWallplaneZigzagLarge => '𝥇',
            SuttonSignWriting::SignwritingMovementDashWallplanePeaksSmall => '𝥈',
            SuttonSignWriting::SignwritingMovementDashWallplanePeaksMedium => '𝥉',
            SuttonSignWriting::SignwritingMovementDashWallplanePeaksLarge => '𝥊',
            SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashWallplaneSingle => '𝥋',
            SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashWallplaneDouble => '𝥌',
            SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashWallplaneAlternating => '𝥍',
            SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashFloorplaneSingle => '𝥎',
            SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashFloorplaneDouble => '𝥏',
            SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashFloorplaneAlternating => '𝥐',
            SuttonSignWriting::SignwritingTravelDashWallplaneShaking => '𝥑',
            SuttonSignWriting::SignwritingTravelDashWallplaneArmSpiralSingle => '𝥒',
            SuttonSignWriting::SignwritingTravelDashWallplaneArmSpiralDouble => '𝥓',
            SuttonSignWriting::SignwritingTravelDashWallplaneArmSpiralTriple => '𝥔',
            SuttonSignWriting::SignwritingMovementDashDiagonalAwaySmall => '𝥕',
            SuttonSignWriting::SignwritingMovementDashDiagonalAwayMedium => '𝥖',
            SuttonSignWriting::SignwritingMovementDashDiagonalAwayLarge => '𝥗',
            SuttonSignWriting::SignwritingMovementDashDiagonalAwayLargest => '𝥘',
            SuttonSignWriting::SignwritingMovementDashDiagonalTowardsSmall => '𝥙',
            SuttonSignWriting::SignwritingMovementDashDiagonalTowardsMedium => '𝥚',
            SuttonSignWriting::SignwritingMovementDashDiagonalTowardsLarge => '𝥛',
            SuttonSignWriting::SignwritingMovementDashDiagonalTowardsLargest => '𝥜',
            SuttonSignWriting::SignwritingMovementDashDiagonalBetweenAwaySmall => '𝥝',
            SuttonSignWriting::SignwritingMovementDashDiagonalBetweenAwayMedium => '𝥞',
            SuttonSignWriting::SignwritingMovementDashDiagonalBetweenAwayLarge => '𝥟',
            SuttonSignWriting::SignwritingMovementDashDiagonalBetweenAwayLargest => '𝥠',
            SuttonSignWriting::SignwritingMovementDashDiagonalBetweenTowardsSmall => '𝥡',
            SuttonSignWriting::SignwritingMovementDashDiagonalBetweenTowardsMedium => '𝥢',
            SuttonSignWriting::SignwritingMovementDashDiagonalBetweenTowardsLarge => '𝥣',
            SuttonSignWriting::SignwritingMovementDashDiagonalBetweenTowardsLargest => '𝥤',
            SuttonSignWriting::SignwritingMovementDashFloorplaneSingleStraightSmall => '𝥥',
            SuttonSignWriting::SignwritingMovementDashFloorplaneSingleStraightMedium => '𝥦',
            SuttonSignWriting::SignwritingMovementDashFloorplaneSingleStraightLarge => '𝥧',
            SuttonSignWriting::SignwritingMovementDashFloorplaneSingleStraightLargest => '𝥨',
            SuttonSignWriting::SignwritingMovementDashFloorplaneSingleWristFlex => '𝥩',
            SuttonSignWriting::SignwritingMovementDashFloorplaneDoubleStraight => '𝥪',
            SuttonSignWriting::SignwritingMovementDashFloorplaneDoubleWristFlex => '𝥫',
            SuttonSignWriting::SignwritingMovementDashFloorplaneDoubleAlternating => '𝥬',
            SuttonSignWriting::SignwritingMovementDashFloorplaneDoubleAlternatingWristFlex => '𝥭',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCross => '𝥮',
            SuttonSignWriting::SignwritingMovementDashFloorplaneTripleStraightMovement => '𝥯',
            SuttonSignWriting::SignwritingMovementDashFloorplaneTripleWristFlex => '𝥰',
            SuttonSignWriting::SignwritingMovementDashFloorplaneTripleAlternatingMovement => '𝥱',
            SuttonSignWriting::SignwritingMovementDashFloorplaneTripleAlternatingWristFlex => '𝥲',
            SuttonSignWriting::SignwritingMovementDashFloorplaneBend => '𝥳',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCornerSmall => '𝥴',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCornerMedium => '𝥵',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCornerLarge => '𝥶',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCheck => '𝥷',
            SuttonSignWriting::SignwritingMovementDashFloorplaneBoxSmall => '𝥸',
            SuttonSignWriting::SignwritingMovementDashFloorplaneBoxMedium => '𝥹',
            SuttonSignWriting::SignwritingMovementDashFloorplaneBoxLarge => '𝥺',
            SuttonSignWriting::SignwritingMovementDashFloorplaneZigzagSmall => '𝥻',
            SuttonSignWriting::SignwritingMovementDashFloorplaneZigzagMedium => '𝥼',
            SuttonSignWriting::SignwritingMovementDashFloorplaneZigzagLarge => '𝥽',
            SuttonSignWriting::SignwritingMovementDashFloorplanePeaksSmall => '𝥾',
            SuttonSignWriting::SignwritingMovementDashFloorplanePeaksMedium => '𝥿',
            SuttonSignWriting::SignwritingMovementDashFloorplanePeaksLarge => '𝦀',
            SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashFloorplaneSingle => '𝦁',
            SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashFloorplaneDouble => '𝦂',
            SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashFloorplaneAlternating => '𝦃',
            SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashWallplaneSingle => '𝦄',
            SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashWallplaneDouble => '𝦅',
            SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashWallplaneAlternating => '𝦆',
            SuttonSignWriting::SignwritingTravelDashFloorplaneShaking => '𝦇',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveQuarterSmall => '𝦈',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveQuarterMedium => '𝦉',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveQuarterLarge => '𝦊',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveQuarterLargest => '𝦋',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveHalfDashCircleSmall => '𝦌',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveHalfDashCircleMedium => '𝦍',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveHalfDashCircleLarge => '𝦎',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveHalfDashCircleLargest => '𝦏',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveThreeDashQuarterCircleSmall => '𝦐',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveThreeDashQuarterCircleMedium => '𝦑',
            SuttonSignWriting::SignwritingMovementDashWallplaneHumpSmall => '𝦒',
            SuttonSignWriting::SignwritingMovementDashWallplaneHumpMedium => '𝦓',
            SuttonSignWriting::SignwritingMovementDashWallplaneHumpLarge => '𝦔',
            SuttonSignWriting::SignwritingMovementDashWallplaneLoopSmall => '𝦕',
            SuttonSignWriting::SignwritingMovementDashWallplaneLoopMedium => '𝦖',
            SuttonSignWriting::SignwritingMovementDashWallplaneLoopLarge => '𝦗',
            SuttonSignWriting::SignwritingMovementDashWallplaneLoopSmallDouble => '𝦘',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveDoubleSmall => '𝦙',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveDoubleMedium => '𝦚',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveDoubleLarge => '𝦛',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveTripleSmall => '𝦜',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveTripleMedium => '𝦝',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveTripleLarge => '𝦞',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveThenStraight => '𝦟',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurvedCrossSmall => '𝦠',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurvedCrossMedium => '𝦡',
            SuttonSignWriting::SignwritingRotationDashWallplaneSingle => '𝦢',
            SuttonSignWriting::SignwritingRotationDashWallplaneDouble => '𝦣',
            SuttonSignWriting::SignwritingRotationDashWallplaneAlternate => '𝦤',
            SuttonSignWriting::SignwritingMovementDashWallplaneShaking => '𝦥',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveHittingFrontWall => '𝦦',
            SuttonSignWriting::SignwritingMovementDashWallplaneHumpHittingFrontWall => '𝦧',
            SuttonSignWriting::SignwritingMovementDashWallplaneLoopHittingFrontWall => '𝦨',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveHittingFrontWall => '𝦩',
            SuttonSignWriting::SignwritingRotationDashWallplaneSingleHittingFrontWall => '𝦪',
            SuttonSignWriting::SignwritingRotationDashWallplaneDoubleHittingFrontWall => '𝦫',
            SuttonSignWriting::SignwritingRotationDashWallplaneAlternatingHittingFrontWall => '𝦬',
            SuttonSignWriting::SignwritingMovementDashWallplaneCurveHittingChest => '𝦭',
            SuttonSignWriting::SignwritingMovementDashWallplaneHumpHittingChest => '𝦮',
            SuttonSignWriting::SignwritingMovementDashWallplaneLoopHittingChest => '𝦯',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveHittingChest => '𝦰',
            SuttonSignWriting::SignwritingRotationDashWallplaneSingleHittingChest => '𝦱',
            SuttonSignWriting::SignwritingRotationDashWallplaneDoubleHittingChest => '𝦲',
            SuttonSignWriting::SignwritingRotationDashWallplaneAlternatingHittingChest => '𝦳',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveDiagonalPathSmall => '𝦴',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveDiagonalPathMedium => '𝦵',
            SuttonSignWriting::SignwritingMovementDashWallplaneWaveDiagonalPathLarge => '𝦶',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCurveHittingCeilingSmall => '𝦷',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCurveHittingCeilingLarge => '𝦸',
            SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingCeilingSmallDouble => '𝦹',
            SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingCeilingLargeDouble => '𝦺',
            SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingCeilingSmallTriple => '𝦻',
            SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingCeilingLargeTriple => '𝦼',
            SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingCeilingSmallSingle => '𝦽',
            SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingCeilingLargeSingle => '𝦾',
            SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingCeilingSmallDouble => '𝦿',
            SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingCeilingLargeDouble => '𝧀',
            SuttonSignWriting::SignwritingMovementDashFloorplaneWaveHittingCeilingSmall => '𝧁',
            SuttonSignWriting::SignwritingMovementDashFloorplaneWaveHittingCeilingLarge => '𝧂',
            SuttonSignWriting::SignwritingRotationDashFloorplaneSingleHittingCeiling => '𝧃',
            SuttonSignWriting::SignwritingRotationDashFloorplaneDoubleHittingCeiling => '𝧄',
            SuttonSignWriting::SignwritingRotationDashFloorplaneAlternatingHittingCeiling => '𝧅',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCurveHittingFloorSmall => '𝧆',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCurveHittingFloorLarge => '𝧇',
            SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingFloorSmallDouble => '𝧈',
            SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingFloorLargeDouble => '𝧉',
            SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingFloorTripleSmallTriple => '𝧊',
            SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingFloorTripleLargeTriple => '𝧋',
            SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingFloorSmallSingle => '𝧌',
            SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingFloorLargeSingle => '𝧍',
            SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingFloorSmallDouble => '𝧎',
            SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingFloorLargeDouble => '𝧏',
            SuttonSignWriting::SignwritingMovementDashFloorplaneWaveHittingFloorSmall => '𝧐',
            SuttonSignWriting::SignwritingMovementDashFloorplaneWaveHittingFloorLarge => '𝧑',
            SuttonSignWriting::SignwritingRotationDashFloorplaneSingleHittingFloor => '𝧒',
            SuttonSignWriting::SignwritingRotationDashFloorplaneDoubleHittingFloor => '𝧓',
            SuttonSignWriting::SignwritingRotationDashFloorplaneAlternatingHittingFloor => '𝧔',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCurveSmall => '𝧕',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCurveMedium => '𝧖',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCurveLarge => '𝧗',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCurveLargest => '𝧘',
            SuttonSignWriting::SignwritingMovementDashFloorplaneCurveCombined => '𝧙',
            SuttonSignWriting::SignwritingMovementDashFloorplaneHumpSmall => '𝧚',
            SuttonSignWriting::SignwritingMovementDashFloorplaneLoopSmall => '𝧛',
            SuttonSignWriting::SignwritingMovementDashFloorplaneWaveSnake => '𝧜',
            SuttonSignWriting::SignwritingMovementDashFloorplaneWaveSmall => '𝧝',
            SuttonSignWriting::SignwritingMovementDashFloorplaneWaveLarge => '𝧞',
            SuttonSignWriting::SignwritingRotationDashFloorplaneSingle => '𝧟',
            SuttonSignWriting::SignwritingRotationDashFloorplaneDouble => '𝧠',
            SuttonSignWriting::SignwritingRotationDashFloorplaneAlternating => '𝧡',
            SuttonSignWriting::SignwritingMovementDashFloorplaneShakingParallel => '𝧢',
            SuttonSignWriting::SignwritingMovementDashWallplaneArmCircleSmallSingle => '𝧣',
            SuttonSignWriting::SignwritingMovementDashWallplaneArmCircleMediumSingle => '𝧤',
            SuttonSignWriting::SignwritingMovementDashWallplaneArmCircleSmallDouble => '𝧥',
            SuttonSignWriting::SignwritingMovementDashWallplaneArmCircleMediumDouble => '𝧦',
            SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallSmallSingle => '𝧧',
            SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallMediumSingle => '𝧨',
            SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallLargeSingle => '𝧩',
            SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallSmallDouble => '𝧪',
            SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallMediumDouble => '𝧫',
            SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallLargeDouble => '𝧬',
            SuttonSignWriting::SignwritingMovementDashWallplaneWristCircleFrontSingle => '𝧭',
            SuttonSignWriting::SignwritingMovementDashWallplaneWristCircleFrontDouble => '𝧮',
            SuttonSignWriting::SignwritingMovementDashFloorplaneWristCircleHittingWallSingle => '𝧯',
            SuttonSignWriting::SignwritingMovementDashFloorplaneWristCircleHittingWallDouble => '𝧰',
            SuttonSignWriting::SignwritingMovementDashWallplaneFingerCirclesSingle => '𝧱',
            SuttonSignWriting::SignwritingMovementDashWallplaneFingerCirclesDouble => '𝧲',
            SuttonSignWriting::SignwritingMovementDashFloorplaneFingerCirclesHittingWallSingle => '𝧳',
            SuttonSignWriting::SignwritingMovementDashFloorplaneFingerCirclesHittingWallDouble => '𝧴',
            SuttonSignWriting::SignwritingDynamicArrowheadSmall => '𝧵',
            SuttonSignWriting::SignwritingDynamicArrowheadLarge => '𝧶',
            SuttonSignWriting::SignwritingDynamicFast => '𝧷',
            SuttonSignWriting::SignwritingDynamicSlow => '𝧸',
            SuttonSignWriting::SignwritingDynamicTense => '𝧹',
            SuttonSignWriting::SignwritingDynamicRelaxed => '𝧺',
            SuttonSignWriting::SignwritingDynamicSimultaneous => '𝧻',
            SuttonSignWriting::SignwritingDynamicSimultaneousAlternating => '𝧼',
            SuttonSignWriting::SignwritingDynamicEveryOtherTime => '𝧽',
            SuttonSignWriting::SignwritingDynamicGradual => '𝧾',
            SuttonSignWriting::SignwritingHead => '𝧿',
            SuttonSignWriting::SignwritingHeadRim => '𝨀',
            SuttonSignWriting::SignwritingHeadMovementDashWallplaneStraight => '𝨁',
            SuttonSignWriting::SignwritingHeadMovementDashWallplaneTilt => '𝨂',
            SuttonSignWriting::SignwritingHeadMovementDashFloorplaneStraight => '𝨃',
            SuttonSignWriting::SignwritingHeadMovementDashWallplaneCurve => '𝨄',
            SuttonSignWriting::SignwritingHeadMovementDashFloorplaneCurve => '𝨅',
            SuttonSignWriting::SignwritingHeadMovementCircle => '𝨆',
            SuttonSignWriting::SignwritingFaceDirectionPositionNoseForwardTilting => '𝨇',
            SuttonSignWriting::SignwritingFaceDirectionPositionNoseUpOrDown => '𝨈',
            SuttonSignWriting::SignwritingFaceDirectionPositionNoseUpOrDownTilting => '𝨉',
            SuttonSignWriting::SignwritingEyebrowsStraightUp => '𝨊',
            SuttonSignWriting::SignwritingEyebrowsStraightNeutral => '𝨋',
            SuttonSignWriting::SignwritingEyebrowsStraightDown => '𝨌',
            SuttonSignWriting::SignwritingDreamyEyebrowsNeutralDown => '𝨍',
            SuttonSignWriting::SignwritingDreamyEyebrowsDownNeutral => '𝨎',
            SuttonSignWriting::SignwritingDreamyEyebrowsUpNeutral => '𝨏',
            SuttonSignWriting::SignwritingDreamyEyebrowsNeutralUp => '𝨐',
            SuttonSignWriting::SignwritingForeheadNeutral => '𝨑',
            SuttonSignWriting::SignwritingForeheadContact => '𝨒',
            SuttonSignWriting::SignwritingForeheadWrinkled => '𝨓',
            SuttonSignWriting::SignwritingEyesOpen => '𝨔',
            SuttonSignWriting::SignwritingEyesSqueezed => '𝨕',
            SuttonSignWriting::SignwritingEyesClosed => '𝨖',
            SuttonSignWriting::SignwritingEyeBlinkSingle => '𝨗',
            SuttonSignWriting::SignwritingEyeBlinkMultiple => '𝨘',
            SuttonSignWriting::SignwritingEyesHalfOpen => '𝨙',
            SuttonSignWriting::SignwritingEyesWideOpen => '𝨚',
            SuttonSignWriting::SignwritingEyesHalfClosed => '𝨛',
            SuttonSignWriting::SignwritingEyesWideningMovement => '𝨜',
            SuttonSignWriting::SignwritingEyeWink => '𝨝',
            SuttonSignWriting::SignwritingEyelashesUp => '𝨞',
            SuttonSignWriting::SignwritingEyelashesDown => '𝨟',
            SuttonSignWriting::SignwritingEyelashesFluttering => '𝨠',
            SuttonSignWriting::SignwritingEyegazeDashWallplaneStraight => '𝨡',
            SuttonSignWriting::SignwritingEyegazeDashWallplaneStraightDouble => '𝨢',
            SuttonSignWriting::SignwritingEyegazeDashWallplaneStraightAlternating => '𝨣',
            SuttonSignWriting::SignwritingEyegazeDashFloorplaneStraight => '𝨤',
            SuttonSignWriting::SignwritingEyegazeDashFloorplaneStraightDouble => '𝨥',
            SuttonSignWriting::SignwritingEyegazeDashFloorplaneStraightAlternating => '𝨦',
            SuttonSignWriting::SignwritingEyegazeDashWallplaneCurved => '𝨧',
            SuttonSignWriting::SignwritingEyegazeDashFloorplaneCurved => '𝨨',
            SuttonSignWriting::SignwritingEyegazeDashWallplaneCircling => '𝨩',
            SuttonSignWriting::SignwritingCheeksPuffed => '𝨪',
            SuttonSignWriting::SignwritingCheeksNeutral => '𝨫',
            SuttonSignWriting::SignwritingCheeksSucked => '𝨬',
            SuttonSignWriting::SignwritingTenseCheeksHigh => '𝨭',
            SuttonSignWriting::SignwritingTenseCheeksMiddle => '𝨮',
            SuttonSignWriting::SignwritingTenseCheeksLow => '𝨯',
            SuttonSignWriting::SignwritingEars => '𝨰',
            SuttonSignWriting::SignwritingNoseNeutral => '𝨱',
            SuttonSignWriting::SignwritingNoseContact => '𝨲',
            SuttonSignWriting::SignwritingNoseWrinkles => '𝨳',
            SuttonSignWriting::SignwritingNoseWiggles => '𝨴',
            SuttonSignWriting::SignwritingAirBlowingOut => '𝨵',
            SuttonSignWriting::SignwritingAirSuckingIn => '𝨶',
            SuttonSignWriting::SignwritingAirBlowSmallRotations => '𝨷',
            SuttonSignWriting::SignwritingAirSuckSmallRotations => '𝨸',
            SuttonSignWriting::SignwritingBreathInhale => '𝨹',
            SuttonSignWriting::SignwritingBreathExhale => '𝨺',
            SuttonSignWriting::SignwritingMouthClosedNeutral => '𝨻',
            SuttonSignWriting::SignwritingMouthClosedForward => '𝨼',
            SuttonSignWriting::SignwritingMouthClosedContact => '𝨽',
            SuttonSignWriting::SignwritingMouthSmile => '𝨾',
            SuttonSignWriting::SignwritingMouthSmileWrinkled => '𝨿',
            SuttonSignWriting::SignwritingMouthSmileOpen => '𝩀',
            SuttonSignWriting::SignwritingMouthFrown => '𝩁',
            SuttonSignWriting::SignwritingMouthFrownWrinkled => '𝩂',
            SuttonSignWriting::SignwritingMouthFrownOpen => '𝩃',
            SuttonSignWriting::SignwritingMouthOpenCircle => '𝩄',
            SuttonSignWriting::SignwritingMouthOpenForward => '𝩅',
            SuttonSignWriting::SignwritingMouthOpenWrinkled => '𝩆',
            SuttonSignWriting::SignwritingMouthOpenOval => '𝩇',
            SuttonSignWriting::SignwritingMouthOpenOvalWrinkled => '𝩈',
            SuttonSignWriting::SignwritingMouthOpenOvalYawn => '𝩉',
            SuttonSignWriting::SignwritingMouthOpenRectangle => '𝩊',
            SuttonSignWriting::SignwritingMouthOpenRectangleWrinkled => '𝩋',
            SuttonSignWriting::SignwritingMouthOpenRectangleYawn => '𝩌',
            SuttonSignWriting::SignwritingMouthKiss => '𝩍',
            SuttonSignWriting::SignwritingMouthKissForward => '𝩎',
            SuttonSignWriting::SignwritingMouthKissWrinkled => '𝩏',
            SuttonSignWriting::SignwritingMouthTense => '𝩐',
            SuttonSignWriting::SignwritingMouthTenseForward => '𝩑',
            SuttonSignWriting::SignwritingMouthTenseSucked => '𝩒',
            SuttonSignWriting::SignwritingLipsPressedTogether => '𝩓',
            SuttonSignWriting::SignwritingLipLowerOverUpper => '𝩔',
            SuttonSignWriting::SignwritingLipUpperOverLower => '𝩕',
            SuttonSignWriting::SignwritingMouthCorners => '𝩖',
            SuttonSignWriting::SignwritingMouthWrinklesSingle => '𝩗',
            SuttonSignWriting::SignwritingMouthWrinklesDouble => '𝩘',
            SuttonSignWriting::SignwritingTongueStickingOutFar => '𝩙',
            SuttonSignWriting::SignwritingTongueLickingLips => '𝩚',
            SuttonSignWriting::SignwritingTongueTipBetweenLips => '𝩛',
            SuttonSignWriting::SignwritingTongueTipTouchingInsideMouth => '𝩜',
            SuttonSignWriting::SignwritingTongueInsideMouthRelaxed => '𝩝',
            SuttonSignWriting::SignwritingTongueMovesAgainstCheek => '𝩞',
            SuttonSignWriting::SignwritingTongueCentreStickingOut => '𝩟',
            SuttonSignWriting::SignwritingTongueCentreInsideMouth => '𝩠',
            SuttonSignWriting::SignwritingTeeth => '𝩡',
            SuttonSignWriting::SignwritingTeethMovement => '𝩢',
            SuttonSignWriting::SignwritingTeethOnTongue => '𝩣',
            SuttonSignWriting::SignwritingTeethOnTongueMovement => '𝩤',
            SuttonSignWriting::SignwritingTeethOnLips => '𝩥',
            SuttonSignWriting::SignwritingTeethOnLipsMovement => '𝩦',
            SuttonSignWriting::SignwritingTeethBiteLips => '𝩧',
            SuttonSignWriting::SignwritingMovementDashWallplaneJaw => '𝩨',
            SuttonSignWriting::SignwritingMovementDashFloorplaneJaw => '𝩩',
            SuttonSignWriting::SignwritingNeck => '𝩪',
            SuttonSignWriting::SignwritingHair => '𝩫',
            SuttonSignWriting::SignwritingExcitement => '𝩬',
            SuttonSignWriting::SignwritingShoulderHipSpine => '𝩭',
            SuttonSignWriting::SignwritingShoulderHipPositions => '𝩮',
            SuttonSignWriting::SignwritingWallplaneShoulderHipMove => '𝩯',
            SuttonSignWriting::SignwritingFloorplaneShoulderHipMove => '𝩰',
            SuttonSignWriting::SignwritingShoulderTiltingFromWaist => '𝩱',
            SuttonSignWriting::SignwritingTorsoDashWallplaneStraightStretch => '𝩲',
            SuttonSignWriting::SignwritingTorsoDashWallplaneCurvedBend => '𝩳',
            SuttonSignWriting::SignwritingTorsoDashFloorplaneTwisting => '𝩴',
            SuttonSignWriting::SignwritingUpperBodyTiltingFromHipJoints => '𝩵',
            SuttonSignWriting::SignwritingLimbCombination => '𝩶',
            SuttonSignWriting::SignwritingLimbLengthDash1 => '𝩷',
            SuttonSignWriting::SignwritingLimbLengthDash2 => '𝩸',
            SuttonSignWriting::SignwritingLimbLengthDash3 => '𝩹',
            SuttonSignWriting::SignwritingLimbLengthDash4 => '𝩺',
            SuttonSignWriting::SignwritingLimbLengthDash5 => '𝩻',
            SuttonSignWriting::SignwritingLimbLengthDash6 => '𝩼',
            SuttonSignWriting::SignwritingLimbLengthDash7 => '𝩽',
            SuttonSignWriting::SignwritingFinger => '𝩾',
            SuttonSignWriting::SignwritingLocationDashWallplaneSpace => '𝩿',
            SuttonSignWriting::SignwritingLocationDashFloorplaneSpace => '𝪀',
            SuttonSignWriting::SignwritingLocationHeight => '𝪁',
            SuttonSignWriting::SignwritingLocationWidth => '𝪂',
            SuttonSignWriting::SignwritingLocationDepth => '𝪃',
            SuttonSignWriting::SignwritingLocationHeadNeck => '𝪄',
            SuttonSignWriting::SignwritingLocationTorso => '𝪅',
            SuttonSignWriting::SignwritingLocationLimbsDigits => '𝪆',
            SuttonSignWriting::SignwritingComma => '𝪇',
            SuttonSignWriting::SignwritingFullStop => '𝪈',
            SuttonSignWriting::SignwritingSemicolon => '𝪉',
            SuttonSignWriting::SignwritingColon => '𝪊',
            SuttonSignWriting::SignwritingParenthesis => '𝪋',
            SuttonSignWriting::SignwritingFillModifierDash2 => '𝪛',
            SuttonSignWriting::SignwritingFillModifierDash3 => '𝪜',
            SuttonSignWriting::SignwritingFillModifierDash4 => '𝪝',
            SuttonSignWriting::SignwritingFillModifierDash5 => '𝪞',
            SuttonSignWriting::SignwritingFillModifierDash6 => '𝪟',
            SuttonSignWriting::SignwritingRotationModifierDash2 => '𝪡',
            SuttonSignWriting::SignwritingRotationModifierDash3 => '𝪢',
            SuttonSignWriting::SignwritingRotationModifierDash4 => '𝪣',
            SuttonSignWriting::SignwritingRotationModifierDash5 => '𝪤',
            SuttonSignWriting::SignwritingRotationModifierDash6 => '𝪥',
            SuttonSignWriting::SignwritingRotationModifierDash7 => '𝪦',
            SuttonSignWriting::SignwritingRotationModifierDash8 => '𝪧',
            SuttonSignWriting::SignwritingRotationModifierDash9 => '𝪨',
            SuttonSignWriting::SignwritingRotationModifierDash10 => '𝪩',
            SuttonSignWriting::SignwritingRotationModifierDash11 => '𝪪',
            SuttonSignWriting::SignwritingRotationModifierDash12 => '𝪫',
            SuttonSignWriting::SignwritingRotationModifierDash13 => '𝪬',
            SuttonSignWriting::SignwritingRotationModifierDash14 => '𝪭',
            SuttonSignWriting::SignwritingRotationModifierDash15 => '𝪮',
        }
    }
}

impl std::convert::TryFrom<char> for SuttonSignWriting {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '𝠀' => Ok(SuttonSignWriting::SignwritingHandDashFistIndex),
            '𝠁' => Ok(SuttonSignWriting::SignwritingHandDashCircleIndex),
            '𝠂' => Ok(SuttonSignWriting::SignwritingHandDashCupIndex),
            '𝠃' => Ok(SuttonSignWriting::SignwritingHandDashOvalIndex),
            '𝠄' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndex),
            '𝠅' => Ok(SuttonSignWriting::SignwritingHandDashAngleIndex),
            '𝠆' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexBent),
            '𝠇' => Ok(SuttonSignWriting::SignwritingHandDashCircleIndexBent),
            '𝠈' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbUnderIndexBent),
            '𝠉' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexRaisedKnuckle),
            '𝠊' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexCupped),
            '𝠋' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexHinged),
            '𝠌' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexHingedLow),
            '𝠍' => Ok(SuttonSignWriting::SignwritingHandDashCircleIndexHinge),
            '𝠎' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddle),
            '𝠏' => Ok(SuttonSignWriting::SignwritingHandDashCircleIndexMiddle),
            '𝠐' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleBent),
            '𝠑' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleRaisedKnuckles),
            '𝠒' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleHinged),
            '𝠓' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexUpMiddleHinged),
            '𝠔' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexHingedMiddleUp),
            '𝠕' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoined),
            '𝠖' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedIndexBent),
            '𝠗' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedMiddleBent),
            '𝠘' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedCupped),
            '𝠙' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedHinged),
            '𝠚' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleCrossed),
            '𝠛' => Ok(SuttonSignWriting::SignwritingHandDashCircleIndexMiddleCrossed),
            '𝠜' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleBentOverIndex),
            '𝠝' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexBentOverMiddle),
            '𝠞' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumb),
            '𝠟' => Ok(SuttonSignWriting::SignwritingHandDashCircleIndexMiddleThumb),
            '𝠠' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleStraightThumbBent),
            '𝠡' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleBentThumbStraight),
            '𝠢' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbBent),
            '𝠣' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleHingedSpreadThumbSide),
            '𝠤' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexUpMiddleHingedThumbSide),
            '𝠥' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexUpMiddleHingedThumbConjoined),
            '𝠦' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexHingedMiddleUpThumbSide),
            '𝠧' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleUpSpreadThumbForward),
            '𝠨' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbCupped),
            '𝠩' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbCircled),
            '𝠪' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbHooked),
            '𝠫' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbHinged),
            '𝠬' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbBetweenIndexMiddleStraight),
            '𝠭' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedThumbSide),
            '𝠮' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedThumbSideConjoined),
            '𝠯' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedThumbSideBent),
            '𝠰' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleThumbHookedIndexUp),
            '𝠱' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbHookedMiddleUp),
            '𝠲' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedHingedThumbSide),
            '𝠳' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleCrossedThumbSide),
            '𝠴' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedThumbForward),
            '𝠵' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleConjoinedCuppedThumbForward),
            '𝠶' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleThumbCuppedIndexUp),
            '𝠷' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbCuppedMiddleUp),
            '𝠸' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleThumbCircledIndexUp),
            '𝠹' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleThumbCircledIndexHinged),
            '𝠺' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbAngledOutMiddleUp),
            '𝠻' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbAngledInMiddleUp),
            '𝠼' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbCircledMiddleUp),
            '𝠽' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbConjoinedHinged),
            '𝠾' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbAngledOut),
            '𝠿' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleThumbAngled),
            '𝡀' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleThumbAngledOutIndexUp),
            '𝡁' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleThumbAngledOutIndexCrossed),
            '𝡂' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleThumbAngledIndexUp),
            '𝡃' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbHookedMiddleHinged),
            '𝡄' => Ok(SuttonSignWriting::SignwritingHandDashFlatFourFingers),
            '𝡅' => Ok(SuttonSignWriting::SignwritingHandDashFlatFourFingersBent),
            '𝡆' => Ok(SuttonSignWriting::SignwritingHandDashFlatFourFingersHinged),
            '𝡇' => Ok(SuttonSignWriting::SignwritingHandDashFlatFourFingersConjoined),
            '𝡈' => Ok(SuttonSignWriting::SignwritingHandDashFlatFourFingersConjoinedSplit),
            '𝡉' => Ok(SuttonSignWriting::SignwritingHandDashClawFourFingersConjoined),
            '𝡊' => Ok(SuttonSignWriting::SignwritingHandDashFistFourFingersConjoinedBent),
            '𝡋' => Ok(SuttonSignWriting::SignwritingHandDashHingeFourFingersConjoined),
            '𝡌' => Ok(SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpread),
            '𝡍' => Ok(SuttonSignWriting::SignwritingHandDashFlatHeelFiveFingersSpread),
            '𝡎' => Ok(SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadFourBent),
            '𝡏' => Ok(SuttonSignWriting::SignwritingHandDashFlatHeelFiveFingersSpreadFourBent),
            '𝡐' => Ok(SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadBent),
            '𝡑' => Ok(SuttonSignWriting::SignwritingHandDashFlatHeelFiveFingersSpreadBent),
            '𝡒' => Ok(SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadThumbForward),
            '𝡓' => Ok(SuttonSignWriting::SignwritingHandDashCupFiveFingersSpread),
            '𝡔' => Ok(SuttonSignWriting::SignwritingHandDashCupFiveFingersSpreadOpen),
            '𝡕' => Ok(SuttonSignWriting::SignwritingHandDashHingeFiveFingersSpreadOpen),
            '𝡖' => Ok(SuttonSignWriting::SignwritingHandDashOvalFiveFingersSpread),
            '𝡗' => Ok(SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadHinged),
            '𝡘' => Ok(SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadHingedThumbSide),
            '𝡙' => Ok(SuttonSignWriting::SignwritingHandDashFlatFiveFingersSpreadHingedNoThumb),
            '𝡚' => Ok(SuttonSignWriting::SignwritingHandDashFlat),
            '𝡛' => Ok(SuttonSignWriting::SignwritingHandDashFlatBetweenPalmFacings),
            '𝡜' => Ok(SuttonSignWriting::SignwritingHandDashFlatHeel),
            '𝡝' => Ok(SuttonSignWriting::SignwritingHandDashFlatThumbSide),
            '𝡞' => Ok(SuttonSignWriting::SignwritingHandDashFlatHeelThumbSide),
            '𝡟' => Ok(SuttonSignWriting::SignwritingHandDashFlatThumbBent),
            '𝡠' => Ok(SuttonSignWriting::SignwritingHandDashFlatThumbForward),
            '𝡡' => Ok(SuttonSignWriting::SignwritingHandDashFlatSplitIndexThumbSide),
            '𝡢' => Ok(SuttonSignWriting::SignwritingHandDashFlatSplitCentre),
            '𝡣' => Ok(SuttonSignWriting::SignwritingHandDashFlatSplitCentreThumbSide),
            '𝡤' => Ok(SuttonSignWriting::SignwritingHandDashFlatSplitCentreThumbSideBent),
            '𝡥' => Ok(SuttonSignWriting::SignwritingHandDashFlatSplitLittle),
            '𝡦' => Ok(SuttonSignWriting::SignwritingHandDashClaw),
            '𝡧' => Ok(SuttonSignWriting::SignwritingHandDashClawThumbSide),
            '𝡨' => Ok(SuttonSignWriting::SignwritingHandDashClawNoThumb),
            '𝡩' => Ok(SuttonSignWriting::SignwritingHandDashClawThumbForward),
            '𝡪' => Ok(SuttonSignWriting::SignwritingHandDashHookCurlicue),
            '𝡫' => Ok(SuttonSignWriting::SignwritingHandDashHook),
            '𝡬' => Ok(SuttonSignWriting::SignwritingHandDashCupOpen),
            '𝡭' => Ok(SuttonSignWriting::SignwritingHandDashCup),
            '𝡮' => Ok(SuttonSignWriting::SignwritingHandDashCupOpenThumbSide),
            '𝡯' => Ok(SuttonSignWriting::SignwritingHandDashCupThumbSide),
            '𝡰' => Ok(SuttonSignWriting::SignwritingHandDashCupOpenNoThumb),
            '𝡱' => Ok(SuttonSignWriting::SignwritingHandDashCupNoThumb),
            '𝡲' => Ok(SuttonSignWriting::SignwritingHandDashCupOpenThumbForward),
            '𝡳' => Ok(SuttonSignWriting::SignwritingHandDashCupThumbForward),
            '𝡴' => Ok(SuttonSignWriting::SignwritingHandDashCurlicueOpen),
            '𝡵' => Ok(SuttonSignWriting::SignwritingHandDashCurlicue),
            '𝡶' => Ok(SuttonSignWriting::SignwritingHandDashCircle),
            '𝡷' => Ok(SuttonSignWriting::SignwritingHandDashOval),
            '𝡸' => Ok(SuttonSignWriting::SignwritingHandDashOvalThumbSide),
            '𝡹' => Ok(SuttonSignWriting::SignwritingHandDashOvalNoThumb),
            '𝡺' => Ok(SuttonSignWriting::SignwritingHandDashOvalThumbForward),
            '𝡻' => Ok(SuttonSignWriting::SignwritingHandDashHingeOpen),
            '𝡼' => Ok(SuttonSignWriting::SignwritingHandDashHingeOpenThumbForward),
            '𝡽' => Ok(SuttonSignWriting::SignwritingHandDashHinge),
            '𝡾' => Ok(SuttonSignWriting::SignwritingHandDashHingeSmall),
            '𝡿' => Ok(SuttonSignWriting::SignwritingHandDashHingeOpenThumbSide),
            '𝢀' => Ok(SuttonSignWriting::SignwritingHandDashHingeThumbSide),
            '𝢁' => Ok(SuttonSignWriting::SignwritingHandDashHingeOpenNoThumb),
            '𝢂' => Ok(SuttonSignWriting::SignwritingHandDashHingeNoThumb),
            '𝢃' => Ok(SuttonSignWriting::SignwritingHandDashHingeThumbSideTouchingIndex),
            '𝢄' => Ok(SuttonSignWriting::SignwritingHandDashHingeThumbBetweenMiddleRing),
            '𝢅' => Ok(SuttonSignWriting::SignwritingHandDashAngle),
            '𝢆' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleRing),
            '𝢇' => Ok(SuttonSignWriting::SignwritingHandDashCircleIndexMiddleRing),
            '𝢈' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndexMiddleRing),
            '𝢉' => Ok(SuttonSignWriting::SignwritingHandDashAngleIndexMiddleRing),
            '𝢊' => Ok(SuttonSignWriting::SignwritingHandDashHingeLittle),
            '𝢋' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleRingBent),
            '𝢌' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleRingConjoined),
            '𝢍' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndexMiddleRingConjoined),
            '𝢎' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleDown),
            '𝢏' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleDownRippleStraight),
            '𝢐' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleDownRippleCurved),
            '𝢑' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleDownOthersCircled),
            '𝢒' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleUp),
            '𝢓' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbUnderLittleUp),
            '𝢔' => Ok(SuttonSignWriting::SignwritingHandDashCircleLittleUp),
            '𝢕' => Ok(SuttonSignWriting::SignwritingHandDashOvalLittleUp),
            '𝢖' => Ok(SuttonSignWriting::SignwritingHandDashAngleLittleUp),
            '𝢗' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleRaisedKnuckle),
            '𝢘' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleBent),
            '𝢙' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleTouchesThumb),
            '𝢚' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleThumb),
            '𝢛' => Ok(SuttonSignWriting::SignwritingHandDashHingeLittleThumb),
            '𝢜' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleIndexThumb),
            '𝢝' => Ok(SuttonSignWriting::SignwritingHandDashHingeLittleIndexThumb),
            '𝢞' => Ok(SuttonSignWriting::SignwritingHandDashAngleLittleIndexThumbIndexThumbOut),
            '𝢟' => Ok(SuttonSignWriting::SignwritingHandDashAngleLittleIndexThumbIndexThumb),
            '𝢠' => Ok(SuttonSignWriting::SignwritingHandDashFistLittleIndex),
            '𝢡' => Ok(SuttonSignWriting::SignwritingHandDashCircleLittleIndex),
            '𝢢' => Ok(SuttonSignWriting::SignwritingHandDashHingeLittleIndex),
            '𝢣' => Ok(SuttonSignWriting::SignwritingHandDashAngleLittleIndex),
            '𝢤' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleLittle),
            '𝢥' => Ok(SuttonSignWriting::SignwritingHandDashCircleIndexMiddleLittle),
            '𝢦' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndexMiddleLittle),
            '𝢧' => Ok(SuttonSignWriting::SignwritingHandDashHingeRing),
            '𝢨' => Ok(SuttonSignWriting::SignwritingHandDashAngleIndexMiddleLittle),
            '𝢩' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexMiddleCrossLittle),
            '𝢪' => Ok(SuttonSignWriting::SignwritingHandDashCircleIndexMiddleCrossLittle),
            '𝢫' => Ok(SuttonSignWriting::SignwritingHandDashFistRingDown),
            '𝢬' => Ok(SuttonSignWriting::SignwritingHandDashHingeRingDownIndexThumbHookMiddle),
            '𝢭' => Ok(SuttonSignWriting::SignwritingHandDashAngleRingDownMiddleThumbIndexCross),
            '𝢮' => Ok(SuttonSignWriting::SignwritingHandDashFistRingUp),
            '𝢯' => Ok(SuttonSignWriting::SignwritingHandDashFistRingRaisedKnuckle),
            '𝢰' => Ok(SuttonSignWriting::SignwritingHandDashFistRingLittle),
            '𝢱' => Ok(SuttonSignWriting::SignwritingHandDashCircleRingLittle),
            '𝢲' => Ok(SuttonSignWriting::SignwritingHandDashOvalRingLittle),
            '𝢳' => Ok(SuttonSignWriting::SignwritingHandDashAngleRingLittle),
            '𝢴' => Ok(SuttonSignWriting::SignwritingHandDashFistRingMiddle),
            '𝢵' => Ok(SuttonSignWriting::SignwritingHandDashFistRingMiddleConjoined),
            '𝢶' => Ok(SuttonSignWriting::SignwritingHandDashFistRingMiddleRaisedKnuckles),
            '𝢷' => Ok(SuttonSignWriting::SignwritingHandDashFistRingIndex),
            '𝢸' => Ok(SuttonSignWriting::SignwritingHandDashFistRingThumb),
            '𝢹' => Ok(SuttonSignWriting::SignwritingHandDashHookRingThumb),
            '𝢺' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexRingLittle),
            '𝢻' => Ok(SuttonSignWriting::SignwritingHandDashCircleIndexRingLittle),
            '𝢼' => Ok(SuttonSignWriting::SignwritingHandDashCurlicueIndexRingLittleOn),
            '𝢽' => Ok(SuttonSignWriting::SignwritingHandDashHookIndexRingLittleOut),
            '𝢾' => Ok(SuttonSignWriting::SignwritingHandDashHookIndexRingLittleIn),
            '𝢿' => Ok(SuttonSignWriting::SignwritingHandDashHookIndexRingLittleUnder),
            '𝣀' => Ok(SuttonSignWriting::SignwritingHandDashCupIndexRingLittle),
            '𝣁' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndexRingLittle),
            '𝣂' => Ok(SuttonSignWriting::SignwritingHandDashAngleIndexRingLittleOut),
            '𝣃' => Ok(SuttonSignWriting::SignwritingHandDashAngleIndexRingLittle),
            '𝣄' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleDown),
            '𝣅' => Ok(SuttonSignWriting::SignwritingHandDashHingeMiddle),
            '𝣆' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleUp),
            '𝣇' => Ok(SuttonSignWriting::SignwritingHandDashCircleMiddleUp),
            '𝣈' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleRaisedKnuckle),
            '𝣉' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleUpThumbSide),
            '𝣊' => Ok(SuttonSignWriting::SignwritingHandDashHookMiddleThumb),
            '𝣋' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleThumbLittle),
            '𝣌' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleLittle),
            '𝣍' => Ok(SuttonSignWriting::SignwritingHandDashFistMiddleRingLittle),
            '𝣎' => Ok(SuttonSignWriting::SignwritingHandDashCircleMiddleRingLittle),
            '𝣏' => Ok(SuttonSignWriting::SignwritingHandDashCurlicueMiddleRingLittleOn),
            '𝣐' => Ok(SuttonSignWriting::SignwritingHandDashCupMiddleRingLittle),
            '𝣑' => Ok(SuttonSignWriting::SignwritingHandDashHingeMiddleRingLittle),
            '𝣒' => Ok(SuttonSignWriting::SignwritingHandDashAngleMiddleRingLittleOut),
            '𝣓' => Ok(SuttonSignWriting::SignwritingHandDashAngleMiddleRingLittleIn),
            '𝣔' => Ok(SuttonSignWriting::SignwritingHandDashAngleMiddleRingLittle),
            '𝣕' => Ok(SuttonSignWriting::SignwritingHandDashCircleMiddleRingLittleBent),
            '𝣖' => Ok(SuttonSignWriting::SignwritingHandDashClawMiddleRingLittleConjoined),
            '𝣗' => Ok(SuttonSignWriting::SignwritingHandDashClawMiddleRingLittleConjoinedSide),
            '𝣘' => Ok(SuttonSignWriting::SignwritingHandDashHookMiddleRingLittleConjoinedOut),
            '𝣙' => Ok(SuttonSignWriting::SignwritingHandDashHookMiddleRingLittleConjoinedIn),
            '𝣚' => Ok(SuttonSignWriting::SignwritingHandDashHookMiddleRingLittleConjoined),
            '𝣛' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndexHinged),
            '𝣜' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbSide),
            '𝣝' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndexThumbSide),
            '𝣞' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbSideThumbDiagonal),
            '𝣟' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbSideThumbConjoined),
            '𝣠' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbSideThumbBent),
            '𝣡' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbSideIndexBent),
            '𝣢' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbSideBothBent),
            '𝣣' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbSideIndexHinge),
            '𝣤' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbForwardIndexStraight),
            '𝣥' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbForwardIndexBent),
            '𝣦' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbHook),
            '𝣧' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbCurlicue),
            '𝣨' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbCurveThumbInside),
            '𝣩' => Ok(SuttonSignWriting::SignwritingHandDashClawIndexThumbCurveThumbInside),
            '𝣪' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbCurveThumbUnder),
            '𝣫' => Ok(SuttonSignWriting::SignwritingHandDashFistIndexThumbCircle),
            '𝣬' => Ok(SuttonSignWriting::SignwritingHandDashCupIndexThumb),
            '𝣭' => Ok(SuttonSignWriting::SignwritingHandDashCupIndexThumbOpen),
            '𝣮' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndexThumbOpen),
            '𝣯' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndexThumbLarge),
            '𝣰' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndexThumb),
            '𝣱' => Ok(SuttonSignWriting::SignwritingHandDashHingeIndexThumbSmall),
            '𝣲' => Ok(SuttonSignWriting::SignwritingHandDashAngleIndexThumbOut),
            '𝣳' => Ok(SuttonSignWriting::SignwritingHandDashAngleIndexThumbIn),
            '𝣴' => Ok(SuttonSignWriting::SignwritingHandDashAngleIndexThumb),
            '𝣵' => Ok(SuttonSignWriting::SignwritingHandDashFistThumb),
            '𝣶' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbHeel),
            '𝣷' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbSideDiagonal),
            '𝣸' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbSideConjoined),
            '𝣹' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbSideBent),
            '𝣺' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbForward),
            '𝣻' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbBetweenIndexMiddle),
            '𝣼' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbBetweenMiddleRing),
            '𝣽' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbBetweenRingLittle),
            '𝣾' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbUnderTwoFingers),
            '𝣿' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbOverTwoFingers),
            '𝤀' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbUnderThreeFingers),
            '𝤁' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbUnderFourFingers),
            '𝤂' => Ok(SuttonSignWriting::SignwritingHandDashFistThumbOverFourRaisedKnuckles),
            '𝤃' => Ok(SuttonSignWriting::SignwritingHandDashFist),
            '𝤄' => Ok(SuttonSignWriting::SignwritingHandDashFistHeel),
            '𝤅' => Ok(SuttonSignWriting::SignwritingTouchSingle),
            '𝤆' => Ok(SuttonSignWriting::SignwritingTouchMultiple),
            '𝤇' => Ok(SuttonSignWriting::SignwritingTouchBetween),
            '𝤈' => Ok(SuttonSignWriting::SignwritingGraspSingle),
            '𝤉' => Ok(SuttonSignWriting::SignwritingGraspMultiple),
            '𝤊' => Ok(SuttonSignWriting::SignwritingGraspBetween),
            '𝤋' => Ok(SuttonSignWriting::SignwritingStrikeSingle),
            '𝤌' => Ok(SuttonSignWriting::SignwritingStrikeMultiple),
            '𝤍' => Ok(SuttonSignWriting::SignwritingStrikeBetween),
            '𝤎' => Ok(SuttonSignWriting::SignwritingBrushSingle),
            '𝤏' => Ok(SuttonSignWriting::SignwritingBrushMultiple),
            '𝤐' => Ok(SuttonSignWriting::SignwritingBrushBetween),
            '𝤑' => Ok(SuttonSignWriting::SignwritingRubSingle),
            '𝤒' => Ok(SuttonSignWriting::SignwritingRubMultiple),
            '𝤓' => Ok(SuttonSignWriting::SignwritingRubBetween),
            '𝤔' => Ok(SuttonSignWriting::SignwritingSurfaceSymbols),
            '𝤕' => Ok(SuttonSignWriting::SignwritingSurfaceBetween),
            '𝤖' => Ok(SuttonSignWriting::SignwritingSqueezeLargeSingle),
            '𝤗' => Ok(SuttonSignWriting::SignwritingSqueezeSmallSingle),
            '𝤘' => Ok(SuttonSignWriting::SignwritingSqueezeLargeMultiple),
            '𝤙' => Ok(SuttonSignWriting::SignwritingSqueezeSmallMultiple),
            '𝤚' => Ok(SuttonSignWriting::SignwritingSqueezeSequential),
            '𝤛' => Ok(SuttonSignWriting::SignwritingFlickLargeSingle),
            '𝤜' => Ok(SuttonSignWriting::SignwritingFlickSmallSingle),
            '𝤝' => Ok(SuttonSignWriting::SignwritingFlickLargeMultiple),
            '𝤞' => Ok(SuttonSignWriting::SignwritingFlickSmallMultiple),
            '𝤟' => Ok(SuttonSignWriting::SignwritingFlickSequential),
            '𝤠' => Ok(SuttonSignWriting::SignwritingSqueezeFlickAlternating),
            '𝤡' => Ok(SuttonSignWriting::SignwritingMovementDashHingeUpDownLarge),
            '𝤢' => Ok(SuttonSignWriting::SignwritingMovementDashHingeUpDownSmall),
            '𝤣' => Ok(SuttonSignWriting::SignwritingMovementDashHingeUpSequential),
            '𝤤' => Ok(SuttonSignWriting::SignwritingMovementDashHingeDownSequential),
            '𝤥' => Ok(SuttonSignWriting::SignwritingMovementDashHingeUpDownAlternatingLarge),
            '𝤦' => Ok(SuttonSignWriting::SignwritingMovementDashHingeUpDownAlternatingSmall),
            '𝤧' => Ok(SuttonSignWriting::SignwritingMovementDashHingeSideToSideScissors),
            '𝤨' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneFingerContact),
            '𝤩' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneFingerContact),
            '𝤪' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneSingleStraightSmall),
            '𝤫' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneSingleStraightMedium),
            '𝤬' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneSingleStraightLarge),
            '𝤭' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneSingleStraightLargest),
            '𝤮' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneSingleWristFlex),
            '𝤯' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneDoubleStraight),
            '𝤰' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneDoubleWristFlex),
            '𝤱' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneDoubleAlternating),
            '𝤲' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneDoubleAlternatingWristFlex),
            '𝤳' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCross),
            '𝤴' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneTripleStraightMovement),
            '𝤵' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneTripleWristFlex),
            '𝤶' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneTripleAlternating),
            '𝤷' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneTripleAlternatingWristFlex),
            '𝤸' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneBendSmall),
            '𝤹' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneBendMedium),
            '𝤺' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneBendLarge),
            '𝤻' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCornerSmall),
            '𝤼' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCornerMedium),
            '𝤽' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCornerLarge),
            '𝤾' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCornerRotation),
            '𝤿' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCheckSmall),
            '𝥀' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCheckMedium),
            '𝥁' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCheckLarge),
            '𝥂' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneBoxSmall),
            '𝥃' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneBoxMedium),
            '𝥄' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneBoxLarge),
            '𝥅' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneZigzagSmall),
            '𝥆' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneZigzagMedium),
            '𝥇' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneZigzagLarge),
            '𝥈' => Ok(SuttonSignWriting::SignwritingMovementDashWallplanePeaksSmall),
            '𝥉' => Ok(SuttonSignWriting::SignwritingMovementDashWallplanePeaksMedium),
            '𝥊' => Ok(SuttonSignWriting::SignwritingMovementDashWallplanePeaksLarge),
            '𝥋' => Ok(SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashWallplaneSingle),
            '𝥌' => Ok(SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashWallplaneDouble),
            '𝥍' => Ok(SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashWallplaneAlternating),
            '𝥎' => Ok(SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashFloorplaneSingle),
            '𝥏' => Ok(SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashFloorplaneDouble),
            '𝥐' => Ok(SuttonSignWriting::SignwritingTravelDashWallplaneRotationDashFloorplaneAlternating),
            '𝥑' => Ok(SuttonSignWriting::SignwritingTravelDashWallplaneShaking),
            '𝥒' => Ok(SuttonSignWriting::SignwritingTravelDashWallplaneArmSpiralSingle),
            '𝥓' => Ok(SuttonSignWriting::SignwritingTravelDashWallplaneArmSpiralDouble),
            '𝥔' => Ok(SuttonSignWriting::SignwritingTravelDashWallplaneArmSpiralTriple),
            '𝥕' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalAwaySmall),
            '𝥖' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalAwayMedium),
            '𝥗' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalAwayLarge),
            '𝥘' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalAwayLargest),
            '𝥙' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalTowardsSmall),
            '𝥚' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalTowardsMedium),
            '𝥛' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalTowardsLarge),
            '𝥜' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalTowardsLargest),
            '𝥝' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalBetweenAwaySmall),
            '𝥞' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalBetweenAwayMedium),
            '𝥟' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalBetweenAwayLarge),
            '𝥠' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalBetweenAwayLargest),
            '𝥡' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalBetweenTowardsSmall),
            '𝥢' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalBetweenTowardsMedium),
            '𝥣' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalBetweenTowardsLarge),
            '𝥤' => Ok(SuttonSignWriting::SignwritingMovementDashDiagonalBetweenTowardsLargest),
            '𝥥' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneSingleStraightSmall),
            '𝥦' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneSingleStraightMedium),
            '𝥧' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneSingleStraightLarge),
            '𝥨' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneSingleStraightLargest),
            '𝥩' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneSingleWristFlex),
            '𝥪' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneDoubleStraight),
            '𝥫' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneDoubleWristFlex),
            '𝥬' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneDoubleAlternating),
            '𝥭' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneDoubleAlternatingWristFlex),
            '𝥮' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCross),
            '𝥯' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneTripleStraightMovement),
            '𝥰' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneTripleWristFlex),
            '𝥱' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneTripleAlternatingMovement),
            '𝥲' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneTripleAlternatingWristFlex),
            '𝥳' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneBend),
            '𝥴' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCornerSmall),
            '𝥵' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCornerMedium),
            '𝥶' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCornerLarge),
            '𝥷' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCheck),
            '𝥸' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneBoxSmall),
            '𝥹' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneBoxMedium),
            '𝥺' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneBoxLarge),
            '𝥻' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneZigzagSmall),
            '𝥼' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneZigzagMedium),
            '𝥽' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneZigzagLarge),
            '𝥾' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplanePeaksSmall),
            '𝥿' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplanePeaksMedium),
            '𝦀' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplanePeaksLarge),
            '𝦁' => Ok(SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashFloorplaneSingle),
            '𝦂' => Ok(SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashFloorplaneDouble),
            '𝦃' => Ok(SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashFloorplaneAlternating),
            '𝦄' => Ok(SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashWallplaneSingle),
            '𝦅' => Ok(SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashWallplaneDouble),
            '𝦆' => Ok(SuttonSignWriting::SignwritingTravelDashFloorplaneRotationDashWallplaneAlternating),
            '𝦇' => Ok(SuttonSignWriting::SignwritingTravelDashFloorplaneShaking),
            '𝦈' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveQuarterSmall),
            '𝦉' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveQuarterMedium),
            '𝦊' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveQuarterLarge),
            '𝦋' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveQuarterLargest),
            '𝦌' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveHalfDashCircleSmall),
            '𝦍' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveHalfDashCircleMedium),
            '𝦎' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveHalfDashCircleLarge),
            '𝦏' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveHalfDashCircleLargest),
            '𝦐' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveThreeDashQuarterCircleSmall),
            '𝦑' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveThreeDashQuarterCircleMedium),
            '𝦒' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneHumpSmall),
            '𝦓' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneHumpMedium),
            '𝦔' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneHumpLarge),
            '𝦕' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneLoopSmall),
            '𝦖' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneLoopMedium),
            '𝦗' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneLoopLarge),
            '𝦘' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneLoopSmallDouble),
            '𝦙' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveDoubleSmall),
            '𝦚' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveDoubleMedium),
            '𝦛' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveDoubleLarge),
            '𝦜' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveTripleSmall),
            '𝦝' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveTripleMedium),
            '𝦞' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveCurveTripleLarge),
            '𝦟' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveThenStraight),
            '𝦠' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurvedCrossSmall),
            '𝦡' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurvedCrossMedium),
            '𝦢' => Ok(SuttonSignWriting::SignwritingRotationDashWallplaneSingle),
            '𝦣' => Ok(SuttonSignWriting::SignwritingRotationDashWallplaneDouble),
            '𝦤' => Ok(SuttonSignWriting::SignwritingRotationDashWallplaneAlternate),
            '𝦥' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneShaking),
            '𝦦' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveHittingFrontWall),
            '𝦧' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneHumpHittingFrontWall),
            '𝦨' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneLoopHittingFrontWall),
            '𝦩' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveHittingFrontWall),
            '𝦪' => Ok(SuttonSignWriting::SignwritingRotationDashWallplaneSingleHittingFrontWall),
            '𝦫' => Ok(SuttonSignWriting::SignwritingRotationDashWallplaneDoubleHittingFrontWall),
            '𝦬' => Ok(SuttonSignWriting::SignwritingRotationDashWallplaneAlternatingHittingFrontWall),
            '𝦭' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneCurveHittingChest),
            '𝦮' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneHumpHittingChest),
            '𝦯' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneLoopHittingChest),
            '𝦰' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveHittingChest),
            '𝦱' => Ok(SuttonSignWriting::SignwritingRotationDashWallplaneSingleHittingChest),
            '𝦲' => Ok(SuttonSignWriting::SignwritingRotationDashWallplaneDoubleHittingChest),
            '𝦳' => Ok(SuttonSignWriting::SignwritingRotationDashWallplaneAlternatingHittingChest),
            '𝦴' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveDiagonalPathSmall),
            '𝦵' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveDiagonalPathMedium),
            '𝦶' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWaveDiagonalPathLarge),
            '𝦷' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCurveHittingCeilingSmall),
            '𝦸' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCurveHittingCeilingLarge),
            '𝦹' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingCeilingSmallDouble),
            '𝦺' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingCeilingLargeDouble),
            '𝦻' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingCeilingSmallTriple),
            '𝦼' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingCeilingLargeTriple),
            '𝦽' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingCeilingSmallSingle),
            '𝦾' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingCeilingLargeSingle),
            '𝦿' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingCeilingSmallDouble),
            '𝧀' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingCeilingLargeDouble),
            '𝧁' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneWaveHittingCeilingSmall),
            '𝧂' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneWaveHittingCeilingLarge),
            '𝧃' => Ok(SuttonSignWriting::SignwritingRotationDashFloorplaneSingleHittingCeiling),
            '𝧄' => Ok(SuttonSignWriting::SignwritingRotationDashFloorplaneDoubleHittingCeiling),
            '𝧅' => Ok(SuttonSignWriting::SignwritingRotationDashFloorplaneAlternatingHittingCeiling),
            '𝧆' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCurveHittingFloorSmall),
            '𝧇' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCurveHittingFloorLarge),
            '𝧈' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingFloorSmallDouble),
            '𝧉' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingFloorLargeDouble),
            '𝧊' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingFloorTripleSmallTriple),
            '𝧋' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneHumpHittingFloorTripleLargeTriple),
            '𝧌' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingFloorSmallSingle),
            '𝧍' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingFloorLargeSingle),
            '𝧎' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingFloorSmallDouble),
            '𝧏' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneLoopHittingFloorLargeDouble),
            '𝧐' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneWaveHittingFloorSmall),
            '𝧑' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneWaveHittingFloorLarge),
            '𝧒' => Ok(SuttonSignWriting::SignwritingRotationDashFloorplaneSingleHittingFloor),
            '𝧓' => Ok(SuttonSignWriting::SignwritingRotationDashFloorplaneDoubleHittingFloor),
            '𝧔' => Ok(SuttonSignWriting::SignwritingRotationDashFloorplaneAlternatingHittingFloor),
            '𝧕' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCurveSmall),
            '𝧖' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCurveMedium),
            '𝧗' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCurveLarge),
            '𝧘' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCurveLargest),
            '𝧙' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneCurveCombined),
            '𝧚' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneHumpSmall),
            '𝧛' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneLoopSmall),
            '𝧜' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneWaveSnake),
            '𝧝' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneWaveSmall),
            '𝧞' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneWaveLarge),
            '𝧟' => Ok(SuttonSignWriting::SignwritingRotationDashFloorplaneSingle),
            '𝧠' => Ok(SuttonSignWriting::SignwritingRotationDashFloorplaneDouble),
            '𝧡' => Ok(SuttonSignWriting::SignwritingRotationDashFloorplaneAlternating),
            '𝧢' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneShakingParallel),
            '𝧣' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneArmCircleSmallSingle),
            '𝧤' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneArmCircleMediumSingle),
            '𝧥' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneArmCircleSmallDouble),
            '𝧦' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneArmCircleMediumDouble),
            '𝧧' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallSmallSingle),
            '𝧨' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallMediumSingle),
            '𝧩' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallLargeSingle),
            '𝧪' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallSmallDouble),
            '𝧫' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallMediumDouble),
            '𝧬' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneArmCircleHittingWallLargeDouble),
            '𝧭' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWristCircleFrontSingle),
            '𝧮' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneWristCircleFrontDouble),
            '𝧯' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneWristCircleHittingWallSingle),
            '𝧰' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneWristCircleHittingWallDouble),
            '𝧱' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneFingerCirclesSingle),
            '𝧲' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneFingerCirclesDouble),
            '𝧳' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneFingerCirclesHittingWallSingle),
            '𝧴' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneFingerCirclesHittingWallDouble),
            '𝧵' => Ok(SuttonSignWriting::SignwritingDynamicArrowheadSmall),
            '𝧶' => Ok(SuttonSignWriting::SignwritingDynamicArrowheadLarge),
            '𝧷' => Ok(SuttonSignWriting::SignwritingDynamicFast),
            '𝧸' => Ok(SuttonSignWriting::SignwritingDynamicSlow),
            '𝧹' => Ok(SuttonSignWriting::SignwritingDynamicTense),
            '𝧺' => Ok(SuttonSignWriting::SignwritingDynamicRelaxed),
            '𝧻' => Ok(SuttonSignWriting::SignwritingDynamicSimultaneous),
            '𝧼' => Ok(SuttonSignWriting::SignwritingDynamicSimultaneousAlternating),
            '𝧽' => Ok(SuttonSignWriting::SignwritingDynamicEveryOtherTime),
            '𝧾' => Ok(SuttonSignWriting::SignwritingDynamicGradual),
            '𝧿' => Ok(SuttonSignWriting::SignwritingHead),
            '𝨀' => Ok(SuttonSignWriting::SignwritingHeadRim),
            '𝨁' => Ok(SuttonSignWriting::SignwritingHeadMovementDashWallplaneStraight),
            '𝨂' => Ok(SuttonSignWriting::SignwritingHeadMovementDashWallplaneTilt),
            '𝨃' => Ok(SuttonSignWriting::SignwritingHeadMovementDashFloorplaneStraight),
            '𝨄' => Ok(SuttonSignWriting::SignwritingHeadMovementDashWallplaneCurve),
            '𝨅' => Ok(SuttonSignWriting::SignwritingHeadMovementDashFloorplaneCurve),
            '𝨆' => Ok(SuttonSignWriting::SignwritingHeadMovementCircle),
            '𝨇' => Ok(SuttonSignWriting::SignwritingFaceDirectionPositionNoseForwardTilting),
            '𝨈' => Ok(SuttonSignWriting::SignwritingFaceDirectionPositionNoseUpOrDown),
            '𝨉' => Ok(SuttonSignWriting::SignwritingFaceDirectionPositionNoseUpOrDownTilting),
            '𝨊' => Ok(SuttonSignWriting::SignwritingEyebrowsStraightUp),
            '𝨋' => Ok(SuttonSignWriting::SignwritingEyebrowsStraightNeutral),
            '𝨌' => Ok(SuttonSignWriting::SignwritingEyebrowsStraightDown),
            '𝨍' => Ok(SuttonSignWriting::SignwritingDreamyEyebrowsNeutralDown),
            '𝨎' => Ok(SuttonSignWriting::SignwritingDreamyEyebrowsDownNeutral),
            '𝨏' => Ok(SuttonSignWriting::SignwritingDreamyEyebrowsUpNeutral),
            '𝨐' => Ok(SuttonSignWriting::SignwritingDreamyEyebrowsNeutralUp),
            '𝨑' => Ok(SuttonSignWriting::SignwritingForeheadNeutral),
            '𝨒' => Ok(SuttonSignWriting::SignwritingForeheadContact),
            '𝨓' => Ok(SuttonSignWriting::SignwritingForeheadWrinkled),
            '𝨔' => Ok(SuttonSignWriting::SignwritingEyesOpen),
            '𝨕' => Ok(SuttonSignWriting::SignwritingEyesSqueezed),
            '𝨖' => Ok(SuttonSignWriting::SignwritingEyesClosed),
            '𝨗' => Ok(SuttonSignWriting::SignwritingEyeBlinkSingle),
            '𝨘' => Ok(SuttonSignWriting::SignwritingEyeBlinkMultiple),
            '𝨙' => Ok(SuttonSignWriting::SignwritingEyesHalfOpen),
            '𝨚' => Ok(SuttonSignWriting::SignwritingEyesWideOpen),
            '𝨛' => Ok(SuttonSignWriting::SignwritingEyesHalfClosed),
            '𝨜' => Ok(SuttonSignWriting::SignwritingEyesWideningMovement),
            '𝨝' => Ok(SuttonSignWriting::SignwritingEyeWink),
            '𝨞' => Ok(SuttonSignWriting::SignwritingEyelashesUp),
            '𝨟' => Ok(SuttonSignWriting::SignwritingEyelashesDown),
            '𝨠' => Ok(SuttonSignWriting::SignwritingEyelashesFluttering),
            '𝨡' => Ok(SuttonSignWriting::SignwritingEyegazeDashWallplaneStraight),
            '𝨢' => Ok(SuttonSignWriting::SignwritingEyegazeDashWallplaneStraightDouble),
            '𝨣' => Ok(SuttonSignWriting::SignwritingEyegazeDashWallplaneStraightAlternating),
            '𝨤' => Ok(SuttonSignWriting::SignwritingEyegazeDashFloorplaneStraight),
            '𝨥' => Ok(SuttonSignWriting::SignwritingEyegazeDashFloorplaneStraightDouble),
            '𝨦' => Ok(SuttonSignWriting::SignwritingEyegazeDashFloorplaneStraightAlternating),
            '𝨧' => Ok(SuttonSignWriting::SignwritingEyegazeDashWallplaneCurved),
            '𝨨' => Ok(SuttonSignWriting::SignwritingEyegazeDashFloorplaneCurved),
            '𝨩' => Ok(SuttonSignWriting::SignwritingEyegazeDashWallplaneCircling),
            '𝨪' => Ok(SuttonSignWriting::SignwritingCheeksPuffed),
            '𝨫' => Ok(SuttonSignWriting::SignwritingCheeksNeutral),
            '𝨬' => Ok(SuttonSignWriting::SignwritingCheeksSucked),
            '𝨭' => Ok(SuttonSignWriting::SignwritingTenseCheeksHigh),
            '𝨮' => Ok(SuttonSignWriting::SignwritingTenseCheeksMiddle),
            '𝨯' => Ok(SuttonSignWriting::SignwritingTenseCheeksLow),
            '𝨰' => Ok(SuttonSignWriting::SignwritingEars),
            '𝨱' => Ok(SuttonSignWriting::SignwritingNoseNeutral),
            '𝨲' => Ok(SuttonSignWriting::SignwritingNoseContact),
            '𝨳' => Ok(SuttonSignWriting::SignwritingNoseWrinkles),
            '𝨴' => Ok(SuttonSignWriting::SignwritingNoseWiggles),
            '𝨵' => Ok(SuttonSignWriting::SignwritingAirBlowingOut),
            '𝨶' => Ok(SuttonSignWriting::SignwritingAirSuckingIn),
            '𝨷' => Ok(SuttonSignWriting::SignwritingAirBlowSmallRotations),
            '𝨸' => Ok(SuttonSignWriting::SignwritingAirSuckSmallRotations),
            '𝨹' => Ok(SuttonSignWriting::SignwritingBreathInhale),
            '𝨺' => Ok(SuttonSignWriting::SignwritingBreathExhale),
            '𝨻' => Ok(SuttonSignWriting::SignwritingMouthClosedNeutral),
            '𝨼' => Ok(SuttonSignWriting::SignwritingMouthClosedForward),
            '𝨽' => Ok(SuttonSignWriting::SignwritingMouthClosedContact),
            '𝨾' => Ok(SuttonSignWriting::SignwritingMouthSmile),
            '𝨿' => Ok(SuttonSignWriting::SignwritingMouthSmileWrinkled),
            '𝩀' => Ok(SuttonSignWriting::SignwritingMouthSmileOpen),
            '𝩁' => Ok(SuttonSignWriting::SignwritingMouthFrown),
            '𝩂' => Ok(SuttonSignWriting::SignwritingMouthFrownWrinkled),
            '𝩃' => Ok(SuttonSignWriting::SignwritingMouthFrownOpen),
            '𝩄' => Ok(SuttonSignWriting::SignwritingMouthOpenCircle),
            '𝩅' => Ok(SuttonSignWriting::SignwritingMouthOpenForward),
            '𝩆' => Ok(SuttonSignWriting::SignwritingMouthOpenWrinkled),
            '𝩇' => Ok(SuttonSignWriting::SignwritingMouthOpenOval),
            '𝩈' => Ok(SuttonSignWriting::SignwritingMouthOpenOvalWrinkled),
            '𝩉' => Ok(SuttonSignWriting::SignwritingMouthOpenOvalYawn),
            '𝩊' => Ok(SuttonSignWriting::SignwritingMouthOpenRectangle),
            '𝩋' => Ok(SuttonSignWriting::SignwritingMouthOpenRectangleWrinkled),
            '𝩌' => Ok(SuttonSignWriting::SignwritingMouthOpenRectangleYawn),
            '𝩍' => Ok(SuttonSignWriting::SignwritingMouthKiss),
            '𝩎' => Ok(SuttonSignWriting::SignwritingMouthKissForward),
            '𝩏' => Ok(SuttonSignWriting::SignwritingMouthKissWrinkled),
            '𝩐' => Ok(SuttonSignWriting::SignwritingMouthTense),
            '𝩑' => Ok(SuttonSignWriting::SignwritingMouthTenseForward),
            '𝩒' => Ok(SuttonSignWriting::SignwritingMouthTenseSucked),
            '𝩓' => Ok(SuttonSignWriting::SignwritingLipsPressedTogether),
            '𝩔' => Ok(SuttonSignWriting::SignwritingLipLowerOverUpper),
            '𝩕' => Ok(SuttonSignWriting::SignwritingLipUpperOverLower),
            '𝩖' => Ok(SuttonSignWriting::SignwritingMouthCorners),
            '𝩗' => Ok(SuttonSignWriting::SignwritingMouthWrinklesSingle),
            '𝩘' => Ok(SuttonSignWriting::SignwritingMouthWrinklesDouble),
            '𝩙' => Ok(SuttonSignWriting::SignwritingTongueStickingOutFar),
            '𝩚' => Ok(SuttonSignWriting::SignwritingTongueLickingLips),
            '𝩛' => Ok(SuttonSignWriting::SignwritingTongueTipBetweenLips),
            '𝩜' => Ok(SuttonSignWriting::SignwritingTongueTipTouchingInsideMouth),
            '𝩝' => Ok(SuttonSignWriting::SignwritingTongueInsideMouthRelaxed),
            '𝩞' => Ok(SuttonSignWriting::SignwritingTongueMovesAgainstCheek),
            '𝩟' => Ok(SuttonSignWriting::SignwritingTongueCentreStickingOut),
            '𝩠' => Ok(SuttonSignWriting::SignwritingTongueCentreInsideMouth),
            '𝩡' => Ok(SuttonSignWriting::SignwritingTeeth),
            '𝩢' => Ok(SuttonSignWriting::SignwritingTeethMovement),
            '𝩣' => Ok(SuttonSignWriting::SignwritingTeethOnTongue),
            '𝩤' => Ok(SuttonSignWriting::SignwritingTeethOnTongueMovement),
            '𝩥' => Ok(SuttonSignWriting::SignwritingTeethOnLips),
            '𝩦' => Ok(SuttonSignWriting::SignwritingTeethOnLipsMovement),
            '𝩧' => Ok(SuttonSignWriting::SignwritingTeethBiteLips),
            '𝩨' => Ok(SuttonSignWriting::SignwritingMovementDashWallplaneJaw),
            '𝩩' => Ok(SuttonSignWriting::SignwritingMovementDashFloorplaneJaw),
            '𝩪' => Ok(SuttonSignWriting::SignwritingNeck),
            '𝩫' => Ok(SuttonSignWriting::SignwritingHair),
            '𝩬' => Ok(SuttonSignWriting::SignwritingExcitement),
            '𝩭' => Ok(SuttonSignWriting::SignwritingShoulderHipSpine),
            '𝩮' => Ok(SuttonSignWriting::SignwritingShoulderHipPositions),
            '𝩯' => Ok(SuttonSignWriting::SignwritingWallplaneShoulderHipMove),
            '𝩰' => Ok(SuttonSignWriting::SignwritingFloorplaneShoulderHipMove),
            '𝩱' => Ok(SuttonSignWriting::SignwritingShoulderTiltingFromWaist),
            '𝩲' => Ok(SuttonSignWriting::SignwritingTorsoDashWallplaneStraightStretch),
            '𝩳' => Ok(SuttonSignWriting::SignwritingTorsoDashWallplaneCurvedBend),
            '𝩴' => Ok(SuttonSignWriting::SignwritingTorsoDashFloorplaneTwisting),
            '𝩵' => Ok(SuttonSignWriting::SignwritingUpperBodyTiltingFromHipJoints),
            '𝩶' => Ok(SuttonSignWriting::SignwritingLimbCombination),
            '𝩷' => Ok(SuttonSignWriting::SignwritingLimbLengthDash1),
            '𝩸' => Ok(SuttonSignWriting::SignwritingLimbLengthDash2),
            '𝩹' => Ok(SuttonSignWriting::SignwritingLimbLengthDash3),
            '𝩺' => Ok(SuttonSignWriting::SignwritingLimbLengthDash4),
            '𝩻' => Ok(SuttonSignWriting::SignwritingLimbLengthDash5),
            '𝩼' => Ok(SuttonSignWriting::SignwritingLimbLengthDash6),
            '𝩽' => Ok(SuttonSignWriting::SignwritingLimbLengthDash7),
            '𝩾' => Ok(SuttonSignWriting::SignwritingFinger),
            '𝩿' => Ok(SuttonSignWriting::SignwritingLocationDashWallplaneSpace),
            '𝪀' => Ok(SuttonSignWriting::SignwritingLocationDashFloorplaneSpace),
            '𝪁' => Ok(SuttonSignWriting::SignwritingLocationHeight),
            '𝪂' => Ok(SuttonSignWriting::SignwritingLocationWidth),
            '𝪃' => Ok(SuttonSignWriting::SignwritingLocationDepth),
            '𝪄' => Ok(SuttonSignWriting::SignwritingLocationHeadNeck),
            '𝪅' => Ok(SuttonSignWriting::SignwritingLocationTorso),
            '𝪆' => Ok(SuttonSignWriting::SignwritingLocationLimbsDigits),
            '𝪇' => Ok(SuttonSignWriting::SignwritingComma),
            '𝪈' => Ok(SuttonSignWriting::SignwritingFullStop),
            '𝪉' => Ok(SuttonSignWriting::SignwritingSemicolon),
            '𝪊' => Ok(SuttonSignWriting::SignwritingColon),
            '𝪋' => Ok(SuttonSignWriting::SignwritingParenthesis),
            '𝪛' => Ok(SuttonSignWriting::SignwritingFillModifierDash2),
            '𝪜' => Ok(SuttonSignWriting::SignwritingFillModifierDash3),
            '𝪝' => Ok(SuttonSignWriting::SignwritingFillModifierDash4),
            '𝪞' => Ok(SuttonSignWriting::SignwritingFillModifierDash5),
            '𝪟' => Ok(SuttonSignWriting::SignwritingFillModifierDash6),
            '𝪡' => Ok(SuttonSignWriting::SignwritingRotationModifierDash2),
            '𝪢' => Ok(SuttonSignWriting::SignwritingRotationModifierDash3),
            '𝪣' => Ok(SuttonSignWriting::SignwritingRotationModifierDash4),
            '𝪤' => Ok(SuttonSignWriting::SignwritingRotationModifierDash5),
            '𝪥' => Ok(SuttonSignWriting::SignwritingRotationModifierDash6),
            '𝪦' => Ok(SuttonSignWriting::SignwritingRotationModifierDash7),
            '𝪧' => Ok(SuttonSignWriting::SignwritingRotationModifierDash8),
            '𝪨' => Ok(SuttonSignWriting::SignwritingRotationModifierDash9),
            '𝪩' => Ok(SuttonSignWriting::SignwritingRotationModifierDash10),
            '𝪪' => Ok(SuttonSignWriting::SignwritingRotationModifierDash11),
            '𝪫' => Ok(SuttonSignWriting::SignwritingRotationModifierDash12),
            '𝪬' => Ok(SuttonSignWriting::SignwritingRotationModifierDash13),
            '𝪭' => Ok(SuttonSignWriting::SignwritingRotationModifierDash14),
            '𝪮' => Ok(SuttonSignWriting::SignwritingRotationModifierDash15),
            _ => Err(()),
        }
    }
}

impl Into<u32> for SuttonSignWriting {
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

impl std::convert::TryFrom<u32> for SuttonSignWriting {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for SuttonSignWriting {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl SuttonSignWriting {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        SuttonSignWriting::SignwritingHandDashFistIndex
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("SuttonSignWriting{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
