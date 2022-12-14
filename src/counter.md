# Counter: a simple widget

*Topics: prelude, `impl_scope!` and `#[widget]` macros, messages, [`Window`] trait*

![Counter](screenshots/counter.png)

Graphical User Interfaces have two main concerns:

1.  Presenting information visually
2.  Handling user input

## Prelude

```rust
use kas::prelude::*;
```
The [`kas::prelude`] includes a bunch of commonly-used, faily unambiguous stuff.

## Macros

### `impl_scope`

We must define widgets within an [`impl_scope!`] (required to support the
`#[widget]` macro we'll use later). This scope lets us define a type
(`struct Counter`) then use `impl Self` syntax:

```rust
use kas::prelude::*;
use kas::widgets::Label;

impl_scope! {
    #[derive(Debug)]
    struct Counter {
        display: Label<String>,
        count: i32,
    }

    // `impl Self` is equivalent to `impl Counter` here.
    // It's more useful when the type has generic parameters!
    impl Self {
        fn new(count: i32) -> Self {
            Counter {
                display: Label::from(count.to_string()),
                count,
            }
        }
    }
}
```

### `#[widget]`

To make `Counter` a widget, we need to implement the [`Widget`] trait. But
before you type `impl Widget for Self`, this trait is special:

-   The implementing type *must* use the `#[widget]` attribute macro; the
    [`impl_scope!`] allows this macro to read and edit all code within its block
-   All widgets need a "core" data field (with type `widget_core!()`)
-   Visual layout needs to be specified somehow
-   Fields which are widgets must use `#[widget]` for correct configuration

You can read the [`Widget`] trait docs for details, but we'll push ahead with
our example for now:
```rust
use kas::prelude::*;
use kas::widgets::Label;

impl_scope! {
    // We add the `#[widget]` attribute and use it to specify layout:
    #[widget{
        layout = column: [
            align(center): self.display,
            row: ["−", "+"],
        ];
    }]
    #[derive(Debug)]
    struct Counter {
        // We add field `core` and attribute `#[widget]`:
        core: widget_core!(),
        #[widget] display: Label<String>,
        count: i32,
    }
}
```

#### Properties and layout

For full documentation of the `#[widget]` attribute macro, see
[`kas::macros::widget`]. Here we will just summarise: the layout is a `column`
over the widget field `self.display` (with centre-alignment) and a row
comprising two [`TextButton`] widgets.

This layout syntax permits four types of item:

-   Another layout item, e.g. `align`, `row`, `column`
-   A field, e.g. `self.display`. This field must use `#[widget]`.
-   A string literal, e.g. `"Some label"`.
    (Unlike [`Label`], string literals do not use line wrapping.)
-   A widget constructing expression, e.g. `Label::new("blah blah blah")`.
    Syntax restriction: the expression must start with an upper-case latter.
    The type (i.e. [`Label`]) must be in scope.

#### Core

As is required by widgets, `Counter` has field `core: widget_core!()`. This
macro is just a marker substituted by the `#[widget]` macro. It either has type
[`CoreData`] or a generated type with the same `rect: Rect` and `id: WidgetId`
fields (which may be accessed directly).
The type always supports [`Debug`], [`Default`] and [`Clone`].

## Messages

KAS uses a fairly simple event-handling model. You can read [`kas::event`] docs,
but lets keep it simple:

-   Input events such as a key press or mouse click yield an [`Event`] which is
    sent down the widget tree, using a [`WidgetId`] to find the correct target.
-   The target widget either handles or discards the [`Event`]. Since this
    widget may not directly modify state outside itself, it returns a *message*.
-   The *message* travels back up the tree until some widget handles it via
    [`Widget::handle_message`]. (If not handled, it is simply discarded while a
    warning is printed to the log.)

In early versions of KAS (up to v0.10), the message was simply the event
handler's return value. Unfortunately this caused problems, including
difficulty converting message types and an inability to return multiple messages
simultaneously (useful e.g. to indicate the index of an item in a list).
Instead, we use a stack:
[`Widget::handle_event`] calls [`EventMgr::push_msg`] to push a message
and [`Widget::handle_message`] uses [`EventMgr::try_pop_msg`] to pop a message.

But what *is* a message? Nearly anything! It is simply an object of any type
supporting [`Debug`]. Use of built-in types like `()` or `i32` is possible but
considered bad practice (mostly due to poor log messages), thus the norm is to
use custom type definitions like this:
```rust
#[derive(Clone, Debug)]
struct Increment(i32);
```

