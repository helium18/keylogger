// Rust keylogger for termput <WIP>
//     Copyright (C) 2022 helium18
//
//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.
//
//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.
//
//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
//                                      ------

// Constants, structs, and arrays derived from /linux/include/linux/input.h
// Help taken from https://github.com/gsingh93/keylogger/blob/master/src/input.rs
// and https://nayandas3234.medium.com/how-to-make-a-linux-based-keylogger-a6088e009aab

// TODO : add capslock

use std::fmt::Display;

use bytemuck::{Pod, Zeroable};
use libc::{suseconds_t, time_t};

const MAX_KEYS: u16 = 112;
const KEY_EVENT: u16 = 1;
const KEY_RELEASE: i32 = 0;
const KEY_PRESS: i32 = 1;
const KEY_LEFT_SHIFT: u16 = 42;
const KEY_RIGHT_SHIFT: u16 = 43;

/// Contains details about the Input Event. Serialized from the `device` file
///
/// ## Fields
///
/// 1. tv_sec: It is the number of seconds since 1/1/1970
/// 2. tv_usec: It is the number of microseconds since tv_sec
/// 3. key_type: The type of the Event. It is used here to check if the event is a key event.
/// 4. code: The unique key code.
/// 5. value: The value held by the key
#[derive(Debug, Pod, Zeroable, Clone, Copy)]
#[repr(C)]
pub struct InputEvent {
    tv_sec: time_t,
    tv_usec: suseconds_t,

    pub key_type: u16,
    pub code: u16,
    pub value: i32,
}

/// The Side of the character on the keyboard
///
/// For example. Left and Right for Shift -> Left Shift and Right Shift
#[derive(Clone)]
pub enum Side {
    Left,
    Right,
}

/// The character's case (UpperCase / LowerCase)
#[derive(Clone)]
pub enum Case {
    Upper,
    Lower,
}

/// The Keypad's characters
#[derive(Clone)]
pub enum KeyPad {
    Asterisk,
    Seven,
    Eight,
    Nine,
    Hyphen,
    Four,
    Five,
    Six,
    Plus,
    One,
    Two,
    Three,
    Zero,
    Period,
    Enter,
    BackSlash,
}

