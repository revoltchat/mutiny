use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;
    
    use gtk::CompositeTemplate;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource="/chat/revolt/Mutiny/ui/server_entry.ui")]
    pub struct ServerEntry {

    }

    impl Default for ServerEntry {
        fn default() -> Self {
            Self {  }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ServerEntry {
        const NAME: &'static str = "ServerEntry";
        type Type = super::ServerEntry;
        type ParentType = gtk::ListBoxRow;

        fn class_init(klass: &mut Self::Class) {

            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ServerEntry {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for ServerEntry {}
    impl ListBoxRowImpl for ServerEntry {}
}

glib::wrapper! {
    pub struct ServerEntry(ObjectSubclass<imp::ServerEntry>)
        @extends gtk::Widget, gtk::ListBoxRow,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ServerEntry {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create ServerEntry")
    }
}