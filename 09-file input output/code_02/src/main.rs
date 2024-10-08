use fltk::{
    app,
    button::Button,
    draw, enums,
    frame::Frame,
    input::{Input, SecretInput},
    prelude::*,
    table::{self, Table},
    window::Window,
};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::Mutex;
use std::thread;

const DEFAULT_USERNAME: &str = "admin";
const DEFAULT_PASSWORD: &str = "1234";
const CSV_FILE_PATH: &str = "users.csv";

#[derive(Clone)]
struct User {
    username: String,
    password: String,
}

lazy_static! {
    static ref USERS: Mutex<Vec<User>> = Mutex::new(Vec::new());
}

fn main() {
    load_users_from_csv();

    let app = app::App::default();
    let (screen_width, screen_height) = app::screen_size();

    // Main window (login form)
    let window_width = 400;
    let window_height = 300;
    let mut login_window = Window::new(
        (screen_width as i32 - window_width) / 2,
        (screen_height as i32 - window_height) / 2,
        window_width,
        window_height,
        "Login Screen",
    );

    // Allow the window to be resizable
    login_window.make_resizable(true);

    let mut username_input = Input::new(150, 50, 200, 30, "Username:");
    let mut password_input = SecretInput::new(150, 100, 200, 30, "Password:");

    // Set default values
    username_input.set_value(DEFAULT_USERNAME);
    password_input.set_value(DEFAULT_PASSWORD);

    let mut login_button = Button::new(150, 150, 80, 40, "Login");
    let mut clear_button = Button::new(250, 150, 80, 40, "Clear");

    let message_frame = Frame::new(150, 200, 200, 40, "");

    login_window.end();
    login_window.show();

    login_button.set_callback({
        let username_input = username_input.clone();
        let password_input = password_input.clone();
        let mut message_frame = message_frame.clone();
        let mut login_window = login_window.clone();
        move |_| {
            let username = username_input.value();
            let password = password_input.value();
            let users = USERS.lock().unwrap();
            if users
                .iter()
                .any(|user| user.username == username && user.password == password)
            {
                login_window.hide();

                // User List window created after successful login
                let mut user_list_window = Window::new(
                    (screen_width as i32 - window_width) / 2,
                    (screen_height as i32 - window_height) / 2,
                    window_width,
                    window_height,
                    "User List",
                );
                user_list_window.make_resizable(true);

                let edit_username_input = Input::new(50, 50, 140, 30, "Username:");
                let mut edit_password_input = Input::new(50, 100, 140, 30, "Password:");

                let mut add_button = Button::new(50, 150, 80, 40, "Add");
                let mut update_button = Button::new(150, 150, 80, 40, "Update");
                let mut delete_button = Button::new(250, 150, 80, 40, "Delete");

                let mut user_table = Table::new(50, 200, 300, 150, "");

                // Initial table setup
                user_table.set_rows(users.len() as i32);
                user_table.set_cols(2);
                user_table.set_col_header(true);
                user_table.set_row_header(true);
                user_table.end();

                user_table.draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                    table::TableContext::StartPage => draw::set_font(enums::Font::Helvetica, 14),
                    table::TableContext::ColHeader => {
                        let header = if col == 0 { "Username" } else { "Password" };
                        draw_header(header, x, y, w, h);
                    }
                    table::TableContext::RowHeader => {
                        draw_header(&format!("{}", row + 1), x, y, w, h);
                    }
                    table::TableContext::Cell => {
                        let users = USERS.lock().unwrap();
                        let user = &users[row as usize];
                        let data = if col == 0 {
                            &user.username
                        } else {
                            &user.password
                        };
                        draw_data(data, x, y, w, h, t.is_selected(row, col));
                    }
                    _ => (),
                });

                user_table.handle({
                    let mut edit_username_input = edit_username_input.clone();
                    let mut edit_password_input = edit_password_input.clone();
                    move |t, ev| match ev {
                        enums::Event::Push => {
                            let row = t.callback_row();
                            if row >= 0 {
                                let users = USERS.lock().unwrap();
                                let user = &users[row as usize];
                                edit_username_input.set_value(&user.username);
                                edit_password_input.set_value(&user.password);
                            }
                            true
                        }
                        _ => false,
                    }
                });

                add_button.set_callback({
                    let mut user_table = user_table.clone();
                    let mut edit_username_input = edit_username_input.clone();
                    let mut edit_password_input = edit_password_input.clone();
                    move |_| {
                        let mut users = USERS.lock().unwrap();
                        let new_username = edit_username_input.value();
                        let new_password = edit_password_input.value();
                        if !new_username.is_empty() && !new_password.is_empty() {
                            users.push(User {
                                username: new_username.clone(),
                                password: new_password.clone(),
                            });
                            user_table.set_rows(users.len() as i32);
                            user_table.redraw();
                            edit_username_input.set_value("");
                            edit_password_input.set_value("");
                            thread::spawn(|| persist_users_to_csv());
                        }
                    }
                });

                update_button.set_callback({
                    let mut user_table = user_table.clone();
                    let mut edit_username_input = edit_username_input.clone();
                    let mut edit_password_input = edit_password_input.clone();
                    move |_| {
                        let username_to_update = edit_username_input.value();
                        let mut users = USERS.lock().unwrap();
                        if let Some(pos) =
                            users.iter().position(|u| u.username == username_to_update)
                        {
                            users[pos].password = edit_password_input.value(); // Only update the password
                            user_table.redraw(); // Refresh the table
                            edit_username_input.set_value("");
                            edit_password_input.set_value("");
                            thread::spawn(|| persist_users_to_csv());
                        }
                    }
                });

                delete_button.set_callback({
                    let mut user_table = user_table.clone();
                    let mut edit_username_input = edit_username_input.clone();
                    move |_| {
                        let username_to_delete = edit_username_input.value();
                        let mut users = USERS.lock().unwrap();
                        if let Some(pos) =
                            users.iter().position(|u| u.username == username_to_delete)
                        {
                            users.remove(pos);
                            user_table.set_rows(users.len() as i32);
                            user_table.redraw(); // Refresh the table
                            edit_username_input.set_value("");
                            edit_password_input.set_value("");
                            thread::spawn(|| persist_users_to_csv());
                        }
                    }
                });

                user_list_window.end();
                user_list_window.show();

                // Add a callback to show login_window when user_list_window is closed
                user_list_window.set_callback({
                    let mut user_list_window = user_list_window.clone();
                    let mut login_window = login_window.clone();
                    move |_| {
                        user_list_window.hide();
                        login_window.show();
                    }
                });
            } else {
                message_frame.set_label("Login failed. Incorrect username or password.");
            }
        }
    });

    clear_button.set_callback({
        let mut username_input = username_input.clone();
        let mut password_input = password_input.clone();
        let mut message_frame = message_frame.clone();
        move |_| {
            username_input.set_value("");
            password_input.set_value("");
            message_frame.set_label("");
        }
    });

    app.run().unwrap();
}

fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(
        enums::FrameType::ThinUpBox,
        x,
        y,
        w,
        h,
        enums::Color::FrameDefault,
    );
    draw::set_draw_color(enums::Color::Black);
    draw::set_font(enums::Font::Helvetica, 14);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::pop_clip();
}

fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(enums::Color::from_u32(0x00D3_D3D3));
    } else {
        draw::set_draw_color(enums::Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(enums::Color::Gray0);
    draw::set_font(enums::Font::Helvetica, 14);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

fn load_users_from_csv() {
    let file = File::open(CSV_FILE_PATH).unwrap_or_else(|_| {
        let mut file = File::create(CSV_FILE_PATH).unwrap();
        writeln!(file, "username,password").unwrap();
        file
    });

    let reader = BufReader::new(file);
    let mut users = USERS.lock().unwrap();
    users.clear(); // Clear the existing users list before loading new data
    for line in reader.lines().skip(1) {
        if let Ok(line) = line {
            let mut parts = line.split(',');
            if let (Some(username), Some(password)) = (parts.next(), parts.next()) {
                users.push(User {
                    username: username.to_string(),
                    password: password.to_string(),
                });
            }
        }
    }
}

fn persist_users_to_csv() {
    let users = USERS.lock().unwrap();
    let mut file = File::create(CSV_FILE_PATH).unwrap();
    writeln!(file, "username,password").unwrap(); // Write the header
    for user in users.iter() {
        writeln!(file, "{},{}", user.username, user.password).unwrap();
    }
}
