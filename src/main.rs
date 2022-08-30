#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod consts;
mod cli;

fn main() {
    gui::start();
}