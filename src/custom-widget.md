# Counter: a simple widget

*Topics: custom widgets*

![Counter](screenshots/counter.png)

Here we rewrite the counter as a custom widget. There's no reason to do so for this particular case, but it serves as a simple example to the topic.

```rust
# extern crate kas;
use kas::prelude::*;
use kas::widgets::{format_value, AccessLabel, Button, Row, Text};

#[derive(Clone, Debug)]
struct Increment(i32);

impl_scope! {
    #[widget{
        layout = column![
            align!(center, self.display),
            self.buttons,
        ];
    }]
    struct Counter {
        core: widget_core!(),
        #[widget(&self.count)]
        display: Text<i32, String>,
        #[widget]
        buttons: Row<Button<AccessLabel>>,
        count: i32,
    }
    impl Self {
        fn new(count: i32) -> Self {
            Counter {
                core: Default::default(),
                display: format_value!("{}"),
                buttons: Row::new([
                    Button::label_msg("-", Increment(-1)),
                    Button::label_msg("+", Increment(1)),
                ]),
                count,
            }
        }
    }
    impl Events for Self {
        type Data = ();

        fn handle_messages(&mut self, cx: &mut EventCx, data: &()) {
            if let Some(Increment(incr)) = cx.try_pop() {
                self.count += incr;
                cx.update(self.as_node(data));
            }
        }
    }
}

fn main() -> kas::app::Result<()> {
    env_logger::init();

    let theme = kas::theme::SimpleTheme::new().with_font_size(24.0);
    kas::app::Default::with_theme(theme)
        .build(())?
        .with(Window::new(Counter::new(0), "Counter"))
        .run()
}
```

## Macros

### `impl_scope`

