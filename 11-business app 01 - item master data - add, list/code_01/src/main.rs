use async_std::task;
use business_app::item::Item;
use fltk::{
    app,
    button::Button,
    draw, enums,
    input::Input,
    prelude::*,
    table::{self, Table},
    window::Window,
};
use tokio_postgres::{Client, NoTls};

async fn create_db_client() -> Client {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=1234 dbname=session10",
        NoTls,
    )
    .await
    .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    client
}

async fn load_items_from_db(client: &Client) -> Vec<Item> {
    business_app::item::list(client)
        .await
        .unwrap_or_else(|_| Vec::new())
}

async fn persist_item_to_db(client: &Client, item: &Item) {
    business_app::item::add(client, item).await.unwrap();
}

async fn delete_item_from_db(client: &Client, code: &str) {
    business_app::item::delete(client, code).await.unwrap();
}

async fn update_item_in_db(client: &Client, item: &Item) {
    business_app::item::update(client, item).await.unwrap();
}

#[tokio::main]
async fn main() {
    let app = app::App::default();
    let (screen_width, screen_height) = app::screen_size();

    let window_width = 600;
    let window_height = 400;
    let mut window = Window::new(
        (screen_width as i32 - window_width) / 2,
        (screen_height as i32 - window_height) / 2,
        window_width,
        window_height,
        "Item List",
    );

    window.make_resizable(true);

    let edit_code_input = Input::new(50, 50, 140, 30, "Code:");
    let edit_name_input = Input::new(50, 100, 140, 30, "Name:");
    let edit_currency_input = Input::new(50, 150, 140, 30, "Currency:");
    let edit_price_input = Input::new(250, 50, 140, 30, "Price:");
    let edit_quantity_input = Input::new(250, 100, 140, 30, "Quantity:");
    let edit_unit_input = Input::new(250, 150, 140, 30, "Unit:");

    let mut add_button = Button::new(50, 200, 80, 40, "Add");
    let mut update_button = Button::new(150, 200, 80, 40, "Update");
    let mut delete_button = Button::new(250, 200, 80, 40, "Delete");

    let mut item_table = Table::new(50, 250, 500, 130, "");

    // Initial table setup
    item_table.set_cols(6); // Code, Name, Currency, Price, Quantity, Unit
    item_table.set_col_header(true);
    item_table.set_row_header(true);
    item_table.set_col_width_all(100); // Set a default column width for all columns
    item_table.set_col_width(0, 100);
    item_table.set_col_width(1, 150);
    item_table.set_col_resize(true);
    item_table.set_col_resize_min(50);
    item_table.end();

    item_table.draw_cell({
        move |t, ctx, row, col, x, y, w, h| match ctx {
            table::TableContext::StartPage => draw::set_font(enums::Font::Helvetica, 14),
            table::TableContext::ColHeader => {
                let headers = ["Code", "Name", "Currency", "Price", "Quantity", "Unit"];
                draw_header(headers[col as usize], x, y, w, h);
            }
            table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),
            table::TableContext::Cell => {
                // Fetch the current items from the database
                let client = task::block_on(create_db_client());
                let items = task::block_on(load_items_from_db(&client));
                let item = &items[row as usize];
                let data = match col {
                    0 => &item.code,
                    1 => &item.name,
                    2 => &item.currency,
                    3 => &item.price.to_string(),
                    4 => &item.quantity.to_string(),
                    5 => item.unit.as_deref().unwrap_or(""),
                    _ => "",
                };
                draw_data(data, x, y, w, h, t.is_selected(row, col));
            }
            _ => (),
        }
    });

    item_table.handle({
        let mut edit_code_input = edit_code_input.clone();
        let mut edit_name_input = edit_name_input.clone();
        let mut edit_currency_input = edit_currency_input.clone();
        let mut edit_price_input = edit_price_input.clone();
        let mut edit_quantity_input = edit_quantity_input.clone();
        let mut edit_unit_input = edit_unit_input.clone();
        move |t, ev| match ev {
            enums::Event::Push => {
                let row = t.callback_row();
                if row >= 0 {
                    let client = task::block_on(create_db_client());
                    let items = task::block_on(load_items_from_db(&client));
                    let item = &items[row as usize];
                    edit_code_input.set_value(&item.code);
                    edit_name_input.set_value(&item.name);
                    edit_currency_input.set_value(&item.currency);
                    edit_price_input.set_value(&item.price.to_string());
                    edit_quantity_input.set_value(&item.quantity.to_string());
                    edit_unit_input.set_value(item.unit.as_deref().unwrap_or(""));
                }
                true
            }
            _ => false,
        }
    });

    add_button.set_callback({
        let mut item_table = item_table.clone();
        let mut edit_code_input = edit_code_input.clone();
        let mut edit_name_input = edit_name_input.clone();
        let mut edit_currency_input = edit_currency_input.clone();
        let mut edit_price_input = edit_price_input.clone();
        let mut edit_quantity_input = edit_quantity_input.clone();
        let mut edit_unit_input = edit_unit_input.clone();
        move |_| {
            let new_item = Item {
                code: edit_code_input.value(),
                name: edit_name_input.value(),
                currency: edit_currency_input.value(),
                price: edit_price_input.value().parse().unwrap_or(0.0),
                quantity: edit_quantity_input.value().parse().unwrap_or(0.0),
                unit: if edit_unit_input.value().is_empty() {
                    None
                } else {
                    Some(edit_unit_input.value())
                },
            };
            if !new_item.code.is_empty() && !new_item.name.is_empty() {
                let client = task::block_on(create_db_client());
                task::block_on(persist_item_to_db(&client, &new_item));
                let items = task::block_on(load_items_from_db(&client));
                item_table.set_rows(items.len() as i32);
                item_table.redraw();
                edit_code_input.set_value("");
                edit_name_input.set_value("");
                edit_currency_input.set_value("");
                edit_price_input.set_value("");
                edit_quantity_input.set_value("");
                edit_unit_input.set_value("");
            }
        }
    });

    update_button.set_callback({
        let mut item_table = item_table.clone();
        let mut edit_code_input = edit_code_input.clone();
        let mut edit_name_input = edit_name_input.clone();
        let mut edit_currency_input = edit_currency_input.clone();
        let mut edit_price_input = edit_price_input.clone();
        let mut edit_quantity_input = edit_quantity_input.clone();
        let mut edit_unit_input = edit_unit_input.clone();
        move |_| {
            let code_to_update = edit_code_input.value();
            let client = task::block_on(create_db_client());
            let mut items = task::block_on(load_items_from_db(&client));
            if let Some(pos) = items.iter().position(|i| i.code == code_to_update) {
                items[pos].name = edit_name_input.value();
                items[pos].currency = edit_currency_input.value();
                items[pos].price = edit_price_input.value().parse().unwrap_or(0.0);
                items[pos].quantity = edit_quantity_input.value().parse().unwrap_or(0.0);
                items[pos].unit = if edit_unit_input.value().is_empty() {
                    None
                } else {
                    Some(edit_unit_input.value())
                };
                task::block_on(update_item_in_db(&client, &items[pos]));
                item_table.set_rows(items.len() as i32);
                item_table.redraw();

                edit_code_input.set_value("");
                edit_name_input.set_value("");
                edit_currency_input.set_value("");
                edit_price_input.set_value("");
                edit_quantity_input.set_value("");
                edit_unit_input.set_value("");
            }
        }
    });

    delete_button.set_callback({
        let mut item_table = item_table.clone();
        let mut edit_code_input = edit_code_input.clone();
        move |_| {
            let code_to_delete = edit_code_input.value();
            let client = task::block_on(create_db_client());
            task::block_on(delete_item_from_db(&client, &code_to_delete));
            let items = task::block_on(load_items_from_db(&client));
            item_table.set_rows(items.len() as i32);
            item_table.redraw();

            edit_code_input.set_value("");
        }
    });

    window.end();
    window.show();

    app.run().unwrap()
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
