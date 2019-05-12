
/// An enum to represent all characters in the AlchemicalSymbols block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AlchemicalSymbols {
    /// \u{1f700}: '🜀'
    AlchemicalSymbolForQuintessence,
    /// \u{1f701}: '🜁'
    AlchemicalSymbolForAir,
    /// \u{1f702}: '🜂'
    AlchemicalSymbolForFire,
    /// \u{1f703}: '🜃'
    AlchemicalSymbolForEarth,
    /// \u{1f704}: '🜄'
    AlchemicalSymbolForWater,
    /// \u{1f705}: '🜅'
    AlchemicalSymbolForAquafortis,
    /// \u{1f706}: '🜆'
    AlchemicalSymbolForAquaRegia,
    /// \u{1f707}: '🜇'
    AlchemicalSymbolForAquaRegiaDash2,
    /// \u{1f708}: '🜈'
    AlchemicalSymbolForAquaVitae,
    /// \u{1f709}: '🜉'
    AlchemicalSymbolForAquaVitaeDash2,
    /// \u{1f70a}: '🜊'
    AlchemicalSymbolForVinegar,
    /// \u{1f70b}: '🜋'
    AlchemicalSymbolForVinegarDash2,
    /// \u{1f70c}: '🜌'
    AlchemicalSymbolForVinegarDash3,
    /// \u{1f70d}: '🜍'
    AlchemicalSymbolForSulfur,
    /// \u{1f70e}: '🜎'
    AlchemicalSymbolForPhilosophersSulfur,
    /// \u{1f70f}: '🜏'
    AlchemicalSymbolForBlackSulfur,
    /// \u{1f710}: '🜐'
    AlchemicalSymbolForMercurySublimate,
    /// \u{1f711}: '🜑'
    AlchemicalSymbolForMercurySublimateDash2,
    /// \u{1f712}: '🜒'
    AlchemicalSymbolForMercurySublimateDash3,
    /// \u{1f713}: '🜓'
    AlchemicalSymbolForCinnabar,
    /// \u{1f714}: '🜔'
    AlchemicalSymbolForSalt,
    /// \u{1f715}: '🜕'
    AlchemicalSymbolForNitre,
    /// \u{1f716}: '🜖'
    AlchemicalSymbolForVitriol,
    /// \u{1f717}: '🜗'
    AlchemicalSymbolForVitriolDash2,
    /// \u{1f718}: '🜘'
    AlchemicalSymbolForRockSalt,
    /// \u{1f719}: '🜙'
    AlchemicalSymbolForRockSaltDash2,
    /// \u{1f71a}: '🜚'
    AlchemicalSymbolForGold,
    /// \u{1f71b}: '🜛'
    AlchemicalSymbolForSilver,
    /// \u{1f71c}: '🜜'
    AlchemicalSymbolForIronOre,
    /// \u{1f71d}: '🜝'
    AlchemicalSymbolForIronOreDash2,
    /// \u{1f71e}: '🜞'
    AlchemicalSymbolForCrocusOfIron,
    /// \u{1f71f}: '🜟'
    AlchemicalSymbolForRegulusOfIron,
    /// \u{1f720}: '🜠'
    AlchemicalSymbolForCopperOre,
    /// \u{1f721}: '🜡'
    AlchemicalSymbolForIronDashCopperOre,
    /// \u{1f722}: '🜢'
    AlchemicalSymbolForSublimateOfCopper,
    /// \u{1f723}: '🜣'
    AlchemicalSymbolForCrocusOfCopper,
    /// \u{1f724}: '🜤'
    AlchemicalSymbolForCrocusOfCopperDash2,
    /// \u{1f725}: '🜥'
    AlchemicalSymbolForCopperAntimoniate,
    /// \u{1f726}: '🜦'
    AlchemicalSymbolForSaltOfCopperAntimoniate,
    /// \u{1f727}: '🜧'
    AlchemicalSymbolForSublimateOfSaltOfCopper,
    /// \u{1f728}: '🜨'
    AlchemicalSymbolForVerdigris,
    /// \u{1f729}: '🜩'
    AlchemicalSymbolForTinOre,
    /// \u{1f72a}: '🜪'
    AlchemicalSymbolForLeadOre,
    /// \u{1f72b}: '🜫'
    AlchemicalSymbolForAntimonyOre,
    /// \u{1f72c}: '🜬'
    AlchemicalSymbolForSublimateOfAntimony,
    /// \u{1f72d}: '🜭'
    AlchemicalSymbolForSaltOfAntimony,
    /// \u{1f72e}: '🜮'
    AlchemicalSymbolForSublimateOfSaltOfAntimony,
    /// \u{1f72f}: '🜯'
    AlchemicalSymbolForVinegarOfAntimony,
    /// \u{1f730}: '🜰'
    AlchemicalSymbolForRegulusOfAntimony,
    /// \u{1f731}: '🜱'
    AlchemicalSymbolForRegulusOfAntimonyDash2,
    /// \u{1f732}: '🜲'
    AlchemicalSymbolForRegulus,
    /// \u{1f733}: '🜳'
    AlchemicalSymbolForRegulusDash2,
    /// \u{1f734}: '🜴'
    AlchemicalSymbolForRegulusDash3,
    /// \u{1f735}: '🜵'
    AlchemicalSymbolForRegulusDash4,
    /// \u{1f736}: '🜶'
    AlchemicalSymbolForAlkali,
    /// \u{1f737}: '🜷'
    AlchemicalSymbolForAlkaliDash2,
    /// \u{1f738}: '🜸'
    AlchemicalSymbolForMarcasite,
    /// \u{1f739}: '🜹'
    AlchemicalSymbolForSalDashAmmoniac,
    /// \u{1f73a}: '🜺'
    AlchemicalSymbolForArsenic,
    /// \u{1f73b}: '🜻'
    AlchemicalSymbolForRealgar,
    /// \u{1f73c}: '🜼'
    AlchemicalSymbolForRealgarDash2,
    /// \u{1f73d}: '🜽'
    AlchemicalSymbolForAuripigment,
    /// \u{1f73e}: '🜾'
    AlchemicalSymbolForBismuthOre,
    /// \u{1f73f}: '🜿'
    AlchemicalSymbolForTartar,
    /// \u{1f740}: '🝀'
    AlchemicalSymbolForTartarDash2,
    /// \u{1f741}: '🝁'
    AlchemicalSymbolForQuickLime,
    /// \u{1f742}: '🝂'
    AlchemicalSymbolForBorax,
    /// \u{1f743}: '🝃'
    AlchemicalSymbolForBoraxDash2,
    /// \u{1f744}: '🝄'
    AlchemicalSymbolForBoraxDash3,
    /// \u{1f745}: '🝅'
    AlchemicalSymbolForAlum,
    /// \u{1f746}: '🝆'
    AlchemicalSymbolForOil,
    /// \u{1f747}: '🝇'
    AlchemicalSymbolForSpirit,
    /// \u{1f748}: '🝈'
    AlchemicalSymbolForTincture,
    /// \u{1f749}: '🝉'
    AlchemicalSymbolForGum,
    /// \u{1f74a}: '🝊'
    AlchemicalSymbolForWax,
    /// \u{1f74b}: '🝋'
    AlchemicalSymbolForPowder,
    /// \u{1f74c}: '🝌'
    AlchemicalSymbolForCalx,
    /// \u{1f74d}: '🝍'
    AlchemicalSymbolForTutty,
    /// \u{1f74e}: '🝎'
    AlchemicalSymbolForCaputMortuum,
    /// \u{1f74f}: '🝏'
    AlchemicalSymbolForScepterOfJove,
    /// \u{1f750}: '🝐'
    AlchemicalSymbolForCaduceus,
    /// \u{1f751}: '🝑'
    AlchemicalSymbolForTrident,
    /// \u{1f752}: '🝒'
    AlchemicalSymbolForStarredTrident,
    /// \u{1f753}: '🝓'
    AlchemicalSymbolForLodestone,
    /// \u{1f754}: '🝔'
    AlchemicalSymbolForSoap,
    /// \u{1f755}: '🝕'
    AlchemicalSymbolForUrine,
    /// \u{1f756}: '🝖'
    AlchemicalSymbolForHorseDung,
    /// \u{1f757}: '🝗'
    AlchemicalSymbolForAshes,
    /// \u{1f758}: '🝘'
    AlchemicalSymbolForPotAshes,
    /// \u{1f759}: '🝙'
    AlchemicalSymbolForBrick,
    /// \u{1f75a}: '🝚'
    AlchemicalSymbolForPowderedBrick,
    /// \u{1f75b}: '🝛'
    AlchemicalSymbolForAmalgam,
    /// \u{1f75c}: '🝜'
    AlchemicalSymbolForStratumSuperStratum,
    /// \u{1f75d}: '🝝'
    AlchemicalSymbolForStratumSuperStratumDash2,
    /// \u{1f75e}: '🝞'
    AlchemicalSymbolForSublimation,
    /// \u{1f75f}: '🝟'
    AlchemicalSymbolForPrecipitate,
    /// \u{1f760}: '🝠'
    AlchemicalSymbolForDistill,
    /// \u{1f761}: '🝡'
    AlchemicalSymbolForDissolve,
    /// \u{1f762}: '🝢'
    AlchemicalSymbolForDissolveDash2,
    /// \u{1f763}: '🝣'
    AlchemicalSymbolForPurify,
    /// \u{1f764}: '🝤'
    AlchemicalSymbolForPutrefaction,
    /// \u{1f765}: '🝥'
    AlchemicalSymbolForCrucible,
    /// \u{1f766}: '🝦'
    AlchemicalSymbolForCrucibleDash2,
    /// \u{1f767}: '🝧'
    AlchemicalSymbolForCrucibleDash3,
    /// \u{1f768}: '🝨'
    AlchemicalSymbolForCrucibleDash4,
    /// \u{1f769}: '🝩'
    AlchemicalSymbolForCrucibleDash5,
    /// \u{1f76a}: '🝪'
    AlchemicalSymbolForAlembic,
    /// \u{1f76b}: '🝫'
    AlchemicalSymbolForBathOfMary,
    /// \u{1f76c}: '🝬'
    AlchemicalSymbolForBathOfVapours,
    /// \u{1f76d}: '🝭'
    AlchemicalSymbolForRetort,
    /// \u{1f76e}: '🝮'
    AlchemicalSymbolForHour,
    /// \u{1f76f}: '🝯'
    AlchemicalSymbolForNight,
    /// \u{1f770}: '🝰'
    AlchemicalSymbolForDayDashNight,
    /// \u{1f771}: '🝱'
    AlchemicalSymbolForMonth,
    /// \u{1f772}: '🝲'
    AlchemicalSymbolForHalfDram,
    /// \u{1f773}: '🝳'
    AlchemicalSymbolForHalfOunce,
}

