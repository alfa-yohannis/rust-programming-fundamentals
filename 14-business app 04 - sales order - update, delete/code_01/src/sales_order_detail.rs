use tokio_postgres::{Client, Error};

#[derive(Debug, Clone)]
pub struct SalesOrderDetail {
    pub id: i32,
    pub order_code: String,
    pub line_num: i32,
    pub item_code: String,
    pub item_name: String,  // New field added
    pub quantity: f32,
    pub unit: Option<String>,
    pub unit_price: f32,
    pub currency: String,
}

pub async fn add_sales_order_detail(
    client: &Client,
    detail: &SalesOrderDetail,
) -> Result<(), Error> {
    let stmt = client
        .prepare(
            "INSERT INTO sales_order_details (order_code, line_num, item_code, item_name, quantity, unit, unit_price, currency)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .await?;

    client
        .execute(
            &stmt,
            &[
                &detail.order_code,
                &detail.line_num,
                &detail.item_code,
                &detail.item_name,  // New value added
                &detail.quantity,
                &detail.unit,
                &detail.unit_price,
                &detail.currency,
            ],
        )
        .await?;

    Ok(())
}

pub async fn get_sales_order_detail(
    client: &Client,
    id: i32,
) -> Result<Option<SalesOrderDetail>, Error> {
    let stmt = client
        .prepare(
            "SELECT id, order_code, line_num, item_code, item_name, quantity, unit, unit_price, currency
             FROM sales_order_details WHERE id = $1",
        )
        .await?;

    let row_opt = client.query_opt(&stmt, &[&id]).await?;

    if let Some(row) = row_opt {
        Ok(Some(SalesOrderDetail {
            id: row.get("id"),
            order_code: row.get("order_code"),
            line_num: row.get("line_num"),
            item_code: row.get("item_code"),
            item_name: row.get("item_name"),  // New value retrieved
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            unit_price: row.get("unit_price"),
            currency: row.get("currency"),
        }))
    } else {
        Ok(None)
    }
}

pub async fn update_sales_order_detail(
    client: &Client,
    detail: &SalesOrderDetail,
) -> Result<(), Error> {
    let stmt = client
        .prepare(
            "UPDATE sales_order_details
             SET item_name = $1, quantity = $2, unit = $3, unit_price = $4, currency = $5
             WHERE order_code = $6 and line_num = $7",
        )
        .await?;

    client
        .execute(
            &stmt,
            &[
                &detail.item_name,  // New value updated
                &detail.quantity,
                &detail.unit,
                &detail.unit_price,
                &detail.currency,
                &detail.order_code,
                &detail.line_num,
            ],
        )
        .await?;

    Ok(())
}

pub async fn delete_sales_order_detail(
    client: &Client,
    detail: &SalesOrderDetail,
) -> Result<(), Error> {
    let stmt = client
        .prepare("DELETE FROM sales_order_details WHERE order_code = $1 and line_num = $2")
        .await?;
    client
        .execute(&stmt, &[&detail.order_code, &detail.line_num])
        .await?;
    Ok(())
}

pub async fn delete_sales_order_detail_by_code(client: &Client, code: &str) -> Result<(), Error> {
    let stmt = client
        .prepare("DELETE FROM sales_order_details WHERE order_code = $1")
        .await?;
    client.execute(&stmt, &[&code]).await?;
    Ok(())
}

pub async fn list_sales_order_details(client: &Client) -> Result<Vec<SalesOrderDetail>, Error> {
    let rows = client
        .query(
            "SELECT id, order_code, line_num, item_code, item_name, quantity, unit, unit_price, currency
             FROM sales_order_details ORDER BY order_code, line_num",
            &[],
        )
        .await?;

    let details = rows
        .iter()
        .map(|row| SalesOrderDetail {
            id: row.get("id"),
            order_code: row.get("order_code"),
            line_num: row.get("line_num"),
            item_code: row.get("item_code"),
            item_name: row.get("item_name"),  // New value added
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            unit_price: row.get("unit_price"),
            currency: row.get("currency"),
        })
        .collect();

    Ok(details)
}

pub async fn get_details_by_order_code(
    client: &Client,
    order_code: &str,
) -> Result<Vec<SalesOrderDetail>, Error> {
    let stmt = client
        .prepare(
            "SELECT id, order_code, line_num, item_code, item_name, quantity, unit, unit_price, currency
             FROM sales_order_details WHERE order_code = $1 ORDER BY line_num",
        )
        .await?;

    let rows = client.query(&stmt, &[&order_code]).await?;

    let details = rows
        .iter()
        .map(|row| SalesOrderDetail {
            id: row.get("id"),
            order_code: row.get("order_code"),
            line_num: row.get("line_num"),
            item_code: row.get("item_code"),
            item_name: row.get("item_name"),  // New value added
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            unit_price: row.get("unit_price"),
            currency: row.get("currency"),
        })
        .collect();

    Ok(details)
}
