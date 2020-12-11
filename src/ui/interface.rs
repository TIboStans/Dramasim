extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;




use std::env;

pub fn gui() {
    let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"),
                                      gio::ApplicationFlags::FLAGS_NONE).expect("Application::new failed");

    uiapp.connect_activate(|app| {
        let win = gtk::ApplicationWindow::new(app);

        let test_box = gtk::Box::new(gtk::Orientation::Vertical, 100);
        //Bestand => Nieuw || open || opslaan || opslaan als || aflsuiten
        //Vertalen => Voorvertalen || Vertalen || Uitvoeringsvenster
        let menu_bar = gtk::MenuBarBuilder::new()
            .child(&gtk::MenuItemBuilder::new()
                .label(&"Files")
                .submenu(&gtk::MenuBuilder::new()
                    .child(&gtk::MenuItemBuilder::new()
                        .label(&"New")
                        .build())
                    .child(&gtk::MenuItemBuilder::new()
                        .label(&"Open")
                        .build())
                    .child(&gtk::MenuItemBuilder::new()
                        .label(&"Save")
                        .build())
                    .child(&gtk::MenuItemBuilder::new()
                        .label(&"Save As")
                        .build())
                    .child(&gtk::MenuItemBuilder::new()
                        .label(&"Quit")
                        .build())
                    .build())
                .build())
            .build();

        let menu_bar2 = gtk::MenuBuilder::new()
            .child(&gtk::MenuItemBuilder::new()
                .label(&"Compile")
                .submenu(&gtk::MenuBuilder::new()
                    .child(&gtk::MenuItemBuilder::new()
                        .label("Pre-Compiler")
                        .build())
                    .child(&gtk::MenuItemBuilder::new()
                        .label("Compiler")
                        .build())
                    .build())
                .build())
            .build();

        test_box.pack_start(&menu_bar2, false, false,0);
        test_box.pack_start(&menu_bar, false, false,0);
        win.add(&test_box);
        //menu_bar.border_width(100);
        //menu_bar.build();
        //menu_bar.visible(true);

        //menu_bar.visible(true);

        win.set_default_size(320, 200);
        win.set_title("New and Improved ~ Bartje-free ~ Dramasim, written in an actual coding language");

        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}