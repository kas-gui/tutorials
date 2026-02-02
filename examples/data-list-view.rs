use kas::prelude::*;
use kas::view::clerk::{Clerk, GeneratorChanges, IndexedGenerator, Len};
use kas::view::{Driver, ListView};
use kas::widgets::edit::{EditBox, EditGuard, Editor};
use kas::widgets::{Label, RadioButton, ScrollRegion, Separator, Text, column};
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Control {
    Select(usize),
    Update(usize, String),
}

#[derive(Debug)]
struct MyData {
    last_change: GeneratorChanges<usize>,
    last_key: usize,
    active: usize,
    strings: HashMap<usize, String>,
}
impl MyData {
    fn new() -> Self {
        MyData {
            last_change: GeneratorChanges::None,
            last_key: 0,
            active: 0,
            strings: HashMap::new(),
        }
    }
    fn get_string(&self, index: usize) -> String {
        self.strings
            .get(&index)
            .cloned()
            .unwrap_or_else(|| format!("Entry #{}", index + 1))
    }
    fn handle(&mut self, control: Control) {
        match control {
            Control::Select(index) => {
                self.last_change = GeneratorChanges::Any;
                self.active = index;
            }
            Control::Update(index, text) => {
                self.last_change = GeneratorChanges::Range(index..index + 1);
                self.last_key = self.last_key.max(index);
                self.strings.insert(index, text);
            }
        }
    }
}

type MyItem = (usize, String); // (active index, entry's text)

#[derive(Debug)]
struct ListEntryGuard(usize);
impl EditGuard for ListEntryGuard {
    type Data = MyItem;

    fn update(&mut self, edit: &mut Editor, cx: &mut ConfigCx, data: &MyItem) {
        edit.set_string(cx, data.1.to_string());
    }

    fn activate(&mut self, _: &mut Editor, cx: &mut EventCx, _: &MyItem) -> IsUsed {
        cx.push(Control::Select(self.0));
        Used
    }

    fn edit(&mut self, edit: &mut Editor, cx: &mut EventCx, _: &MyItem) {
        cx.push(Control::Update(self.0, edit.clone_string()));
    }
}

#[impl_self]
mod ListEntry {
    // The list entry
    #[widget]
    #[layout(column! [
        row! [self.label, self.radio],
        self.edit,
    ])]
    struct ListEntry {
        core: widget_core!(),
        #[widget(&())]
        label: Label<String>,
        #[widget]
        radio: RadioButton<MyItem>,
        #[widget]
        edit: EditBox<ListEntryGuard>,
    }

    impl Events for Self {
        type Data = MyItem;
    }
}

struct ListEntryDriver;
impl Driver<usize, MyItem> for ListEntryDriver {
    const TAB_NAVIGABLE: bool = true;

    type Widget = ListEntry;

    fn make(&mut self, key: &usize) -> Self::Widget {
        let n = *key;
        ListEntry {
            core: Default::default(),
            label: Label::new(format!("Entry number {}", n + 1)),
            radio: RadioButton::new_msg(
                "display this entry",
                move |_, data: &MyItem| data.0 == n,
                move || Control::Select(n),
            ),
            edit: EditBox::new(ListEntryGuard(n)),
        }
    }

    fn navigable(_: &Self::Widget) -> bool {
        false
    }
}

#[derive(Default)]
struct Generator;
impl Clerk<usize> for Generator {
    type Data = MyData;
    type Item = MyItem;

    fn len(&self, data: &Self::Data, lbound: usize) -> Len<usize> {
        Len::LBound((data.active.max(data.last_key) + 1).max(lbound))
    }
}
impl IndexedGenerator<usize> for Generator {
    fn update(&mut self, data: &Self::Data) -> GeneratorChanges<usize> {
        // We assume that `MyData::handle` has only been called once since this
        // method was last called.
        data.last_change.clone()
    }

    fn generate(&self, data: &Self::Data, index: usize) -> Self::Item {
        (data.active, data.get_string(index))
    }
}

fn main() -> kas::runner::Result<()> {
    env_logger::init();

    let clerk = Generator::default();
    let list = ListView::down(clerk, ListEntryDriver);
    let tree = column![
        "Contents of selected entry:",
        Text::new_gen(|_, data: &MyData| data.get_string(data.active)),
        Separator::new(),
        ScrollRegion::new_viewport(list).with_fixed_bars(false, true),
    ];

    let ui = tree
        .with_state(MyData::new())
        .on_message(|_, data, control| data.handle(control));

    let window = Window::new(ui, "Data list view");

    kas::runner::Runner::new(())?.with(window).run()
}