We can now construct a [`TextButton`] which returns an `Increment` message when
pressed: `TextButton::new_msg("+", Increment(1))`.

### Buttons and message handling

Our last example didn't use real buttons. Lets add them, along with a message
handler:
```rust
use kas::prelude::*;
use kas::widgets::{Label, TextButton};

#[derive(Clone, Debug)]
struct Increment(i32);

impl_scope! {
    #[widget{
        layout = column: [
            align(center): self.display,
            // We construct buttons within our layout:
            row: [
                TextButton::new_msg("−", Increment(-1)),
                TextButton::new_msg("+", Increment(1)),
            ],
        ];
    }]
    #[derive(Debug)]
    struct Counter {
        core: widget_core!(),
        #[widget] display: Label<String>,
        count: i32,
    }

    // We set up a message handler.
    impl Widget for Self {
        fn handle_message(&mut self, mgr: &mut EventMgr, _: usize) {
            if let Some(Increment(incr)) = mgr.try_pop_msg() {
                // Since this handler runs on `Counter`, we can update self.count:
                self.count += incr;
                // Unfortunately, we must update self.display manually:
                *mgr |= self.display.set_string(self.count.to_string());
            }
        }
    }
}
```

Note: the method [`HasString::set_string`] called on `self.display` returns a
[`TkAction`] to notify when a redraw is required. This is passed to the
[`EventMgr`] with `*mgr |= action` (or [`EventState::send_action`]).

And, yes, it is not a good thing that we have to update `self.display` by hand
here. KAS has a *partial* solution to this which we'll see later.

## Window

Finally, since we want to use `Counter` as a window, we need to implement the
[`Window`] trait:
```rust,ignore
impl Window for Counter {
    fn title(&self) -> &str { "Counter" }
}
```

[The full code for this example can be found here](https://github.com/kas-gui/tutorials/blob/master/examples/counter.rs).

[`kas::prelude`]: https://docs.rs/kas/latest/kas/prelude/index.html
[`Clone`]: https://doc.rust-lang.org/stable/std/clone/trait.Clone.html
[`Debug`]: https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html
[`Default`]: https://doc.rust-lang.org/stable/std/default/trait.Default.html
[`Widget`]: https://docs.rs/kas/latest/kas/trait.Widget.html
[`Widget::handle_event`]: https://docs.rs/kas/latest/kas/trait.Widget.html#method.handle_event
[`Widget::handle_message`]: https://docs.rs/kas/latest/kas/trait.Widget.html#method.handle_message
[`Layout`]: https://docs.rs/kas/latest/kas/trait.Layout.html
[`impl_scope!`]: https://docs.rs/impl-tools/latest/impl_tools/macro.impl_scope.html
[impl-tools]: https://crates.io/crates/impl-tools
[`kas::macros::widget`]: https://docs.rs/kas/latest/kas/macros/attr.widget.html
[`CoreData`]: https://docs.rs/kas/latest/kas/struct.CoreData.html
[`TextButton`]: https://docs.rs/kas/latest/kas/widgets/struct.TextButton.html
[`TextButton::new_msg`]: https://docs.rs/kas/latest/kas/widgets/struct.TextButton.html#method.new_msg
[`EventMgr`]: https://docs.rs/kas/latest/kas/event/struct.EventMgr.html
[`EventMgr::push_msg`]: https://docs.rs/kas/latest/kas/event/struct.EventMgr.html#method.push_msg
[`EventMgr::try_pop_msg`]: https://docs.rs/kas/latest/kas/event/struct.EventMgr.html#method.try_pop_msg
[`HasString::set_string`]: https://docs.rs/kas/latest/kas/class/trait.HasString.html#tymethod.set_string
[`TkAction`]: https://docs.rs/kas/latest/kas/struct.TkAction.html
[`EventState::send_action`]: https://docs.rs/kas/latest/kas/event/struct.EventState.html#method.send_action
[`Window`]: https://docs.rs/kas/latest/kas/trait.Window.html
[`Label`]: https://docs.rs/kas/latest/kas/widgets/struct.Label.html
[`WidgetId`]: https://docs.rs/kas/latest/kas/struct.WidgetId.html
[`Event`]: https://docs.rs/kas/latest/kas/event/enum.Event.html
[`Event::on_activate`]: https://docs.rs/kas/latest/kas/event/enum.Event.html#method.on_activate
[`kas::event`]: https://docs.rs/kas/latest/kas/event/index.html
