# Counter: an interactive widget

*Topics: layout, input data, messages*

![Counter](screenshots/counter.png)

The last example was a bit boring. Lets get interactive!

```rust
# extern crate kas;
use kas::prelude::*;
use kas::widgets::{format_value, Adapt, Button};

#[derive(Clone, Debug)]
struct Increment(i32);

fn counter() -> impl Widget<Data = ()> {
    let tree = kas::column![
        align!(center, format_value!("{}")),
        kas::row![
            Button::label_msg("−", Increment(-1)),
            Button::label_msg("+", Increment(1)),
        ]
        .map_any(),
    ];

    Adapt::new(tree, 0).on_message(|_, count, Increment(add)| *count += add)
}

fn main() -> kas::app::Result<()> {
    env_logger::init();

    let theme = kas::theme::SimpleTheme::new().with_font_size(24.0);
    kas::app::Default::with_theme(theme)
        .build(())?
        .with(Window::new(counter(), "Counter"))
        .run()
}
```

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


## Layout

Our user interface should be a widget tree: lets use a column layout over the count and \[a row layout over the buttons\].
```rust
# extern crate kas;
# use kas::prelude::*;
# use kas::widgets::{format_value, Adapt, Button};
# #[derive(Clone, Debug)]
# struct Increment(i32);
# fn counter() -> impl Widget<Data = ()> {
let tree = kas::column![
    align!(center, format_value!("{}")),
    kas::row![
        Button::label_msg("−", Increment(-1)),
        Button::label_msg("+", Increment(1)),
    ]
    .map_any(),
];
# Adapt::new(tree, 0)
# }
```

### Layout macros

[`kas::column!`] and [`kas::row!`] are layout macros which, as the name suggests, construct a column/row over other widgets.

[`kas::align!`] is another layout macro. Above, the `kas::` prefix is skipped, *not* because `kas::align` was imported, *but* because layout macros (in this case [`kas::column!`]) have direct support for parsing and evaluating other layout macros. (If you wrote `kas::align!` instead the result would function identically but with slightly different code generation.)

Now, you could, if you prefer, import the layout macros: `use kas::{align, column, row};`. *However,*

