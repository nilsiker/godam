# godam

![rust 1.77](https://img.shields.io/badge/rust-1.77-orange)

> ☝ Godam does not yet support install addons to Godot project.
>
> This is a prioritized planned feature.

Godam (**God**ot **A**sset **M**anager) is a minimal command-line tool to manage addons from the Godot Asset Library.

This is a work-in-progress project, meaning that crucial features are currently missing from the tool.

## Roadmap

- **Feature:** Install command properly installs addons to the Godot project.

## How it works

Initializing your Godot project with `godam init` creates a `godam.json` file that registers your Godot version and starts keeping track of your addons.

You can add and remove addons using `godam add <name>` and `godam rm <name>`.

To install your addons, use `godam install`.

## Features

This section outlines the current features available.

- Register addons using `godam add <name>`
  - If multiple addons are found, possible candidates are listed by their complete names.
  - If one singular addon is found, it is added to the `godam.json` file.
- Unregister addons using `godam rm <name>`
  - If the name exactly matches the name of a registered addon, it is removed from the `godam.json` file.
- Clone addon repositories to a `cache` folder using `godam install`
  - The clones repositories are checked out to the specified `download_commit` from the Godot Asset Library API.
