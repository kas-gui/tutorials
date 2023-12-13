# Calculator: make_widget and grid layout

*Topics: grid layout, access keys*

![Calculator](screenshots/calculator.png)

```rust
# extern crate kas;
use kas::event::NamedKey;
use kas::prelude::*;
use kas::widgets::{AccessLabel, Adapt, Button, EditBox};
use std::num::ParseFloatError;
use std::str::FromStr;

type Key = kas::event::Key<kas::event::SmolStr>;

fn key_button(label: &str) -> Button<AccessLabel> {
    let string = AccessString::from(label);
    let key = string.key().unwrap().clone();
    Button::label_msg(string, key)
}
fn key_button_with(label: &str, key: Key) -> Button<AccessLabel> {
    Button::label_msg(label, key.clone()).with_access_key(key)
}

fn calc_ui() -> impl Widget<Data = ()> {
    // We could use kas::widget::Text, but EditBox looks better.
    let display = EditBox::string(|calc: &Calculator| calc.display())
        .with_multi_line(true)
        .with_lines(3, 3)
        .with_width_em(5.0, 10.0);

    // We use map_any to avoid passing input data (not wanted by buttons):
    let buttons = kas::grid! {
        // Key bindings: C, Del
        (0, 0) => Button::label_msg("&clear", Key::Named(NamedKey::Clear))
            .with_access_key(NamedKey::Delete.into()),
        // Widget is hidden but has key binding.
        // TODO(opt): exclude from layout & drawing.
        (0, 0) => key_button_with("", NamedKey::Backspace.into()),
        (1, 0) => key_button_with("&÷", Key::Character("/".into())),
        (2, 0) => key_button_with("&×", Key::Character("*".into())),
        (3, 0) => key_button_with("&−", Key::Character("-".into())),
        (0, 1) => key_button("&7"),
        (1, 1) => key_button("&8"),
        (2, 1) => key_button("&9"),
        (3, 1..3) => key_button("&+"),
        (0, 2) => key_button("&4"),
        (1, 2) => key_button("&5"),
        (2, 2) => key_button("&6"),
        (0, 3) => key_button("&1"),
        (1, 3) => key_button("&2"),
        (2, 3) => key_button("&3"),
        (3, 3..5) => key_button_with("&=", NamedKey::Enter.into()),
        (0..2, 4) => key_button("&0"),
        (2, 4) => key_button("&."),
    }
    .map_any();

    Adapt::new(kas::column![display, buttons], Calculator::new())
        .on_message(|_, calc, key| calc.handle(key))
        .on_configure(|cx, _| {
            cx.disable_nav_focus(true);
            cx.enable_alt_bypass(true);
        })
}

fn main() -> kas::app::Result<()> {
    env_logger::init();

    let theme = kas_wgpu::ShadedTheme::new().with_font_size(16.0);
    kas::app::Default::with_theme(theme)
        .build(())?
        .with(Window::new(calc_ui(), "Calculator"))
        .run()
}

#[derive(Clone, Debug)]
struct Calculator {
    // ...
}

impl Calculator {
    fn new() -> Calculator {
        Calculator {
            // ...
        }
    }

    fn display(&self) -> String {
        // ...
        # String::new()
    }

    fn handle(&mut self, key: Key) {
        // ...
        # let _ = key;
    }
}
```

## The back-end: `Calculator`

First things first, lets define our backend, `Calculator`. It must have:

-   Internal state (fields)
-   A constructor (`new`)
-   Some type of output. A `display` function returning a `String` will do.
-   A handler for button presses. We'll just pass a [`Key`] to `fn handle`.