impl Into<char> for AlchemicalSymbols {
    fn into(self) -> char {
        match self {
            AlchemicalSymbols::AlchemicalSymbolForQuintessence => '🜀',
            AlchemicalSymbols::AlchemicalSymbolForAir => '🜁',
            AlchemicalSymbols::AlchemicalSymbolForFire => '🜂',
            AlchemicalSymbols::AlchemicalSymbolForEarth => '🜃',
            AlchemicalSymbols::AlchemicalSymbolForWater => '🜄',
            AlchemicalSymbols::AlchemicalSymbolForAquafortis => '🜅',
            AlchemicalSymbols::AlchemicalSymbolForAquaRegia => '🜆',
            AlchemicalSymbols::AlchemicalSymbolForAquaRegiaDash2 => '🜇',
            AlchemicalSymbols::AlchemicalSymbolForAquaVitae => '🜈',
            AlchemicalSymbols::AlchemicalSymbolForAquaVitaeDash2 => '🜉',
            AlchemicalSymbols::AlchemicalSymbolForVinegar => '🜊',
            AlchemicalSymbols::AlchemicalSymbolForVinegarDash2 => '🜋',
            AlchemicalSymbols::AlchemicalSymbolForVinegarDash3 => '🜌',
            AlchemicalSymbols::AlchemicalSymbolForSulfur => '🜍',
            AlchemicalSymbols::AlchemicalSymbolForPhilosophersSulfur => '🜎',
            AlchemicalSymbols::AlchemicalSymbolForBlackSulfur => '🜏',
            AlchemicalSymbols::AlchemicalSymbolForMercurySublimate => '🜐',
            AlchemicalSymbols::AlchemicalSymbolForMercurySublimateDash2 => '🜑',
            AlchemicalSymbols::AlchemicalSymbolForMercurySublimateDash3 => '🜒',
            AlchemicalSymbols::AlchemicalSymbolForCinnabar => '🜓',
            AlchemicalSymbols::AlchemicalSymbolForSalt => '🜔',
            AlchemicalSymbols::AlchemicalSymbolForNitre => '🜕',
            AlchemicalSymbols::AlchemicalSymbolForVitriol => '🜖',
            AlchemicalSymbols::AlchemicalSymbolForVitriolDash2 => '🜗',
            AlchemicalSymbols::AlchemicalSymbolForRockSalt => '🜘',
            AlchemicalSymbols::AlchemicalSymbolForRockSaltDash2 => '🜙',
            AlchemicalSymbols::AlchemicalSymbolForGold => '🜚',
            AlchemicalSymbols::AlchemicalSymbolForSilver => '🜛',
            AlchemicalSymbols::AlchemicalSymbolForIronOre => '🜜',
            AlchemicalSymbols::AlchemicalSymbolForIronOreDash2 => '🜝',
            AlchemicalSymbols::AlchemicalSymbolForCrocusOfIron => '🜞',
            AlchemicalSymbols::AlchemicalSymbolForRegulusOfIron => '🜟',
            AlchemicalSymbols::AlchemicalSymbolForCopperOre => '🜠',
            AlchemicalSymbols::AlchemicalSymbolForIronDashCopperOre => '🜡',
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfCopper => '🜢',
            AlchemicalSymbols::AlchemicalSymbolForCrocusOfCopper => '🜣',
            AlchemicalSymbols::AlchemicalSymbolForCrocusOfCopperDash2 => '🜤',
            AlchemicalSymbols::AlchemicalSymbolForCopperAntimoniate => '🜥',
            AlchemicalSymbols::AlchemicalSymbolForSaltOfCopperAntimoniate => '🜦',
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfSaltOfCopper => '🜧',
            AlchemicalSymbols::AlchemicalSymbolForVerdigris => '🜨',
            AlchemicalSymbols::AlchemicalSymbolForTinOre => '🜩',
            AlchemicalSymbols::AlchemicalSymbolForLeadOre => '🜪',
            AlchemicalSymbols::AlchemicalSymbolForAntimonyOre => '🜫',
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfAntimony => '🜬',
            AlchemicalSymbols::AlchemicalSymbolForSaltOfAntimony => '🜭',
            AlchemicalSymbols::AlchemicalSymbolForSublimateOfSaltOfAntimony => '🜮',
            AlchemicalSymbols::AlchemicalSymbolForVinegarOfAntimony => '🜯',
            AlchemicalSymbols::AlchemicalSymbolForRegulusOfAntimony => '🜰',
            AlchemicalSymbols::AlchemicalSymbolForRegulusOfAntimonyDash2 => '🜱',
            AlchemicalSymbols::AlchemicalSymbolForRegulus => '🜲',
            AlchemicalSymbols::AlchemicalSymbolForRegulusDash2 => '🜳',
            AlchemicalSymbols::AlchemicalSymbolForRegulusDash3 => '🜴',
            AlchemicalSymbols::AlchemicalSymbolForRegulusDash4 => '🜵',
            AlchemicalSymbols::AlchemicalSymbolForAlkali => '🜶',
            AlchemicalSymbols::AlchemicalSymbolForAlkaliDash2 => '🜷',
            AlchemicalSymbols::AlchemicalSymbolForMarcasite => '🜸',
            AlchemicalSymbols::AlchemicalSymbolForSalDashAmmoniac => '🜹',
            AlchemicalSymbols::AlchemicalSymbolForArsenic => '🜺',
            AlchemicalSymbols::AlchemicalSymbolForRealgar => '🜻',
            AlchemicalSymbols::AlchemicalSymbolForRealgarDash2 => '🜼',
            AlchemicalSymbols::AlchemicalSymbolForAuripigment => '🜽',
            AlchemicalSymbols::AlchemicalSymbolForBismuthOre => '🜾',
            AlchemicalSymbols::AlchemicalSymbolForTartar => '🜿',
            AlchemicalSymbols::AlchemicalSymbolForTartarDash2 => '🝀',
            AlchemicalSymbols::AlchemicalSymbolForQuickLime => '🝁',
            AlchemicalSymbols::AlchemicalSymbolForBorax => '🝂',
            AlchemicalSymbols::AlchemicalSymbolForBoraxDash2 => '🝃',
            AlchemicalSymbols::AlchemicalSymbolForBoraxDash3 => '🝄',
            AlchemicalSymbols::AlchemicalSymbolForAlum => '🝅',
            AlchemicalSymbols::AlchemicalSymbolForOil => '🝆',
            AlchemicalSymbols::AlchemicalSymbolForSpirit => '🝇',
            AlchemicalSymbols::AlchemicalSymbolForTincture => '🝈',
            AlchemicalSymbols::AlchemicalSymbolForGum => '🝉',
            AlchemicalSymbols::AlchemicalSymbolForWax => '🝊',
            AlchemicalSymbols::AlchemicalSymbolForPowder => '🝋',
            AlchemicalSymbols::AlchemicalSymbolForCalx => '🝌',
            AlchemicalSymbols::AlchemicalSymbolForTutty => '🝍',
            AlchemicalSymbols::AlchemicalSymbolForCaputMortuum => '🝎',
            AlchemicalSymbols::AlchemicalSymbolForScepterOfJove => '🝏',
            AlchemicalSymbols::AlchemicalSymbolForCaduceus => '🝐',
            AlchemicalSymbols::AlchemicalSymbolForTrident => '🝑',
            AlchemicalSymbols::AlchemicalSymbolForStarredTrident => '🝒',
            AlchemicalSymbols::AlchemicalSymbolForLodestone => '🝓',
            AlchemicalSymbols::AlchemicalSymbolForSoap => '🝔',
            AlchemicalSymbols::AlchemicalSymbolForUrine => '🝕',
            AlchemicalSymbols::AlchemicalSymbolForHorseDung => '🝖',
            AlchemicalSymbols::AlchemicalSymbolForAshes => '🝗',
            AlchemicalSymbols::AlchemicalSymbolForPotAshes => '🝘',
            AlchemicalSymbols::AlchemicalSymbolForBrick => '🝙',
            AlchemicalSymbols::AlchemicalSymbolForPowderedBrick => '🝚',
            AlchemicalSymbols::AlchemicalSymbolForAmalgam => '🝛',
            AlchemicalSymbols::AlchemicalSymbolForStratumSuperStratum => '🝜',
            AlchemicalSymbols::AlchemicalSymbolForStratumSuperStratumDash2 => '🝝',
            AlchemicalSymbols::AlchemicalSymbolForSublimation => '🝞',
            AlchemicalSymbols::AlchemicalSymbolForPrecipitate => '🝟',
            AlchemicalSymbols::AlchemicalSymbolForDistill => '🝠',
            AlchemicalSymbols::AlchemicalSymbolForDissolve => '🝡',
            AlchemicalSymbols::AlchemicalSymbolForDissolveDash2 => '🝢',
            AlchemicalSymbols::AlchemicalSymbolForPurify => '🝣',
            AlchemicalSymbols::AlchemicalSymbolForPutrefaction => '🝤',
            AlchemicalSymbols::AlchemicalSymbolForCrucible => '🝥',
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash2 => '🝦',
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash3 => '🝧',
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash4 => '🝨',
            AlchemicalSymbols::AlchemicalSymbolForCrucibleDash5 => '🝩',
            AlchemicalSymbols::AlchemicalSymbolForAlembic => '🝪',
            AlchemicalSymbols::AlchemicalSymbolForBathOfMary => '🝫',
            AlchemicalSymbols::AlchemicalSymbolForBathOfVapours => '🝬',
            AlchemicalSymbols::AlchemicalSymbolForRetort => '🝭',
            AlchemicalSymbols::AlchemicalSymbolForHour => '🝮',
            AlchemicalSymbols::AlchemicalSymbolForNight => '🝯',
            AlchemicalSymbols::AlchemicalSymbolForDayDashNight => '🝰',
            AlchemicalSymbols::AlchemicalSymbolForMonth => '🝱',
            AlchemicalSymbols::AlchemicalSymbolForHalfDram => '🝲',
            AlchemicalSymbols::AlchemicalSymbolForHalfOunce => '🝳',
        }
    }
}

