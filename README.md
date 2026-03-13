## Description

My personal website.
This is a total rewrite, intended to replace [PersonalWebsiteV2](https://github.com/brstrutt/PersonalWebsiteV2).
The goal is to use Leptos to rebuild the website from the ground up.

## Development

### Tools

- Language: Rust
- Build tool: Trunk
- Web Framework: Leptos (client-side rendering only)
- Formatter: leptosfmt

### Setup

- This repo comes with a devcontainer. This container contains all the tools you should need to work on this repo.

### Build

`trunk build`

### Run

`trunk serve`

### Format

`leptosfmt ./**/*.rs`

### Publish

When the `main` branch is updated it will automatically be built and pushed live
