import GObject from 'gi://GObject';
import Gtk from 'gi://Gtk';
import Adw from 'gi://Adw';

export const MutinyPreferences = GObject.registerClass({
    GTypeName: 'MutinyPreferences',
    Template: 'resource:///chat/revolt/Mutiny/preferences.ui',
}, class MutinyPreferences extends Adw.PreferencesWindow {
    constructor(application) {
        super({ application });
    }
});

