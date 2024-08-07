use crate::github::create_registration_token_for_repository;
use crate::webhook::Webhook;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::env;

pub(crate) fn create_user_data(webhook: &Webhook, spot: bool, volume_size: &i32) -> String {
    let repository_full_name = &webhook.repository.full_name;
    let repository_registration_token = create_registration_token_for_repository(&webhook);

    let aws_region = env::var("AWS_REGION").unwrap();
    let tag_script = BASE64_STANDARD.encode(format!("#!/bin/sh

aws ec2 create-tags --region {aws_region} --resources $(curl -H \"X-aws-ec2-metadata-token: $(curl -X PUT http://169.254.169.254/latest/api/token -H 'X-aws-ec2-metadata-token-ttl-seconds: 21600')\" http://169.254.169.254/latest/meta-data/instance-id/) --tags Key=Name,Value=\"${{GITHUB_REPOSITORY}}/${{GITHUB_RUN_ID}}/${{GITHUB_RUN_NUMBER}}/${{GITHUB_RUN_ATTEMPT}}/${{GITHUB_JOB}}\""));

    BASE64_STANDARD.encode(format!("#!/bin/sh

sysctl vm.swappiness=1
mkswap /dev/nvme1n1
swapon /dev/nvme1n1

adduser runner
mkdir /home/runner/actions-runner
chown runner:runner /home/runner/actions-runner

echo ACTIONS_RUNNER_HOOK_JOB_STARTED=/home/runner/tag.sh > /home/runner/actions-runner/.env
chown runner:runner /home/runner/actions-runner/.env

mkdir -p /etc/apt/keyrings/
curl https://apt.grafana.com/gpg.key | gpg --dearmor > /etc/apt/keyrings/grafana.gpg
echo 'deb [signed-by=/etc/apt/keyrings/grafana.gpg] https://apt.grafana.com stable main' > /etc/apt/sources.list.d/grafana.list

echo '{tag_script}' | base64 -d > /home/runner/tag.sh
chown runner:runner /home/runner/tag.sh

add-apt-repository ppa:ansible/ansible # https://github.com/ansible/ansible/issues/77624
apt-get update
apt-get -y install ansible-core awscli alloy
apt-get clean
ansible-galaxy collection install amazon.aws community.general
ansible-pull --url https://github.com/drakon64/github-actions-runner-aws.git --checkout canary --extra-vars 'url=https://github.com/{repository_full_name}' --extra-vars 'token={repository_registration_token}' --extra-vars '{{ \"spot\": {spot} }}' --extra-vars 'ebs_volume_size={volume_size}' ansible/runner.yml"))
}
