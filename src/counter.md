# Counter: an interactive widget

*Topics: layout, input data, messages*

![Counter](screenshots/counter.png)

The last example was a bit boring. Lets get interactive!

```rust
# extern crate kas;
use kas::prelude::*;
use kas::widgets::{Button, column, format_label, row};

#[derive(Clone, Debug)]
struct Increment(i32);

fn counter() -> impl Widget<Data = ()> {
    let tree = column![
        format_label!("{}").align(AlignHints::CENTER),
        row![
            Button::label_msg("−", Increment(-1)),
            Button::label_msg("+", Increment(1)),
        ]
        .map_any(),
    ];

    tree.with_state(0)
        .on_message(|_, count, Increment(add)| *count += add)
}

fn main() -> kas::runner::Result<()> {
    env_logger::init();

    let theme = kas::theme::SimpleTheme::new();
    let mut app = kas::runner::Runner::with_theme(theme).build(())?;
    let _ = app.config_mut().font.set_size(24.0);
    let window = Window::new(counter(), "Counter").escapable();
    app.with(window).run()
}
```

## Preamble

#### Prelude

The [`kas::prelude`] includes a bunch of commonly-used, faily unambiguous stuff:
```rust
# extern crate kas;
use kas::prelude::*;
```

#### Impl trait

