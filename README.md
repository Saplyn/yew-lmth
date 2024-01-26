# Yew LMTH

A macro crate for writing HTML-like syntax for [Yew](https://yew.rs/) application, highly inspired by [Sycamore](https://sycamore-rs.netlify.app) and [Dioxus](https://dioxuslabs.com/). It works by translating it into a corresponding `yew::prelude::html!()` macro.

## Features

- **Basic Tags**
  - [x] **Types**: built-in, component, generic, void (non-closing).
  - [x] **Attributes**: string literal, expression or code block binding.
  - [x] **Content**: tag children, string literal, or code block.
- **Yew**
  - [x] **[Fragment tag](https://yew.rs/docs/concepts/html/fragments)**: `! { ... }`
  - [x] **[Dynamic tag names](https://yew.rs/docs/concepts/html/elements#dynamic-tag-names)**: `@{expr} ( ... ) { ... }`
  - [ ] **[Conditional rendering](https://yew.rs/docs/concepts/html/conditional-rendering)**: NOT YET IMPLEMENTED
  - [ ] **[List rendering](https://yew.rs/docs/concepts/html/lists)**: not tested
- **Others**
  - [ ] **[Tag classes](https://yew.rs/docs/concepts/html/classes)**: not tested
  - [ ] **[Inner HTML](https://github.com/yewstack/yew/tree/master/examples/inner_html)**: NOT IMPLEMENTED

## Syntax

### Tags

| `lmth!` syntax        | meaning                         | `html!` syntax              |
| --------------------- | ------------------------------- | -------------------------- |
| `! { ... }`           | Yew's fragment                  | `<> ... </>`               |
| `tag (attrs) { ... }` | Tag with attributes and content | `<tag attrs>{ ... }</tag>` |
| `tag (attrs)`         | Void tag with attributes        | `<tag attrs />`            |
| `tag { ... }`         | Tag with content                | `<tag>{ ... }</tag>`       |
| `tag`                 | Void tag with no attribute      | `<tag />`                  |

### Attributes

Attributes are separated by commas: `tag (attr: val, attr: val, ...) { ... }`

| `lmth!` syntax  | meaning                                | `html!` syntax  |
| --------------- | -------------------------------------- | -------------- |
| `attr: expr`    | Attribute with expression as value     | `attr={expr}`  |
| `attr: {code}`  | Attribute with code block as value     | `attr={code}`  |
| `attr="litstr"` | Attribute with literal string as value | `attr="litstr"`|
| `attr`          | Shorthand for `{attr}` in yew          | `{attr}`       |

### Content

| `lmth!` syntax  | meaning                   | `html!` syntax     |
| --------------- | ------------------------- | ----------------- |
| `{code}`        | Code as content           | `{code}`          |
| `"litstr"`      | Literal string as content | `"litstr"`        |
| `tag ...`       | Tag                       | corresponding tag |

## Example

Please refer to [GitHub repo's examples folder](https://github.com/Saplyn/yew-lmth/tree/main/examples).

```rust
use yew_lmth::lmth;

lmth! {
    div (class="container") {
       h1 { "Hello, world!" }
       button (onclick: handle_click()) { "Click me!" }
       img (src="https://yew.rs/img/logo.svg")
    }
}
```

will expands to:

```rust
yew::prelude::html! {
    <div class="container">
        <h1>{ "Hello, world!" }</h1>
        <button onclick={handle_click()}>{ "Click me!" }</button>
        <img src="https://yew.rs/img/logo.svg" />
    </div>
}
```
