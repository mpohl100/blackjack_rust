#[derive(Default, Clone)]
pub struct StrategyConfiguration{
    pub nb_threads: i32, // the number of threads to use during startegy calculation
}

#[derive(Default, Clone)]
pub struct PlayConfiguration{
    pub nb_hands: i32, // the number of hands to play during the play phase
    pub play_normal: bool, // true if we use EightDecks during play phase, false if we use CountedDeck(0)
}