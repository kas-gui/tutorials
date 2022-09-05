# Counter: a simple widget

*Topics: prelude, `impl_scope!` and `#[widget]` macros, messages, [`Window`] trait*

![Counter](screenshots/counter.png)

Graphical User Interfaces have two main concerns:

1.  Presenting information visually
2.  Handling user input

[The code for this example can be found here](https://github.com/kas-gui/tutorials/blob/master/examples/counter.rs). Lets dive in.

## Prelude

```rust
use kas::prelude::*;
use kas::widgets::{Label, TextButton};
```
The [`kas::prelude`] includes a bunch of commonly-used, faily unambiguous stuff.
Besides that, we only use a couple of widgets.

## Defining Counter with the help of macros

```rust
# use kas::prelude::*;
# use kas::widgets::{Label, TextButton};
# #[derive(Clone, Debug)] struct Increment(i32);
impl_scope! {
    #[widget{
        layout = column: [
            align(center): self.display,
            row: [
                TextButton::new_msg("−", Increment(-1)),
                TextButton::new_msg("+", Increment(1)),
            ],
        ];
    }]
    #[derive(Debug)]
    struct Counter {
        core: widget_core!(),
        #[widget]
        display: Label<String>,
        count: i32,
    }
}
```

### Macros

The above snippet includes five macro usages:

1.  [`impl_scope!`] is a proc-macro which:

    -   encompasses a type definition (enum/struct/type alias/union) and `impl` blocks
    -   provides `impl Self` syntax (`Self` expands to the type's name, with correct
        handling of generic parameters and bounds)
    -   supports some attribute macros with slightly non-standard behaviour

2.  The `#[widget { layout = ..; }]` attribute macro ([`kas::macros::widget`]) is used to
    implement [`Widget`] for `struct Counter`. Note that this is the only
    supported method of implementing the [`Widget`] trait family (including
    super traits). Further note that this macro may only be used within an `impl_scope!`.

3.  `#[derive(Debug)]` — you already know this.

4.  `widget_core!()` is a type generator. This is not a stand-alone macro, but a
    marker substituted by the `#[widget]` macro (2). Every widget must have one
    field with this type. The substition may be the [`CoreData`] type or may be
    a generated type with the same `rect: Rect` and `id: WidgetId` fields (which
    may be accessed directly). The type always supports [`Debug`], [`Default`]
    and [`Clone`].

5.  `#[widget] display: Label<String>` is also a marker supported by the outer
    `#[widget]` attribute macro (2). This attribute may appear on a field to
    mark this field as a child widget (and is thus configured correctly).

### Constructor and layout

```rust
# use kas::prelude::*;
# use kas::widgets::{Label, TextButton};
# #[derive(Clone, Debug)] struct Increment(i32);
impl_scope! {
    #[widget{
        layout = column: [
            align(center): self.display,
            row: [
                TextButton::new_msg("−", Increment(-1)),
                TextButton::new_msg("+", Increment(1)),
            ],
        ];
    }]
    #[derive(Debug)]
    struct Counter {
        core: widget_core!(),
        #[widget]
        display: Label<String>,
        count: i32,
    }

    impl Self {
        fn new(count: i32) -> Self {
            Counter {
                core: Default::default(),
                display: Label::from(count.to_string()),
                count,
            }
        }
    }
};
```

First note the simple constructor: `fn Counter::new`. (Reminder: `impl Self`
is expanded to `impl Counter` by the `impl_scope!` macro.)

Our widget's layout is here defined by the `layout = ..` parameter.
(An alternative is to implement [`Layout`] directly.)
The `layout` syntax is new, but should be easy to understand:

-   Our top layout is a `column` containing ...
-   ... the field `self.display`, annotated with `align(center)` ...
-   ... and a `row` of two [`TextButton`] widgets

In general, layout syntax is one of four things:

-   a keyword such as `column` or `align(..)`, followed by a `:` and some
    sub-layout (or list of sub-layouts)
-   `self.foo` where `foo` is a field implementing [`Layout`] (possibly but not necessarily a widget)
-   `Foo::new(..)` where `Foo` is a type name (starting upper-case), the type is
    a [`Widget`], and the whole expression `Foo::new(..)` constructs an object
    which coerces to `dyn Widget`
-   `"blah blah"` — a simple label

It's a *bit* more complex than this, but that should get you started. You can
read the reference here: [`kas::macros::widget`].

Aside: since KAS is a *stateful* UI system, the [`TextButton`] widgets in our
row must be stored *somewhere*, right? Yes: in the `widget_core!` field
(a generated type).

## Messages

### The message type

We skipped this type definition above:
```rust
#[derive(Clone, Debug)]
struct Increment(i32);
```
A simple tuple struct, used for our button messages. The [`Debug`] implementation
is required to send as a message; [`Clone`] is needed for the `new_msg` method.

Aside: we *could* simply use `i32` and forget the `Increment` type, but using a
named type provides useful documentation, especially in log messages.

### The message stack

Our button widgets are constructed using [`TextButton::new_msg`].
Referring to the [`TextButton`] documentation (or the source) we see that
this is equivalent to `TextButton::new("+").on_push(|mgr| mgr.push_msg(Increment(1)))`,
using the method [`EventMgr::push_msg`], which pushes to the "message stack".

KAS's [event-handling model is described here](https://docs.rs/kas/latest/kas/event/index.html#event-handling-model).
The relevant parts to our example are:

1.  On activation (via mouse click or keyboard), [`TextButton`]'s handler is called.
2.  This handler calls [`EventMgr::push_msg`] to push `Increment(1)` to
    the message stack.
3.  While the message stack is non-empty, [`Widget::handle_message`] is called
    on *each* ancestor of the `TextButton` widget.
4.  Our `Counter` should implement this method, using [`EventMgr::try_pop_msg`]
    to retrieve the message from the stack.

### Handling messages

Lets implement [`Widget::handle_message`] on `Counter`:
```rust
# use kas::prelude::*;
# use kas::widgets::{Label, TextButton};
# #[derive(Clone, Debug)] struct Increment(i32);
impl_scope! {
    #[widget{
        layout = column: [
            align(center): self.display,
            row: [TextButton::new_msg("−", Increment(-1)), TextButton::new_msg("+", Increment(1))],
        ];
    }]
    #[derive(Debug)]
    struct Counter {
        core: widget_core!(),
        #[widget]
        display: Label<String>,
        count: i32,
    }
    impl Widget for Self {
        fn handle_message(&mut self, mgr: &mut EventMgr, _: usize) {
            if let Some(Increment(incr)) = mgr.try_pop_msg() {
                self.count += incr;
                *mgr |= self.display.set_string(self.count.to_string());
            }
        }
    }
};
```

The method [`HasString::set_string`] returns a [`TkAction`] (to notify that a
redraw is required). [`TkAction`] is a `#[must_use`] type which should be fed
to [`EventState::send_action`], or equivalently: `*mgr |= action`.

