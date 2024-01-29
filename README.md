# Ruffles, a helpful package manager

### Ruffles is an in-development package manager written in Rust that aims to be quick and cross-platform, allowing for apps to be installed on Linux (and soon macOS and Windows)
### Ruffles uses developer-personalised scripts to install and configure applications in the /ruffles folder.

## Features currently avaliable:

- Download a script to install an app
- Download a script to remove an app
- Support for Linux systems

## Features that should come:

- Allow app developers to create their own scripts to install their apps
- Allow app developers to send their scripts to Ruffles repos
- Change your script repo
- Install apps from Flatpak and allow easy execution with the terminal from Linux
- Support for Windows and macOS apps

## Some advantages of using scripts:

- Developers choose where and how to install their apps
- Developers can choose where in the world to host their apps
- Anyone can create a repo with a GitHub or GitLab repository

## What Ruffles isn't

- Complete (still very early in progress)
- A standalone package manager for installing offline packages

### Please take note that Ruffles is still in VERY EARLY STAGES of development and is pretty underpowered right now

## Building yourself

1. Install cargo and rustc on your system, if you don't know how here's a guide: https://www.rust-lang.org/learn/get-started
2. Clone the GitHub repository
3. Run "cargo build" (this should download all the dependencies and compile them for you)
4. Run the executable in (ruffles folder)/target/debug
