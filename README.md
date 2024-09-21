# GODAM - A minimal Godot Asset Manager

![rust 1.81](https://img.shields.io/badge/rust-1.81-orange)

> 💡 godam is my personal tool for installing assets from the Godot AssetLib in a CLI environment (such as a CD pipeline).
> 
> Because of this, the CLI has very limited functionality.
>
> Feel free to raise issues and provide feedback!

GODAM (**God**ot **A**sset **M**anager) is a minimal command-line tool to manage assets from the Godot Asset Library. 

The CLI aims to operate in the same way the Godot Editor downloads and installs assets from the Godot Asset Library. The objective is to allow developers to fully omit addons from their VCS, and keep track of them using a declarative configuration file (similar to `Cargo.toml` or `package.json`)

This is a personal work-in-progress project - its use case is strictly scoped to my current needs.

## Features ✨

This section outlines the current features available.

- Searching the Godot Asset Library API for asset IDs.
- Installing assets using the asset ID found in Godot Asset Library ID.
- Uninstalling assets using the asset ID found in Godot Asset Library ID.
- Listing all assets managed by GODAM
- Cleaning the asset cache
  
## How it works ❔

GODAM scaffolds your Godot project folder with a `godam.toml` file, a `.godam` cache folder and a `.gitignore` to your `addons` folder. The .gitignore sets up your git to ignore all contents of the addon folder except for the TOML configuration file.

When adding an asset using `godam install <ID>` the following happens:

1. The asset information is fetched from the Godot Asset Library API and added to the `godam.toml` file.

2. The `.godam` cache folder is checked, to see if the addon is already downloaded. If not, the zip is downloaded and saved to the cache folder.

3. The zip is crawled and the `addons` folder in the zip is copied (with all its contents) from the zip to your addons folder.

4. The asset folder name is stored in the `godam.toml`, to keep track of what asset ID is mapped to what install folder.
  
    > ⚠️ This is sort of messy, but the Godot Asset Library API does not store information about the folder structure of the zip download URL specified in the Asset Library. 
    > 
    > godam expects the zip to either directly contain the addons folder, or that the addons folder is found directly under any  folder of the zip.

5. These steps are repeated for each asset registered in the `godam.toml` file.


## Example ⚙️

To install LimboAI with GODAM, perform the following steps:

1. `cd` to your Godot project directory (this is were your `project.godot` file resides)
   
2. Initialize your Godot project with `godam init`

3. Look up Godot Asset Library IDs using `godam search LimboAI`

4. Install the asset using `godam install <ID RETURNED BY STEP 3>`

5. List GODAM-managed assets using `godam list`
 
6. Install all managed assets using `godam install`
  
7. Clean the cache using `godam clean` 


# Future ideas 🔮

- Upgrading assets
- Stabilize codebase, removing expects and streamlining messy portions (such as the `install_folder` lookup)