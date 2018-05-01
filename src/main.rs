extern crate gio;

use gio::{ Settings, SettingsExt };

const SCHEMA: &str = "org.gnome.shell";
const KEY: &str = "enabled-extensions";
const EXTENSION: &str = "Hide_Top_Bar@leopoldluley.de";

fn main() {
    let settings = Settings::new(SCHEMA);
    let mut strv: Vec<String> = settings.get_strv(KEY);

    if strv.contains(&EXTENSION.to_owned()) {
        strv = strv.into_iter().filter(|it| it != EXTENSION).collect();
    } else {
        strv.push(EXTENSION.to_owned());
    }

    let strv: Vec<&str> = strv.iter().map(|s| &s[..]).collect();
    settings.set_strv(KEY, strv.as_slice());

    Settings::sync();
}
