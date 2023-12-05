use crate::{
    state::{actions::Action, reducers::profile_settings::sort_credentials, user_prompt::CurrentUserPrompt, app_state::AppState},
    verifiable_credential_record::VerifiableCredentialRecord,
};
use log::info;
use oid4vci::{
    credential_format_profiles::{CredentialFormats, WithCredential},
    credential_offer::{CredentialOffer, CredentialOfferQuery, CredentialsObject, Grants},
    credential_response::CredentialResponseType,
    token_request::{PreAuthorizedCode, TokenRequest},
};
use serde_json::json;
use uuid::Uuid;

/// Reads Credential Offer and prompts the user to accept it.
pub async fn read_credential_offer(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("read_credential_offer");
    let state_guard = state.managers.lock().await;
    let wallet = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(anyhow::anyhow!("no identity manager found"))?
        .wallet;

    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;

    let mut credential_offer: CredentialOffer = match serde_json::from_value(payload)? {
        CredentialOfferQuery::CredentialOffer(credential_offer) => credential_offer,
        CredentialOfferQuery::CredentialOfferUri(credential_offer_uri) => {
            wallet.get_credential_offer(credential_offer_uri).await?
        }
    };
    info!("credential offer: {:?}", credential_offer);

    // The credential offer contains a credential issuer url.
    let credential_issuer_url = credential_offer.clone().credential_issuer;

    info!("credential issuer url: {:?}", credential_issuer_url);

    // Get the credential issuer metadata.
    let credential_issuer_metadata = if credential_offer
        .credentials
        .iter()
        .any(|credential| matches!(credential, CredentialsObject::ByReference(_)))
    {
        wallet
            .get_credential_issuer_metadata(credential_issuer_url.clone())
            .await
            .ok()
    } else {
        None
    };

    info!("credential issuer metadata: {:?}", credential_issuer_metadata);

    // For all credentials by reference, replace them with credentials by value using the CredentialIssuerMetadata.
    for credential in credential_offer.credentials.iter_mut() {
        match credential {
            CredentialsObject::ByReference(by_reference) => {
                *credential = CredentialsObject::ByValue(
                    credential_issuer_metadata
                        .as_ref()
                        .and_then(|credential_issuer_metadata| {
                            credential_issuer_metadata
                                .credentials_supported
                                .iter()
                                .find(|credential_supported| {
                                    credential_supported.scope == Some(by_reference.to_owned())
                                })
                                .map(|credential_supported| credential_supported.credential_format.clone())
                        })
                        .ok_or(anyhow::anyhow!("unable to find credential"))?,
                );
            }
            _by_value => (),
        }
    }

    // Get the credential issuer display if present.
    let display = credential_issuer_metadata
        .and_then(|credential_issuer_metadata| {
            credential_issuer_metadata
                .display
                .map(|display| display.first().cloned())
        })
        .flatten();

    // Get the credential issuer name and logo uri or use the credential issuer url.
    let (issuer_name, logo_uri) = display
        .map(|display| {
            let issuer_name = display["client_name"]
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or(credential_issuer_url.to_string());
            let logo_uri = display["logo_uri"].as_str().map(|s| s.to_string());
            (issuer_name, logo_uri)
        })
        .unwrap_or((credential_issuer_url.to_string(), None));

    info!("issuer_name in credential_offer: {:?}", issuer_name);
    info!("logo_uri in credential_offer: {:?}", logo_uri);

    *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::CredentialOffer {
        issuer_name,
        logo_uri,
        credential_offer,
    });
    Ok(())
}

