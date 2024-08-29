#[macro_use]
extern crate lazy_static;

use async_std::task;
use business_app::sales_order::{SalesOrder, list_sales_orders, get_sales_order};
use business_app::sales_order_detail::{SalesOrderDetail, get_details_by_order_code};
use fltk::{
    app,
    button::Button,
    draw, enums,
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
    static ref ORDER_INDEX: Mutex<usize> = Mutex::new(0);
    // static ref ORDER_LIST: Mutex<Vec<SalesOrder>> = Mutex::new(vec![]);
    static ref ORDER_LIST: Mutex<Vec<SalesOrder>> = Mutex::new(task::block_on(load_all_orders_from_db(&DB_CLIENT.lock().unwrap())));
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

async fn load_all_orders_from_db(client: &Client) -> Vec<SalesOrder> {
    list_sales_orders(client).await.unwrap_or_else(|_| Vec::new())
}

async fn load_order_details_from_db(client: &Client, code: &str) -> Vec<SalesOrderDetail> {
    get_details_by_order_code(client, code).await.unwrap_or_else(|_| Vec::new())
}

fn update_order_details(order_code_input: &mut Input, order_date_input: &mut Input, order_note_input: &mut Input, details_table: &mut Table) {
    let orders = ORDER_LIST.lock().unwrap();
    let index = *ORDER_INDEX.lock().unwrap();

    if let Some(order) = orders.get(index) {
        order_code_input.set_value(&order.code);
        order_date_input.set_value(&order.order_date.to_string());
        order_note_input.set_value(order.note.as_deref().unwrap_or(""));

        let client = DB_CLIENT.lock().unwrap();
        let details = task::block_on(load_order_details_from_db(&client, &order.code));
        *ORDER_DETAILS.lock().unwrap() = details.clone();

        details_table.set_rows(details.len() as i32);
        details_table.redraw();
    }
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

    let mut first_button = Button::new(50, 10, 60, 30, "First");
    let mut prev_button = Button::new(120, 10, 60, 30, "Prev");
    let mut next_button = Button::new(190, 10, 60, 30, "Next");
    let mut last_button = Button::new(260, 10, 60, 30, "Last");

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

  
    first_button.set_callback({
        let mut order_code_input = order_code_input.clone();
        let mut order_date_input = order_date_input.clone();
        let mut order_note_input = order_note_input.clone();
        let mut details_table = details_table.clone();
        move |_| {
            *ORDER_INDEX.lock().unwrap() = 0;
            update_order_details(&mut order_code_input, &mut order_date_input, &mut order_note_input, &mut details_table);
        }
    });

    prev_button.set_callback({
        let mut order_code_input = order_code_input.clone();
        let mut order_date_input = order_date_input.clone();
        let mut order_note_input = order_note_input.clone();
        let mut details_table = details_table.clone();
        move |_| {
            let index = ORDER_INDEX.lock().unwrap().clone();
            if index > 0 {
                *ORDER_INDEX.lock().unwrap() = index - 1;
            }
            update_order_details(&mut order_code_input, &mut order_date_input, &mut order_note_input, &mut details_table);
        }
    });

    next_button.set_callback({
        let mut order_code_input = order_code_input.clone();
        let mut order_date_input = order_date_input.clone();
        let mut order_note_input = order_note_input.clone();
        let mut details_table = details_table.clone();
        move |_| {
            let index = ORDER_INDEX.lock().unwrap().clone();
            let max_index = ORDER_LIST.lock().unwrap().len() - 1;
            if index < max_index {
                *ORDER_INDEX.lock().unwrap() = index + 1;
            }
            update_order_details(&mut order_code_input, &mut order_date_input, &mut order_note_input, &mut details_table);
        }
    });

    last_button.set_callback({
        let mut order_code_input = order_code_input.clone();
        let mut order_date_input = order_date_input.clone();
        let mut order_note_input = order_note_input.clone();
        let mut details_table = details_table.clone();
        move |_| {
            let max_index = ORDER_LIST.lock().unwrap().len() - 1;
            *ORDER_INDEX.lock().unwrap() = max_index;
            update_order_details(&mut order_code_input, &mut order_date_input, &mut order_note_input, &mut details_table);
        }
    });


    // Load the first sales order
    update_order_details(&mut order_code_input, &mut order_date_input, &mut order_note_input, &mut details_table);

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
    draw::draw_rect(x, y, w, h);
    draw::draw_text2(txt, x, y, w, h, enums::Align::Center);
}