/// Keyboard keys
#[derive(Clone)]
pub enum Key {
    Unknown,
    Escape,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    Hyphen,
    Equals,
    Backspace,
    Tab,
    Q(Case),
    W(Case),
    E(Case),
    R(Case),
    T(Case),
    Y(Case),
    U(Case),
    I(Case),
    O(Case),
    P(Case),
    SquareBracket(Side),
    Enter,
    Ctrl(Side),
    A(Case),
    S(Case),
    D(Case),
    F(Case),
    G(Case),
    H(Case),
    J(Case),
    K(Case),
    L(Case),
    SemiColon,
    Apostrophe,
    BackTick,
    Shift(Side),
    FrontSlash,
    Z(Case),
    X(Case),
    C(Case),
    V(Case),
    B(Case),
    N(Case),
    M(Case),
    Comma,
    Period,
    BackSlash,
    KeyPad(KeyPad),
    Alt(Side),
    Space,
    CapsLock,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    NumLock,
    ScrollLock,
    F11,
    F12,
    SysRq,
    Home,
    Up,
    PageUp,
    Left,
    Right,
    End,
    Down,
    PageDown,
    Insert,
    Delete,
    Exclamation,
    AtSign,
    Hash,
    Dollar,
    Percentage,
    Caret,
    Ampersand,
    Asterisk,
    RoundBraces(Side),
    UnderScore,
    Plus,
    CurlyBraces(Side),
    Colon,
    Tilde,
    Pipe,
    LessThan,
    GreaterThan,
    QuestionMark,
    Quotes,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let character: &str = match &self {
            Key::One => "1",
            Key::Two => "2",
            Key::Three => "3",
            Key::Four => "4",
            Key::Five => "5",
            Key::Six => "6",
            Key::Seven => "7",
            Key::Eight => "8",
            Key::Nine => "9",
            Key::Zero => "0",
            Key::Hyphen => "-",
            Key::Equals => "=",
            Key::Q(case) => match case {
                Case::Upper => "Q",
                Case::Lower => "q",
            },
            Key::W(case) => match case {
                Case::Upper => "W",
                Case::Lower => "w",
            },
            Key::E(case) => match case {
                Case::Upper => "E",
                Case::Lower => "e",
            },
            Key::R(case) => match case {
                Case::Upper => "R",
                Case::Lower => "r",
            },
            Key::T(case) => match case {
                Case::Upper => "T",
                Case::Lower => "t",
            },
            Key::Y(case) => match case {
                Case::Upper => "Y",
                Case::Lower => "y",
            },
            Key::U(case) => match case {
                Case::Upper => "U",
                Case::Lower => "u",
            },
            Key::I(case) => match case {
                Case::Upper => "I",
                Case::Lower => "i",
            },
            Key::O(case) => match case {
                Case::Upper => "O",
                Case::Lower => "o",
            },
            Key::P(case) => match case {
                Case::Upper => "P",
                Case::Lower => "p",
            },
            Key::SquareBracket(side) => match side {
                Side::Left => "[",
                Side::Right => "]",
            },
            Key::Enter => "\n",
            Key::A(case) => match case {
                Case::Upper => "A",
                Case::Lower => "a",
            },
            Key::S(case) => match case {
                Case::Upper => "S",
                Case::Lower => "s",
            },
            Key::D(case) => match case {
                Case::Upper => "D",
                Case::Lower => "d",
            },
            Key::F(case) => match case {
                Case::Upper => "F",
                Case::Lower => "f",
            },
            Key::G(case) => match case {
                Case::Upper => "G",
                Case::Lower => "g",
            },
            Key::H(case) => match case {
                Case::Upper => "H",
                Case::Lower => "h",
            },
            Key::J(case) => match case {
                Case::Upper => "J",
                Case::Lower => "j",
            },
            Key::K(case) => match case {
                Case::Upper => "K",
                Case::Lower => "k",
            },
            Key::L(case) => match case {
                Case::Upper => "L",
                Case::Lower => "l",
            },
            Key::SemiColon => ";",
            Key::Apostrophe => "'",
            Key::BackTick => "`",
            Key::FrontSlash => "\\",
            Key::Z(case) => match case {
                Case::Upper => "Z",
                Case::Lower => "z",
            },
            Key::X(case) => match case {
                Case::Upper => "X",
                Case::Lower => "x",
            },
            Key::C(case) => match case {
                Case::Upper => "C",
                Case::Lower => "c",
            },
            Key::V(case) => match case {
                Case::Upper => "V",
                Case::Lower => "v",
            },
            Key::B(case) => match case {
                Case::Upper => "B",
                Case::Lower => "b",
            },
            Key::N(case) => match case {
                Case::Upper => "N",
                Case::Lower => "n",
            },
            Key::M(case) => match case {
                Case::Upper => "M",
                Case::Lower => "m",
            },
            Key::Comma => ",",
            Key::Period => ".",
            Key::BackSlash => "/",
            Key::KeyPad(key) => match key {
                KeyPad::Asterisk => "*",
                KeyPad::Seven => "7",
                KeyPad::Eight => "8",
                KeyPad::Nine => "9",
                KeyPad::Hyphen => "-",
                KeyPad::Four => "4",
                KeyPad::Five => "5",
                KeyPad::Six => "6",
                KeyPad::Plus => "+",
                KeyPad::One => "1",
                KeyPad::Two => "2",
                KeyPad::Three => "3",
                KeyPad::Zero => "0",
                KeyPad::Period => ".",
                KeyPad::Enter => "\n",
                KeyPad::BackSlash => "/",
            },
            Key::Space => " ",
            Key::Exclamation => "!",
            Key::AtSign => "@",
            Key::Hash => "#",
            Key::Dollar => "#",
            Key::Percentage => "%",
            Key::Caret => "^",
            Key::Ampersand => "&",
            Key::Asterisk => "*",
            Key::RoundBraces(side) => match side {
                Side::Left => "(",
                Side::Right => ")",
            },
            Key::UnderScore => "_",
            Key::Plus => "+",
            Key::CurlyBraces(side) => match side {
                Side::Left => "{",
                Side::Right => "}",
            },
            Key::Colon => ":",
            Key::Tilde => "~",
            Key::Pipe => "|",
            Key::LessThan => "<",
            Key::GreaterThan => ">",
            Key::QuestionMark => "?",
            Key::Quotes => "\"",
            _ => "",
        };
        write!(f, "{}", character)
    }
}

