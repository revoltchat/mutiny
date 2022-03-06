use gettextrs::gettext;
use log::{debug, info};

use adw::subclass::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::window::MutinyAppWindow;

mod imp {
    use super::*;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct MutinyApp {
        pub window: OnceCell<WeakRef<MutinyAppWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MutinyApp {
        const NAME: &'static str = "MutinyApp";
        type Type = super::MutinyApp;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for MutinyApp {}

    impl ApplicationImpl for MutinyApp {
        fn activate(&self, app: &Self::Type) {
            debug!("GtkApplication<MutinyApp>::activate");
            self.parent_activate(app);

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = MutinyAppWindow::new(app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<MutinyApp>::startup");
            self.parent_startup(app);

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for MutinyApp {}
    impl AdwApplicationImpl for MutinyApp {}
}

glib::wrapper! {
    pub struct MutinyApp(ObjectSubclass<imp::MutinyApp>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl MutinyApp {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
            ("resource-base-path", &Some("/chat/revolt/Mutiny/")),
        ])
        .expect("Application initialization failed...")
    }

    fn main_window(&self) -> MutinyAppWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.quit", &["<Control>q"]);
    }

    fn show_about_dialog(&self) {
        let dialog = gtk::AboutDialog::builder()
            .logo_icon_name(APP_ID)
            // Insert your license of choice here
            // .license_type(gtk::License::MitX11)
            // Insert your website here
            // .website("https://gitlab.gnome.org/bilelmoussaoui/mutiny/")
            .version(VERSION)
            .transient_for(&self.main_window())
            .translator_credits(&gettext("translator-credits"))
            .modal(true)
            .authors(vec!["Revolt".into()])
            .artists(vec!["Revolt".into()])
            .build();

        dialog.present();
    }

    pub fn run(&self) {
        info!("Mutiny ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}

impl Default for MutinyApp {
    fn default() -> Self {
        Self::new()
    }
}
