use crate::webhook::Webhook;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize)]
struct RepositoryRegistrationToken {
    token: String,
}

#[derive(Serialize)]
pub(crate) struct Claims {
    iat: u64,
    exp: u64,
    iss: String,
}

#[derive(Deserialize)]
struct InstallationAccessToken {
    token: String,
}

pub(crate) fn create_registration_token_for_repository(webhook: &Webhook) -> String {
    let http_client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!(
            "Bearer {}",
            generate_installation_access_token(&webhook, &http_client)
        )
        .parse()
        .unwrap(),
    );
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    headers.insert(
        "User-Agent",
        "github-actions-runner-aws 0.1.0".parse().unwrap(),
    );

    http_client
        .post(format!(
            "https://api.github.com/repos/{}/actions/runners/registration-token",
            webhook.repository.full_name
        ))
        .headers(headers)
        .send()
        .unwrap()
        .json::<RepositoryRegistrationToken>()
        .unwrap()
        .token
}

pub(crate) fn generate_installation_access_token(
    webhook: &Webhook,
    http_client: &Client,
) -> String {
    let mut headers = HeaderMap::new();

    headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", generate_json_web_token())
            .parse()
            .unwrap(),
    );
    headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
    headers.insert(
        "User-Agent",
        "github-actions-runner-aws 0.1.0".parse().unwrap(),
    );

    http_client
        .post(format!(
            "https://api.github.com/app/installations/{}/access_tokens",
            webhook.installation.id
        ))
        .headers(headers)
        .send()
        .unwrap()
        .json::<InstallationAccessToken>()
        .unwrap()
        .token
}

pub(crate) fn generate_json_web_token() -> String {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    jsonwebtoken::encode(
        &Header::new(Algorithm::RS256),
        &Claims {
            iat: time - 60,
            exp: time + (10 * 60),
            iss: env::var("CLIENT_ID").unwrap(),
        },
        &EncodingKey::from_rsa_pem(
            env::var("PRIVATE_KEY")
                .unwrap()
                .replace("\\n", "\n")
                .as_ref(),
        )
        .unwrap(), // TODO: Get the private key from AWS Secrets Manager
    )
    .unwrap()
}
