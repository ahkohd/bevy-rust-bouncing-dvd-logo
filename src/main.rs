mod dvd_logo;

use bevy::prelude::*;
use dvd_logo::DVDLogoPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DVDLogoPlugin)
        .run();
}