impl std::convert::TryFrom<char> for AlchemicalSymbols {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🜀' => Ok(AlchemicalSymbols::AlchemicalSymbolForQuintessence),
            '🜁' => Ok(AlchemicalSymbols::AlchemicalSymbolForAir),
            '🜂' => Ok(AlchemicalSymbols::AlchemicalSymbolForFire),
            '🜃' => Ok(AlchemicalSymbols::AlchemicalSymbolForEarth),
            '🜄' => Ok(AlchemicalSymbols::AlchemicalSymbolForWater),
            '🜅' => Ok(AlchemicalSymbols::AlchemicalSymbolForAquafortis),
            '🜆' => Ok(AlchemicalSymbols::AlchemicalSymbolForAquaRegia),
            '🜇' => Ok(AlchemicalSymbols::AlchemicalSymbolForAquaRegiaDash2),
            '🜈' => Ok(AlchemicalSymbols::AlchemicalSymbolForAquaVitae),
            '🜉' => Ok(AlchemicalSymbols::AlchemicalSymbolForAquaVitaeDash2),
            '🜊' => Ok(AlchemicalSymbols::AlchemicalSymbolForVinegar),
            '🜋' => Ok(AlchemicalSymbols::AlchemicalSymbolForVinegarDash2),
            '🜌' => Ok(AlchemicalSymbols::AlchemicalSymbolForVinegarDash3),
            '🜍' => Ok(AlchemicalSymbols::AlchemicalSymbolForSulfur),
            '🜎' => Ok(AlchemicalSymbols::AlchemicalSymbolForPhilosophersSulfur),
            '🜏' => Ok(AlchemicalSymbols::AlchemicalSymbolForBlackSulfur),
            '🜐' => Ok(AlchemicalSymbols::AlchemicalSymbolForMercurySublimate),
            '🜑' => Ok(AlchemicalSymbols::AlchemicalSymbolForMercurySublimateDash2),
            '🜒' => Ok(AlchemicalSymbols::AlchemicalSymbolForMercurySublimateDash3),
            '🜓' => Ok(AlchemicalSymbols::AlchemicalSymbolForCinnabar),
            '🜔' => Ok(AlchemicalSymbols::AlchemicalSymbolForSalt),
            '🜕' => Ok(AlchemicalSymbols::AlchemicalSymbolForNitre),
            '🜖' => Ok(AlchemicalSymbols::AlchemicalSymbolForVitriol),
            '🜗' => Ok(AlchemicalSymbols::AlchemicalSymbolForVitriolDash2),
            '🜘' => Ok(AlchemicalSymbols::AlchemicalSymbolForRockSalt),
            '🜙' => Ok(AlchemicalSymbols::AlchemicalSymbolForRockSaltDash2),
            '🜚' => Ok(AlchemicalSymbols::AlchemicalSymbolForGold),
            '🜛' => Ok(AlchemicalSymbols::AlchemicalSymbolForSilver),
            '🜜' => Ok(AlchemicalSymbols::AlchemicalSymbolForIronOre),
            '🜝' => Ok(AlchemicalSymbols::AlchemicalSymbolForIronOreDash2),
            '🜞' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrocusOfIron),
            '🜟' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusOfIron),
            '🜠' => Ok(AlchemicalSymbols::AlchemicalSymbolForCopperOre),
            '🜡' => Ok(AlchemicalSymbols::AlchemicalSymbolForIronDashCopperOre),
            '🜢' => Ok(AlchemicalSymbols::AlchemicalSymbolForSublimateOfCopper),
            '🜣' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrocusOfCopper),
            '🜤' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrocusOfCopperDash2),
            '🜥' => Ok(AlchemicalSymbols::AlchemicalSymbolForCopperAntimoniate),
            '🜦' => Ok(AlchemicalSymbols::AlchemicalSymbolForSaltOfCopperAntimoniate),
            '🜧' => Ok(AlchemicalSymbols::AlchemicalSymbolForSublimateOfSaltOfCopper),
            '🜨' => Ok(AlchemicalSymbols::AlchemicalSymbolForVerdigris),
            '🜩' => Ok(AlchemicalSymbols::AlchemicalSymbolForTinOre),
            '🜪' => Ok(AlchemicalSymbols::AlchemicalSymbolForLeadOre),
            '🜫' => Ok(AlchemicalSymbols::AlchemicalSymbolForAntimonyOre),
            '🜬' => Ok(AlchemicalSymbols::AlchemicalSymbolForSublimateOfAntimony),
            '🜭' => Ok(AlchemicalSymbols::AlchemicalSymbolForSaltOfAntimony),
            '🜮' => Ok(AlchemicalSymbols::AlchemicalSymbolForSublimateOfSaltOfAntimony),
            '🜯' => Ok(AlchemicalSymbols::AlchemicalSymbolForVinegarOfAntimony),
            '🜰' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusOfAntimony),
            '🜱' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusOfAntimonyDash2),
            '🜲' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulus),
            '🜳' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusDash2),
            '🜴' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusDash3),
            '🜵' => Ok(AlchemicalSymbols::AlchemicalSymbolForRegulusDash4),
            '🜶' => Ok(AlchemicalSymbols::AlchemicalSymbolForAlkali),
            '🜷' => Ok(AlchemicalSymbols::AlchemicalSymbolForAlkaliDash2),
            '🜸' => Ok(AlchemicalSymbols::AlchemicalSymbolForMarcasite),
            '🜹' => Ok(AlchemicalSymbols::AlchemicalSymbolForSalDashAmmoniac),
            '🜺' => Ok(AlchemicalSymbols::AlchemicalSymbolForArsenic),
            '🜻' => Ok(AlchemicalSymbols::AlchemicalSymbolForRealgar),
            '🜼' => Ok(AlchemicalSymbols::AlchemicalSymbolForRealgarDash2),
            '🜽' => Ok(AlchemicalSymbols::AlchemicalSymbolForAuripigment),
            '🜾' => Ok(AlchemicalSymbols::AlchemicalSymbolForBismuthOre),
            '🜿' => Ok(AlchemicalSymbols::AlchemicalSymbolForTartar),
            '🝀' => Ok(AlchemicalSymbols::AlchemicalSymbolForTartarDash2),
            '🝁' => Ok(AlchemicalSymbols::AlchemicalSymbolForQuickLime),
            '🝂' => Ok(AlchemicalSymbols::AlchemicalSymbolForBorax),
            '🝃' => Ok(AlchemicalSymbols::AlchemicalSymbolForBoraxDash2),
            '🝄' => Ok(AlchemicalSymbols::AlchemicalSymbolForBoraxDash3),
            '🝅' => Ok(AlchemicalSymbols::AlchemicalSymbolForAlum),
            '🝆' => Ok(AlchemicalSymbols::AlchemicalSymbolForOil),
            '🝇' => Ok(AlchemicalSymbols::AlchemicalSymbolForSpirit),
            '🝈' => Ok(AlchemicalSymbols::AlchemicalSymbolForTincture),
            '🝉' => Ok(AlchemicalSymbols::AlchemicalSymbolForGum),
            '🝊' => Ok(AlchemicalSymbols::AlchemicalSymbolForWax),
            '🝋' => Ok(AlchemicalSymbols::AlchemicalSymbolForPowder),
            '🝌' => Ok(AlchemicalSymbols::AlchemicalSymbolForCalx),
            '🝍' => Ok(AlchemicalSymbols::AlchemicalSymbolForTutty),
            '🝎' => Ok(AlchemicalSymbols::AlchemicalSymbolForCaputMortuum),
            '🝏' => Ok(AlchemicalSymbols::AlchemicalSymbolForScepterOfJove),
            '🝐' => Ok(AlchemicalSymbols::AlchemicalSymbolForCaduceus),
            '🝑' => Ok(AlchemicalSymbols::AlchemicalSymbolForTrident),
            '🝒' => Ok(AlchemicalSymbols::AlchemicalSymbolForStarredTrident),
            '🝓' => Ok(AlchemicalSymbols::AlchemicalSymbolForLodestone),
            '🝔' => Ok(AlchemicalSymbols::AlchemicalSymbolForSoap),
            '🝕' => Ok(AlchemicalSymbols::AlchemicalSymbolForUrine),
            '🝖' => Ok(AlchemicalSymbols::AlchemicalSymbolForHorseDung),
            '🝗' => Ok(AlchemicalSymbols::AlchemicalSymbolForAshes),
            '🝘' => Ok(AlchemicalSymbols::AlchemicalSymbolForPotAshes),
            '🝙' => Ok(AlchemicalSymbols::AlchemicalSymbolForBrick),
            '🝚' => Ok(AlchemicalSymbols::AlchemicalSymbolForPowderedBrick),
            '🝛' => Ok(AlchemicalSymbols::AlchemicalSymbolForAmalgam),
            '🝜' => Ok(AlchemicalSymbols::AlchemicalSymbolForStratumSuperStratum),
            '🝝' => Ok(AlchemicalSymbols::AlchemicalSymbolForStratumSuperStratumDash2),
            '🝞' => Ok(AlchemicalSymbols::AlchemicalSymbolForSublimation),
            '🝟' => Ok(AlchemicalSymbols::AlchemicalSymbolForPrecipitate),
            '🝠' => Ok(AlchemicalSymbols::AlchemicalSymbolForDistill),
            '🝡' => Ok(AlchemicalSymbols::AlchemicalSymbolForDissolve),
            '🝢' => Ok(AlchemicalSymbols::AlchemicalSymbolForDissolveDash2),
            '🝣' => Ok(AlchemicalSymbols::AlchemicalSymbolForPurify),
            '🝤' => Ok(AlchemicalSymbols::AlchemicalSymbolForPutrefaction),
            '🝥' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrucible),
            '🝦' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrucibleDash2),
            '🝧' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrucibleDash3),
            '🝨' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrucibleDash4),
            '🝩' => Ok(AlchemicalSymbols::AlchemicalSymbolForCrucibleDash5),
            '🝪' => Ok(AlchemicalSymbols::AlchemicalSymbolForAlembic),
            '🝫' => Ok(AlchemicalSymbols::AlchemicalSymbolForBathOfMary),
            '🝬' => Ok(AlchemicalSymbols::AlchemicalSymbolForBathOfVapours),
            '🝭' => Ok(AlchemicalSymbols::AlchemicalSymbolForRetort),
            '🝮' => Ok(AlchemicalSymbols::AlchemicalSymbolForHour),
            '🝯' => Ok(AlchemicalSymbols::AlchemicalSymbolForNight),
            '🝰' => Ok(AlchemicalSymbols::AlchemicalSymbolForDayDashNight),
            '🝱' => Ok(AlchemicalSymbols::AlchemicalSymbolForMonth),
            '🝲' => Ok(AlchemicalSymbols::AlchemicalSymbolForHalfDram),
            '🝳' => Ok(AlchemicalSymbols::AlchemicalSymbolForHalfOunce),
            _ => Err(()),
        }
    }
}

impl Into<u32> for AlchemicalSymbols {
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

impl std::convert::TryFrom<u32> for AlchemicalSymbols {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for AlchemicalSymbols {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl AlchemicalSymbols {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        AlchemicalSymbols::AlchemicalSymbolForQuintessence
    }

    /// The character's name, in sentence case
    pub fn name(&self) -> String {
        let s = std::format!("AlchemicalSymbols{:#?}", self);
        string_morph::to_sentence_case(&s)
    }
}
