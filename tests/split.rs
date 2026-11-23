use log::info;
use simple_logger::SimpleLogger;
use std::fs::{read_to_string, write};
use std::io;
use std::process::Command;
use tempfile::tempdir;

#[test]
#[allow(clippy::similar_names)] // shortcut: noms parallèles voulus (autoconf_py/pl, featurea/b), pas de boucle
fn split() -> io::Result<()> {
    SimpleLogger::new().init().ok();

    let tmp = tempdir()?;
    info!("created temporary directory: {:?}", tmp.path());

    let mut autoconf_h = tmp.path().to_path_buf();
    autoconf_h.push("autoconf.h");
    info!("create {}", autoconf_h.display());
    write(autoconf_h, include_bytes!("autoconf.h"))?;

    let bin = env!("CARGO_BIN_EXE_split-include");
    info!("execute {} {:?}", bin, tmp.path());
    let status = Command::new(bin).arg(tmp.path()).arg("-v").status()?;
    assert!(status.success());

    let mut autoconf_py = tmp.path().to_path_buf();
    autoconf_py.push("autoconf.py");
    assert_eq!(
        read_to_string(autoconf_py)?,
        format!(include_str!("autoconf.py"), bin)
    );

    let mut autoconf_pl = tmp.path().to_path_buf();
    autoconf_pl.push("autoconf.pl");
    assert_eq!(
        read_to_string(autoconf_pl)?,
        format!(include_str!("autoconf.pl"), bin)
    );

    let mut featurea_h = tmp.path().to_path_buf();
    featurea_h.push("config.featurea.h");
    assert_eq!(
        read_to_string(featurea_h)?,
        format!(include_str!("config.featurea.h"), bin)
    );

    let mut featureb_h = tmp.path().to_path_buf();
    featureb_h.push("config.featureb.h");
    assert_eq!(
        read_to_string(featureb_h)?,
        format!(include_str!("config.featureb.h"), bin)
    );

    let mut featurea_inc = tmp.path().to_path_buf();
    featurea_inc.push("config.featurea.inc");
    assert_eq!(
        read_to_string(featurea_inc)?,
        format!(include_str!("config.featurea.inc"), bin)
    );

    let mut featureb_inc = tmp.path().to_path_buf();
    featureb_inc.push("config.featureb.inc");
    assert_eq!(
        read_to_string(featureb_inc)?,
        format!(include_str!("config.featureb.inc"), bin)
    );

    info!("cleanup {:?}", tmp.path());
    tmp.close()?;

    Ok(())
}
