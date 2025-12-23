# Yewi-kit
A **Component-driven UI kit for Yew** inspired by **React / Next.ts architecture**, featuring:
- Component-local folders
- Separated logic into custom hooks
- Styling with `tailwind-css` + `scss` (using `@apply` to avoid messy and unreusable inline classes)

## Component Structure

| Concern              | Location   |
| -------------------- | ---------- |
| View (JSX / HTML)    | `mod.rs`   |
| Logic (custom hooks) | `use_*.rs` |
| Utilities            | `(types/data/utils/contants).rs` |
| Styles               | `*.scss`   |


``` bash
components/
├── button/
│   ├── mod.rs          # Button view (Yew component)
│   ├── use_button.rs   # Button logic (custom hook)
│   ├── types.rs        # Component-specific enums / types
│   └── button.scss     # Tailwind + SCSS styles
│
├── input/
│   ├── mod.rs
│   ├── use_input.rs
│   ├── utils.rs
│   └── input.scss
│
├── badge/
│   ├── mod.rs
│   ├── use_badge.rs
│   ├── types.rs
│   └── badge.scss
│
└── mod.rs              # Public exports
```
