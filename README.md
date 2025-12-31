# Yewi-kit
A **Component-driven UI kit for Yew** inspired by **React / Next.ts architecture**, featuring:
- Component-local folders
- Separated logic into custom hooks
- Styling with `tailwind-css` + `scss` (using `@apply` to avoid messy and unreusable inline classes)

## Component Structure

| Concern              | Location                         |
| -------------------- |----------------------------------|
| View (JSX / HTML)    | `mod.rs`                         |
| Logic (custom hooks) | `hooks.rs`                       |
| Utilities            | `(types/data/utils/contants).rs` |
| Styles               | `style.scss`                     |


``` bash
components/
├── button/
│   ├── mod.rs          # Button view (Yew component)
│   ├── hooks.rs   # Button logic (custom hook)
│   ├── types.rs        # Component-specific enums / types
│   └── style.scss     # Tailwind + SCSS styles
│
├── input/
│   ├── mod.rs
│   ├── hooks.rs
│   ├── utils.rs
│   └── style.scss
│
├── badge/
│   ├── mod.rs
│   ├── hooks.rs
│   ├── types.rs
│   └── style.scss
│
└── mod.rs              # Public exports
```