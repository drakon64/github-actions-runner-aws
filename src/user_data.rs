use crate::github::create_registration_token_for_repository;
use crate::webhook::Webhook;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::env;

pub(crate) fn create_user_data(webhook: &Webhook, spot: bool, volume_size: &i32) -> String {
    let repository_full_name = &webhook.repository.full_name;
    let repository_registration_token = create_registration_token_for_repository(&webhook);

    let user_data_script = include_str!("files/user-data.sh");

    let aws_region = env::var("AWS_REGION").unwrap();
    let tag_script = BASE64_STANDARD.encode(format!("#!/bin/sh

TOKEN=$(curl -X PUT http://169.254.169.254/latest/api/token -H 'X-aws-ec2-metadata-token-ttl-seconds: 21600')
INSTANCE_ID=$(curl -H 'X-aws-ec2-metadata-token: $TOKEN' http://169.254.169.254/latest/meta-data/instance-id/)

aws ec2 create-tags --region {aws_region} --resources $INSTANCE_ID --tags Key=Name,Value=\"${{GITHUB_REPOSITORY}}/${{GITHUB_RUN_ID}}/${{GITHUB_RUN_NUMBER}}/${{GITHUB_RUN_ATTEMPT}}/${{GITHUB_JOB}}\""));

    BASE64_STANDARD.encode(format!("{user_data_script}

base64 -d '{tag_script}' > /home/runner/tag.sh
chown runner:runner /home/runner/tag.sh

ansible-pull --url https://github.com/drakon64/github-actions-runner-aws.git --checkout canary --extra-vars 'url=https://github.com/{repository_full_name}' --extra-vars 'token={repository_registration_token}' --extra-vars '{{ \"spot\": {spot} }}' --extra-vars 'ebs_volume_size={volume_size}' ansible/runner.yml"))
}
