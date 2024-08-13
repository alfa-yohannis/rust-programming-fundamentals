use fltk::{
    app, button::Button, frame::Frame, input::Input, misc::Spinner, prelude::*, window::Window,
};

fn main() {
    let app = app::App::default();
    let (screen_width, screen_height) = app::screen_size();
    let win_width = 400;
    let win_height = 300;

    let mut win = Window::new(
        (screen_width as i32 - win_width) / 2,
        (screen_height as i32 - win_height) / 2,
        win_width,
        win_height,
        "Greetings App",
    );
    win.make_resizable(true);

    let name_input = Input::new(160, 50, 200, 30, "Name:");
    let age_input = Spinner::new(160, 90, 200, 30, "Age:");

    let mut btn = Button::new(160, 130, 80, 40, "Say Hi!");
    let mut frame = Frame::new(60, 170, 280, 40, "");

    btn.set_callback(move |_| {
        let name = name_input.value();
        let age = age_input.value();
        frame.set_label(&format!("Hello, {}! I'm {} years old.", name, age));
    });

    win.end();
    win.show();

    app.run().unwrap();
}
