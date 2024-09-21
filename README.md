# godam

![rust 1.81](https://img.shields.io/badge/rust-1.81-orange)

> 💡 Godam is my personal tool for installing addons from the Godot AssetLib in a CLI environment (such as a CD pipeline).
> 
> Because of this, the CLI has very limited functionality.
>
> Feel free to raise issues, formulate solutions and contribute!

Godam (**God**ot **A**sset **M**anager) is a minimal command-line tool to manage addons from the Godot Asset Library. 

The objective is to support downloading and installing addons from the Godot Asset Library in the same manner as the Godot client.

This is a personal work-in-progress project, making its use case very narrow.

## Roadmap 🗺️

- **Feature:** Cache downloaded addon zips in a **tmp** folder
  - **Feature:** Clean `tmp` cache folder
- **Feature:** Uninstall addons using **godam**

## How it works ⚙️

Initializing your Godot project with `godam init` creates a `godam.toml` file that registers your Godot version and starts keeping track of your addons.

You can add and remove addons using `godam add <name>` and `godam rm <name>`.

To install your addons, use `godam install`.

## Features ✨

This section outlines the current features available.

- Register addons using `godam add <name>`
  - If an addon is found, it is added to the `godam.toml` file.
  - If multiple addons are found, the top result returned from the API is used.
- Unregister addons using `godam rm <name>`
  - If the name exactly matches the name of a registered addon, it is removed from the `godam.json` file.
