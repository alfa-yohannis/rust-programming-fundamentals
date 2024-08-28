#[macro_use]
extern crate lazy_static;

use async_std::task;
use business_app::sales_order::{get_sales_order, list_sales_orders, SalesOrder};
use business_app::sales_order_detail::{get_details_by_order_code, SalesOrderDetail};
use fltk::{
    app, button::Button, draw, enums,
    input::Input,
    prelude::*,
    table::{self, Table},
    window::Window,
};
use std::sync::Mutex;
use tokio_postgres::{Client, NoTls};

lazy_static! {
    static ref DB_CLIENT: Mutex<Client> = Mutex::new(task::block_on(create_db_client()));
    static ref ORDER_DETAILS: Mutex<Vec<SalesOrderDetail>> = Mutex::new(vec![]);
    static ref ORDER: Mutex<Option<SalesOrder>> = Mutex::new(None);
    static ref ALL_ORDERS: Mutex<Vec<SalesOrder>> = Mutex::new(vec![]);
    static ref CURRENT_ORDER_INDEX: Mutex<usize> = Mutex::new(0);
}

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

async fn load_orders_from_db(client: &Client) -> Vec<SalesOrder> {
    list_sales_orders(client).await.unwrap_or_else(|_| Vec::new())
}

async fn load_order_details_from_db(client: &Client, code: &str) -> Vec<SalesOrderDetail> {
    get_details_by_order_code(client, code)
        .await
        .unwrap_or_else(|_| Vec::new())
}

fn display_order_and_details(order: &SalesOrder, table: &mut Table, order_code_input: &Input, order_date_input: &Input, order_note_input: &Input) {
    order_code_input.set_value(&order.code);
    order_date_input.set_value(&order.order_date.to_string());
    order_note_input.set_value(order.note.as_deref().unwrap_or(""));

    let client = DB_CLIENT.lock().unwrap();
    let details = task::block_on(load_order_details_from_db(&client, &order.code));
    *ORDER_DETAILS.lock().unwrap() = details.clone();

    table.set_rows(details.len() as i32);
    table.redraw();
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
        "Sales Order Details",
    );

    window.make_resizable(true);

    let mut first_button = Button::new(50, 10, 80, 30, "First");
    let mut prev_button = Button::new(140, 10, 80, 30, "Prev");
    let mut next_button = Button::new(230, 10, 80, 30, "Next");
    let mut last_button = Button::new(320, 10, 80, 30, "Last");

    let mut order_code_input = Input::new(50, 50, 140, 30, "Order Code:");
    let mut order_date_input = Input::new(250, 50, 140, 30, "Order Date:");
    let mut order_note_input = Input::new(50, 100, 340, 30, "Note:");

    let mut details_table = Table::new(50, 150, 500, 200, "");

    details_table.set_rows(0);
    details_table.set_cols(5); // Line, Item Code, Quantity, Unit, Unit Price
    details_table.set_col_header(true);
    details_table.set_row_header(true);
    details_table.set_col_width_all(100);
    details_table.set_col_resize(true);
    details_table.set_col_resize_min(50);
    details_table.end();

    details_table.draw_cell({
        move |t, ctx, row, col, x, y, w, h| match ctx {
            table::TableContext::StartPage => draw::set_font(enums::Font::Helvetica, 14),
            table::TableContext::ColHeader => {
                let headers = ["Line", "Item Code", "Quantity", "Unit", "Unit Price"];
                draw_header(headers[col as usize], x, y, w, h);
            }
            table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),
            table::TableContext::Cell => {
                let details = ORDER_DETAILS.lock().unwrap();
                let detail = &details[row as usize];
                let data = match col {
                    0 => detail.line_num.to_string(),
                    1 => detail.item_code.clone(),
                    2 => detail.quantity.to_string(),
                    3 => detail.unit.clone().unwrap_or_else(|| "".to_string()),
                    4 => detail.unit_price.to_string(),
                    _ => "".to_string(),
                };
                draw_data(&data, x, y, w, h, t.is_selected(row, col));
            }
            _ => (),
        }
    });

    // Load all orders and display the first one
    let client = DB_CLIENT.lock().unwrap();
    let orders = task::block_on(load_orders_from_db(&client));
    if !orders.is_empty() {
        *ALL_ORDERS.lock().unwrap() = orders.clone();
        let first_order = &orders[0];
        display_order_and_details(first_order, &mut details_table, &order_code_input, &order_date_input, &order_note_input);
    }

    // Navigation buttons callbacks
    first_button.set_callback({
        let details_table = details_table.clone();
        let order_code_input = order_code_input.clone();
        let order_date_input = order_date_input.clone();
        let order_note_input = order_note_input.clone();
        move |_| {
            let mut index = CURRENT_ORDER_INDEX.lock().unwrap();
            *index = 0;
            let orders = ALL_ORDERS.lock().unwrap();
            if let Some(order) = orders.get(*index) {
                display_order_and_details(order, &details_table, &order_code_input, &order_date_input, &order_note_input);
            }
        }
    });

    prev_button.set_callback({
        let details_table = details_table.clone();
        let order_code_input = order_code_input.clone();
        let order_date_input = order_date_input.clone();
        let order_note_input = order_note_input.clone();
        move |_| {
            let mut index = CURRENT_ORDER_INDEX.lock().unwrap();
            if *index > 0 {
                *index -= 1;
            }
            let orders = ALL_ORDERS.lock().unwrap();
            if let Some(order) = orders.get(*index) {
                display_order_and_details(order, &details_table, &order_code_input, &order_date_input, &order_note_input);
            }
        }
    });

    next_button.set_callback({
        let details_table = details_table.clone();
        let order_code_input = order_code_input.clone();
        let order_date_input = order_date_input.clone();
        let order_note_input = order_note_input.clone();
        move |_| {
            let mut index = CURRENT_ORDER_INDEX.lock().unwrap();
            let orders = ALL_ORDERS.lock().unwrap();
            if *index < orders.len() - 1 {
                *index += 1;
            }
            if let Some(order) = orders.get(*index) {
                display_order_and_details(order, &details_table, &order_code_input, &order_date_input, &order_note_input);
            }
        }
    });

    last_button.set_callback({
        let details_table = details_table.clone();
        let order_code_input = order_code_input.clone();
        let order_date_input = order_date_input.clone();
        let order_note_input = order_note_input.clone();
        move |_| {
            let mut index = CURRENT_ORDER_INDEX.lock().unwrap();
            let orders = ALL_ORDERS.lock().unwrap();
            *index = orders.len().saturating_sub(1);
            if let Some(order) = orders.get(*index) {
                display_order_and_details(order, &details_table, &order_code_input, &order_date_input, &order_note_input);
            }
        }
    });

    window.end();
    window.show();
    app.run().unwrap();
}

fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::draw_box(
        enums::FrameType::FlatBox,
        x,
        y,
        w,
        h,
        enums::Color::from_rgb(180, 180, 180),
    );
    draw::set_draw_color(enums::Color::Black);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
}

fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::set_draw_color(if selected {
        enums::Color::from_rgb(200, 200, 255)
    } else {
        enums::Color::White
    });
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(enums::Color::Black);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Left);
}
