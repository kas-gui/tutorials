# Calculator: make_widget and grid layout

*Topics: grid layout, accelerator bindings*

![Calculator](screenshots/calculator.png)

The full example [can be found here](https://github.com/kas-gui/tutorials/blob/master/examples/calculator.rs).
Abbreviated code is listed below.
```rust
use kas::event::{Command, VirtualKeyCode as VK};
use kas::prelude::*;
use kas::widgets::{EditBox, TextButton};

#[derive(Clone, Debug)]
enum Key {
    Clear, DelBack, Divide, Multiply, Subtract, Add, Equals, Char(char),
}

fn main() -> kas::shell::Result<()> {
    env_logger::init();

    impl_scope! {
        #[widget{
            layout = grid: {
                0, 0: TextButton::new_msg("&clear", Key::Clear).with_keys(&[VK::Delete]);
                1, 0: TextButton::new_msg("&÷", Key::Divide).with_keys(&[VK::Slash]);
                2, 0: TextButton::new_msg("&×", Key::Multiply).with_keys(&[VK::Asterisk]);
                3, 0: TextButton::new_msg("&−", Key::Subtract);
                0, 1: TextButton::new_msg("&7", Key::Char('7'));
                1, 1: TextButton::new_msg("&8", Key::Char('8'));
                2, 1: TextButton::new_msg("&9", Key::Char('9'));
                3, 1..3: TextButton::new_msg("&+", Key::Add);
                0, 2: TextButton::new_msg("&4", Key::Char('4'));
                1, 2: TextButton::new_msg("&5", Key::Char('5'));
                2, 2: TextButton::new_msg("&6", Key::Char('6'));
                0, 3: TextButton::new_msg("&1", Key::Char('1'));
                1, 3: TextButton::new_msg("&2", Key::Char('2'));
                2, 3: TextButton::new_msg("&3", Key::Char('3'));
                3, 3..5:  TextButton::new_msg("&=", Key::Equals)
                    .with_keys(&[VK::Return, VK::NumpadEnter]);
                0..2, 4: TextButton::new_msg("&0", Key::Char('0'));
                2, 4: TextButton::new_msg("&.", Key::Char('.'));
            };
        }]
        #[derive(Debug, Default)]
        struct Buttons(widget_core!());
    };

    impl_scope! {
        #[impl_default]
        #[widget{
            layout = column: [
                self.display,
                Buttons::default(),
            ];
        }]
        #[derive(Debug)]
        struct CalcUI {
            core: widget_core!(),
            #[widget] display: EditBox = EditBox::new("0")
                .with_editable(false)
                .with_multi_line(true)
                .with_lines(3, 3)
                .with_width_em(5.0, 10.0),
            calc: Calculator = Calculator::new(),
        }
        impl Widget for Self {
            fn configure(&mut self, mgr: &mut ConfigMgr) {
                mgr.disable_nav_focus(true);

                // Enable key bindings without Alt held:
                mgr.enable_alt_bypass(self.id_ref(), true);

                mgr.register_nav_fallback(self.id());
            }

            fn handle_event(&mut self, mgr: &mut EventMgr, event: Event) -> Response {
                match event {
                    Event::Command(Command::DelBack) => {
                        if self.calc.handle(Key::DelBack) {
                            *mgr |= self.display.set_string(self.calc.display());
                        }
                        Response::Used
                    }
                    _ => Response::Unused,
                }
            }

            fn handle_message(&mut self, mgr: &mut EventMgr, _: usize) {
                if let Some(msg) = mgr.try_pop_msg::<Key>() {
                    if self.calc.handle(msg) {
                        *mgr |= self.display.set_string(self.calc.display());
                    }
                }
            }
        }
        impl Window for Self {
            fn title(&self) -> &str { "Calculator" }
        }
    }

    let theme = kas::theme::ShadedTheme::new().with_font_size(16.0);
    kas::shell::Toolkit::new(theme)?
        .with(CalcUI::default())?
        .run()
}

#[derive(Clone, Debug)]
struct Calculator {
    // ...
}

impl Calculator {
    fn new() -> Calculator {
        Calculator {}
    }

    fn display(&self) -> String {
        todo!()
    }

    // return true if display changes
    fn handle(&mut self, key: Key) -> bool {
        todo!()
    }
}
```

## Button grid

Q: What does a calculator have that a counter doesn't?\
A: More buttons!

We define these within `layout = grid: { ... }`. Each cell has syntax
`cols, rows: widget;` where `cols` and `rows` are defined by a range or an
index (where `n` expands to `n..n+1`).

## Key bindings

Standard UIs allow navigation via the <kbd>Tab</kbd> key. We disable this
completely ([`ConfigMgr::disable_nav_focus`]).

Instead, we give each button one or more accelerator keys. These are
key shortcuts usually used on menus, with the <kbd>Alt</kbd> key held.
To avoid requiring <kbd>Alt</kbd>, we enable "alt bypass" mode
([`ConfigMgr::enable_alt_bypass`]).

Finally, we assign our widget as the "nav fallback" ([`ConfigMgr::register_nav_fallback`])
and define a handler for the <kbd>Backspace</kbd> key (in [`Widget::handle_event`]),
which does not have a button.

```rust,ignore
impl Widget for Self {
    fn configure(&mut self, mgr: &mut ConfigMgr) {
        mgr.disable_nav_focus(true);

        // Enable key bindings without Alt held:
        mgr.enable_alt_bypass(self.id_ref(), true);

        mgr.register_nav_fallback(self.id());
    }

    fn handle_event(&mut self, mgr: &mut EventMgr, event: Event) -> Response {
        match event {
            Event::Command(Command::DelBack) => {
                if self.calc.handle(Key::DelBack) {
                    *mgr |= self.display.set_string(self.calc.display());
                }
                Response::Used
            }
            _ => Response::Unused,
        }
    }
}
```

[`TextButton`] supports two methods of defining accelerator keys, as exemplified
by the `clear` button:
```ignore
0, 0: TextButton::new_msg("&clear", Key::Clear).with_keys(&[VK::Delete]);
```
The label is an [`AccelString`], which supports simple mark-up: `&c` adds a
binding to <kbd>c</kbd> and is displayed as `c` (with an underline when
<kbd>Alt</kbd> is held).

[`TextButton::with_keys`] explicitly assigns additional key bindings.

[`AccelString`]: https://docs.rs/kas/latest/kas/text/struct.AccelString.html
[`TextButton`]: https://docs.rs/kas/latest/kas/widgets/struct.TextButton.html
[`TextButton::with_keys`]: https://docs.rs/kas/latest/kas/widgets/struct.TextButton.html#method.with_keys
[`ConfigMgr::enable_alt_bypass`]: https://docs.rs/kas/latest/kas/event/struct.ConfigMgr.html#method.enable_alt_bypass
[`ConfigMgr::disable_nav_focus`]: https://docs.rs/kas/latest/kas/event/struct.ConfigMgr.html#method.disable_nav_focus
[`ConfigMgr::register_nav_fallback`]: https://docs.rs/kas/latest/kas/event/struct.ConfigMgr.html#method.register_nav_fallback
