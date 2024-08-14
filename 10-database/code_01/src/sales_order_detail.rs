use chrono::Utc;
use tokio_postgres::{Client, Error};

#[derive(Debug, Clone)]
pub struct SalesOrderDetail {
    pub id: i32,
    pub order_code: String,
    pub line_num: i32,
    pub item_id: i32,
    pub quantity: f32,
    pub unit: Option<String>,
    pub unit_price: f32,
}

pub async fn add_sales_order_detail(
    client: &Client,
    detail: &SalesOrderDetail,
) -> Result<(), Error> {
    let stmt = client
        .prepare(
            "INSERT INTO sales_order_details (order_code, line_num, item_id, quantity, unit, unit_price)
             VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .await?;

    client
        .execute(
            &stmt,
            &[
                &detail.order_code,
                &detail.line_num,
                &detail.item_id,
                &detail.quantity,
                &detail.unit,
                &detail.unit_price,
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
            "SELECT id, order_code, line_num, item_id, quantity, unit, unit_price
                  FROM sales_order_details WHERE id = $1",
        )
        .await?;

    let row_opt = client.query_opt(&stmt, &[&id]).await?;

    if let Some(row) = row_opt {
        Ok(Some(SalesOrderDetail {
            id: row.get("id"),
            order_code: row.get("order_code"),
            line_num: row.get("line_num"),
            item_id: row.get("item_id"),
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            unit_price: row.get("unit_price"),
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
             SET quantity = $1, unit = $2, unit_price = $3
             WHERE order_code = $4 and line_num = $5",
        )
        .await?;

    client
        .execute(
            &stmt,
            &[
                &detail.quantity,
                &detail.unit,
                &detail.unit_price,
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
            "SELECT id, order_code, line_num, item_id, quantity, unit, unit_price
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
            item_id: row.get("item_id"),
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            unit_price: row.get("unit_price"),
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
            "SELECT id, order_code, line_num, item_id, quantity, unit, unit_price
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
            item_id: row.get("item_id"),
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            unit_price: row.get("unit_price"),
        })
        .collect();

    Ok(details)
}
