
/// An enum to represent all characters in the PlayingCards block.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PlayingCards {
    /// \u{1f0a0}: '🂠'
    PlayingCardBack,
    /// \u{1f0a1}: '🂡'
    PlayingCardAceOfSpades,
    /// \u{1f0a2}: '🂢'
    PlayingCardTwoOfSpades,
    /// \u{1f0a3}: '🂣'
    PlayingCardThreeOfSpades,
    /// \u{1f0a4}: '🂤'
    PlayingCardFourOfSpades,
    /// \u{1f0a5}: '🂥'
    PlayingCardFiveOfSpades,
    /// \u{1f0a6}: '🂦'
    PlayingCardSixOfSpades,
    /// \u{1f0a7}: '🂧'
    PlayingCardSevenOfSpades,
    /// \u{1f0a8}: '🂨'
    PlayingCardEightOfSpades,
    /// \u{1f0a9}: '🂩'
    PlayingCardNineOfSpades,
    /// \u{1f0aa}: '🂪'
    PlayingCardTenOfSpades,
    /// \u{1f0ab}: '🂫'
    PlayingCardJackOfSpades,
    /// \u{1f0ac}: '🂬'
    PlayingCardKnightOfSpades,
    /// \u{1f0ad}: '🂭'
    PlayingCardQueenOfSpades,
    /// \u{1f0ae}: '🂮'
    PlayingCardKingOfSpades,
    /// \u{1f0b1}: '🂱'
    PlayingCardAceOfHearts,
    /// \u{1f0b2}: '🂲'
    PlayingCardTwoOfHearts,
    /// \u{1f0b3}: '🂳'
    PlayingCardThreeOfHearts,
    /// \u{1f0b4}: '🂴'
    PlayingCardFourOfHearts,
    /// \u{1f0b5}: '🂵'
    PlayingCardFiveOfHearts,
    /// \u{1f0b6}: '🂶'
    PlayingCardSixOfHearts,
    /// \u{1f0b7}: '🂷'
    PlayingCardSevenOfHearts,
    /// \u{1f0b8}: '🂸'
    PlayingCardEightOfHearts,
    /// \u{1f0b9}: '🂹'
    PlayingCardNineOfHearts,
    /// \u{1f0ba}: '🂺'
    PlayingCardTenOfHearts,
    /// \u{1f0bb}: '🂻'
    PlayingCardJackOfHearts,
    /// \u{1f0bc}: '🂼'
    PlayingCardKnightOfHearts,
    /// \u{1f0bd}: '🂽'
    PlayingCardQueenOfHearts,
    /// \u{1f0be}: '🂾'
    PlayingCardKingOfHearts,
    /// \u{1f0bf}: '🂿'
    PlayingCardRedJoker,
    /// \u{1f0c1}: '🃁'
    PlayingCardAceOfDiamonds,
    /// \u{1f0c2}: '🃂'
    PlayingCardTwoOfDiamonds,
    /// \u{1f0c3}: '🃃'
    PlayingCardThreeOfDiamonds,
    /// \u{1f0c4}: '🃄'
    PlayingCardFourOfDiamonds,
    /// \u{1f0c5}: '🃅'
    PlayingCardFiveOfDiamonds,
    /// \u{1f0c6}: '🃆'
    PlayingCardSixOfDiamonds,
    /// \u{1f0c7}: '🃇'
    PlayingCardSevenOfDiamonds,
    /// \u{1f0c8}: '🃈'
    PlayingCardEightOfDiamonds,
    /// \u{1f0c9}: '🃉'
    PlayingCardNineOfDiamonds,
    /// \u{1f0ca}: '🃊'
    PlayingCardTenOfDiamonds,
    /// \u{1f0cb}: '🃋'
    PlayingCardJackOfDiamonds,
    /// \u{1f0cc}: '🃌'
    PlayingCardKnightOfDiamonds,
    /// \u{1f0cd}: '🃍'
    PlayingCardQueenOfDiamonds,
    /// \u{1f0ce}: '🃎'
    PlayingCardKingOfDiamonds,
    /// \u{1f0cf}: '🃏'
    PlayingCardBlackJoker,
    /// \u{1f0d1}: '🃑'
    PlayingCardAceOfClubs,
    /// \u{1f0d2}: '🃒'
    PlayingCardTwoOfClubs,
    /// \u{1f0d3}: '🃓'
    PlayingCardThreeOfClubs,
    /// \u{1f0d4}: '🃔'
    PlayingCardFourOfClubs,
    /// \u{1f0d5}: '🃕'
    PlayingCardFiveOfClubs,
    /// \u{1f0d6}: '🃖'
    PlayingCardSixOfClubs,
    /// \u{1f0d7}: '🃗'
    PlayingCardSevenOfClubs,
    /// \u{1f0d8}: '🃘'
    PlayingCardEightOfClubs,
    /// \u{1f0d9}: '🃙'
    PlayingCardNineOfClubs,
    /// \u{1f0da}: '🃚'
    PlayingCardTenOfClubs,
    /// \u{1f0db}: '🃛'
    PlayingCardJackOfClubs,
    /// \u{1f0dc}: '🃜'
    PlayingCardKnightOfClubs,
    /// \u{1f0dd}: '🃝'
    PlayingCardQueenOfClubs,
    /// \u{1f0de}: '🃞'
    PlayingCardKingOfClubs,
    /// \u{1f0df}: '🃟'
    PlayingCardWhiteJoker,
    /// \u{1f0e0}: '🃠'
    PlayingCardFool,
    /// \u{1f0e1}: '🃡'
    PlayingCardTrumpDash1,
    /// \u{1f0e2}: '🃢'
    PlayingCardTrumpDash2,
    /// \u{1f0e3}: '🃣'
    PlayingCardTrumpDash3,
    /// \u{1f0e4}: '🃤'
    PlayingCardTrumpDash4,
    /// \u{1f0e5}: '🃥'
    PlayingCardTrumpDash5,
    /// \u{1f0e6}: '🃦'
    PlayingCardTrumpDash6,
    /// \u{1f0e7}: '🃧'
    PlayingCardTrumpDash7,
    /// \u{1f0e8}: '🃨'
    PlayingCardTrumpDash8,
    /// \u{1f0e9}: '🃩'
    PlayingCardTrumpDash9,
    /// \u{1f0ea}: '🃪'
    PlayingCardTrumpDash10,
    /// \u{1f0eb}: '🃫'
    PlayingCardTrumpDash11,
    /// \u{1f0ec}: '🃬'
    PlayingCardTrumpDash12,
    /// \u{1f0ed}: '🃭'
    PlayingCardTrumpDash13,
    /// \u{1f0ee}: '🃮'
    PlayingCardTrumpDash14,
    /// \u{1f0ef}: '🃯'
    PlayingCardTrumpDash15,
    /// \u{1f0f0}: '🃰'
    PlayingCardTrumpDash16,
    /// \u{1f0f1}: '🃱'
    PlayingCardTrumpDash17,
    /// \u{1f0f2}: '🃲'
    PlayingCardTrumpDash18,
    /// \u{1f0f3}: '🃳'
    PlayingCardTrumpDash19,
    /// \u{1f0f4}: '🃴'
    PlayingCardTrumpDash20,
    /// \u{1f0f5}: '🃵'
    PlayingCardTrumpDash21,
}

