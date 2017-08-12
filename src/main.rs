#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    App::new(crate_name!())
        .version(crate_version!())
        .get_matches();
}
