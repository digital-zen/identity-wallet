use crate::state::actions::Action;

pub async fn profile_setting_update(state: &AppState, action: Action) -> anyhow::Result<()> {
    //let setting_update = serde_json::from_value(action.payload.unwrap()).unwrap();

    match action.payload["target"] {
        "credentials_sort" => {
            update_setting(state, action.payload);
            sort_credentials(state, action.payload);
        },
        "connections_sort" => {
            sort_connections(state, action.payload)
        },
    }
}
