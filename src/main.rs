use clap::{App, Arg};
use log::LevelFilter;
use split_include::Autoconf;

fn main() {
    let app = App::new("split-include").arg(Arg::with_name("directory").required(true));

    if let Some(directory) = app.get_matches().value_of_os("directory") {
        simple_logger::SimpleLogger::new()
            .with_level(LevelFilter::Info)
            .init()
            .ok();

        let autoconf = Autoconf::load(directory);
        autoconf.gen_py(directory);
        autoconf.gen_pl(directory);
        autoconf.gen_h(directory);
        autoconf.gen_inc(directory);
    }
}
