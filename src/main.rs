use iced::widget::container;
use iced::{Sandbox, Element, Settings, Color, Background, Length, Padding};
use iced::widget::{Row, Container, Text, Column};
use rand::Rng;
use data::Suggestion;
use container_contents::{create_letter_container, create_result_container};
use guess::make_guess;
mod data;
mod container_contents;
mod guess;

fn main() -> iced::Result {
    KnownState::run(Settings::default())
}

#[derive(Clone)]
struct KnownState {
    suggestions: Vec<Vec<Suggestion>>,
    position_regex: [String; 5],
    position_is_not: [Vec<char>; 5],
    position_is: [Option<char>; 5],
    word_contains: Vec<char>,
    word_does_not_contain: Vec<char>,
    word_contains_only_one: Vec<char>,
    word_contains_multiple: Vec<char>,
    answer_found: bool,
    game_lost: bool
}

#[derive(Debug, Clone)]
enum Message {
    LetterStatusChange(usize, usize),
    MakeGuess,
    Restart
}

#[derive(Clone, PartialEq)]
pub enum State {
    Unknown,
    WrongPlace,
    Correct,
    Wrong,
}

struct AppContainer {}

impl container::StyleSheet for AppContainer {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.07058823529411765, 0.07058823529411765, 0.07058823529411765))),
            text_color: Some(Color::from_rgb(1.0, 1.0, 1.0)),
            ..Default::default()
        }
    }
}

impl Sandbox for KnownState {
    type Message = Message;

    fn new() -> Self {
        Self {
            suggestions: vec![data::start_list()[rand::thread_rng().gen_range(0..data::start_list().len())].clone()],
            position_regex: [String::new(), String::new(), String::new(), String::new(), String::new()],
            position_is_not: [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            position_is: [None, None, None, None, None],
            word_contains: Vec::new(),
            word_does_not_contain: Vec::new(),
            word_contains_only_one: Vec::new(),
            word_contains_multiple: Vec::new(),
            answer_found: false,
            game_lost: false
        }
    }

    fn title(&self) -> String {
        String::from("Wordle Helper")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::LetterStatusChange(x, y) => {
                match self.suggestions[y][x].state {
                    State::Unknown => {
                        self.suggestions[y][x].state = State::WrongPlace;
                    },
                    State::WrongPlace => {
                        self.suggestions[y][x].state = State::Correct;
                    },
                    State::Correct => {
                        self.suggestions[y][x].state = State::Wrong;
                    },
                    State::Wrong => {
                        self.suggestions[y][x].state = State::WrongPlace;
                    }
                }
            }
            Message::MakeGuess => {
                *self = make_guess(self.clone());
            }
            Message::Restart => {
                *self = KnownState::new();
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut col = Column::new()
            .align_items(iced::Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill);

        col = col.push(Container::new(Text::new("Wordle Helper").size(72)).padding(Padding::from(10.0)));
        for y in 0..self.suggestions.len() {
            let mut row = Row::new();
            for x in 0..self.suggestions[y].len() {
                row = row.push(create_letter_container(self, x, y));
            }
            col = col.push(row);
        }

        col = col.push(create_result_container(self));
        Container::new(col)
            .style(iced::theme::Container::Custom(Box::new(AppContainer {})))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
