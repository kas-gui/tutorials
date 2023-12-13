use kas::widgets::dialog::MessageBox;

fn main() -> kas::app::Result<()> {
    env_logger::init();

    let window = MessageBox::new("Message").into_window("Hello world");

    kas::app::Default::new(())?.with(window).run()
}
