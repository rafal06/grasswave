# Grasswave CDN
A stupidly simple and easy to self-host, personal server for file hosting on the web. Written in Rust.

Thanks, [@Maciejowski](https://github.com/maciejowski2006/), for the stylesheet!

## Screenshots

<img src="screenshots/desktop.png" alt="screenshot of the main page">
<details>
    <summary>Show more</summary>
    <h3>Mobile layout</h3>
    <img src="screenshots/mobile.png"     alt="mobile layout" />
    <h3>Search</h3>
    <img src="screenshots/search.png"     alt="search"        />
    <h3>Tag search</h3>
    <img src="screenshots/tag-search.png" alt="tag search"    />
    </details>

## Installation

### Docker (recommended)
0. Install Docker
1. Create a new directory and, inside it, a file called `docker-compose.yml`
2. Copy the following contents into that file:

```yml
version: '3.3'

services:
  grasswave:
    image: rafal06/grasswave:latest
    container_name: grasswave
    restart: unless-stopped
    ports:
      - '7000:7000'
    volumes:
      - ./data:/data
```
3. Start the container with the command `docker compose up -d`
4. Copy the files for serving to the `./data/files` directory (see the section [How to use](#how-to-use))
5. If the container is not starting, check the logs with `docker compose logs`

### Compile from source
0. Install the [Rust toolchain](https://rustup.rs)
1. Clone this repository
2. Create the `files` folder and put your files inside (see [How to use](#how-to-use))
3. Run `cargo run --release`
4. This will compile and run the project

## How to use
For every file you want to publish, create a seperate directory in the `files` folder, and place the files inside of them. In every directory, create a file named `info.toml`.
```
files
├── lorem
│   ├── lorem.tar.xz
│   └── info.toml
├── ipsum
│   ├── info.toml
│   └── ipsum.tar.xz
└── dolor
    ├── info.toml
    └── dolor.tar.xz
```
In the toml files, set the name and description to display, and the name of the file to publish, in the following format:
```toml
name = "Lorem"
description = "Lorem ipsum dolor sit amet"
tags = ["lorem", "ipsum", "dolor"]
path = "lorem.tar.xz"
```
Run the executable, and that's it! You can visit it in the browser at the link displayed in the terminal.

You can configure it in a config file created automatically on the first run.

<details>
<summary>Default config</summary>

```toml
displayed_name = "Grasswave CDN"
files_path = "files"
accent_colors = ["#1D9F00", "#4DE928"]
http_port = 7000
```
</details>
