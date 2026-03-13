## Personal Website V3
![GitHub Last Commit](https://img.shields.io/github/last-commit/brstrutt/PersonalWebsiteV3?logo=github)
![Build and Publish](https://github.com/brstrutt/PersonalWebsiteV3/actions/workflows/build-and-publish.yml/badge.svg?branch=main)

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

This repo provides the development environment you need in the form of a devcontainer.
I recommend opening the repo in VSCode and selecting "Reopen in container" when the option appears.

To build the code: `trunk build`

To run the dev server locally: `trunk serve`

To auto format the codebase: `leptosfmt ./**/*.rs`

### Publish

Push your changes to the `main` branch. Github Actions will build the latest version and push it live automatically.

The live site can be seen [here](https://brstrutt.github.io/PersonalWebsiteV3/).
Due to quirks in Leptos routing and Github Pages it will open on a 404 page initially. Navigating around the site works but if you refresh it'll redirect to a 404 page.
These issues will solve themselves when I switch to using a custom URL instead of github pages default URL.