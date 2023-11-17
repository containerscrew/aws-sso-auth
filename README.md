<p align="center" >
    <img src="assets/rust-logo.svg" alt="logo" width="250"/>
<h3 align="center">aws-sso-auth</h3>
<p align="center">Fetch your local ~/.aws/credentials using AWS SSO</p>
<p align="center">Build with ❤ in Rust</p>
</p>

<p align="center" >
    <a href="#">
      <img alt="GitHub code size in bytes" src="https://img.shields.io/github/languages/code-size/containerscrew/aws-sso-auth">
    </a>
    <a href="/LICENSE">
      <img alt="License" src="https://img.shields.io/github/license/containerscrew/aws-sso-auth">
    </a>
    <a href="https://codecov.io/gh/containerscrew/aws-sso-auth" >
      <img src="https://codecov.io/gh/containerscrew/aws-sso-auth/graph/badge.svg?token=4AI2U4PX4V"/>
    </a>
    <a href="https://github.com/containerscrew/aws-sso-auth/releases/latest">
      <img alt="Release" src="https://img.shields.io/github/release/containerscrew/aws-sso-auth">
    </a>
    <a href="https://somsubhra.github.io/github-release-stats/?username=containerscrew&repository=aws-sso-auth">
      <img alt="GitHub Releases Stats" src="https://img.shields.io/github/downloads/containerscrew/aws-sso-auth/total.svg?logo=github">
    </a>
</p>


<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [Pipeline badges](#pipeline-badges)
- [Introduction](#introduction)
- [Installation](#installation)
- [Usage](#usage)
  - [Switching accounts in your terminal](#switching-accounts-in-your-terminal)
- [Fish shell](#fish-shell)
  - [Setting AWS_PROFILE](#setting-aws_profile)
- [TO DO](#to-do)
- [TO DO (not implemented yet)](#to-do-not-implemented-yet)
- [Contribution](#contribution)
- [LICENSE](#license)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# Pipeline badges
![Test Status](https://github.com/containerscrew/aws-sso-auth/actions/workflows/test.yml/badge.svg)
![Release Status](https://github.com/containerscrew/aws-sso-auth/actions/workflows/release.yml/badge.svg)
![Git Leaks Status](https://github.com/containerscrew/aws-sso-auth/actions/workflows/gitleaks.yml/badge.svg)
![Coverage](https://github.com/containerscrew/aws-sso-auth/actions/workflows/coverage.yml/badge.svg)

# Introduction

....blablbla

# Installation

...TO DO


# Usage

```bash
aws-sso-auth --start-url https://XXXXXX.awsapps.com/start --region eu-west-1
```

* **start_url:** your start URL of SSO AWS app (https://docs.aws.amazon.com/singlesignon/latest/userguide/howtochangeURL.html)
* **region:** AWS region where you have your AWS SSO configured. By the default is `eu-west-1` (Ireland)

> All the credentials will be saved in your $HOME/.aws/credentials with the following pattern: [AccountName@RoleName] you are assuming

## Switching accounts in your terminal

Copy the following function in your `~/.zshrc` or `~/.bashrc`:

```shell
aws-profile () {
        PROFILE=$(cat ~/.aws/credentials|grep "^\["|sed "s/]$//"|sed "s/^\[//"| fzf)
        export AWS_PROFILE=$PROFILE
}
```

Then, `source` the file if needed:
```shell
source ~/.zshrc or source ~/.bashrc
```

# Fish shell

Copy the following function inside `~/.config/fish/function/aws-profile.fish`

```shell
function aws-profile
    set -Ux AWS_PROFILES $(cat ~/.aws/credentials | sed -n -e 's/^\[\(.*\)\]/\1/p' | fzf)
    if test -n "$AWS_PROFILES"
        set -xg AWS_PROFILE $AWS_PROFILES
        echo "Selected profile: $AWS_PROFILES"
    else
        echo "No profile selected"
    end
end
```

Then `source` the fish configuration:

```shell
source ~/.config/fish/config.fish
```

## Setting AWS_PROFILE

Type `aws-profile` in your terminal, and you will see all the accounts you have credentials in your `$HOME/.aws/credentials`

> **fzf** is needed as a dependency for the interactive account switcher

[Official documentation](https://github.com/junegunn/fzf#installation)

# TO DO

> https://github.com/awslabs/aws-sdk-rust/discussions/771
* Github actions pipeline to create binary and push to `releases`
* Testing
* Imagine you have 600 accounts with access in your AWS SSO portal, but you only want to fetch 100. How you can limit that?


# TO DO (not implemented yet)

* Multiple AWS SSO account configurations inside `aws-sso-auth.json` Imagine you are working in a consultant, and you have multiple customers with AWS SSO, and you want to save
all their config (start-url, region) inside the config file.
* If you have 200 accounts, only 123 (max), will be fetched
* Select which account credentials (with prefix) do you want to fetch.
* Remove
* Testing
* In console output, exists and empty new line when after info message `Type ENTER to continue`. Need to flush console
* Implement multiple retries when you have 429 errors in API calls
* Overwrite `~/.aws/credentials` file every time you fetch account credentials
* Create function to open file
* Codecoverage pipeline not working
* Changelog with release-please
* Create Homebrew Formula

# Contribution

Pull requests are welcome! Any code refactoring, improvement, implementation. I just want to learn Rust! I'm a rookie

# LICENSE

[LICENSE](./LICENSE)
