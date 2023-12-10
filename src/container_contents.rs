use iced::widget::button;
use iced::{Color, Background, Length, Padding, BorderRadius};
use iced::widget::{Button, Container, Text, Column};
use crate::{KnownState, Message, State};

struct UnknownButton {}

impl button::StyleSheet for UnknownButton {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.4117647058823529, 0.4117647058823529, 0.4117647058823529))),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            ..Default::default()
        }
    }
    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

struct CorrectButton {}

impl button::StyleSheet for CorrectButton {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.5019607843137255, 0.0))),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            ..Default::default()
        }
    }
    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

struct WrongPlaceButton {}

impl button::StyleSheet for WrongPlaceButton {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.8549019607843137, 0.6470588235294118, 0.12549019607843137))),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            ..Default::default()
        }
    }
    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

struct WrongButton {}

impl button::StyleSheet for WrongButton {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.0, 0.0))),
            text_color: Color::from_rgb(1.0, 1.0, 1.0),
            ..Default::default()
        }
    }
    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

struct SubmitButton {}

impl button::StyleSheet for SubmitButton {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            border_radius: BorderRadius::from(4.0),
            background: Some(Background::Color(Color::from_rgb(0.3058823529411765, 0.6235294117647059, 0.23921568627450981))),
            text_color: Color::from_rgb(0.0, 0.0, 0.0),
            ..Default::default()
        }
    }

    fn disabled(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(0.19607843137254902, 0.34509803921568627, 0.1568627450980392))),
            border_radius: BorderRadius::from(4.0),
            text_color: Color::from_rgb(0.0, 0.0, 0.0),
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

pub fn create_letter_container(known_state: &KnownState, x: usize, y: usize) -> Container<'static, Message> {
    Container::new(
        create_letter_button(known_state, x, y)
    ).padding(Padding::from(3.0))
}
fn create_letter_button(known_state: &KnownState, x: usize, y: usize) -> Button<'static, Message> {
    let text: Text = Text::new(known_state.suggestions[y][x].letter.to_string().to_ascii_uppercase())
        .size(60.0)
        .width(Length::Fixed(75.0))
        .height(Length::Fixed(75.0))
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center);
    if y == known_state.suggestions.len() - 1 {
        Button::new(text)
            .on_press(Message::LetterStatusChange(x, y))
            .style(get_button_theme(known_state, x, y))
    } else {
        Button::new(text)
            .style(get_button_theme(known_state, x, y))
    }
}

fn get_button_theme(known_state : &KnownState, x: usize, y: usize) -> iced::theme::Button {
    match known_state.suggestions[y][x].state {
        State::Unknown => iced::theme::Button::Custom(Box::new(UnknownButton {})),
        State::WrongPlace => iced::theme::Button::Custom(Box::new(WrongPlaceButton {})),
        State::Correct => iced::theme::Button::Custom(Box::new(CorrectButton {})),
        State::Wrong => iced::theme::Button::Custom(Box::new(WrongButton {})),
    }
}

pub fn create_result_container(known_state: &KnownState) -> Container<'static, Message> {
    Container::new(
        if known_state.answer_found {
            Column::new()
                .push(Text::new("Congratulations!").size(48))
                .push(create_restart_button())
                .align_items(iced::Alignment::Center)
        } else if known_state.game_lost {
            Column::new()
                .push(Text::new("I Give Up, Sorry!").size(48))
                .push(create_restart_button())
                .align_items(iced::Alignment::Center)
        } else {
                let y = known_state.suggestions.len() - 1;
                let all_selected = known_state.suggestions[y].iter().fold(true, |mut acc, suggestion| {
                    match suggestion.state {
                        State::Unknown => {
                            acc = false
                        },
                        _ => {}
                    }
                    acc
                });

                let mut button = Button::new(Text::new("Guess").size(28))
                    .style(iced::theme::Button::Custom(Box::new(SubmitButton {})));
                if all_selected {
                    button = button.on_press(Message::MakeGuess);
                }
                Column::new().push(button)
            }
    ).padding(Padding::from(10.0))
}

fn create_restart_button() -> Container<'static, Message> {
    Container::new(
    Button::new(Text::new("Try Again").size(28))
        .on_press(Message::Restart)
        .style(iced::theme::Button::Custom(Box::new(SubmitButton {})))
    ).padding(Padding::from(10.0))
}
