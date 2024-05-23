# github-actions-runner-aws

`github-actions-runner-aws` is a serverless function for running ephemeral GitHub Actions runners in AWS EC2.

For each workflow job in a repository, a webhook is sent to AWS API Gateway. This runs a Lambda function that creates an EC2 instance with the GitHub Actions runner deployed. When the workflow job is completed, another webhook terminates the EC2 instance.

# Limitations
* All runner instances use Ubuntu 22.04 AArch64 as their OS

# Building
To compile `github-actions-runner-aws` so that the included OpenTofu configuration can deploy it, run:

`cargo lambda build --release --arm64 --output-format zip`

# Deploying
`github-actions-runner-aws` can be deployed with OpenTofu by running:

```shell
cd terraform
tofu apply
```

# Roadmap
* Add CloudWatch Agent to instances
* Remove repository owner hack
* Implement our own JWT
* Documentation