/// This function sends a credential request and receives the credentials from the Credential Issuer.
/// Then it saves the credential(s) to the storage and the appstate and ultimately redirects User Prompt to `me` page.
pub async fn send_credential_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("send_credential_request");
    let state_guard = state.managers.lock().await;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(anyhow::anyhow!("no stronghold manager found"))?;
    let wallet = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(anyhow::anyhow!("no identity manager found"))?
        .wallet;

    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let offer_indices: Vec<usize> = serde_json::from_value(payload["offer_indices"].clone())?;

    let current_user_prompt = state
        .current_user_prompt
        .lock()
        .unwrap()
        .clone()
        .ok_or(anyhow::anyhow!(
            "no current user prompt found, unable to send credential request"
        ))?;

    info!("current_user_prompt: {:?}", current_user_prompt);

    let credential_offer = match current_user_prompt {
        CurrentUserPrompt::CredentialOffer { credential_offer, .. } => credential_offer,
        _ => unreachable!(),
    };

    // The credential offer contains a credential issuer url.
    let credential_issuer_url = credential_offer.credential_issuer;

    info!("credential issuer url: {:?}", credential_issuer_url);

    // Get the authorization server metadata.
    let authorization_server_metadata = wallet
        .get_authorization_server_metadata(credential_issuer_url.clone())
        .await?;

    info!("authorization server metadata: {:?}", authorization_server_metadata);

    // Get the credential issuer metadata.
    let credential_issuer_metadata = wallet
        .get_credential_issuer_metadata(credential_issuer_url.clone())
        .await?;

    info!("credential issuer metadata: {:?}", credential_issuer_metadata);

    // Get the credential issuer display.
    let display = credential_issuer_metadata
        .display
        .as_ref()
        .and_then(|display| display.first().cloned());

    // Get the credential issuer name or use the credential issuer url.
    let issuer_name = display
        .map(|display| {
            let issuer_name = display["client_name"]
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or(credential_issuer_url.to_string());
            issuer_name
        })
        .unwrap_or(credential_issuer_url.to_string());

    let credential_offer_formats = offer_indices
        .into_iter()
        .map(|offer_index| match credential_offer.credentials.get(offer_index) {
            Some(CredentialsObject::ByValue(credential_format)) => credential_format.to_owned(),
            _ => unreachable!(),
        })
        .collect::<Vec<CredentialFormats>>();

    info!("credential_offer_formats: {:?}", credential_offer_formats);

    // Create a token request with grant_type `pre_authorized_code`.
    let token_request = match credential_offer.grants {
        Some(Grants {
            pre_authorized_code, ..
        }) => TokenRequest::PreAuthorizedCode {
            grant_type: PreAuthorizedCode,
            pre_authorized_code: pre_authorized_code.unwrap().pre_authorized_code,
            user_pin: None,
        },
        None => unreachable!(),
    };
    info!("token_request: {:?}", token_request);

    // Get an access token.
    let token_response = wallet
        .get_access_token(authorization_server_metadata.token_endpoint.unwrap(), token_request)
        .await?;

    info!("token_response: {:?}", token_response);

    let credentials: Vec<CredentialFormats<WithCredential>> = match credential_offer_formats.len() {
        0 => vec![],
        1 => {
            // Get the credential.
            let credential_response = wallet
                .get_credential(
                    credential_issuer_metadata,
                    &token_response,
                    credential_offer_formats[0].to_owned(),
                )
                .await?;

            let credential = match credential_response.credential {
                CredentialResponseType::Immediate(credential) => credential,
                _ => panic!("Credential was not a jwt_vc_json."),
            };

            vec![credential]
        }
        _batch => {
            let batch_credential_response = wallet
                .get_batch_credential(credential_issuer_metadata, &token_response, credential_offer_formats)
                .await?;

            batch_credential_response
                .credential_responses
                .into_iter()
                .map(|credential_response| match credential_response {
                    CredentialResponseType::Immediate(credential) => credential,
                    _ => panic!("Credential was not a jwt_vc_json."),
                })
                .collect()
        }
    };
    info!("credentials: {:?}", credentials);

    for credential in credentials.into_iter() {
        let mut verifiable_credential_record = VerifiableCredentialRecord::from(credential);
        verifiable_credential_record.display_credential.issuer_name = Some(issuer_name.clone());
        let key: Uuid = verifiable_credential_record
            .display_credential
            .id
            .parse()
            .expect("invalid uuid");

        info!("generated hash-key: {:?}", key);

        // Remove the old credential from the stronghold if it exists.
        stronghold_manager.remove(key)?;

        stronghold_manager.insert(key, json!(verifiable_credential_record).to_string().as_bytes().to_vec())?;
    }

    *state.credentials.lock().unwrap() = stronghold_manager
        .values()?
        .unwrap()
        .into_iter()
        .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
        .collect();

    // Resort according to active_profile.settings
    sort_credentials(state).await;

    state
        .current_user_prompt
        .lock()
        .unwrap()
        .replace(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });

    Ok(())
}