Fill out the implementation yourself or copy from the [full source](https://github.com/kas-gui/tutorials/blob/master/examples/calculator.rs).


## Access keys

To make the calculator keyboard-accessible, we'll use *access keys* (see more on [Wikipedia](https://en.wikipedia.org/wiki/Access_key) or [Windows app docs](https://learn.microsoft.com/en-us/windows/apps/design/input/access-keys)). Kas supports these via:

-   [`AccessString`] supports parsing things like `&File` as the label `File` with access key <kbd>F</kbd>
-   [`AccessLabel`] is a label widget over an [`AccessString`] which sends the message [`Activate`] when its access key is pressed. A [`Button`] widget will react to this message.
-   [`Button::with_access_key`] may be used to specify arbitrary access keys.

### Button helper fns

To make constructing buttons easier, we define some helper functions. (These facilitate defining the button *message* more than they do the *access keys*.)
```rust
# extern crate kas;
# use kas::text::AccessString;
# use kas::widgets::{AccessLabel, Button};
# type Key = kas::event::Key<kas::event::SmolStr>;
fn key_button(label: &str) -> Button<AccessLabel> {
    let string = AccessString::from(label);
    let key = string.key().unwrap().clone();
    Button::label_msg(string, key)
}
fn key_button_with(label: &str, key: Key) -> Button<AccessLabel> {
    Button::label_msg(label, key.clone()).with_access_key(key)
}
```

### Navigation focus and <kbd>Alt</kbd>-bypass

Normally, access keys are only active while holding <kbd>Alt</kbd>. To avoid this requirement we call [`enable_alt_bypass`]. Further, we disable <kbd>Tab</kbd> key navigation with [`disable_nav_focus`].
```rust
# extern crate kas;
# use kas::{Widget, widgets::{label_any, Adapt}};
# #[derive(Debug)]
# struct Calculator;
# impl Calculator {
#   fn new() -> Self { Calculator }
#   fn handle(&mut self, _key: ()) {}
# }
# fn ui() -> impl Widget<Data = ()> {
# let display = label_any("");
# let buttons = label_any("");
Adapt::new(kas::column![display, buttons], Calculator::new())
    .on_message(|_, calc, key| calc.handle(key))
    .on_configure(|cx, _| {
        cx.disable_nav_focus(true);
        cx.enable_alt_bypass(true);
    })
# }
```


## Grid layout

We already saw column and row layouts. This time, we'll use [`kas::grid!`] for layout.
```rust
# extern crate kas;
# use kas::event::NamedKey;
# use kas::prelude::*;
# use kas::widgets::{AccessLabel, Button};
# type Key = kas::event::Key<kas::event::SmolStr>;
# fn key_button(label: &str) -> Button<AccessLabel> {
#     let string = AccessString::from(label);
#     let key = string.key().unwrap().clone();
#     Button::label_msg(string, key)
# }
# fn key_button_with(label: &str, key: Key) -> Button<AccessLabel> {
#     Button::label_msg(label, key.clone()).with_access_key(key)
# }
# fn ui() -> impl Widget<Data = i32> {
let buttons = kas::grid! {
    // Key bindings: C, Del
    (0, 0) => Button::label_msg("&clear", Key::Named(NamedKey::Clear))
        .with_access_key(NamedKey::Delete.into()),
    // Widget is hidden but has key binding.
    // TODO(opt): exclude from layout & drawing.
    (0, 0) => key_button_with("", NamedKey::Backspace.into()),
    (1, 0) => key_button_with("&÷", Key::Character("/".into())),
    (2, 0) => key_button_with("&×", Key::Character("*".into())),
    (3, 0) => key_button_with("&−", Key::Character("-".into())),
    (0, 1) => key_button("&7"),
    (1, 1) => key_button("&8"),
    (2, 1) => key_button("&9"),
    (3, 1..3) => key_button("&+"),
    (0, 2) => key_button("&4"),
    (1, 2) => key_button("&5"),
    (2, 2) => key_button("&6"),
    (0, 3) => key_button("&1"),
    (1, 3) => key_button("&2"),
    (2, 3) => key_button("&3"),
    (3, 3..5) => key_button_with("&=", NamedKey::Enter.into()),
    (0..2, 4) => key_button("&0"),
    (2, 4) => key_button("&."),
}
.map_any();
# buttons
# }
```

Worth noting is our hidden `Backspace` button. This is just another cell, but hidden under the `clear` button. Yes, this is a sub-optimal hack (the widget is still sized and drawn); it works but might see a less hacky solution in the future.

Again, we must use [`map_any`] to make our buttons (input `Data = ()`) compatible with the parent UI element (input `Data = Calculator`).


[`Key`]: https://docs.rs/kas/latest/kas/event/enum.Key.html
[`AccessString`]: https://docs.rs/kas/latest/kas/text/struct.AccessString.html
[`AccessLabel`]: https://docs.rs/kas/latest/kas/widgets/struct.AccessLabel.html
[`Button`]: https://docs.rs/kas/latest/kas/widgets/struct.Button.html
[`Button::with_access_key`]: https://docs.rs/kas/latest/kas/widgets/struct.Button.html#method.with_access_key
[`Activate`]: https://docs.rs/kas/latest/kas/messages/struct.Activate.html
[`disable_nav_focus`]: https://docs.rs/kas/latest/kas/event/struct.ConfigCx.html#method.disable_nav_focus
[`enable_alt_bypass`]: https://docs.rs/kas/latest/kas/event/struct.EventState.html#method.enable_alt_bypass
[`kas::grid!`]: https://docs.rs/kas/latest/kas/macro.grid.html
[`map_any`]: https://docs.rs/kas/latest/kas/widgets/trait.AdaptWidgetAny.html#method.map_any
