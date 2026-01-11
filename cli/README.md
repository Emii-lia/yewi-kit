# Yewi-Kit CLI

[Yewi-cli](https://crates.io/crates/yewi-cli) is a command-line tool for managing UI components in your Yew projects, inspired by shadcn/ui.

## Features

- **Create new projects** from a pre-configured template
- **Add components** from the yewi-kit repository to any yewi project
- **Automatic style imports** - component styles are auto-imported
- **Zero config** - all setup is handled by the CLI
- **Modern architecture** - component-driven, organized structure

## Installation

### From Source

```bash
git clone https://github.com/Emii-lia/yewi-kit.git
cd yewi-kit/cli
cargo install --path .
```

Or install globally:

```bash
cargo install yewi-cli
```

## Quick Start

### 1. Create a New Project

```bash
yewi new my-app
cd my-app
yarn install
yarn build
trunk serve
```

Your project is now ready to go.

### 2. Add Components

Add any component from [yewi-kit](https://github.com/Emii-lia/yewi-kit):

```bash
yewi add button
yewi add input
yewi add badge
```

This automatically:
- Downloads the component from yewi-kit repository
- Places it in `src/components/<component-name>/`
- Adds module declarations to `src/components/mod.rs`
- Imports styles in `src/styles/components.scss`

## Usage

### Create a New Project

```bash
yewi new my-project
yewi create my-project  # alias
```

### Add a Component

```bash
yewi add button
yewi add input
yewi add select
yewi add badge
yewi add card
```

## Project Structure

After `yewi new`, your project will have this structure:

```
my-app/
├── src/
│   ├── components/           # Reusable UI components (add with yewi add)
│   │   └── mod.rs
│   ├── features/             # Feature-level components
│   ├── app/                  # App routes and layout (next.ts style)
│   │   ├── not-found/        # 404 page
│   │   ├── page.rs           
│   │   ├── route.rs
│   │   └── mod.rs            # App component
│   ├── styles/               # Global and component styles
│   │   ├── main.scss         # Global Tailwind + SCSS imports
│   │   ├── global.css        # Generated Tailwind CSS after `yarn build`
│   │   ├── components.scss   # Auto-imported component styles
│   │   └── features.scss
│   ├── types/                # Custom types and enums
│   └── main.rs               # Entry point
├── index.html
├── Cargo.toml
├── Trunk.toml
├── package.json
├── tailwind.config.js
├── postcss.config.js
└── yarn.lock
```

## Component Structure

Each component downloaded with `yewi add` has this structure:

```
components/
└── button/
    ├── mod.rs              # Component view
    ├── hooks.rs            # Component logic (if needed)
    ├── types.rs            # Component types (if needed)
    └── button.scss         # Component styles
    └── ...
```

## Styling with Tailwind + SCSS

Each component uses Tailwind CSS utilities combined with SCSS:

```scss
// src/components/divider/divider.scss
.Divider {
  @apply flex items-center gap-2 w-full;
  @apply text-slate-500 font-medium;
  &:before, &:after {
    @apply w-1/2 h-0.5 rounded-full bg-slate-500 content-[""];
  }
  &:not(&.has-children) {
    @apply w-full h-0.5 bg-slate-500 rounded-full;
  }
  &.vertical {
    @apply h-full flex flex-col items-center w-0.5 relative min-h-10 bg-inherit;
    &:before, &:after {
      @apply w-0.5 h-1/2 rounded-full bg-slate-500 content-[""] min-h-4;
    }
    &:not(&.has-children) {
      @apply w-0.5 h-full bg-slate-500;
    }
  }
}

```

## Configuration Files

The template includes pre-configured:

- **tailwind.config.js** - Tailwind CSS configuration
- **postcss.config.js** - PostCSS with Tailwind & Autoprefixer
- **Trunk.toml** - Rust/Wasm build configuration with Sass support
- **Cargo.toml** - Rust dependencies (yew, wasm-bindgen, web-sys)
- **package.json** - Node dependencies (tailwindcss, postcss, sass, etc.)

## Customization

### Global Styles

Edit `src/styles/main.scss` for global styles:

```scss
@tailwind base;
@tailwind components;
@tailwind utilities;

body {
  @apply font-sans;
}
```

## Troubleshooting

### Component not found

Make sure the component exists in the yewi-kit repository:
- Check: https://github.com/Emii-lia/yewi-kit/tree/master/src/components

### Styles not applied

1. Run `yarn install` to install all dependencies, then run `yarn build` to generate Tailwind CSS
2. Check that `src/styles/components.scss` has the component import
3. Restart `trunk serve` after adding components

### Port already in use

```bash
trunk serve --port <another-port>
```

### Git not available

If git is not installed, the CLI will fall back to downloading the template via GitHub API automatically.

## Creating Custom Components

To add your own components to the project, follow these steps:

1. Create a directory under `src/components/{component-name}/`
2. Add at least `mod.rs` and `{component-name}.scss`
3. Export the component in `src/components/mod.rs`
4. Import styles in `src/styles/components.scss`

## License

MIT

## Support

For issues, questions, or suggestions, please open an issue on the [GitHub repository](https://github.com/Emii-lia/yewi-kit)

## Yewi-Kit website

Visit the [Yewi-Kit website](https://yewi.pages.dev) for documentation, examples, and more information about the components available.