-   (`std`) [`column!`](https://doc.rust-lang.org/stable/std/macro.column.html) is a *very* different macro. This can result in surprising error messages if you forget to import `kas::column`.
-   If you replace `kas::row!` with `row!` you will get a compile error: the layout macro parser cannot handle <code>.[map_any](https://docs.rs/kas/latest/kas/widgets/trait.AdaptWidgetAny.html#method.map_any)()</code>. `kas::row![..]` evaluates to a complete widget; `row![..]` as an embedded layout does not.


## Input data

So, you may have wondered what the [`Widget::Data`] type encountered above is about. All widgets in Kas are provided *input data* (via [`Events::update`]) when the UI is initialised and *whenever that data changes* (not strictly true as you'll see when we get to custom widgets).

The point is, a widget like [`Text`] is essentially a function `Fn(&A) -> String` where `&A` is *your input data*. [`format_value!`] is just a convenient macro to construct a [`Text`] widget.

Thus, `format_value!("{}")` is a [`Text`] widget which formats some input data to a `String`. But *what* input data?

### Providing input data: Adapt

There are three methods of providing *input data* to a UI:

-   Custom widgets (advanced topic)
-   Top-level app data (the `()` of `.build(())`; we'll be using this in the next chapter)
-   [`Adapt`] nodes

All widgets in Kas may store state (though some are not persistent, namely view widgets (another advanced topic)). [`Adapt`] is a widget which stores user-defined data and message handlers.

Thus,
```rust
# extern crate kas;
# use kas::prelude::*;
# use kas::widgets::{format_value, Adapt};
# #[derive(Clone, Debug)]
# struct Increment(i32);
# fn counter() -> impl Widget<Data = ()> {
# let tree = format_value!("{}");
Adapt::new(tree, 0)
# }
```
is a widget which wraps `tree`, providing it with *input data* of 0.

But to make this *do something* we need one more concept: *messages*.

### Mapping data

We should briefly justify `.map_any()` in our example: our [`Text`] widget expects input data (of type `i32`), while `Button::label_msg` constructs a <code>[Button](https://docs.rs/kas/latest/kas/widgets/struct.Button.html)\<[AccessLabel](https://docs.rs/kas/latest/kas/widgets/struct.AccessLabel.html)\></code> expecting data of type `()`.

The method <code>.[map_any](https://docs.rs/kas/latest/kas/widgets/trait.AdaptWidgetAny.html#method.map_any)()</code> maps the row of buttons to a new widget supporting (and ignoring) *any* input data.

We could instead use <code>Button::new([label_any](https://docs.rs/kas/latest/kas/widgets/fn.label_any.html)("+"))</code> which serves the same purpose, but ignoring that input data much further down the tree.


## Messages

Kas has a fairly simple event-handling model: **events** (like mouse clicks) and **input data** go *down* the tree, **messages** come back *up*. You can read more about this in [`kas::event`] docs.

When widgets receive an event, *often* this must be handled by some widget higher up the tree (an ancestor). For example, our "+" button must cause our [`Adapt`] widget to increment its state. To do that,

1.  We define a message type, `Increment`
2.  The button [`push`]es a message to the message stack
3.  Our [`Adapt`] widget uses [`try_pop`] to retrieve that message

Aside: widgets have an associated type `Data`. So why don't they also have an associated type `Message` (or `Msg` for short)? Early versions of Kas (up to v0.10) did in fact have an `Msg` type, but this had some issues: translating message types between child and parent widgets was a pain, and supporting multiple message types was even more of a pain (mapping to a custom enum), and the `Msg` type must be specified when using `dyn Widget`. Using a variadic (type-erased) message stack completely avoids these issues, and at worst you'll see an `unhandled` warning in the log. In contrast, compile-time typing of input data is considerably more useful and probably a little easier to deal with (the main nuisance being mapping input data to `()` for widgets like labels which don't use it).

### Message types

What *is* a message? Nearly anything: the type *must* support [`Debug`] and *should* have a unique name. Our example defines:
```rust
#[derive(Clone, Debug)]
struct Increment(i32);
```
Note that if your UI pushes a message to the stack but fails to handle it, you will get a warning message like this:
```text
[WARN  kas_core::erased] unhandled: counter::Increment::Increment(1)
```
Use of built-in types like `()` or `i32` is possible but considered bad practice (imagine if the above warning was just `unhandled: 1`).

### Buttons

This should be obvious: `Button::label_msg("+", Increment(1))` constructs a [`Button`] which pushes the message `Increment(1)` when pressed.

### Handling messages

Finally, we can handle our button click:
```rust
# extern crate kas;
# use kas::prelude::*;
# use kas::widgets::{format_value, Adapt};
# #[derive(Clone, Debug)]
# struct Increment(i32);
# fn counter() -> impl Widget<Data = ()> {
# let tree = format_value!("{}");
Adapt::new(tree, 0)
    .on_message(|_, count, Increment(add)| *count += add)
# }
```
[`Adapt::on_message`] calls our closure whenever an `Increment` message is pushed with a mutable reference to its state, `count`. After handling our message, [`Adapt`] will update its descendants with the new value of `count`, thus refreshing the label: `format_value!("{}"))`.

[`kas::prelude`]: https://docs.rs/kas/latest/kas/prelude/index.html
[`kas::column!`]: https://docs.rs/kas/latest/kas/macro.column.html
[`kas::row!`]: https://docs.rs/kas/latest/kas/macro.row.html
[`kas::align!`]: https://docs.rs/kas/latest/kas/macro.align.html
[`Widget::Data`]: https://docs.rs/kas/latest/kas/trait.Widget.html#associatedtype.Data
[`Events::update`]: https://docs.rs/kas/latest/kas/trait.Events.html#method.update
[`Text`]: https://docs.rs/kas/latest/kas/widgets/struct.Text.html
[`format_value!`]: https://docs.rs/kas/latest/kas/widgets/macro.format_value.html
[`Adapt`]: https://docs.rs/kas/latest/kas/widgets/struct.Adapt.html
[`kas::event`]: https://docs.rs/kas/latest/kas/event/index.html
[`push`]: https://docs.rs/kas/latest/kas/event/struct.EventCx.html#method.push
[`try_pop`]: https://docs.rs/kas/latest/kas/event/struct.EventCx.html#method.try_pop
[`Button`]: https://docs.rs/kas/latest/kas/widgets/struct.Button.html
[`Adapt::on_message`]: https://docs.rs/kas/latest/kas/widgets/struct.Adapt.html#method.on_message
