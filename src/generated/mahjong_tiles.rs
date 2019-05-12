
/// An enum to represent all characters in the MahjongTiles block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum MahjongTiles {
    /// \u{1f000}: '🀀'
    MahjongTileEastWind,
    /// \u{1f001}: '🀁'
    MahjongTileSouthWind,
    /// \u{1f002}: '🀂'
    MahjongTileWestWind,
    /// \u{1f003}: '🀃'
    MahjongTileNorthWind,
    /// \u{1f004}: '🀄'
    MahjongTileRedDragon,
    /// \u{1f005}: '🀅'
    MahjongTileGreenDragon,
    /// \u{1f006}: '🀆'
    MahjongTileWhiteDragon,
    /// \u{1f007}: '🀇'
    MahjongTileOneOfCharacters,
    /// \u{1f008}: '🀈'
    MahjongTileTwoOfCharacters,
    /// \u{1f009}: '🀉'
    MahjongTileThreeOfCharacters,
    /// \u{1f00a}: '🀊'
    MahjongTileFourOfCharacters,
    /// \u{1f00b}: '🀋'
    MahjongTileFiveOfCharacters,
    /// \u{1f00c}: '🀌'
    MahjongTileSixOfCharacters,
    /// \u{1f00d}: '🀍'
    MahjongTileSevenOfCharacters,
    /// \u{1f00e}: '🀎'
    MahjongTileEightOfCharacters,
    /// \u{1f00f}: '🀏'
    MahjongTileNineOfCharacters,
    /// \u{1f010}: '🀐'
    MahjongTileOneOfBamboos,
    /// \u{1f011}: '🀑'
    MahjongTileTwoOfBamboos,
    /// \u{1f012}: '🀒'
    MahjongTileThreeOfBamboos,
    /// \u{1f013}: '🀓'
    MahjongTileFourOfBamboos,
    /// \u{1f014}: '🀔'
    MahjongTileFiveOfBamboos,
    /// \u{1f015}: '🀕'
    MahjongTileSixOfBamboos,
    /// \u{1f016}: '🀖'
    MahjongTileSevenOfBamboos,
    /// \u{1f017}: '🀗'
    MahjongTileEightOfBamboos,
    /// \u{1f018}: '🀘'
    MahjongTileNineOfBamboos,
    /// \u{1f019}: '🀙'
    MahjongTileOneOfCircles,
    /// \u{1f01a}: '🀚'
    MahjongTileTwoOfCircles,
    /// \u{1f01b}: '🀛'
    MahjongTileThreeOfCircles,
    /// \u{1f01c}: '🀜'
    MahjongTileFourOfCircles,
    /// \u{1f01d}: '🀝'
    MahjongTileFiveOfCircles,
    /// \u{1f01e}: '🀞'
    MahjongTileSixOfCircles,
    /// \u{1f01f}: '🀟'
    MahjongTileSevenOfCircles,
    /// \u{1f020}: '🀠'
    MahjongTileEightOfCircles,
    /// \u{1f021}: '🀡'
    MahjongTileNineOfCircles,
    /// \u{1f022}: '🀢'
    MahjongTilePlum,
    /// \u{1f023}: '🀣'
    MahjongTileOrchid,
    /// \u{1f024}: '🀤'
    MahjongTileBamboo,
    /// \u{1f025}: '🀥'
    MahjongTileChrysanthemum,
    /// \u{1f026}: '🀦'
    MahjongTileSpring,
    /// \u{1f027}: '🀧'
    MahjongTileSummer,
    /// \u{1f028}: '🀨'
    MahjongTileAutumn,
    /// \u{1f029}: '🀩'
    MahjongTileWinter,
    /// \u{1f02a}: '🀪'
    MahjongTileJoker,
    /// \u{1f02b}: '🀫'
    MahjongTileBack,
}

