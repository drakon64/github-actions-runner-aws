use crate::github::create_registration_token_for_repository;
use crate::webhook::Webhook;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::env;

pub(crate) fn create_user_data(
    webhook: &Webhook,
    spot: bool,
    volume_size: &i32,
    swap_volume_size: &i32,
) -> String {
    let repository_full_name = &webhook.repository.full_name;
    let repository_registration_token = create_registration_token_for_repository(&webhook);

    let alloy_config = BASE64_STANDARD.encode(include_str!("files/config.alloy"));
    let grafana_cloud_stack_name = env::var("GRAFANA_CLOUD_STACK_NAME").unwrap();
    let grafana_cloud_token = env::var("GRAFANA_CLOUD_TOKEN").unwrap();

    let aws_region = env::var("AWS_REGION").unwrap();
    let tag_script = BASE64_STANDARD.encode(format!("#!/bin/sh

aws ec2 create-tags --region {aws_region} --resources $(curl -H \"X-aws-ec2-metadata-token: $(curl -X PUT http://169.254.169.254/latest/api/token -H 'X-aws-ec2-metadata-token-ttl-seconds: 21600')\" http://169.254.169.254/latest/meta-data/instance-id/) --tags Key=Name,Value=\"${{GITHUB_REPOSITORY}}/${{GITHUB_RUN_ID}}/${{GITHUB_RUN_NUMBER}}/${{GITHUB_RUN_ATTEMPT}}/${{GITHUB_JOB}}\""));

    let user_data = BASE64_STANDARD.encode(format!("#!/bin/sh

sysctl vm.swappiness=1
mkswap /dev/nvme1n1
swapon /dev/nvme1n1

mkdir -p /etc/apt/keyrings/
curl https://apt.grafana.com/gpg.key | gpg --dearmor > /etc/apt/keyrings/grafana.gpg
echo 'deb [signed-by=/etc/apt/keyrings/grafana.gpg] https://apt.grafana.com stable main' > /etc/apt/sources.list.d/grafana.list

add-apt-repository ppa:ansible/ansible # https://github.com/ansible/ansible/issues/77624
apt-get update
apt-get -y install alloy awscli ansible-core
apt-get clean

echo '{alloy_config}' | base64 -d > /etc/alloy/config.alloy
echo \"
GRAFANA_CLOUD_STACK_NAME=\"{grafana_cloud_stack_name}\"
GRAFANA_CLOUD_TOKEN=\"{grafana_cloud_token}\"\" >> /etc/default/alloy
systemctl restart alloy

adduser runner
mkdir /home/runner/actions-runner
chown runner:runner /home/runner/actions-runner

echo ACTIONS_RUNNER_HOOK_JOB_STARTED=/home/runner/tag.sh > /home/runner/actions-runner/.env
chown runner:runner /home/runner/actions-runner/.env

echo '{tag_script}' | base64 -d > /home/runner/tag.sh
chown runner:runner /home/runner/tag.sh

ansible-galaxy collection install amazon.aws community.general
ansible-pull --url https://github.com/drakon64/github-actions-runner-aws.git --extra-vars 'url=https://github.com/{repository_full_name}' --extra-vars 'token={repository_registration_token}' --extra-vars '{{ \"spot\": {spot} }}' --extra-vars 'ebs_volume_size={volume_size}' --extra-vars 'swap_volume_size={swap_volume_size}' ansible/runner.yml"));

    user_data
}
