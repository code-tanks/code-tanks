use ctsimlib::c_command::CCommands;

#[test]
fn empty() {
    assert_eq!(
        format!(
            "{:?}",
            CCommands::all_commands_from_file(
                "/workspaces/codetanks/apps/simulator/tests/empty.ct"
            )
        ),
        "[[], [], [], []]"
    )
}

#[test]
fn do_nothing() {
    assert_eq!(
        format!(
            "{:?}",
            CCommands::all_commands_from_file(
                "/workspaces/codetanks/apps/simulator/tests/do_nothing.ct"
            )
        ),
        format!(
            "{:?}",
            vec![
                vec![CCommands::NONE, CCommands::NONE,],
                vec![CCommands::NONE, CCommands::NONE,],
                vec![CCommands::NONE, CCommands::NONE,],
                vec![CCommands::NONE, CCommands::NONE,],
            ]
        )
    );
}
