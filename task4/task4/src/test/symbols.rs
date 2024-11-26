#![allow(non_upper_case_globals)]
use crate::symbol::Symbol;
use std::sync::LazyLock;

macro_rules! define_symbol {
    ($name:ident, $symbol_name:expr, $is_terminal:expr) => {
        pub static $name: LazyLock<Symbol> = LazyLock::new(|| Symbol {
            name: $symbol_name.to_string(),
            is_terminal: $is_terminal,
        });
    };
}

macro_rules! define_symbols {
    ($($name:ident, $symbol_name:expr, $is_terminal:expr),*) => {
        $(define_symbol!($name, $symbol_name, $is_terminal);)*
    };
}

#[rustfmt::skip]
define_symbols!(
//  symbols         name                is_terminal
    Epsilon,        "",                 true,

    letter_a,       "a",                true,
    letter_b,       "b",                true,
    letter_c,       "c",                true,
    letter_d,       "d",                true,
    letter_e,       "e",                true,
    letter_f,       "f",                true,
    letter_g,       "g",                true,
    letter_h,       "h",                true,
    letter_i,       "i",                true,
    letter_j,       "j",                true,
    letter_k,       "k",                true,
    letter_l,       "l",                true,
    letter_m,       "m",                true,
    letter_n,       "n",                true,
    letter_o,       "o",                true,
    letter_p,       "p",                true,
    letter_q,       "q",                true,
    letter_r,       "r",                true,
    letter_s,       "s",                true,
    letter_t,       "t",                true,
    letter_u,       "u",                true,
    letter_v,       "v",                true,
    letter_w,       "w",                true,
    letter_x,       "x",                true,
    letter_y,       "y",                true,
    letter_z,       "z",                true,
    letter_A,       "A",                false,
    letter_B,       "B",                false,
    letter_C,       "C",                false,
    letter_D,       "D",                false,
    letter_E,       "E",                true,
    letter_F,       "F",                true,
    letter_G,       "G",                true,
    letter_H,       "H",                true,
    letter_I,       "I",                true,
    letter_J,       "J",                true,
    letter_K,       "K",                true,
    letter_L,       "L",                true,
    letter_M,       "M",                true,
    letter_N,       "N",                true,
    letter_O,       "O",                true,
    letter_P,       "P",                true,
    letter_Q,       "Q",                true,
    letter_R,       "R",                true,
    letter_S,       "S",                true,
    letter_T,       "T",                true,
    letter_U,       "U",                true,
    letter_V,       "V",                true,
    letter_W,       "W",                true,
    letter_X,       "X",                true,
    letter_Y,       "Y",                true,
    letter_Z,       "Z",                true,
    digit_0,        "0",                true,
    digit_1,        "1",                true,
    digit_2,        "2",                true,
    digit_3,        "3",                true,
    digit_4,        "4",                true,
    digit_5,        "5",                true,
    digit_6,        "6",                true,
    digit_7,        "7",                true,
    digit_8,        "8",                true,
    digit_9,        "9",                true,
    Ident,          "Ident",            false
);
