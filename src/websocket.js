"use strict"
import Soup from "gi://Soup?version=3.0";
import Gio from 'gi://Gio';
import GLib from 'gi://GLib';

const decoder = new TextDecoder();
const session = new Soup.Session();

Gio._promisify(
	Soup.Session.prototype,
	"send_and_read_async",
	"send_and_read_finish",
);

const instance = "revolt.chat";
const token = "TOKEN";
const api_url = `https://api.${instance}`;
const ws_url = `wss://ws.${instance}/?token=${token}`;

const user_info_request = Soup.Message.new("GET", `${api_url}/users/@me`);
user_info_request.request_headers.append(
	"x-session-token",
	`${token}`,
);

export const bytes = await session.send_and_read_async(
	user_info_request,
	GLib.PRIORITY_DEFAULT,
	null,
);

if (user_info_request.status_code !== 200) {
	console.error(`HTTP Status ${user_info_request.status_code}`);
}

const decoded_text = decoder.decode(bytes.toArray());
const json = JSON.parse(decoded_text);

export const username = json.username;
export const discriminator = json.discriminator;

console.log(username);
console.log(discriminator);


let connection;

export async function wsConnect() {

	const message = new Soup.Message({
		method: "GET",
		uri: GLib.Uri.parse(ws_url, GLib.UriFlags.NONE),
	});

	session.websocket_connect_async(
		message,
		null,
		[],
		null,
		null,
		ws_connect_callback,
	);
}

function ws_connect_callback(_session, res) {
	try {
		connection = session.websocket_connect_finish(res);
	} catch (err) {
		logError(err);
		return;
	}

	connection.connect('closed', () => {
		log('closed');
	});

	connection.connect('error', (self, err) => {
		logError(err);
	});

	connection.connect('message', (self, type, data) => {
		if (type !== Soup.WebsocketDataType.TEXT) return;

		const str = decoder.decode(data.toArray());
		const json = JSON.parse(str);

		if (json.type == "Ready") {
			for (let server = 0; server < json.servers.length; server++) {
				log(json.servers[server].name)
			}
		}
		//connection.close(Soup.WebsocketCloseCode.NORMAL, null);
	});

	log('open');
}

wsConnect();

