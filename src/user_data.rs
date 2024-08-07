use crate::github::create_registration_token_for_repository;
use crate::webhook::Webhook;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;

pub(crate) fn create_user_data(webhook: &Webhook, spot: bool, volume_size: &i32) -> String {
    let repository_full_name = &webhook.repository.full_name;
    let repository_registration_token = create_registration_token_for_repository(&webhook);

    let user_data_script = include_str!("files/user-data.sh");

    BASE64_STANDARD.encode(format!("{user_data_script}
ansible-pull --url https://github.com/drakon64/github-actions-runner-aws.git --checkout canary --extra-vars 'url=https://github.com/{repository_full_name}' --extra-vars 'token={repository_registration_token}' --extra-vars '{{ \"spot\": {spot} }}' --extra-vars 'ebs_volume_size={volume_size}' ansible/runner.yml"))
}
