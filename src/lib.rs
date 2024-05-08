#![warn(clippy::all, rust_2018_idioms, clippy::double_neg)]
#![deny(clippy::perf)]
mod app;
mod game;
mod game_widget;
pub use app::SuperTicTacToe;
