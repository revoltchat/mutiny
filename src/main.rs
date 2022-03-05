use adw::{prelude::*, Avatar};

use adw::{ActionRow, ApplicationWindow, HeaderBar};
use adw::gtk::{Application, Box, ListBox, Orientation};

fn main() {
    // Configure a new GTK4 application.
    let application = Application::builder()
        .application_id("chat.revolt.Mutiny")
        .build();

    // Configure Adwaita on start.
    application.connect_startup(|_| {
        adw::init();
    });

    // Once the application is ready, build the UI.
    application.connect_activate(|app| {
        // This is our main Window frame.
        let frame = Box::new(Orientation::Vertical, 0);

        // Add the header bar, not included by default.
        frame.append(
            &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("Mutiny", ""))
                .build(),
        );

        // Add the main content.
        frame.append(&{
            let panels = Box::builder()
                .orientation(Orientation::Horizontal)
                .margin_top(16)
                .margin_start(16)
                .build();

            // Some stuff on the left.
            panels.append(&{
                let servers_or_something = Box::new(Orientation::Vertical, 24);

                servers_or_something.append(&Avatar::new(64, Some("deez nuts"), true));
                servers_or_something.append(&Avatar::new(64, Some("hm yes aaa"), true));
                servers_or_something.append(&Avatar::new(64, Some("abc def"), true));
                servers_or_something.append(&Avatar::new(64, Some("aaaa"), true));

                servers_or_something
            });

            // This is the default content from the Adwaita demo code.
            panels.append(&{
                let list = ListBox::builder()
                    .margin_top(32)
                    .margin_end(32)
                    .margin_bottom(32)
                    .margin_start(32)
                    .css_classes(vec![String::from("content")])
                    .build();
                
                list.append(&{
                    let row = ActionRow::builder()
                        .activatable(true)
                        .selectable(false)
                        .title("Click me")
                        .build();
                    
                    row.connect_activated(|_| {
                        eprintln!("Clicked!");
                    });
                    
                    row
                });

                list
            });
            
            panels
        });

        // Construct the Window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            .content(&frame)
            .build();
        
        // Display the Window.
        window.show();
    });

    // Run the GTK application.
    application.run();
}
