// here
extern crate gtk;
use gtk::prelude::*;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};

fn gui() {
    if gtk::init().is_err() {
        println!("Error");
        return;
    }
    MessageDialog::new(None::<&Window>,
                       DialogFlags::empty(),
                       MessageType::Info,
                       ButtonsType::Ok,
                       "Hello World").run();

}