impl Into<char> for PlayingCards {
    fn into(self) -> char {
        match self {
            PlayingCards::PlayingCardBack => '🂠',
            PlayingCards::PlayingCardAceOfSpades => '🂡',
            PlayingCards::PlayingCardTwoOfSpades => '🂢',
            PlayingCards::PlayingCardThreeOfSpades => '🂣',
            PlayingCards::PlayingCardFourOfSpades => '🂤',
            PlayingCards::PlayingCardFiveOfSpades => '🂥',
            PlayingCards::PlayingCardSixOfSpades => '🂦',
            PlayingCards::PlayingCardSevenOfSpades => '🂧',
            PlayingCards::PlayingCardEightOfSpades => '🂨',
            PlayingCards::PlayingCardNineOfSpades => '🂩',
            PlayingCards::PlayingCardTenOfSpades => '🂪',
            PlayingCards::PlayingCardJackOfSpades => '🂫',
            PlayingCards::PlayingCardKnightOfSpades => '🂬',
            PlayingCards::PlayingCardQueenOfSpades => '🂭',
            PlayingCards::PlayingCardKingOfSpades => '🂮',
            PlayingCards::PlayingCardAceOfHearts => '🂱',
            PlayingCards::PlayingCardTwoOfHearts => '🂲',
            PlayingCards::PlayingCardThreeOfHearts => '🂳',
            PlayingCards::PlayingCardFourOfHearts => '🂴',
            PlayingCards::PlayingCardFiveOfHearts => '🂵',
            PlayingCards::PlayingCardSixOfHearts => '🂶',
            PlayingCards::PlayingCardSevenOfHearts => '🂷',
            PlayingCards::PlayingCardEightOfHearts => '🂸',
            PlayingCards::PlayingCardNineOfHearts => '🂹',
            PlayingCards::PlayingCardTenOfHearts => '🂺',
            PlayingCards::PlayingCardJackOfHearts => '🂻',
            PlayingCards::PlayingCardKnightOfHearts => '🂼',
            PlayingCards::PlayingCardQueenOfHearts => '🂽',
            PlayingCards::PlayingCardKingOfHearts => '🂾',
            PlayingCards::PlayingCardRedJoker => '🂿',
            PlayingCards::PlayingCardAceOfDiamonds => '🃁',
            PlayingCards::PlayingCardTwoOfDiamonds => '🃂',
            PlayingCards::PlayingCardThreeOfDiamonds => '🃃',
            PlayingCards::PlayingCardFourOfDiamonds => '🃄',
            PlayingCards::PlayingCardFiveOfDiamonds => '🃅',
            PlayingCards::PlayingCardSixOfDiamonds => '🃆',
            PlayingCards::PlayingCardSevenOfDiamonds => '🃇',
            PlayingCards::PlayingCardEightOfDiamonds => '🃈',
            PlayingCards::PlayingCardNineOfDiamonds => '🃉',
            PlayingCards::PlayingCardTenOfDiamonds => '🃊',
            PlayingCards::PlayingCardJackOfDiamonds => '🃋',
            PlayingCards::PlayingCardKnightOfDiamonds => '🃌',
            PlayingCards::PlayingCardQueenOfDiamonds => '🃍',
            PlayingCards::PlayingCardKingOfDiamonds => '🃎',
            PlayingCards::PlayingCardBlackJoker => '🃏',
            PlayingCards::PlayingCardAceOfClubs => '🃑',
            PlayingCards::PlayingCardTwoOfClubs => '🃒',
            PlayingCards::PlayingCardThreeOfClubs => '🃓',
            PlayingCards::PlayingCardFourOfClubs => '🃔',
            PlayingCards::PlayingCardFiveOfClubs => '🃕',
            PlayingCards::PlayingCardSixOfClubs => '🃖',
            PlayingCards::PlayingCardSevenOfClubs => '🃗',
            PlayingCards::PlayingCardEightOfClubs => '🃘',
            PlayingCards::PlayingCardNineOfClubs => '🃙',
            PlayingCards::PlayingCardTenOfClubs => '🃚',
            PlayingCards::PlayingCardJackOfClubs => '🃛',
            PlayingCards::PlayingCardKnightOfClubs => '🃜',
            PlayingCards::PlayingCardQueenOfClubs => '🃝',
            PlayingCards::PlayingCardKingOfClubs => '🃞',
            PlayingCards::PlayingCardWhiteJoker => '🃟',
            PlayingCards::PlayingCardFool => '🃠',
            PlayingCards::PlayingCardTrumpDash1 => '🃡',
            PlayingCards::PlayingCardTrumpDash2 => '🃢',
            PlayingCards::PlayingCardTrumpDash3 => '🃣',
            PlayingCards::PlayingCardTrumpDash4 => '🃤',
            PlayingCards::PlayingCardTrumpDash5 => '🃥',
            PlayingCards::PlayingCardTrumpDash6 => '🃦',
            PlayingCards::PlayingCardTrumpDash7 => '🃧',
            PlayingCards::PlayingCardTrumpDash8 => '🃨',
            PlayingCards::PlayingCardTrumpDash9 => '🃩',
            PlayingCards::PlayingCardTrumpDash10 => '🃪',
            PlayingCards::PlayingCardTrumpDash11 => '🃫',
            PlayingCards::PlayingCardTrumpDash12 => '🃬',
            PlayingCards::PlayingCardTrumpDash13 => '🃭',
            PlayingCards::PlayingCardTrumpDash14 => '🃮',
            PlayingCards::PlayingCardTrumpDash15 => '🃯',
            PlayingCards::PlayingCardTrumpDash16 => '🃰',
            PlayingCards::PlayingCardTrumpDash17 => '🃱',
            PlayingCards::PlayingCardTrumpDash18 => '🃲',
            PlayingCards::PlayingCardTrumpDash19 => '🃳',
            PlayingCards::PlayingCardTrumpDash20 => '🃴',
            PlayingCards::PlayingCardTrumpDash21 => '🃵',
        }
    }
}

