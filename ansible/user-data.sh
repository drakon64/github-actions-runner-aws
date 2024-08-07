#!/bin/sh

sysctl vm.swappiness=1
mkswap /dev/nvme1n1
swapon /dev/nvme1n1

mkdir -p /etc/apt/keyrings/
wget -q -O - https://apt.grafana.com/gpg.key | gpg --dearmor > /etc/apt/keyrings/grafana.gpg
echo 'deb [signed-by=/etc/apt/keyrings/grafana.gpg] https://apt.grafana.com stable main' > /etc/apt/sources.list.d/grafana.list

add-apt-repository ppa:ansible/ansible # https://github.com/ansible/ansible/issues/77624
apt-get update
apt-get -y install ansible-core awscli alloy
apt-get clean
ansible-galaxy collection install amazon.aws community.general
