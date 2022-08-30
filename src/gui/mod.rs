mod splash_screen;
mod password_generator;

use nwg::NativeUi;
use password_generator::TestApp;
use splash_screen::SplashScreen;

use std::io;
use std::io::prelude::*;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn start() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let splash = SplashScreen::build_ui(Default::default()).expect("Fauled to build SplashScreen");
    std::thread::sleep(std::time::Duration::from_secs(5));
    let _app = TestApp::build_ui(Default::default()).expect("Failed to build TreeView");
    splash.exit();

    nwg::dispatch_thread_events();
}