#### Unhandled messages

Aside: what if we forgot to write our `handle_message` implementation and left
the message on the stack? Assuming you have a logger enabled, you'd see
something like this:
```sh
[2022-08-20T14:33:47Z DEBUG kas_core::event::manager::messages] push_msg: counter::main::Increment::Increment(1)
[2022-08-20T14:33:47Z WARN  kas_core::event::manager::messages] unhandled: counter::main::Increment::Increment(1)
```
Now perhaps you see why we defined our `Increment` type and didn't simply push an `i32`!

## Window

Windows must implement the [`Window`] trait. We could use
[`kas::widgets::dialog::Window`](https://docs.rs/kas/latest/kas/widgets/dialog/struct.Window.html),
but implementing [`Window`] on a custom widget is very easy:
```rust,ignore
impl Window for Self {
    fn title(&self) -> &str { "Counter" }
}
```

[`kas::prelude`]: https://docs.rs/kas/latest/kas/prelude/index.html
[`Clone`]: https://doc.rust-lang.org/stable/std/clone/trait.Clone.html
[`Debug`]: https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html
[`Default`]: https://doc.rust-lang.org/stable/std/default/trait.Default.html
[`Widget`]: https://docs.rs/kas/latest/kas/trait.Widget.html
[`Widget::handle_message`]: https://docs.rs/kas/latest/kas/trait.Widget.html#method.handle_message
[`Layout`]: https://docs.rs/kas/latest/kas/trait.Layout.html
[`impl_scope!`]: https://docs.rs/impl-tools/latest/impl_tools/macro.impl_scope.html
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
