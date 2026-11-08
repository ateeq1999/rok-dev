//! End-to-end tests of the `rok-dev` binary and the command `rok` embeds.

use std::process::{Command, Output};

const WORDS: [&str; 4] = ["dev", "run", "doctor", "port"];

fn rok_dev(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rok-dev"))
        .args(args)
        .output()
        .expect("run rok-dev")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn help_lists_the_four_words() {
    let output = rok_dev(&["--help"]);
    assert_eq!(output.status.code(), Some(0));
    let help = text(&output.stdout);
    for word in WORDS {
        assert!(
            help.lines().any(|line| line.trim_start().starts_with(word)),
            "`{word}` missing from:\n{help}"
        );
    }
}

#[test]
fn the_embedded_command_has_exactly_the_four_words() {
    let names: Vec<String> = rok_dev_cli::command()
        .get_subcommands()
        .map(|c| c.get_name().to_owned())
        .collect();
    assert_eq!(names, WORDS);
}

#[test]
fn each_word_is_not_built_yet_and_names_its_ticket() {
    for (args, ticket) in [
        (&["dev"][..], "02-04"),
        (&["dev", "api", "web"][..], "02-04"),
        (&["run", "api"][..], "05"),
        (&["doctor"][..], "08"),
        (&["port", "list"][..], "09"),
        (&["port", "free", "3001"][..], "09"),
    ] {
        let output = rok_dev(args);
        assert_eq!(output.status.code(), Some(1), "{args:?}");
        let err = text(&output.stderr);
        assert!(err.contains("not built yet"), "{args:?}: {err}");
        assert!(err.contains(ticket), "{args:?}: {err}");
    }
}

#[test]
fn usage_errors_exit_2() {
    for args in [
        &[][..],
        &["frobnicate"][..],
        &["run"][..],
        &["port"][..],
        &["port", "free"][..],
        &["port", "free", "not-a-port"][..],
    ] {
        assert_eq!(rok_dev(args).status.code(), Some(2), "{args:?}");
    }
}

#[test]
fn help_is_named_after_the_program() {
    // `rok-dev.exe` on Windows.
    let help = text(&rok_dev(&["port", "--help"]).stdout);
    assert!(help.contains("Usage: rok-dev"), "{help}");
    assert!(help.contains(" port <COMMAND>"), "{help}");
    // Embedded in `rok`, the program name is `rok`.
    let help = rok_dev_cli::command()
        .try_get_matches_from(["rok", "port", "--help"])
        .unwrap_err()
        .to_string();
    assert!(help.contains("Usage: rok port"), "{help}");
}
