# github-actions-runner-aws

`github-actions-runner-aws` is a serverless function for running ephemeral GitHub Actions runners in AWS EC2.

# Limitations
* Only the `eu-west-2` AWS region is supported
* All runner instances use Ubuntu 22.04 AArch64 as their OS
* All runner instances use the `r7g.large` instance type

# Building
To compile `github-actions-runner-aws` so that the included OpenTofu configuration can deploy it, run:

`cargo lambda build --release --arm64 --output-format zip`

# Deploying
`github-actions-runner-aws` can be deployed with OpenTofu by running:

```shell
cd terraform
tofu apply
```