const KEY_NAMES: [Key; MAX_KEYS as usize] = [
    Key::Unknown,
    Key::Escape,
    Key::One,
    Key::Two,
    Key::Three,
    Key::Four,
    Key::Five,
    Key::Six,
    Key::Seven,
    Key::Eight,
    Key::Nine,
    Key::Zero,
    Key::Hyphen,
    Key::Equals,
    Key::Backspace,
    Key::Enter,
    Key::Q(Case::Lower),
    Key::W(Case::Lower),
    Key::E(Case::Lower),
    Key::R(Case::Lower),
    Key::T(Case::Lower),
    Key::Y(Case::Lower),
    Key::U(Case::Lower),
    Key::I(Case::Lower),
    Key::O(Case::Lower),
    Key::P(Case::Lower),
    Key::SquareBracket(Side::Left),
    Key::SquareBracket(Side::Right),
    Key::Enter,
    Key::Ctrl(Side::Left),
    Key::A(Case::Lower),
    Key::S(Case::Lower),
    Key::D(Case::Lower),
    Key::F(Case::Lower),
    Key::G(Case::Lower),
    Key::H(Case::Lower),
    Key::J(Case::Lower),
    Key::K(Case::Lower),
    Key::L(Case::Lower),
    Key::SemiColon,
    Key::Apostrophe,
    Key::BackTick,
    Key::Shift(Side::Left),
    Key::FrontSlash,
    Key::Z(Case::Lower),
    Key::X(Case::Lower),
    Key::C(Case::Lower),
    Key::V(Case::Lower),
    Key::B(Case::Lower),
    Key::N(Case::Lower),
    Key::M(Case::Lower),
    Key::Comma,
    Key::Period,
    Key::BackSlash,
    Key::Shift(Side::Right),
    Key::KeyPad(KeyPad::Asterisk),
    Key::Alt(Side::Left),
    Key::Space,
    Key::CapsLock,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::NumLock,
    Key::ScrollLock,
    Key::KeyPad(KeyPad::Seven),
    Key::KeyPad(KeyPad::Eight),
    Key::KeyPad(KeyPad::Nine),
    Key::KeyPad(KeyPad::Hyphen),
    Key::KeyPad(KeyPad::Four),
    Key::KeyPad(KeyPad::Five),
    Key::KeyPad(KeyPad::Six),
    Key::KeyPad(KeyPad::Plus),
    Key::KeyPad(KeyPad::One),
    Key::KeyPad(KeyPad::Two),
    Key::KeyPad(KeyPad::Three),
    Key::KeyPad(KeyPad::Zero),
    Key::KeyPad(KeyPad::Period),
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::F11,
    Key::F12,
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::KeyPad(KeyPad::Enter),
    Key::Ctrl(Side::Right),
    Key::KeyPad(KeyPad::BackSlash),
    Key::SysRq,
    Key::Alt(Side::Right),
    Key::Unknown,
    Key::Home,
    Key::Up,
    Key::PageUp,
    Key::Left,
    Key::Right,
    Key::End,
    Key::Down,
    Key::PageDown,
    Key::Insert,
    Key::Delete,
];