impl Into<char> for MahjongTiles {
    fn into(self) -> char {
        match self {
            MahjongTiles::MahjongTileEastWind => '🀀',
            MahjongTiles::MahjongTileSouthWind => '🀁',
            MahjongTiles::MahjongTileWestWind => '🀂',
            MahjongTiles::MahjongTileNorthWind => '🀃',
            MahjongTiles::MahjongTileRedDragon => '🀄',
            MahjongTiles::MahjongTileGreenDragon => '🀅',
            MahjongTiles::MahjongTileWhiteDragon => '🀆',
            MahjongTiles::MahjongTileOneOfCharacters => '🀇',
            MahjongTiles::MahjongTileTwoOfCharacters => '🀈',
            MahjongTiles::MahjongTileThreeOfCharacters => '🀉',
            MahjongTiles::MahjongTileFourOfCharacters => '🀊',
            MahjongTiles::MahjongTileFiveOfCharacters => '🀋',
            MahjongTiles::MahjongTileSixOfCharacters => '🀌',
            MahjongTiles::MahjongTileSevenOfCharacters => '🀍',
            MahjongTiles::MahjongTileEightOfCharacters => '🀎',
            MahjongTiles::MahjongTileNineOfCharacters => '🀏',
            MahjongTiles::MahjongTileOneOfBamboos => '🀐',
            MahjongTiles::MahjongTileTwoOfBamboos => '🀑',
            MahjongTiles::MahjongTileThreeOfBamboos => '🀒',
            MahjongTiles::MahjongTileFourOfBamboos => '🀓',
            MahjongTiles::MahjongTileFiveOfBamboos => '🀔',
            MahjongTiles::MahjongTileSixOfBamboos => '🀕',
            MahjongTiles::MahjongTileSevenOfBamboos => '🀖',
            MahjongTiles::MahjongTileEightOfBamboos => '🀗',
            MahjongTiles::MahjongTileNineOfBamboos => '🀘',
            MahjongTiles::MahjongTileOneOfCircles => '🀙',
            MahjongTiles::MahjongTileTwoOfCircles => '🀚',
            MahjongTiles::MahjongTileThreeOfCircles => '🀛',
            MahjongTiles::MahjongTileFourOfCircles => '🀜',
            MahjongTiles::MahjongTileFiveOfCircles => '🀝',
            MahjongTiles::MahjongTileSixOfCircles => '🀞',
            MahjongTiles::MahjongTileSevenOfCircles => '🀟',
            MahjongTiles::MahjongTileEightOfCircles => '🀠',
            MahjongTiles::MahjongTileNineOfCircles => '🀡',
            MahjongTiles::MahjongTilePlum => '🀢',
            MahjongTiles::MahjongTileOrchid => '🀣',
            MahjongTiles::MahjongTileBamboo => '🀤',
            MahjongTiles::MahjongTileChrysanthemum => '🀥',
            MahjongTiles::MahjongTileSpring => '🀦',
            MahjongTiles::MahjongTileSummer => '🀧',
            MahjongTiles::MahjongTileAutumn => '🀨',
            MahjongTiles::MahjongTileWinter => '🀩',
            MahjongTiles::MahjongTileJoker => '🀪',
            MahjongTiles::MahjongTileBack => '🀫',
        }
    }
}

impl std::convert::TryFrom<char> for MahjongTiles {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🀀' => Ok(MahjongTiles::MahjongTileEastWind),
            '🀁' => Ok(MahjongTiles::MahjongTileSouthWind),
            '🀂' => Ok(MahjongTiles::MahjongTileWestWind),
            '🀃' => Ok(MahjongTiles::MahjongTileNorthWind),
            '🀄' => Ok(MahjongTiles::MahjongTileRedDragon),
            '🀅' => Ok(MahjongTiles::MahjongTileGreenDragon),
            '🀆' => Ok(MahjongTiles::MahjongTileWhiteDragon),
            '🀇' => Ok(MahjongTiles::MahjongTileOneOfCharacters),
            '🀈' => Ok(MahjongTiles::MahjongTileTwoOfCharacters),
            '🀉' => Ok(MahjongTiles::MahjongTileThreeOfCharacters),
            '🀊' => Ok(MahjongTiles::MahjongTileFourOfCharacters),
            '🀋' => Ok(MahjongTiles::MahjongTileFiveOfCharacters),
            '🀌' => Ok(MahjongTiles::MahjongTileSixOfCharacters),
            '🀍' => Ok(MahjongTiles::MahjongTileSevenOfCharacters),
            '🀎' => Ok(MahjongTiles::MahjongTileEightOfCharacters),
            '🀏' => Ok(MahjongTiles::MahjongTileNineOfCharacters),
            '🀐' => Ok(MahjongTiles::MahjongTileOneOfBamboos),
            '🀑' => Ok(MahjongTiles::MahjongTileTwoOfBamboos),
            '🀒' => Ok(MahjongTiles::MahjongTileThreeOfBamboos),
            '🀓' => Ok(MahjongTiles::MahjongTileFourOfBamboos),
            '🀔' => Ok(MahjongTiles::MahjongTileFiveOfBamboos),
            '🀕' => Ok(MahjongTiles::MahjongTileSixOfBamboos),
            '🀖' => Ok(MahjongTiles::MahjongTileSevenOfBamboos),
            '🀗' => Ok(MahjongTiles::MahjongTileEightOfBamboos),
            '🀘' => Ok(MahjongTiles::MahjongTileNineOfBamboos),
            '🀙' => Ok(MahjongTiles::MahjongTileOneOfCircles),
            '🀚' => Ok(MahjongTiles::MahjongTileTwoOfCircles),
            '🀛' => Ok(MahjongTiles::MahjongTileThreeOfCircles),
            '🀜' => Ok(MahjongTiles::MahjongTileFourOfCircles),
            '🀝' => Ok(MahjongTiles::MahjongTileFiveOfCircles),
            '🀞' => Ok(MahjongTiles::MahjongTileSixOfCircles),
            '🀟' => Ok(MahjongTiles::MahjongTileSevenOfCircles),
            '🀠' => Ok(MahjongTiles::MahjongTileEightOfCircles),
            '🀡' => Ok(MahjongTiles::MahjongTileNineOfCircles),
            '🀢' => Ok(MahjongTiles::MahjongTilePlum),
            '🀣' => Ok(MahjongTiles::MahjongTileOrchid),
            '🀤' => Ok(MahjongTiles::MahjongTileBamboo),
            '🀥' => Ok(MahjongTiles::MahjongTileChrysanthemum),
            '🀦' => Ok(MahjongTiles::MahjongTileSpring),
            '🀧' => Ok(MahjongTiles::MahjongTileSummer),
            '🀨' => Ok(MahjongTiles::MahjongTileAutumn),
            '🀩' => Ok(MahjongTiles::MahjongTileWinter),
            '🀪' => Ok(MahjongTiles::MahjongTileJoker),
            '🀫' => Ok(MahjongTiles::MahjongTileBack),
            _ => Err(()),
        }
    }
}

