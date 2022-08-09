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
Run the executable, and that's it! You can visit it in the browser at `http://127.0.0.1:8000`
