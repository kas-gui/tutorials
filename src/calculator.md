# Calculator: make_widget and grid layout

![Calculator](screenshots/calculator.png)

The previous tutorial built a custom widget with `derive(Widget)`, used
`layout(column)` and used a message for event handling.
This tutorial follows on from there, introducing the `make_widget!` macro, using
`layout(grid)`, and some more message-based event handling.

We use this to build a simple but fully functional calculator, including the
expected keyboard bindings, in under 200 lines of code.


## The make_widget macro

If you thought in the last tutorial that having to derive a whole custom widget
just for simple layout and event handling was a bit much, you're not alone. The
 `make_widget!` macro makes this much easier, but does have some rough edges.

Looking back at our [`Counter` widget](counter.md#implementing-widget), there
are a few things which are either "boring details of a widget" or redundant:

-   `#[derive(Debug, Widget)]`: all widgets must have these derives
-   the `#[widget_core]` and `#[layout_data]` fields
-   declaring the field types, then constructing in a separate function
-   having to write the type when this could be deduced
-   having to *name* a field we never access

The `make_widget!` macro addresses *all* of the above. But be warned, Rust does
not support type-inference on fields so the macro has to emulate this with
generics, and that has its rough edges (including atrocious error messages).

Enough talk, lets see it in action.

### Buttons

A calculator needs a lot of buttons, and we'll use the `grid` layout with
`make_widget`:
```rust
use kas::prelude::*;
use kas::widgets::TextButton;
use kas::event::VirtualKeyCode as VK;

# #[derive(Clone, Debug, VoidMsg)]
# enum Key {
#    Clear,
#    Divide,
#    Multiply,
#    Subtract,
#    Add,
#    Equals,
#    Char(char),
# }
let buttons = make_widget! {
    #[layout(grid)]
    #[handler(msg = Key)]
    #[widget(config=noauto)]
    struct {
        #[widget(col = 0, row = 0)]
        _ = TextButton::new_msg("&clear", Key::Clear).with_keys(&[VK::Delete]),
        #[widget(col = 1, row = 0)]
        _ = TextButton::new_msg("&÷", Key::Divide).with_keys(&[VK::Slash]),
        #[widget(col = 2, row = 0)]
        _ = TextButton::new_msg("&×", Key::Multiply).with_keys(&[VK::Asterisk]),
        #[widget(col = 3, row = 0)]
        _ = TextButton::new_msg("&−", Key::Subtract),
        #[widget(col = 0, row = 1)]
        _ = TextButton::new_msg("&7", Key::Char('7')),
        #[widget(col = 1, row = 1)]
        _ = TextButton::new_msg("&8", Key::Char('8')),
        #[widget(col = 2, row = 1)]
        _ = TextButton::new_msg("&9", Key::Char('9')),
        #[widget(col = 3, row = 1, rspan = 2, align = stretch)]
        _ = TextButton::new_msg("&+", Key::Add),
        #[widget(col = 0, row = 2)]
        _ = TextButton::new_msg("&4", Key::Char('4')),
        #[widget(col = 1, row = 2)]
        _ = TextButton::new_msg("&5", Key::Char('5')),
        #[widget(col = 2, row = 2)]
        _ = TextButton::new_msg("&6", Key::Char('6')),
        #[widget(col = 0, row = 3)]
        _ = TextButton::new_msg("&1", Key::Char('1')),
        #[widget(col = 1, row = 3)]
        _ = TextButton::new_msg("&2", Key::Char('2')),
        #[widget(col = 2, row = 3)]
        _ = TextButton::new_msg("&3", Key::Char('3')),
        #[widget(col = 3, row = 3, rspan = 2, align = stretch)]
        _ = TextButton::new_msg("&=", Key::Equals).with_keys(&[VK::Return, VK::NumpadEnter]),
        #[widget(col = 0, row = 4, cspan = 2)]
        _ = TextButton::new_msg("&0", Key::Char('0')),
        #[widget(col = 2, row = 4)]
        _ = TextButton::new_msg("&.", Key::Char('.')),
    }
    impl kas::WidgetConfig {
        fn configure(&mut self, mgr: &mut Manager) {
            // Enable key bindings without Alt held:
            mgr.enable_alt_bypass(true);
        }
    }
};
```

Here, you see, we didn't need to use `#[derive(Debug, Widget)]` since the
`make_widget` macro does it for us. We also didn't need to mention the core or
layout-data fields. None of the buttons are named (`_` is used instead) or typed
since we never refer to them again and the type can be inferred.
Also notice that the `struct` type is unnamed, and similarly the following
`impl` block doesn't name type: `make_widget` creates an anonymous struct.
(Since this is all implemented in macros and Rust does not natively support
anonymous items or untyped struct fields, implementation details may sometimes
slip through, especially in error messages.)

#### Grid layout

This is pretty simple to use: `#[layout(grid)]`, and specify `col` and `row` in
the `#[widget]` attribute. The number of rows and columns is auto-detected.
Both `row` and `col` are optional, with the default value 0.

In a few cases we see spans: e.g. `cspan = 2` implies that two columns are
spanned, starting from the column selected by `col`. Both `rspan` and `cspan`
default to 1 if not specified.

#### Key bindings

Like several other GUI toolkits, KAS allows shortcut keys to be specified
through the label, e.g. `&clear` binds the C key. (Usually this method of
binding is only seen in menus, but in KAS it can be used on most "activatable"
controls.) [`TextButton`] allows additional keys can be specified through
[`TextButton::with_keys`].

Both the above types of bindings are usually only accessible while the Alt key
is held — the difference being that those specified through a label also
underline the letter/symbol following the `&`.

In a calculator, it is desirable that these keys are accessible without Alt
held. [`Manager::enable_alt_bypass`] enables this (for the whole window).
We could potentially call this method elsewhere, e.g. in an event handler, but
we do so in the [`WidgetConfig::configure`] method. (Notice how we used
`#[widget(config=noauto)]` to opt out of deriving [`WidgetConfig`]).

#### The Key message

One last thing to notice from the above snippet is that each button returns
some variant of the `Key` enum as its message when pressed. We should go ahead
and define this:
```rust
# use kas::prelude::*;
#[derive(Clone, Debug, VoidMsg)]
enum Key {
    Clear,
    Divide,
    Multiply,
    Subtract,
    Add,
    Equals,
    Char(char),
}
```
The only thing of note here is that `Key` uses `derive(VoidMsg)`: this is a
macro which implements `From<VoidMsg>` for the type. All message types should
do this.


## Building our calculator

Now, lets put our buttons in a calculator:
```rust
# use kas::prelude::*;
# use kas::widgets::{EditBox, TextButton, Window};
# use kas::event::VirtualKeyCode as VK;
# #[derive(Debug)]
# struct Calculator;
# impl Calculator {
#    fn new() -> Self { Calculator }
#    fn display(&self) -> String { String::new() }
#    fn handle(&mut self, _msg: Key) -> bool { false }
# }
# #[derive(Clone, Debug, VoidMsg)]
# enum Key {
#    Clear,
#    Divide,
#    Multiply,
#    Subtract,
#    Add,
#    Equals,
#    Char(char),
# }
fn main() -> Result<(), kas::shell::Error> {
    env_logger::init();

    // hidden: let buttons = ...;
#    let buttons = make_widget! {
#        #[layout(grid)]
#        #[handler(msg = Key)]
#        #[widget(config=noauto)]
#        struct {
#            #[widget(col = 0, row = 0)]
#            _ = TextButton::new_msg("&clear", Key::Clear).with_keys(&[VK::Delete]),
#            #[widget(col = 1, row = 0)]
#            _ = TextButton::new_msg("&÷", Key::Divide).with_keys(&[VK::Slash]),
#            #[widget(col = 2, row = 0)]
#            _ = TextButton::new_msg("&×", Key::Multiply).with_keys(&[VK::Asterisk]),
#            #[widget(col = 3, row = 0)]
#            _ = TextButton::new_msg("&−", Key::Subtract),
#            #[widget(col = 0, row = 1)]
#            _ = TextButton::new_msg("&7", Key::Char('7')),
#            #[widget(col = 1, row = 1)]
#            _ = TextButton::new_msg("&8", Key::Char('8')),
#            #[widget(col = 2, row = 1)]
#            _ = TextButton::new_msg("&9", Key::Char('9')),
#            #[widget(col = 3, row = 1, rspan = 2, align = stretch)]
#            _ = TextButton::new_msg("&+", Key::Add),
#            #[widget(col = 0, row = 2)]
#            _ = TextButton::new_msg("&4", Key::Char('4')),
#            #[widget(col = 1, row = 2)]
#            _ = TextButton::new_msg("&5", Key::Char('5')),
#            #[widget(col = 2, row = 2)]
#            _ = TextButton::new_msg("&6", Key::Char('6')),
#            #[widget(col = 0, row = 3)]
#            _ = TextButton::new_msg("&1", Key::Char('1')),
#            #[widget(col = 1, row = 3)]
#            _ = TextButton::new_msg("&2", Key::Char('2')),
#            #[widget(col = 2, row = 3)]
#            _ = TextButton::new_msg("&3", Key::Char('3')),
#            #[widget(col = 3, row = 3, rspan = 2, align = stretch)]
#            _ = TextButton::new_msg("&=", Key::Equals).with_keys(&[VK::Return, VK::NumpadEnter]),
#            #[widget(col = 0, row = 4, cspan = 2)]
#            _ = TextButton::new_msg("&0", Key::Char('0')),
#            #[widget(col = 2, row = 4)]
#            _ = TextButton::new_msg("&.", Key::Char('.')),
#        }
#        impl kas::WidgetConfig {
#            fn configure(&mut self, mgr: &mut Manager) {
#                // Enable key bindings without Alt held:
#                mgr.enable_alt_bypass(true);
#            }
#        }
#    };

    let content = make_widget! {
        #[layout(column)]
        #[handler(msg = VoidMsg)]
        struct {
            #[widget] display: impl HasString = EditBox::new("0").editable(false).multi_line(true),
            #[widget(handler = handle_button)] buttons -> Key = buttons,
            calc: Calculator = Calculator::new(),
        }
        impl {
            fn handle_button(&mut self, mgr: &mut Manager, msg: Key) -> Response<VoidMsg> {
                if self.calc.handle(msg) {
                    *mgr |= self.display.set_string(self.calc.display());
                }
                Response::None
            }
        }
    };
    let window = Window::new("Calculator", content);

    let theme = kas::theme::FlatTheme::new().with_font_size(16.0);
    kas::shell::Toolkit::new(theme)?.with(window)?.run()
}
```
By now, most of this code should be clear enough, but a few things are worth
mentioning:

-   We use an `EditBox` instead of a `Label` for our display
-   Instead of giving `display` an explicit type, we write `display: impl HasString`.
    This mirrors Rust's "argument position impl trait" (APIT) syntax, and just
    means that `display` has some unnamed type which implements `HasString`.
-   `buttons -> Key` is a different kind of type-restriction, only usable on
    widgets, which says that the type is some widget with message type `Key`

The calculator logic itself is pushed into the `Calculator` struct, with
interaction limited to `new`, `handle(msg)` and `display` methods. Implementing
this is left as an exercise to the user (you have 106 lines left to fit within
our arbitrary goal of "under 200 lines").


## Conclusion

The full code for our example [can be found here](https://github.com/kas-gui/tutorials/blob/master/examples/calculator.rs).
Run it with:
```sh
cargo run --example calculator
```

We introduced the `grid` layout, the `make_widget` macro and implemented a
significantly more complex app than the previous tutorial's counter.

This tutorial series has now introduced all three of KAS's macros, but without
refering to an API reference. If you've been using Rust (or other programming
languages) for a while you probably know why that is: there isn't a standard way
to document macros. What there *is* is the [`kas::macros`] module documentation,
but, like most macro documentation, it is more "by example" than a true
reference.

[`TextButton`]: https://docs.rs/kas/latest/kas/widget/struct.TextButton.html
[`TextButton::with_keys`]: https://docs.rs/kas/latest/kas/widget/struct.TextButton.html#method.with_keys
[`Manager::enable_alt_bypass`]: https://docs.rs/kas/latest/kas/event/struct.Manager.html#method.enable_alt_bypass
[`WidgetConfig::configure`]: https://docs.rs/kas/latest/kas/trait.WidgetConfig.html#method.configure
[`WidgetConfig`]: https://docs.rs/kas/latest/kas/trait.WidgetConfig.html
[`kas::macros`]: https://docs.rs/kas/latest/kas/macros/index.html
