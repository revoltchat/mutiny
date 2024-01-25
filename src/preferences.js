import GObject from 'gi://GObject';
import Adw from 'gi://Adw';

export const MutinyPreferences = GObject.registerClass({
	GTypeName: 'MutinyPreferences',
	Template: 'resource:///chat/revolt/Mutiny/ui/preferences.ui',
}, class MutinyPreferences extends Adw.PreferencesWindow {
	constructor(application) {
		super({ application });
	}
});

