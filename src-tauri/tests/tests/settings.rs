use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::{actions::Action, app_state::AppState};

#[tokio::test]
#[serial_test::serial]
async fn test_credentials_sort_setting() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_sort_setting.json");
    assert_state_update(state, vec![action], vec![Some(expected_state)]).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_connections_sort_setting() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_search_query.json");
    assert_state_update(state, vec![action], vec![Some(expected_state)]).await;
}

