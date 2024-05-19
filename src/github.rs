use reqwest::header::HeaderMap;
use serde::Deserialize;

#[derive(Deserialize)]
struct RepositoryRegistrationToken {
    token: String,
}

pub(crate) fn create_registration_token_for_repository(repository_full_name: &String) -> String {
    let http_client = reqwest::blocking::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
    headers.insert(
        "Authorization",
        format!("Bearer {}", std::env::var("TOKEN").unwrap())
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
            repository_full_name
        ))
        .headers(headers)
        .send()
        .unwrap()
        .json::<RepositoryRegistrationToken>()
        .unwrap()
        .token
}
