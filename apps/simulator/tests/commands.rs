use ctsim::c_command::Commands;

#[test]
fn empty() {
    assert_eq!(
        format!(
            "{:?}",
            Commands::all_commands_from_file("/workspaces/codetanks/apps/simulator/tests/empty.ct")
        ),
        "[[], [], [], []]"
    )
}

#[test]
fn do_nothing() {
    assert_eq!(
        format!(
            "{:?}",
            Commands::all_commands_from_file(
                "/workspaces/codetanks/apps/simulator/tests/do_nothing.ct"
            )
        ),
        format!(
            "{:?}",
            vec![
                vec![Commands::NONE, Commands::NONE,],
                vec![Commands::NONE, Commands::NONE,],
                vec![Commands::NONE, Commands::NONE,],
                vec![Commands::NONE, Commands::NONE,],
            ]
        )
    );
}