impl std::convert::TryFrom<char> for PlayingCards {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '🂠' => Ok(PlayingCards::PlayingCardBack),
            '🂡' => Ok(PlayingCards::PlayingCardAceOfSpades),
            '🂢' => Ok(PlayingCards::PlayingCardTwoOfSpades),
            '🂣' => Ok(PlayingCards::PlayingCardThreeOfSpades),
            '🂤' => Ok(PlayingCards::PlayingCardFourOfSpades),
            '🂥' => Ok(PlayingCards::PlayingCardFiveOfSpades),
            '🂦' => Ok(PlayingCards::PlayingCardSixOfSpades),
            '🂧' => Ok(PlayingCards::PlayingCardSevenOfSpades),
            '🂨' => Ok(PlayingCards::PlayingCardEightOfSpades),
            '🂩' => Ok(PlayingCards::PlayingCardNineOfSpades),
            '🂪' => Ok(PlayingCards::PlayingCardTenOfSpades),
            '🂫' => Ok(PlayingCards::PlayingCardJackOfSpades),
            '🂬' => Ok(PlayingCards::PlayingCardKnightOfSpades),
            '🂭' => Ok(PlayingCards::PlayingCardQueenOfSpades),
            '🂮' => Ok(PlayingCards::PlayingCardKingOfSpades),
            '🂱' => Ok(PlayingCards::PlayingCardAceOfHearts),
            '🂲' => Ok(PlayingCards::PlayingCardTwoOfHearts),
            '🂳' => Ok(PlayingCards::PlayingCardThreeOfHearts),
            '🂴' => Ok(PlayingCards::PlayingCardFourOfHearts),
            '🂵' => Ok(PlayingCards::PlayingCardFiveOfHearts),
            '🂶' => Ok(PlayingCards::PlayingCardSixOfHearts),
            '🂷' => Ok(PlayingCards::PlayingCardSevenOfHearts),
            '🂸' => Ok(PlayingCards::PlayingCardEightOfHearts),
            '🂹' => Ok(PlayingCards::PlayingCardNineOfHearts),
            '🂺' => Ok(PlayingCards::PlayingCardTenOfHearts),
            '🂻' => Ok(PlayingCards::PlayingCardJackOfHearts),
            '🂼' => Ok(PlayingCards::PlayingCardKnightOfHearts),
            '🂽' => Ok(PlayingCards::PlayingCardQueenOfHearts),
            '🂾' => Ok(PlayingCards::PlayingCardKingOfHearts),
            '🂿' => Ok(PlayingCards::PlayingCardRedJoker),
            '🃁' => Ok(PlayingCards::PlayingCardAceOfDiamonds),
            '🃂' => Ok(PlayingCards::PlayingCardTwoOfDiamonds),
            '🃃' => Ok(PlayingCards::PlayingCardThreeOfDiamonds),
            '🃄' => Ok(PlayingCards::PlayingCardFourOfDiamonds),
            '🃅' => Ok(PlayingCards::PlayingCardFiveOfDiamonds),
            '🃆' => Ok(PlayingCards::PlayingCardSixOfDiamonds),
            '🃇' => Ok(PlayingCards::PlayingCardSevenOfDiamonds),
            '🃈' => Ok(PlayingCards::PlayingCardEightOfDiamonds),
            '🃉' => Ok(PlayingCards::PlayingCardNineOfDiamonds),
            '🃊' => Ok(PlayingCards::PlayingCardTenOfDiamonds),
            '🃋' => Ok(PlayingCards::PlayingCardJackOfDiamonds),
            '🃌' => Ok(PlayingCards::PlayingCardKnightOfDiamonds),
            '🃍' => Ok(PlayingCards::PlayingCardQueenOfDiamonds),
            '🃎' => Ok(PlayingCards::PlayingCardKingOfDiamonds),
            '🃏' => Ok(PlayingCards::PlayingCardBlackJoker),
            '🃑' => Ok(PlayingCards::PlayingCardAceOfClubs),
            '🃒' => Ok(PlayingCards::PlayingCardTwoOfClubs),
            '🃓' => Ok(PlayingCards::PlayingCardThreeOfClubs),
            '🃔' => Ok(PlayingCards::PlayingCardFourOfClubs),
            '🃕' => Ok(PlayingCards::PlayingCardFiveOfClubs),
            '🃖' => Ok(PlayingCards::PlayingCardSixOfClubs),
            '🃗' => Ok(PlayingCards::PlayingCardSevenOfClubs),
            '🃘' => Ok(PlayingCards::PlayingCardEightOfClubs),
            '🃙' => Ok(PlayingCards::PlayingCardNineOfClubs),
            '🃚' => Ok(PlayingCards::PlayingCardTenOfClubs),
            '🃛' => Ok(PlayingCards::PlayingCardJackOfClubs),
            '🃜' => Ok(PlayingCards::PlayingCardKnightOfClubs),
            '🃝' => Ok(PlayingCards::PlayingCardQueenOfClubs),
            '🃞' => Ok(PlayingCards::PlayingCardKingOfClubs),
            '🃟' => Ok(PlayingCards::PlayingCardWhiteJoker),
            '🃠' => Ok(PlayingCards::PlayingCardFool),
            '🃡' => Ok(PlayingCards::PlayingCardTrumpDash1),
            '🃢' => Ok(PlayingCards::PlayingCardTrumpDash2),
            '🃣' => Ok(PlayingCards::PlayingCardTrumpDash3),
            '🃤' => Ok(PlayingCards::PlayingCardTrumpDash4),
            '🃥' => Ok(PlayingCards::PlayingCardTrumpDash5),
            '🃦' => Ok(PlayingCards::PlayingCardTrumpDash6),
            '🃧' => Ok(PlayingCards::PlayingCardTrumpDash7),
            '🃨' => Ok(PlayingCards::PlayingCardTrumpDash8),
            '🃩' => Ok(PlayingCards::PlayingCardTrumpDash9),
            '🃪' => Ok(PlayingCards::PlayingCardTrumpDash10),
            '🃫' => Ok(PlayingCards::PlayingCardTrumpDash11),
            '🃬' => Ok(PlayingCards::PlayingCardTrumpDash12),
            '🃭' => Ok(PlayingCards::PlayingCardTrumpDash13),
            '🃮' => Ok(PlayingCards::PlayingCardTrumpDash14),
            '🃯' => Ok(PlayingCards::PlayingCardTrumpDash15),
            '🃰' => Ok(PlayingCards::PlayingCardTrumpDash16),
            '🃱' => Ok(PlayingCards::PlayingCardTrumpDash17),
            '🃲' => Ok(PlayingCards::PlayingCardTrumpDash18),
            '🃳' => Ok(PlayingCards::PlayingCardTrumpDash19),
            '🃴' => Ok(PlayingCards::PlayingCardTrumpDash20),
            '🃵' => Ok(PlayingCards::PlayingCardTrumpDash21),
            _ => Err(()),
        }
    }
}

