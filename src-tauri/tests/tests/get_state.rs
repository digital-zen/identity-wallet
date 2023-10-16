use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::{json_example, test_managers};
use identity_wallet::state::Profile;
use identity_wallet::state::{actions::Action, AppState, TransferState};
use std::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_get_state_create_new() {
    setup_state_file();
    setup_stronghold();

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state1 = json_example::<TransferState>("tests/tests/fixtures/states/no_profile_redirect_welcome.json");
    let state2 = json_example::<TransferState>("tests/tests/fixtures/states/active_pf_redirect_me.json");
    let action1 = json_example::<Action>("tests/tests/fixtures/actions/get_state.json");
    let action2 = json_example::<Action>("tests/tests/fixtures/actions/create_new.json");
    assert_state_update(
        // Initial state.
        AppState::default(),
        vec![
            // Get the initial state.
            action1, // Create a new profile.
            action2,
        ],
        vec![
            // There is no profile yet, so the user is redirected to the welcome page.
            Some(state1),
            // The profile was created, so the user is redirected to the profile page.
            Some(state2),
        ],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_state_unlock_storage() {
    setup_state_file();
    setup_stronghold();

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state1 = json_example::<TransferState>("tests/tests/fixtures/states/active_pf_password_required.json");
    let state2 = json_example::<TransferState>("tests/tests/fixtures/states/active_pf_redirect_me.json");
    let action1 = json_example::<Action>("tests/tests/fixtures/actions/get_state.json");
    let action2 = json_example::<Action>("tests/tests/fixtures/actions/unlock_storage.json");
    assert_state_update(
        // Initial state.
        AppState {
            managers: test_managers(vec![]),
            active_profile: Mutex::new(Some(Profile {
                name: "Ferris Crabman".to_string(),
                picture: Some("&#129408".to_string()),
                theme: Some("system".to_string()),
                primary_did: "did:example:placeholder".to_string(),
            })),
            ..AppState::default()
        },
        vec![
            // Get the initial state.
            action1, // Unlock the storage.
            action2,
        ],
        vec![
            // The storage is locked, so the user is prompted to unlock it.
            Some(state1),
            // The storage is unlocked, so the user is redirected to the profile page.
            Some(state2),
        ],
    )
    .await;
}
