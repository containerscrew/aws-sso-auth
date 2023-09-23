<p align="center" >
    <img src="assets/rust-logo.svg" alt="logo" width="250"/>
<h3 align="center">aws-sso-rs</h3>
<p align="center">Get your AWS credentials using SSO</p>
<p style="text-align: center;">Build with ❤ in Rust</p>
</p>


<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [Disclaimer](#disclaimer)
- [Introduction](#introduction)
- [Installation](#installation)
- [How to use](#how-to-use)
- [TO DO](#to-do)
- [Contribution](#contribution)
- [LICENSE](#license)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# Disclaimer

> I don't have any experience with Rust. I'm not a professional software developer. But, I enjoy trying to create things.
I have some experience in python and Golang. I've only been programming in Rust for 2 weeks and trying to understand the language well. Rust is a new language which I find very interesting and I have fallen in love with it from the first moment, that is why I decided to make this tool in Rust.
Probably this tool is more easy to do in Go or Python, but simple things are boring :)

# Introduction

This tools is just in **BETA** mode. It's working fine and you can download all credentials in your `$HOME/.aws/credentials`. In fact, `this README`is still in progress.

# Installation

TO DO: provide automated pipelines to generate binaries with ARM&AMD binaries to `github releases`. Provide formula for brew mac users, and more!

# Usage

```bash
aws-sso-rs --start-url https://XXXXXX.awsapps.com/start --region eu-west-1
```

* **start_url:** your start URL of SSO AWS app (https://docs.aws.amazon.com/singlesignon/latest/userguide/howtochangeURL.html)
* **region:** AWS region where you have your AWS SSO configured. By the default is `eu-west-1` (Ireland)

## Switching accounts

Copy the following function in your `.zshrc` or `.bashrc`:

```shell
aws-profile () {
        PROFILE=$(cat ~/.aws/credentials|grep "^\["|sed "s/]$//"|sed "s/^\[//"| fzf)
        export AWS_PROFILE=$PROFILE
}
```

# TO DO

* Logging with env_logger (aws_config it's printing some unnecessary info)
> https://docs.aws.amazon.com/sdk-for-rust/latest/dg/logging.html
> https://github.com/awslabs/aws-sdk-rust/discussions/771
* Speed credentials download with `async` + `tokio:spawn` threading.
* Code refactoring
* Github actions pipeline to create binary and push to `releases`
* Testing
* more...

# Developing

## Clone

```bash
git clone https://github.com/containerscrew/aws-sso-rs.git
cd aws-sso-rs
```

## Cargo run

```bash
cargo run --start-url https://XXXXXX.awsapps.com/start --region eu-west-1
```

## Build

```bash
cargo build --release # --release flag for production environment, without --release flag for testing
```


# Contribution

Pull requests are welcome! Any code refactoring, improvement, implementation. I just want to learn Rust! I'm a rookie

# LICENSE

[LICENSE](./LICENSE)
