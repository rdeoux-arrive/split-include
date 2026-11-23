use log::info;
use simple_logger::SimpleLogger;
use std::fs::{create_dir, read_to_string, write};
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::{TempDir, tempdir};

const NAME: &str = env!("CARGO_PKG_NAME");

/// Crée `<tmp>/work/autoconf.h` ; `work` est un sous-dossier pour que le
/// dossier de modèles par défaut (`<tmp>/templates`) reste dans `tmp`.
fn setup(autoconf_h: &[u8]) -> io::Result<(TempDir, PathBuf)> {
    SimpleLogger::new().init().ok();
    let tmp = tempdir()?;
    info!("created temporary directory: {:?}", tmp.path());
    let work = tmp.path().join("work");
    create_dir(&work)?;
    write(work.join("autoconf.h"), autoconf_h)?;
    Ok((tmp, work))
}

/// Lance l’outil sur `work`, sans `CONFIG_TEMPLATES` hérité de
/// l’environnement sauf si `templates` est donné.
fn run(work: &Path, templates: Option<&Path>) -> io::Result<Output> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_split-include"));
    cmd.arg(work).arg("-v").env_remove("CONFIG_TEMPLATES");
    if let Some(templates) = templates {
        cmd.env("CONFIG_TEMPLATES", templates);
    }
    info!("execute {cmd:?}");
    cmd.output()
}

/// Les logs vont sur stdout ou stderr selon leur niveau.
fn logs(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn split() -> io::Result<()> {
    let (tmp, work) = setup(include_bytes!("autoconf.h"))?;
    assert!(run(&work, None)?.status.success());

    let read = |file: &str| read_to_string(work.join(file));
    assert_eq!(
        read("autoconf.py")?,
        format!(include_str!("autoconf.py"), NAME)
    );
    assert_eq!(
        read("autoconf.pl")?,
        format!(include_str!("autoconf.pl"), NAME)
    );
    assert_eq!(
        read("config.featurea.h")?,
        format!(include_str!("config.featurea.h"), NAME)
    );
    assert_eq!(
        read("config.featureb.h")?,
        format!(include_str!("config.featureb.h"), NAME)
    );
    assert_eq!(
        read("config.featurea.inc")?,
        format!(include_str!("config.featurea.inc"), NAME)
    );
    assert_eq!(
        read("config.featureb.inc")?,
        format!(include_str!("config.featureb.inc"), NAME)
    );

    info!("cleanup {:?}", tmp.path());
    tmp.close()
}

/// Place le modèle dans `<tmp>/<dir>` et vérifie le fichier généré ;
/// `use_env` passe ce dossier via `CONFIG_TEMPLATES`.
fn check_template(dir: &str, use_env: bool) -> io::Result<()> {
    let (tmp, work) = setup(include_bytes!("template.autoconf.h"))?;
    let templates = tmp.path().join(dir);
    create_dir(&templates)?;
    write(
        templates.join("config.featurea.h"),
        include_bytes!("template.in.h"),
    )?;

    assert!(run(&work, use_env.then_some(&*templates))?.status.success());
    assert_eq!(
        read_to_string(work.join("config.featurea.h"))?,
        include_str!("template.out.h")
    );
    Ok(())
}

#[test]
fn template_next_to_work_directory() -> io::Result<()> {
    check_template("templates", false)
}

#[test]
fn template_from_config_templates() -> io::Result<()> {
    check_template("elsewhere", true)
}

#[test]
fn missing_autoconf_h_fails() -> io::Result<()> {
    let tmp = tempdir()?;

    let output = run(tmp.path(), None)?;
    assert!(!output.status.success());
    assert!(logs(&output).contains("failed to read"));
    Ok(())
}

#[test]
fn same_name_in_two_groups_warns() -> io::Result<()> {
    let (_tmp, work) = setup(include_bytes!("collision.autoconf.h"))?;

    let output = run(&work, None)?;
    assert!(output.status.success());
    assert!(logs(&output).contains("CONFIG_A_X and CONFIG_B_X"));
    Ok(())
}

#[test]
fn name_starting_with_digit_is_ignored() -> io::Result<()> {
    let (_tmp, work) = setup(include_bytes!("digit.autoconf.h"))?;

    let output = run(&work, None)?;
    assert!(output.status.success());
    assert!(logs(&output).contains("CONFIG_ARM_64BIT"));
    let py = read_to_string(work.join("autoconf.py"))?;
    assert!(py.contains("V8 = 1") && !py.contains("64BIT"));
    Ok(())
}

#[test]
fn name_with_invalid_character_is_ignored() -> io::Result<()> {
    let (_tmp, work) = setup(include_bytes!("invalid.autoconf.h"))?;

    let output = run(&work, None)?;
    assert!(output.status.success());
    assert!(logs(&output).contains("CONFIG_A_B(x)"));
    let py = read_to_string(work.join("autoconf.py"))?;
    assert!(py.contains("OK = 1") && !py.contains("B(x)"));
    Ok(())
}

#[test]
fn define_with_empty_value_is_ignored() -> io::Result<()> {
    let (_tmp, work) = setup(include_bytes!("empty.autoconf.h"))?;

    let output = run(&work, None)?;
    assert!(output.status.success());
    assert!(logs(&output).contains("CONFIG_A_EMPTY"));
    let py = read_to_string(work.join("autoconf.py"))?;
    assert!(py.contains("OK = 1") && !py.contains("EMPTY"));
    Ok(())
}

#[test]
fn template_without_marker_fails() -> io::Result<()> {
    let (tmp, work) = setup(include_bytes!("template.autoconf.h"))?;
    let templates = tmp.path().join("templates");
    create_dir(&templates)?;
    write(
        templates.join("config.featurea.h"),
        include_bytes!("template.no_marker.h"),
    )?;

    let output = run(&work, None)?;
    assert!(!output.status.success());
    assert!(logs(&output).contains("// end"));
    Ok(())
}

#[test]
fn perl_does_not_interpolate_strings() -> io::Result<()> {
    let (_tmp, work) = setup(include_bytes!("perl.autoconf.h"))?;

    assert!(run(&work, None)?.status.success());
    let pl = read_to_string(work.join("autoconf.pl"))?;
    assert!(pl.contains(r#"$HOST = "a\@b\$c";"#));
    Ok(())
}