const SHIFT_KEY_NAMES: [Key; MAX_KEYS as usize] = [
    Key::Unknown,
    Key::Escape,
    Key::Exclamation,
    Key::AtSign,
    Key::Hash,
    Key::Dollar,
    Key::Percentage,
    Key::Caret,
    Key::Ampersand,
    Key::Asterisk,
    Key::RoundBraces(Side::Left),
    Key::RoundBraces(Side::Right),
    Key::UnderScore,
    Key::Plus,
    Key::Backspace,
    Key::Tab,
    Key::Q(Case::Upper),
    Key::W(Case::Upper),
    Key::E(Case::Upper),
    Key::R(Case::Upper),
    Key::T(Case::Upper),
    Key::Y(Case::Upper),
    Key::U(Case::Upper),
    Key::I(Case::Upper),
    Key::O(Case::Upper),
    Key::P(Case::Upper),
    Key::CurlyBraces(Side::Left),
    Key::CurlyBraces(Side::Right),
    Key::Enter,
    Key::Ctrl(Side::Left),
    Key::A(Case::Upper),
    Key::S(Case::Upper),
    Key::D(Case::Upper),
    Key::F(Case::Upper),
    Key::G(Case::Upper),
    Key::H(Case::Upper),
    Key::J(Case::Upper),
    Key::K(Case::Upper),
    Key::L(Case::Upper),
    Key::Colon,
    Key::Quotes,
    Key::Tilde,
    Key::Shift(Side::Left),
    Key::Pipe,
    Key::Z(Case::Upper),
    Key::X(Case::Upper),
    Key::C(Case::Upper),
    Key::V(Case::Upper),
    Key::B(Case::Upper),
    Key::N(Case::Upper),
    Key::Z(Case::Upper),
    Key::LessThan,
    Key::GreaterThan,
    Key::QuestionMark,
    Key::Shift(Side::Right),
    Key::KeyPad(KeyPad::Asterisk),
    Key::Alt(Side::Left),
    Key::Space,
    Key::CapsLock,
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::F9,
    Key::F10,
    Key::NumLock,
    Key::ScrollLock,
    Key::KeyPad(KeyPad::Seven),
    Key::KeyPad(KeyPad::Eight),
    Key::KeyPad(KeyPad::Nine),
    Key::KeyPad(KeyPad::Hyphen),
    Key::KeyPad(KeyPad::Four),
    Key::KeyPad(KeyPad::Five),
    Key::KeyPad(KeyPad::Six),
    Key::KeyPad(KeyPad::Plus),
    Key::KeyPad(KeyPad::One),
    Key::KeyPad(KeyPad::Two),
    Key::KeyPad(KeyPad::Three),
    Key::KeyPad(KeyPad::Zero),
    Key::KeyPad(KeyPad::Period),
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::F11,
    Key::F12,
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::Unknown,
    Key::KeyPad(KeyPad::Enter),
    Key::Ctrl(Side::Right),
    Key::KeyPad(KeyPad::BackSlash),
    Key::SysRq,
    Key::Alt(Side::Right),
    Key::Unknown,
    Key::Home,
    Key::Up,
    Key::PageUp,
    Key::Left,
    Key::Right,
    Key::End,
    Key::Down,
    Key::PageDown,
    Key::Insert,
    Key::Delete,
];

/// Gets the key pressed
///
/// ## Returns
/// any one variant of the `Key` enum
pub fn get_key(code: u16, shift_pressed: u8) -> Key {
    let key_type = match shift_pressed {
        0 => KEY_NAMES,
        _ => SHIFT_KEY_NAMES,
    };

    if code < MAX_KEYS {
        key_type[code as usize].clone()
    } else {
        Key::Unknown
    }
}

/// Checks if the `Shift` key is pressed.
pub fn is_shift(code: u16) -> bool {
    code == KEY_LEFT_SHIFT || code == KEY_RIGHT_SHIFT
}

/// Checks if there's a Key event on which one can act upon
pub fn is_key_event(key_type: u16) -> bool {
    key_type == KEY_EVENT
}

/// Notifies when the key is pressed
pub fn is_key_press(value: i32) -> bool {
    value == KEY_PRESS
}

/// Notifies when the key is released
pub fn is_key_release(value: i32) -> bool {
    value == KEY_RELEASE
}
