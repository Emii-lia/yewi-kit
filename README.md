# Yewi-kit

Official component source and documentation website for **Yewi UI**.

Live documentation:
[https://yewi.pages.dev](https://yewi.pages.dev)

Crate:

[https://crates.io/crates/yewi-cli](https://crates.io/crates/yewi-cli)

## Purpose

`yewi-kit` is the canonical source of truth for Yewi components.

This repository:

-   Defines and maintains official components
-   Provides implementation references
-   Powers the documentation website
-   Supplies templates consumed by `yewi-cli`

It is not intended to be installed as a dependency.
End users generate and own their components through the CLI.


## Architecture Overview

All components live in isolated folders under `src/components`.

Example structure:
```
    components/
    └── button/
        ├── mod.rs
        ├── types.rs
        ├── hooks.rs
        └── button.scss
```

Design principles:

-   Strong typing
-   Clear separation of view and logic
-   Localised styling
-   Explicit props
-   No hidden side effects

Each component is self contained and exportable.


## Tech Stack

-   Rust
-   Yew
-   SCSS
-   Tailwind CSS
-   Trunk
-   wasm32-unknown-unknown target


## Relationship with yewi-cli

[yewi-cli](https://crates.io/crates/yewi-cli) is the official way to use Yewi in real projects.

Workflow:

1.  Components are authored and maintained in this repository.
2.  `yewi-cli` pulls component templates.
3.  The CLI injects selected components into the user project.
4.  The user owns the generated code.

There is no runtime dependency on `yewi-kit`.

This ensures:

-   Full control over component code
-   No vendor lock in
-   Clean project structure
-   Direct customisation


# Yewi Setup Guide

Install and use Yewi through `yewi-cli`.

## 1. Install the CLI

Requires Rust stable.

``` bash
cargo install yewi-cli
```

Install the wasm target:

``` bash
rustup target add wasm32-unknown-unknown
```


## 2. Create a New Project

``` bash
yewi new my-app
cd my-app
yarn install
yarn build
```

This command:

-   Creates a Yew project
-   Configures Tailwind
-   Sets up Trunk
-   Prepares the project structure

Run the project:

``` bash
trunk serve
```


## 3. Add Components

Add components individually:

``` bash
yewi add button
yewi add input
yewi add card
```

The CLI will:

-   Copy component source into your project
-   Register modules
-   Inject required styles
-   Keep everything local

You can modify the generated code freely.


## 4. Development Workflow

Start development:

``` bash
trunk serve
```

Build for production:

``` bash
trunk build --release
```
