use std::process::{Command, Output, Stdio};

fn call_toy_engine(args: &[&str]) -> Output {
    Command::new("./target/release/toy-engine")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap()
}

#[test]
fn missing_argument() {
    let output = call_toy_engine(&[]);

    assert!(!output.status.success());
    assert_eq!(
        "Error: missing argument\n",
        String::from_utf8_lossy(output.stderr.as_slice())
    );
}

#[test]
fn ignores_invalid_orders() {
    let output = call_toy_engine(&["tests/data/some_invalid.csv"]);

    assert!(output.status.success());
    assert_eq!(
        "client,available,held,total,locked\n1,2.0005,0,2.0005,false\n",
        String::from_utf8_lossy(output.stdout.as_slice())
    );
}
