use assert_cmd::Command;

#[test]
fn should_print_a_single_line_when_matche_single_line() {
    let mut cmd = Command::cargo_bin("cli").unwrap();

    cmd.arg("joyful life")
        .arg("./tests/happiness.txt")
        .assert()
        .stdout(predicates::str::contains("4: Savor the simple joys for a joyful life."));
}
