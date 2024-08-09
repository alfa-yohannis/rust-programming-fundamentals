use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};

fn main() {
    let app = app::App::default();
    let (screen_width, screen_height) = app::screen_size();
    let win_width = 220;
    let win_height = 340;

    let mut win = Window::new(
        (screen_width as i32 - win_width) / 2,
        (screen_height as i32 - win_height) / 2,
        win_width,
        win_height,
        "Simple Calculator",
    );
    win.make_resizable(true);

    // Display frame
    let mut display = Frame::new(20, 20, 200, 40, "");
    display.set_label_size(24);

    // Button creation
    let mut buttons = vec![];

    // Digits 1-9
    for i in 1..=9 {
        let x = 20 + ((i - 1) % 3) * 60;
        let y = 80 + ((i - 1) / 3) * 60;
        let mut btn = Button::new(x, y, 50, 50, i.to_string().as_str());
        btn.set_callback({
            let mut display = display.clone();
            move |_| {
                let current_text = display.label();
                display.set_label(&(current_text + &i.to_string()));
            }
        });
        buttons.push(btn);
    }

    // Plus button
    let mut plus_btn = Button::new(20, 260, 50, 50, "+");
    plus_btn.set_callback({
        let mut display = display.clone();
        move |_| {
            let current_text = display.label();
            if !current_text.is_empty() && !current_text.ends_with('+') {
                display.set_label(&(current_text + "+"));
            }
        }
    });
    buttons.push(plus_btn);

    // Zero button
    let mut zero_btn = Button::new(80, 260, 50, 50, "0");
    zero_btn.set_callback({
        let mut display = display.clone();
        move |_| {
            let current_text = display.label();
            display.set_label(&(current_text + "0"));
        }
    });
    buttons.push(zero_btn);

    // Equals button
    let mut equals_btn = Button::new(140, 260, 50, 50, "=");
    equals_btn.set_callback({
        let mut display = display.clone();
        move |_| {
            let current_text = display.label();
            if let Some(result) = calculate_expression(&current_text) {
                display.set_label(&result.to_string());
            }
        }
    });
    buttons.push(equals_btn);

    win.end();
    win.show();

    app.run().unwrap();
}

// A simple function to calculate the result of a string expression like "12+34"
fn calculate_expression(expr: &str) -> Option<i32> {
    let parts: Vec<&str> = expr.split('+').collect();
    if parts.len() > 2 {
        let sum: i32 = parts
            .iter() // Create an iterator over the vector
            .map(|s| s.parse::<i32>().unwrap()) // Convert each &str to an i32
            .sum();
        Some(sum)
    } else {
        None
    }
}
