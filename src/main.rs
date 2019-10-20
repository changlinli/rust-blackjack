use std::io::{self, BufRead};
use rand::{thread_rng, Rng};
use std::convert::identity;

#[derive(Debug)]
enum Action {
    Hit,
    Stand,
    DoubleDown,
    SplitCards,
    Surrender
}

#[derive(Debug, Eq, PartialEq)]
enum GameState {
    GameWon,
    GameLost,
    Continuing(ContinuingGameState)
}

impl GameState {
    fn start<R: Rng>(rng: &mut R) -> GameState {
        let internal_state = ContinuingGameState {
            deck: {
                let mut unshuffled_deck = Deck::new();
                unshuffled_deck.shuffle(rng);
                unshuffled_deck
            },
            hand: Vec::new()
        };
        GameState::Continuing(internal_state)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CardSuit {
    Clubs,
    Hearts,
    Diamonds,
    Spades
}

impl CardSuit {
    const ALL_VALUES: [CardSuit; 4] = [
        CardSuit::Clubs,
        CardSuit::Hearts,
        CardSuit::Diamonds,
        CardSuit::Spades
    ];
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HandValue {
    value: u32
}

impl HandValue {
    pub fn from_u32(x: u32) -> Option<HandValue> {
        if x < 21 {
            Option::Some(
                HandValue {
                    value: x,
                }
            )
        } else {
            Option::None
        }
    }

    pub fn unsafe_from_u32(x: u32) -> HandValue {
        HandValue::from_u32(x).unwrap()
    }

    pub fn value(self) -> u32 {
        self.value
    }

    pub fn combine_with_separate_value(&self, other_value: &HandValue) -> Option<HandValue> {
        let raw_result = self.value + other_value.value;
        if raw_result <= 21 {
            Option::Some(
                HandValue {
                    value: raw_result
                }
            )
        } else {
            Option::None
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum CardValue {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl CardValue {
    const ALL_VALUES: [CardValue; 13] = [
        CardValue::Two,
        CardValue::Three,
        CardValue::Four,
        CardValue::Five,
        CardValue::Six,
        CardValue::Seven,
        CardValue::Eight,
        CardValue::Nine,
        CardValue::Ten,
        CardValue::Jack,
        CardValue::Queen,
        CardValue::King,
        CardValue::Ace,
    ];
}

fn card_value_to_hand_value(card_value: &CardValue) -> Vec<HandValue> {
    match card_value {
        CardValue::Two => vec![HandValue::unsafe_from_u32(2)],
        CardValue::Three=> vec![HandValue::unsafe_from_u32(3)],
        CardValue::Four => vec![HandValue::unsafe_from_u32(4)],
        CardValue::Five => vec![HandValue::unsafe_from_u32(5)],
        CardValue::Six => vec![HandValue::unsafe_from_u32(6)],
        CardValue::Seven => vec![HandValue::unsafe_from_u32(7)],
        CardValue::Eight => vec![HandValue::unsafe_from_u32(8)],
        CardValue::Nine => vec![HandValue::unsafe_from_u32(9)],
        CardValue::Ten => vec![HandValue::unsafe_from_u32(10)],
        CardValue::Jack => vec![HandValue::unsafe_from_u32(10)],
        CardValue::Queen => vec![HandValue::unsafe_from_u32(10)],
        CardValue::King => vec![HandValue::unsafe_from_u32(10)],
        CardValue::Ace => vec![HandValue::unsafe_from_u32(1), HandValue::unsafe_from_u32(11)],
    }
}

fn combine_possible_values(values_0: &Vec<HandValue>, values_1: &Vec<HandValue>) -> Vec<HandValue> {
    values_0
        .iter()
        .flat_map::<Vec<Option<HandValue>>, _>(
            |x: &HandValue| {
                let result = values_1
                    .iter()
                    .map(|y: &HandValue| x.combine_with_separate_value(y))
                    .collect();
                result
            }
        )
        .filter_map(identity)
        .collect()
}

fn calculate_current_hand_value(hand: &Vec<CardValue>) -> Vec<HandValue> {
    hand
        .iter()
        .map(card_value_to_hand_value)
        .fold(
            vec![HandValue::unsafe_from_u32(0)],
            |x, y| combine_possible_values(&x, &y)
        )
}

#[derive(Debug, Eq, PartialEq)]
struct Card {
    suit: CardSuit,
    value: CardValue
}

#[derive(Debug, Eq, PartialEq)]
struct ContinuingGameState {
    deck: Deck,
    hand: Vec<Card>
}

fn parse_action(str: &String) -> Option<Action> {
    match str.trim() {
        "hit" => Option::Some(Action::Hit),
        "stand" => Option::Some(Action::Stand),
        "double-down" => Option::Some(Action::DoubleDown),
        "split" => Option::Some(Action::SplitCards),
        "surrender" => Option::Some(Action::Surrender),
        _ => Option::None,
    }
}

enum CardStatus {
    CardHasBeenDrawn,
    CardNotYetDrawn
}

#[derive(Debug, Eq, PartialEq)]
struct Deck {
    remaining_cards: Vec<Card>,
    drawn_cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        let mut result = Vec::new();
        for suit in CardSuit::ALL_VALUES.iter() {
            for value in CardValue::ALL_VALUES.iter() {
                result.push(
                    Card {
                        suit: suit.clone(),
                        value: value.clone()
                    }
                )
            }
        }
        Deck {
            remaining_cards: result,
            drawn_cards: Vec::new()
        }
    }

    fn shuffle<R: Rng>(&mut self, rng: &mut R) {
        rng.shuffle(&mut self.remaining_cards);
    }

    fn draw_card(&mut self) -> Option<Card> {
        draw_card(self)
    }
}

fn draw_card(deck: &mut Deck) -> Option<Card> {
    let card_opt = deck.remaining_cards.pop();
    match card_opt {
        Option::Some(card) => {
            let card_ref = &card;
            let value = card_ref.value.clone();
            let suit = card_ref.suit.clone();
            let new_card = Card {
                value: value,
                suit: suit
            };
            deck.drawn_cards.push(card);
            Option::Some(new_card)
        }
        Option::None => Option::None
    }
}

fn is_hand_too_large(hand: &Vec<Card>) -> bool {
    let card_values = &hand.iter().map(|card| card.value.clone()).collect();
    if !hand.is_empty() {
        calculate_current_hand_value(card_values).is_empty()
    } else {
        false
    }
}

fn deal_with_action(action: &Action, state: GameState) -> GameState {
    match state {
        GameState::GameLost => GameState::GameLost,
        GameState::GameWon => GameState::GameWon,
        GameState::Continuing(mut continuingGameState) =>
            match action {
                Action::Surrender => GameState::GameLost,
                Action::Hit => {
                    let card_opt = continuingGameState.deck.draw_card();
                    if let Option::Some(card) = card_opt {
                        continuingGameState.hand.push(card);
                    }
                    if is_hand_too_large(&continuingGameState.hand) {
                        GameState::GameLost
                    } else {
                        GameState::Continuing(continuingGameState)
                    }
                },
                Action::Stand => GameState::GameLost,
                Action::DoubleDown => GameState::GameLost,
                Action::SplitCards => GameState::GameLost,
            }
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn full_deck_of_cards() -> &'static[Card] {
    unimplemented!()
}

fn main() {
    println!("Play blackjack!");

    println!("Please input what you'd like to do (hit/stand/double-down/split/surrender):");

    let mut game_over = false;

    let mut raw_action = String::new();

    let mut game_state = GameState::start(&mut thread_rng());

    let stdin = io::stdin();

    let mut stdin_lines = stdin.lock().lines();

    while game_state != GameState::GameLost {
        if let GameState::Continuing(continuingGameState) = &game_state {
            println!("Your hand is {:?}", &continuingGameState.hand);
            println!("Your hand value is {:?}", calculate_current_hand_value(&continuingGameState.hand.iter().map(|x| x.value.clone()).collect()));

        }

        if let Option::Some(line) = stdin_lines.next() {
            raw_action = line.expect("Failed to read line!")
        }

        println!("raw_action: {:?}", raw_action);

        let action = parse_action(&raw_action);

        println!("You wanted to: {:?}", action);

        match action {
            Option::Some(action) =>
                game_state = deal_with_action(&action, game_state),
            Option::None=>
                ()
        }



    }

}
