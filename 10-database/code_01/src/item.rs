use tokio_postgres::{Client, Error};
use chrono::Utc;

#[derive(Debug)]
pub struct Item {
    pub code: String,
    pub name: String,
    pub currency: String,
    pub price: f32,
    pub quantity: f32,
    pub unit: Option<String>,
    pub created: chrono::DateTime<Utc>,
    pub updated: chrono::DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
}

pub async fn add(client: &Client, item: &Item) -> Result<(), Error> {
    let stmt = client
        .prepare("
            INSERT INTO item (code, name, currency, price, quantity, unit, created_by, updated_by)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ")
        .await?;

    client
        .execute(
            &stmt,
            &[
                &item.code,
                &item.name,
                &item.currency,
                &item.price,
                &item.quantity,
                &item.unit,
                &item.created_by,
                &item.updated_by,
            ],
        )
        .await?;

    Ok(())
}

pub async fn delete(client: &Client, code: &str) -> Result<(), Error> {
    let stmt = client.prepare("DELETE FROM item WHERE code = $1").await?;
    client.execute(&stmt, &[&code]).await?;
    Ok(())
}

pub async fn update(client: &Client, item: &Item) -> Result<(), Error> {
    let stmt = client
        .prepare("
            UPDATE item SET
            name = $1, currency = $2, price = $3, quantity = $4, 
            unit = $5, updated = CURRENT_TIMESTAMP, updated_by = $6
            WHERE code = $7
        ")
        .await?;

    client
        .execute(
            &stmt,
            &[
                &item.name,
                &item.currency,
                &item.price,
                &item.quantity,
                &item.unit,
                &item.updated_by,
                &item.code,
            ],
        )
        .await?;

    Ok(())
}

pub async fn list(client: &Client) -> Result<Vec<Item>, Error> {
    let rows = client.query("SELECT * FROM item ORDER BY code", &[]).await?;

    let items = rows
        .iter()
        .map(|row| Item {
            code: row.get("code"),
            name: row.get("name"),
            currency: row.get("currency"),
            price: row.get("price"),
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            created: row.get("created"),
            updated: row.get("updated"),
            created_by: row.get("created_by"),
            updated_by: row.get("updated_by"),
        })
        .collect();

    Ok(items)
}

pub async fn get(client: &Client, code: &str) -> Result<Option<Item>, Error> {
    let stmt = client.prepare("SELECT * FROM item WHERE code = $1").await?;
    let row = client.query_opt(&stmt, &[&code]).await?;

    if let Some(row) = row {
        Ok(Some(Item {
            code: row.get("code"),
            name: row.get("name"),
            currency: row.get("currency"),
            price: row.get("price"),
            quantity: row.get("quantity"),
            unit: row.get("unit"),
            created: row.get("created"),
            updated: row.get("updated"),
            created_by: row.get("created_by"),
            updated_by: row.get("updated_by"),
        }))
    } else {
        Ok(None)
    }
}
