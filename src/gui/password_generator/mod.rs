// use native_windows_derive as nwd;

use nwg::{Button, NativeUi, Window, WindowFlags, modal_info_message, Clipboard, unbind_event_handler};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::cli::password_generator;

#[derive(Default)]
pub struct TestApp {
    window: Window,
    gen_password_btn: Button,
    get_version_btn: Button,
}

pub struct TestAppUi {
    inner: Rc<TestApp>,
    default_handler: RefCell<Option<nwg::EventHandler>>,
}

impl TestApp {
    fn generate_password(&self) {
        let password = password_generator::generate();

        modal_info_message(&self.window, "Your password", &*format!("{}\n\nThis password has been copied to your clipboard", password));

        Clipboard::set_data_text(&self.window, &password);
    }

    fn get_version(&self) {
        let version = clap::crate_version!();

        modal_info_message(&self.window, "Password generator version", &*format!("This program runs under version {}", version));
    }
}

impl NativeUi<TestAppUi> for TestApp {
    fn build_ui(mut data: TestApp) -> Result<TestAppUi, nwg::NwgError> {
        use nwg::Event as E;

        Window::builder()
            .flags(WindowFlags::WINDOW | WindowFlags::VISIBLE)
            .size((300, 110))
            .position((700, 300))
            .title("Password generator")
            .build(&mut data.window)?;

        Button::builder()
            .size((280, 35))
            .position((10, 10))
            .text("Generate my password now")
            .parent(&data.window)
            .build(&mut data.gen_password_btn)?;

        Button::builder()
            .size((280, 35))
            .position((10, 65))
            .text("Version")
            .parent(&data.window)
            .build(&mut data.get_version_btn)?;

        let ui = TestAppUi {
            inner: Rc::new(data),
            default_handler: Default::default(),
        };

        // Events
        let evt_ui = Rc::downgrade(&ui.inner);
        let handle_events = move |evt, _evt_data, handle| {
            if let Some(ui) = evt_ui.upgrade() {
                match evt {
                    E::OnButtonClick => {
                        if &handle == &ui.gen_password_btn {
                            TestApp::generate_password(&ui);
                        } else if &handle == &ui.get_version_btn {
                            TestApp::get_version(&ui);
                        }
                    }
                    E::OnWindowClose => {
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        };

        *ui.default_handler.borrow_mut() = Some(nwg::full_bind_event_handler(
            &ui.window.handle,
            handle_events,
        ));

        return Ok(ui);
    }
}

impl Drop for TestAppUi {
    /// To make sure that everything is freed without issues, the default handler must be unbound.
    fn drop(&mut self) {
        let handler = self.default_handler.borrow();
        if handler.is_some() {
            unbind_event_handler(handler.as_ref().unwrap());
        }
    }
}

impl Deref for TestAppUi {
    type Target = TestApp;

    fn deref(&self) -> &TestApp {
        &self.inner
    }
}