impl Into<u32> for PlayingCards {
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

impl std::convert::TryFrom<u32> for PlayingCards {
    type Error = ();
    fn try_from(u: u32) -> Result<Self, Self::Error> {
        if let Ok(c) = char::try_from(u) {
            Self::try_from(c)
        } else {
            Err(())
        }
    }
}

impl Iterator for PlayingCards {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        let index: u32 = (*self).into();
        use std::convert::TryFrom;
        Self::try_from(index + 1).ok()
    }
}

impl PlayingCards {
    /// The character with the lowest index in this unicode block
    pub fn new() -> Self {
        PlayingCards::PlayingCardBack
    }

    /// The character's name, all lowercase and space-separated
    pub fn name(&self) -> &str {
        match self {
            PlayingCards::PlayingCardBack => "playing card back",
            PlayingCards::PlayingCardAceOfSpades => "playing card ace of spades",
            PlayingCards::PlayingCardTwoOfSpades => "playing card two of spades",
            PlayingCards::PlayingCardThreeOfSpades => "playing card three of spades",
            PlayingCards::PlayingCardFourOfSpades => "playing card four of spades",
            PlayingCards::PlayingCardFiveOfSpades => "playing card five of spades",
            PlayingCards::PlayingCardSixOfSpades => "playing card six of spades",
            PlayingCards::PlayingCardSevenOfSpades => "playing card seven of spades",
            PlayingCards::PlayingCardEightOfSpades => "playing card eight of spades",
            PlayingCards::PlayingCardNineOfSpades => "playing card nine of spades",
            PlayingCards::PlayingCardTenOfSpades => "playing card ten of spades",
            PlayingCards::PlayingCardJackOfSpades => "playing card jack of spades",
            PlayingCards::PlayingCardKnightOfSpades => "playing card knight of spades",
            PlayingCards::PlayingCardQueenOfSpades => "playing card queen of spades",
            PlayingCards::PlayingCardKingOfSpades => "playing card king of spades",
            PlayingCards::PlayingCardAceOfHearts => "playing card ace of hearts",
            PlayingCards::PlayingCardTwoOfHearts => "playing card two of hearts",
            PlayingCards::PlayingCardThreeOfHearts => "playing card three of hearts",
            PlayingCards::PlayingCardFourOfHearts => "playing card four of hearts",
            PlayingCards::PlayingCardFiveOfHearts => "playing card five of hearts",
            PlayingCards::PlayingCardSixOfHearts => "playing card six of hearts",
            PlayingCards::PlayingCardSevenOfHearts => "playing card seven of hearts",
            PlayingCards::PlayingCardEightOfHearts => "playing card eight of hearts",
            PlayingCards::PlayingCardNineOfHearts => "playing card nine of hearts",
            PlayingCards::PlayingCardTenOfHearts => "playing card ten of hearts",
            PlayingCards::PlayingCardJackOfHearts => "playing card jack of hearts",
            PlayingCards::PlayingCardKnightOfHearts => "playing card knight of hearts",
            PlayingCards::PlayingCardQueenOfHearts => "playing card queen of hearts",
            PlayingCards::PlayingCardKingOfHearts => "playing card king of hearts",
            PlayingCards::PlayingCardRedJoker => "playing card red joker",
            PlayingCards::PlayingCardAceOfDiamonds => "playing card ace of diamonds",
            PlayingCards::PlayingCardTwoOfDiamonds => "playing card two of diamonds",
            PlayingCards::PlayingCardThreeOfDiamonds => "playing card three of diamonds",
            PlayingCards::PlayingCardFourOfDiamonds => "playing card four of diamonds",
            PlayingCards::PlayingCardFiveOfDiamonds => "playing card five of diamonds",
            PlayingCards::PlayingCardSixOfDiamonds => "playing card six of diamonds",
            PlayingCards::PlayingCardSevenOfDiamonds => "playing card seven of diamonds",
            PlayingCards::PlayingCardEightOfDiamonds => "playing card eight of diamonds",
            PlayingCards::PlayingCardNineOfDiamonds => "playing card nine of diamonds",
            PlayingCards::PlayingCardTenOfDiamonds => "playing card ten of diamonds",
            PlayingCards::PlayingCardJackOfDiamonds => "playing card jack of diamonds",
            PlayingCards::PlayingCardKnightOfDiamonds => "playing card knight of diamonds",
            PlayingCards::PlayingCardQueenOfDiamonds => "playing card queen of diamonds",
            PlayingCards::PlayingCardKingOfDiamonds => "playing card king of diamonds",
            PlayingCards::PlayingCardBlackJoker => "playing card black joker",
            PlayingCards::PlayingCardAceOfClubs => "playing card ace of clubs",
            PlayingCards::PlayingCardTwoOfClubs => "playing card two of clubs",
            PlayingCards::PlayingCardThreeOfClubs => "playing card three of clubs",
            PlayingCards::PlayingCardFourOfClubs => "playing card four of clubs",
            PlayingCards::PlayingCardFiveOfClubs => "playing card five of clubs",
            PlayingCards::PlayingCardSixOfClubs => "playing card six of clubs",
            PlayingCards::PlayingCardSevenOfClubs => "playing card seven of clubs",
            PlayingCards::PlayingCardEightOfClubs => "playing card eight of clubs",
            PlayingCards::PlayingCardNineOfClubs => "playing card nine of clubs",
            PlayingCards::PlayingCardTenOfClubs => "playing card ten of clubs",
            PlayingCards::PlayingCardJackOfClubs => "playing card jack of clubs",
            PlayingCards::PlayingCardKnightOfClubs => "playing card knight of clubs",
            PlayingCards::PlayingCardQueenOfClubs => "playing card queen of clubs",
            PlayingCards::PlayingCardKingOfClubs => "playing card king of clubs",
            PlayingCards::PlayingCardWhiteJoker => "playing card white joker",
            PlayingCards::PlayingCardFool => "playing card fool",
            PlayingCards::PlayingCardTrumpDash1 => "playing card trump-1",
            PlayingCards::PlayingCardTrumpDash2 => "playing card trump-2",
            PlayingCards::PlayingCardTrumpDash3 => "playing card trump-3",
            PlayingCards::PlayingCardTrumpDash4 => "playing card trump-4",
            PlayingCards::PlayingCardTrumpDash5 => "playing card trump-5",
            PlayingCards::PlayingCardTrumpDash6 => "playing card trump-6",
            PlayingCards::PlayingCardTrumpDash7 => "playing card trump-7",
            PlayingCards::PlayingCardTrumpDash8 => "playing card trump-8",
            PlayingCards::PlayingCardTrumpDash9 => "playing card trump-9",
            PlayingCards::PlayingCardTrumpDash10 => "playing card trump-10",
            PlayingCards::PlayingCardTrumpDash11 => "playing card trump-11",
            PlayingCards::PlayingCardTrumpDash12 => "playing card trump-12",
            PlayingCards::PlayingCardTrumpDash13 => "playing card trump-13",
            PlayingCards::PlayingCardTrumpDash14 => "playing card trump-14",
            PlayingCards::PlayingCardTrumpDash15 => "playing card trump-15",
            PlayingCards::PlayingCardTrumpDash16 => "playing card trump-16",
            PlayingCards::PlayingCardTrumpDash17 => "playing card trump-17",
            PlayingCards::PlayingCardTrumpDash18 => "playing card trump-18",
            PlayingCards::PlayingCardTrumpDash19 => "playing card trump-19",
            PlayingCards::PlayingCardTrumpDash20 => "playing card trump-20",
            PlayingCards::PlayingCardTrumpDash21 => "playing card trump-21",
        }
    }
}
