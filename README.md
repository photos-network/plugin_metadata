# Photos.network

[![License](https://img.shields.io/github/license/photos-network/plugin_metadata)](./LICENSE.md)
[![GitHub contributors](https://img.shields.io/github/contributors/photos-network/plugin_metadata?color=success)](https://github.com/photos-network/plugin_interface/graphs/contributors)
[![Discord](https://img.shields.io/discord/793235453871390720)](https://discord.gg/dGFDpmWp46)


[Photos.network](https://photos.network) is an open source project for self hosted photo management.
Its core features are:

- Share photos with friends, family or public
- Filter / Search photos by attributes like location or date
- Group photos by objects like people of objects

## Metadata Plugin

This repository contains the `Metadata plugin` for the core.

Its responsibility is to extract metadata from the filename and exif date in a media file.

## Development

The core is written in [Rust](https://rust-lang.org/) and highly customizably by using a Plugin-system.

Plugins are realized by **dynamic loading** by using [abi_stable_crates](https://github.com/rodrimati1992/abi_stable_crates)

---

### Visual Studio Code

The fastest start into development can be archived by using [Visual Studio Code](https://code.visualstudio.com/) and [Docker](https://www.docker.com/get-started).

1. Install [Docker](https://www.docker.com/get-started)
2. Install [Visual Studio Code](https://code.visualstudio.com/)
3. Install [Visual Studio Code Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
4. Clone and Open this repository in Visual Studio Code
5. Click the "Reopen in Container" Dialog
6. Launch **Photos.network** from the `RUN` window.

![VS Code with devcontainers](vscode.gif)

---