If you're new to Rust, you might find the following confusing:
```rust
# extern crate kas;
# use kas::prelude::*;
fn counter() -> impl Widget<Data = ()> {
    // ...
    # kas::widgets::Label::new("")
}
```
This is (return position) [impl trait](https://doc.rust-lang.org/stable/rust-by-example/trait/impl_trait.html), specifying that the [`Widget`] trait's associated type `Data` is `()`.
(We'll get back to this type `Data` in a bit.)


## Widgets

What is a widget? Simply a type implementing the [`Widget`] trait (or, depending on the context, an instance of such a type).

Widgets must implement the super-traits [`Layout`] and [`Tile`], both of which are object-safe (use [`Tile::as_tile`] to get a `&dyn Tile`). [`Widget`] is also object-safe, but only where its associated [`Widget::Data`] type is specified (see [Input Data](#input-data) below).

In this example we'll only use library widgets and macro-synthesized widgets; [custom widgets](custom-widget.md) will be covered later.

### Layout macros

Our user interface should be a widget tree: lets use a [`row!`] of buttons and a [`column!`] layout for the top-level UI tree:
```rust
# extern crate kas;
# use kas::prelude::*;
# use kas::widgets::{Adapt, AdaptWidget, Button, column, format_label, row};
# #[derive(Clone, Debug)]
# struct Increment(i32);
# fn counter() -> impl Widget<Data = ()> {
    let tree = column![
        format_label!("{}").align(AlignHints::CENTER),
        row![
            Button::label_msg("−", Increment(-1)),
            Button::label_msg("+", Increment(1)),
        ]
        .map_any(),
    ];
    # tree.with_state(0)
# }
```

[`row!`] and [`column!`] are deceptively simple macros which construct a column or row over other widgets. I say *deceptively* simple because a fair amount of these macro's functionality is hidden, such as constructing a label widget from a string literal and emulating the [`.align(..)`](`AdaptWidget::align`) and [`.map_any()`](`AdaptWidgetAny::map_any`) (see [Input Data](#input-data)) method calls we see here. Still, you *should* be able to ignore this complexity.


## Input Data

The [`Widget::Data`] type mentioned above is used to provide all Kas widgets with *input data*. This is passed into [`Events::update`] (called whenever the data may have changed) and to a number of event-handling methods.

Why? Most UIs need some form of mutable state. Some modern UI toolkits like [Iced](https://github.com/iced-rs/iced) and [Xilem](https://github.com/linebender/xilem) reconstruct their view tree (over a hidden widget tree) when this state changes; [egui](https://github.com/emilk/egui) goes even further and reconstructs the whole widget tree. Older stateful toolkits like GTK and Qt require binding widget properties or explicitly updating widgets. Kas finds a compromise between these models: widgets are stateful, yet derived from a common object and updated as required.

In our case, [`format_label!`] constructs a [`Text`] widget which formats its input data (an `i32`) to a `String` and displays that.

Since it would be inconvenient to require an entire UI tree to use the same input data, Kas provides some tools to map that data (or in Xilem/Druid terminology, view that data through a lens):

-   [`AdaptWidget::map`] takes a closure which can, for example, map a struct-reference to a struct-field-reference. (In fact this is effectively all it can do due to lifetime restrictions; anything more complex requires using [`Adapt`] or similar.)
-   [`AdaptWidgetAny::map_any`] simply discards its input, passing `&()` to its child.
-   [`Adapt`] stores a mutable value in the UI tree, passing this value to its child.
-   [Custom widgets](custom-widget.md) may store state in the UI tree and pass arbitrary references to children.

### Providing input data: Adapt

In this case, we'll use `()` as our top-level data and an [`Adapt`] node for the mutable state (the count). The [next chapter](sync-counter.md) will use top-level data instead.

The code:
```rust
# extern crate kas;
# use kas::prelude::*;
# use kas::widgets::{format_label, Adapt};
# #[derive(Clone, Debug)]
# struct Increment(i32);
# fn counter() -> impl Widget<Data = ()> {
# let tree = format_label!("{}");
    tree.with_state(0)
# }
```
calls [`AdaptWidget::with_state`] to construct an [`Adapt`] widget over `0` (with type `i32`).

A reference to this (i.e. `&i32`) is passed into our display widget (`format_label!("{}")`). Meanwhile,
we used `buttons.map_any()` to ignore this value and pass `&()` to the [`Button`] widgets.

## Messages

While *input data* gets state *into* widgets, *messages* let us get, well, messages *out* of widgets.

Any widget in the UI tree may post a message. While sometimes such messages have an intended recipient, often they are simply pushed to a message stack. Any widget above the source in the UI tree may handle messages (of known type).

In practice, message handling has three steps:

1.  Define a message type, in this case `Increment`. The only requirement of this type is that it supports `Debug`. (While we could in this case just use `i32`, using a custom type improves type safety and provides a better message in the log should any message go unhandled.)
2.  A widget (e.g. our buttons) pushes a message to the stack using [`EventCx::push`]. Many widgets provide convenience methods to do this, for example [`Button::label_msg`].
3.  Some widget above the sender in the UI tree retrieves the message using [`EventCx::try_pop`] and handles it somehow. [`Adapt::on_message`] provides a convenient way to write such a handler.

```rust
# extern crate kas;
# use kas::prelude::*;
# use kas::widgets::{format_label, Adapt};
# #[derive(Clone, Debug)]
# struct Increment(i32);
# fn counter() -> impl Widget<Data = ()> {
# let tree = format_label!("{}");
    tree.with_state(0)
        .on_message(|_, count, Increment(add)| *count += add)
# }
```

Aside: feel free to write your message emitters first and handlers later. If you miss a handler you will see a message like this in your log:
```text
[2025-09-10T14:38:06Z WARN  kas_core::erased] unhandled: Erased(Increment(1))
```
While the custom message types like `Increment` will not save you from *forgetting* to handle something, they will at least yield a comprehensible message in your log and prevent something else from handling the wrong message.

Should multiple messages use `enum` variants or discrete struct types? Either option works fine. Consider perhaps where the messages will be handled.


[`Widget`]: https://docs.rs/kas/latest/kas/trait.Widget.html
[`Layout`]: https://docs.rs/kas/latest/kas/trait.Layout.html
[`Tile`]: https://docs.rs/kas/latest/kas/trait.Tile.html
[`Tile::as_tile`]: https://docs.rs/kas/latest/kas/trait.Tile.html#method.as_tile
[`AdaptWidgetAny::map_any`]: https://docs.rs/kas/latest/kas/widgets/trait.AdaptWidgetAny.html#method.map_any
[`kas::prelude`]: https://docs.rs/kas/latest/kas/prelude/index.html
[`column!`]: https://docs.rs/kas/latest/kas/widgets/macro.column.html
[`row!`]: https://docs.rs/kas/latest/kas/widgets/macro.row.html
[`AdaptWidget::align`]: https://docs.rs/kas/latest/kas/widgets/trait.AdaptWidget.html#method.align
[`AdaptWidget::map`]: https://docs.rs/kas/latest/kas/widgets/trait.AdaptWidget.html#method.map
[`AdaptWidget::with_state`]: https://docs.rs/kas/latest/kas/widgets/trait.AdaptWidget.html#method.with_state
[`Widget::Data`]: https://docs.rs/kas/latest/kas/trait.Widget.html#associatedtype.Data
[`Events::update`]: https://docs.rs/kas/latest/kas/trait.Events.html#method.update
[`Text`]: https://docs.rs/kas/latest/kas/widgets/struct.Text.html
[`format_label!`]: https://docs.rs/kas/latest/kas/widgets/macro.format_label.html
[`Adapt`]: https://docs.rs/kas/latest/kas/widgets/struct.Adapt.html
[`EventCx::push`]: https://docs.rs/kas/latest/kas/event/struct.EventCx.html#method.push
[`EventCx::try_pop`]: https://docs.rs/kas/latest/kas/event/struct.EventCx.html#method.try_pop
[`Button`]: https://docs.rs/kas/latest/kas/widgets/struct.Button.html
[`Adapt::on_message`]: https://docs.rs/kas/latest/kas/widgets/struct.Adapt.html#method.on_message
[`Button::label_msg`]: https://docs.rs/kas/latest/kas/widgets/struct.Button.html#method.label_msg
