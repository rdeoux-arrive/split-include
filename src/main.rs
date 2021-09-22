use clap::{clap_app, crate_authors, crate_description, crate_name, crate_version};
use log::LevelFilter;
use simple_logger::SimpleLogger;
use split_include::Autoconf;

fn main() {
    let app = clap_app!((crate_name!()) =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg verbose: -v --verbose)
        (@arg directory: ...)
    );
    let matches = app.get_matches();

    if let Some(directory) = matches.value_of_os("directory") {
        SimpleLogger::new()
            .with_level(if matches.is_present("verbose") {
                LevelFilter::Trace
            } else {
                LevelFilter::Info
            })
            .init()
            .ok();

        let autoconf = Autoconf::load(directory);
        autoconf.gen_py(directory);
        autoconf.gen_pl(directory);
        autoconf.gen_h(directory);
        autoconf.gen_inc(directory);
    }
}
