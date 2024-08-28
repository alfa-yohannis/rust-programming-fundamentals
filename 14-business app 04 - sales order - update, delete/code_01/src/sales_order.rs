use chrono::{DateTime, Utc};
use tokio_postgres::{Client, Error};

#[derive(Debug, Clone)]
pub struct SalesOrder {
    pub code: String,
    pub order_date: DateTime<Utc>,
    pub note: Option<String>,
}

pub async fn add_sales_order(client: &Client, order: &SalesOrder) -> Result<(), Error> {
    let stmt = client
        .prepare(
            "INSERT INTO sales_orders (code, order_date, note)
             VALUES ($1, $2, $3)",
        )
        .await?;

    client
        .execute(&stmt, &[&order.code, &order.order_date, &order.note])
        .await?;

    Ok(())
}

pub async fn get_sales_order(client: &Client, code: &str) -> Result<Option<SalesOrder>, Error> {
    let stmt = client
        .prepare("SELECT code, order_date, note FROM sales_orders WHERE code = $1")
        .await?;
    
    let row_opt = client.query_opt(&stmt, &[&code]).await?;
    
    if let Some(row) = row_opt {
        Ok(Some(SalesOrder {
            code: row.get("code"),
            order_date: row.get("order_date"),
            note: row.get("note"),
        }))
    } else {
        Ok(None)
    }
}

pub async fn update_sales_order(client: &Client, order: &SalesOrder) -> Result<(), Error> {
    let stmt = client
        .prepare(
            "UPDATE sales_orders
             SET order_date = $1, note = $2
             WHERE code = $3",
        )
        .await?;

    client
        .execute(&stmt, &[&order.order_date, &order.note, &order.code])
        .await?;

    Ok(())
}

pub async fn delete_sales_order(client: &Client, code: &str) -> Result<(), Error> {
    let stmt = client.prepare("DELETE FROM sales_orders WHERE code = $1").await?;
    client.execute(&stmt, &[&code]).await?;
    Ok(())
}

pub async fn list_sales_orders(client: &Client) -> Result<Vec<SalesOrder>, Error> {
    let rows = client.query("SELECT code, order_date, note FROM sales_orders ORDER BY code", &[]).await?;

    let orders = rows
        .iter()
        .map(|row| SalesOrder {
            code: row.get("code"),
            order_date: row.get("order_date"),
            note: row.get("note"),
        })
        .collect();

    Ok(orders)
}
