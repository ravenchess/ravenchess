use super::types::{Command, CommandType};
use super::cmd_uci::CmdUci;

#[test]
fn test_create_good_uci_command() {
    let r = CmdUci::new("uci");
    assert!(r.is_ok());

    let cmd = r.ok().unwrap();
    assert!(cmd.get_type() == CommandType::UCI)
}

#[test]
fn test_create_bad_uci_command() {
    let r = CmdUci::new("nonexistence");
    assert!(r.is_err());
}

#[test]
fn test_create_good_uci_command_but_with_additional_parameters() {
    let r = CmdUci::new("uci param1 param2");
    assert!(r.is_err());
}