impl Into<u32> for MahjongTiles {
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

impl std::convert::TryFrom<u32> for MahjongTiles {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for MahjongTiles {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl MahjongTiles {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        MahjongTiles::MahjongTileEastWind
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            MahjongTiles::MahjongTileEastWind => "mahjong tile east wind",
            MahjongTiles::MahjongTileSouthWind => "mahjong tile south wind",
            MahjongTiles::MahjongTileWestWind => "mahjong tile west wind",
            MahjongTiles::MahjongTileNorthWind => "mahjong tile north wind",
            MahjongTiles::MahjongTileRedDragon => "mahjong tile red dragon",
            MahjongTiles::MahjongTileGreenDragon => "mahjong tile green dragon",
            MahjongTiles::MahjongTileWhiteDragon => "mahjong tile white dragon",
            MahjongTiles::MahjongTileOneOfCharacters => "mahjong tile one of characters",
            MahjongTiles::MahjongTileTwoOfCharacters => "mahjong tile two of characters",
            MahjongTiles::MahjongTileThreeOfCharacters => "mahjong tile three of characters",
            MahjongTiles::MahjongTileFourOfCharacters => "mahjong tile four of characters",
            MahjongTiles::MahjongTileFiveOfCharacters => "mahjong tile five of characters",
            MahjongTiles::MahjongTileSixOfCharacters => "mahjong tile six of characters",
            MahjongTiles::MahjongTileSevenOfCharacters => "mahjong tile seven of characters",
            MahjongTiles::MahjongTileEightOfCharacters => "mahjong tile eight of characters",
            MahjongTiles::MahjongTileNineOfCharacters => "mahjong tile nine of characters",
            MahjongTiles::MahjongTileOneOfBamboos => "mahjong tile one of bamboos",
            MahjongTiles::MahjongTileTwoOfBamboos => "mahjong tile two of bamboos",
            MahjongTiles::MahjongTileThreeOfBamboos => "mahjong tile three of bamboos",
            MahjongTiles::MahjongTileFourOfBamboos => "mahjong tile four of bamboos",
            MahjongTiles::MahjongTileFiveOfBamboos => "mahjong tile five of bamboos",
            MahjongTiles::MahjongTileSixOfBamboos => "mahjong tile six of bamboos",
            MahjongTiles::MahjongTileSevenOfBamboos => "mahjong tile seven of bamboos",
            MahjongTiles::MahjongTileEightOfBamboos => "mahjong tile eight of bamboos",
            MahjongTiles::MahjongTileNineOfBamboos => "mahjong tile nine of bamboos",
            MahjongTiles::MahjongTileOneOfCircles => "mahjong tile one of circles",
            MahjongTiles::MahjongTileTwoOfCircles => "mahjong tile two of circles",
            MahjongTiles::MahjongTileThreeOfCircles => "mahjong tile three of circles",
            MahjongTiles::MahjongTileFourOfCircles => "mahjong tile four of circles",
            MahjongTiles::MahjongTileFiveOfCircles => "mahjong tile five of circles",
            MahjongTiles::MahjongTileSixOfCircles => "mahjong tile six of circles",
            MahjongTiles::MahjongTileSevenOfCircles => "mahjong tile seven of circles",
            MahjongTiles::MahjongTileEightOfCircles => "mahjong tile eight of circles",
            MahjongTiles::MahjongTileNineOfCircles => "mahjong tile nine of circles",
            MahjongTiles::MahjongTilePlum => "mahjong tile plum",
            MahjongTiles::MahjongTileOrchid => "mahjong tile orchid",
            MahjongTiles::MahjongTileBamboo => "mahjong tile bamboo",
            MahjongTiles::MahjongTileChrysanthemum => "mahjong tile chrysanthemum",
            MahjongTiles::MahjongTileSpring => "mahjong tile spring",
            MahjongTiles::MahjongTileSummer => "mahjong tile summer",
            MahjongTiles::MahjongTileAutumn => "mahjong tile autumn",
            MahjongTiles::MahjongTileWinter => "mahjong tile winter",
            MahjongTiles::MahjongTileJoker => "mahjong tile joker",
            MahjongTiles::MahjongTileBack => "mahjong tile back",
        }
    }
}
