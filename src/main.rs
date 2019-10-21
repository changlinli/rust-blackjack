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
enum GameState<'a> {
    GameWon(PlayerState<'a>),
    GameLost(PlayerState<'a>),
    Continuing(PlayerState<'a>)
}

impl<'a> GameState<'a> {
    fn start<'b>(deck: &'b mut Deck) -> GameState<'b> {
        let internal_state = PlayerState {
            deck: deck,
            hand: Vec::new()
        };
        GameState::Continuing(internal_state)
    }

    fn player_state(&self) -> &PlayerState<'a> {
        match self {
            GameState::GameLost(p) => p,
            GameState::GameWon(p) => p,
            GameState::Continuing(p) => p,
        }
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

fn cartesian_product<'a, 'b, A, B>(xs: &'a Vec<A>, ys: &'b Vec<B>) -> Vec<(&'a A, &'b B)> {
    xs
        .iter()
        .flat_map::<Vec<(&A, &B)>, _>(|x| ys.iter().map(|y| (x, y)).collect())
        .collect()
}

fn raw_calculate_current_hand_value(hand: &Vec<CardValue>) -> Vec<u32> {
    hand
        .iter()
        .map(card_value_to_hand_value)
        .fold(
            vec![0],
            |x, y|
                cartesian_product(&x, &y)
                    .iter()
                    .map(|x_and_y| x_and_y.0 + x_and_y.1.value)
                    .collect()
        )
}

#[derive(Debug, Eq, PartialEq)]
struct Card {
    suit: CardSuit,
    value: CardValue
}

#[derive(Debug, Eq, PartialEq)]
struct PlayerState<'a> {
    deck: &'a mut Deck,
    hand: Vec<Card>
}

impl<'a> PlayerState<'a> {
    fn create_hand_values(&self) -> Vec<CardValue> {
        self.hand.iter().map(|card| card.value.clone()).collect()
    }
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

fn deal_with_action<'a>(action: &Action, state: GameState<'a>) -> GameState<'a> {
    match state {
        x @ GameState::GameLost(_) => x,
        x @ GameState::GameWon(_) => x,
        GameState::Continuing(mut player_state) =>
            match action {
                Action::Surrender => GameState::GameLost(player_state),
                Action::Hit => {
                    let card_opt = player_state.deck.draw_card();
                    if let Option::Some(card) = card_opt {
                        player_state.hand.push(card);
                    }
                    if is_hand_too_large(&player_state.hand) {
                        GameState::GameLost(player_state)
                    } else {
                        let card_values = &player_state.create_hand_values();
                        let possible_hand_values = calculate_current_hand_value(card_values);
                        let are_any_hand_values_21 =
                            possible_hand_values.iter().find(|x| x.value == 21).is_some();
                        if are_any_hand_values_21 {
                            GameState::GameWon(player_state)
                        } else {
                            GameState::Continuing(player_state)
                        }
                    }
                },
                Action::Stand => GameState::GameLost(player_state),
                Action::DoubleDown => GameState::GameLost(player_state),
                Action::SplitCards => GameState::GameLost(player_state),
            }
    }
}

fn continue_with_game(game_state: &GameState) -> bool {
    match game_state {
        GameState::GameWon(_) => false,
        GameState::GameLost(_) => false,
        GameState::Continuing(_) => true,
    }
}

fn game_message(game_state: &GameState) -> &'static str {
    match game_state {
        GameState::GameWon(_) => "You won",
        GameState::GameLost(_) => "You lost",
        GameState::Continuing(_) => "The game is still going",
    }
}

fn main() {
    println!("Play blackjack!");

    println!("Please input what you'd like to do (hit/stand/double-down/split/surrender):");

    let mut raw_action = String::new();

    let mut deck = Deck::new();

    deck.shuffle(&mut thread_rng());

    let mut game_state = GameState::start(&mut deck);

    let stdin = io::stdin();

    let mut stdin_lines = stdin.lock().lines();

    while continue_with_game(&game_state) {
        if let GameState::Continuing(continuing_game_state) = &game_state {
            println!("Your hand is {:?}", &continuing_game_state.hand);
            println!("Your hand value is {:?}", calculate_current_hand_value(&continuing_game_state.hand.iter().map(|x| x.value.clone()).collect()));

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

    println!("{}", game_message(&game_state));

    println!("Final hand: {:?}", game_state.player_state().hand);
    println!("Final hand value: {:?}", raw_calculate_current_hand_value(&game_state.player_state().create_hand_values()));

}