[`impl_scope!`] is a macro from [impl-tools]. This macro wraps a type definition and `impl`s on that type. (Unfortunately it also inhibits `rustfmt` from working, [for now](https://github.com/rust-lang/rustfmt/pull/5538).)  Here, it serves two purposes:

1.  `impl Self` syntax (not important here, but much more useful on structs with generics)
2.  To support the [`#[widget]`][attr-widget] attribute-macro. This attribute-macro is a Kas extension to [`impl_scope!`], and can act on anything within that scope (namely, it will check existing impls of [`Layout`], [`Events`] and [`Widget`], reading definitions of associated `type Data`, injecting certain missing methods into these impls, and write new impls).

### `#[widget]`

The [`#[widget]`][attr-widget] attribute-macro is used to implement the [`Widget`] trait. *This is the only supported way to implement [`Widget`].* There are a few parts to this.

**First**, we must apply [`#[widget]`][attr-widget] to the struct. (The `layout = ...;` argument (and `{ ... }` braces) are optional; some other arguments might also occur here.)
```ignore
    #[widget{
        layout = column![
            align!(center, self.display),
            self.buttons,
        ];
    }]
```

**Second**, all widgets must have "core data". This *might* be an instance of [`CoreData`] or *might* be some custom generated struct (but with the same public `rect` and `id` fields and constructible via [`Default`]). We *must* provide a field of type `widget_core!()`.
```ignore
        core: widget_core!(),
```

**Third**, any fields which are child widgets must be annotated with `#[widget]`. (This enables them to be configured and updated.)

We can use this attribute to configure the child widget's input data too: in this case, `display` is passed `&self.count`. Beware only that there is no automatic update mechanism: when mutating a field used as input data it may be necessary to explicitly update the affected widget(s) (see the note after the fourth step below).
```rust
# extern crate kas;
# use kas::impl_scope;
# use kas::widgets::{AccessLabel, Button, Row, Text};
# impl_scope! {
#     #[widget{
#         Data = ();
#         layout = "";
#     }]
    struct Counter {
        core: widget_core!(),
        #[widget(&self.count)]
        display: Text<i32, String>,
        #[widget]
        buttons: Row<Button<AccessLabel>>,
        count: i32,
    }
# }
```

**Fourth**, the input `Data` type to our `Counter` widget must be specified somewhere. In our case, we specify this by implementing [`Events`]. (If this trait impl was omitted, you could write `Data = ();` as an argument to [`#[widget]`][attr-widget].)
```rust
# extern crate kas;
# use kas::prelude::*;
# #[derive(Clone, Debug)]
# struct Increment(i32);
# impl_scope! {
#     #[widget{
#         layout = "";
#     }]
#     struct Counter {
#         core: widget_core!(),
#         count: i32,
#     }
    impl Events for Self {
        type Data = ();

        fn handle_messages(&mut self, cx: &mut EventCx, data: &()) {
            if let Some(Increment(incr)) = cx.try_pop() {
                self.count += incr;
                cx.update(self.as_node(data));
            }
        }
    }
# }
```

Notice here that after mutating `self.count` we call `cx.update(self.as_node(data))` in order to update `self` (and all children recursively). (In this case it would suffice to update only `display`, e.g. via `cx.update(self.display.as_node(&self.count))`, if you prefer to trade complexity for slightly more efficient code.)

**Fifth**, we must specify widget layout somehow. There are two main ways of doing this: implement [`Layout`] or use the `layout` argument of [`#[widget]`][attr-widget]. To recap, we use:
```ignore
    #[widget{
        layout = column![
            align!(center, self.display),
            self.buttons,
        ];
    }]
```
This is macro-parsed layout syntax (not real macros). Don't use [`kas::column!`] here; it won't know what `self.display` is!

Don't worry about remembering each step; macro diagnostics should point you in the right direction. Detection of fields which are child widgets is however imperfect (nor can it be), so try to at least remember to apply `#[widget]` attributes.

### Aside: child widget type

Our `Counter` has two (explicit) child widgets, and we must specify the type of each:
```rust
# extern crate kas;
# use kas::impl_scope;
# use kas::widgets::{AccessLabel, Button, Row, Text};
# impl_scope! {
#     #[widget{
#         Data = ();
#         layout = "";
#     }]
#     struct Counter {
#         core: widget_core!(),
        #[widget(&self.count)]
        display: Text<i32, String>,
        #[widget]
        buttons: Row<Button<AccessLabel>>,
#         count: i32,
#     }
# }
```
Here, this is no problem (though note that we used `Row::new([..])` not `kas::row![..]` specifically to have a known widget type). In other cases, widget types can get hard (or even impossible) to write.

It would therefore be nice if we could just write `impl Widget<Data = ()>` in these cases and be done. Alas, Rust does not support this. We are not completely without options however:

-   We could define our `buttons` directly within `layout` instead of as a field. Alas, this doesn't work when passing a field as input data (as used by `display`), or when code must refer to the child by name.
-   We could box the widget with `Box<dyn Widget<Data = ()>>`. (This is what the `layout` syntax does for embedded widgets.)
-   The [`impl_anon!`] macro *does* support `impl Trait` syntax. The required code is unfortunately a bit hacky (hidden type generics) and might sometimes cause issues.
-   It looks likely that Rust will stabilise support for [`impl Trait` in type aliases](https://doc.rust-lang.org/nightly/unstable-book/language-features/type-alias-impl-trait.html) "soon". This requires writing a type-def outside of the widget definition but is supported in nightly:

    ```rust,ignore
    type MyButtons = impl Widget<Data = ()>;
    ```

### Aside: uses

Before Kas 0.14, *all* widgets were custom widgets. (Yes, this made simple things hard.)

In the future, custom widgets *might* become obsolete, or might at least change significantly.

But for now, custom widgets still have their uses:

-   Anything with a custom [`Layout`] implementation. E.g. if you want some custom graphics, you can either use [`kas::resvg::Canvas`] or a custom widget.
-   Child widgets as named fields allows direct read/write access on these widgets. For example, instead of passing a [`Text`] widget the count to display via input data, we *could* use a simple [`Label`] widget and re-write it every time `count` changes.
-   `Adapt` is the "standard" way of storing local state, but as seen here custom widgets may also do so, and you may have good reasons for this (e.g. to provide different data to different children without lots of mapping).
-   Since *input data* is a new feature, there are probably some cases it doesn't support yet. One notable example is anything requring a lifetime.

[`impl_scope!`]: https://docs.rs/impl-tools/latest/impl_tools/macro.impl_scope.html
[`impl_anon!`]: https://docs.rs/impl-tools/latest/impl_tools/macro.impl_anon.html
[attr-widget]: https://docs.rs/kas/latest/kas/attr.widget.html
[`Widget`]: https://docs.rs/kas/latest/kas/trait.Widget.html
[`Events`]: https://docs.rs/kas/latest/kas/trait.Events.html
[`kas::column!`]: https://docs.rs/kas/latest/kas/macro.column.html
[`Default`]: https://doc.rust-lang.org/stable/std/default/trait.Default.html
[`Layout`]: https://docs.rs/kas/latest/kas/trait.Layout.html
[impl-tools]: https://crates.io/crates/impl-tools
[`CoreData`]: https://docs.rs/kas/latest/kas/struct.CoreData.html
[`Label`]: https://docs.rs/kas/latest/kas/widgets/struct.Label.html
[`Text`]: https://docs.rs/kas/latest/kas/widgets/struct.Text.html
[`kas::resvg::Canvas`]: https://docs.rs/kas/latest/kas/resvg/struct.Canvas.html
