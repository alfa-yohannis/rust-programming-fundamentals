use tokio_postgres::{Client, Error};

#[derive(Debug)]
pub struct Item {
    pub code: String,
    pub name: String,
    pub currency: String,
    pub price: f32,
    pub quantity: f32,
    pub unit: Option<String>,
}

pub async fn add(client: &Client, item: &Item) -> Result<(), Error> {
    let stmt = client
        .prepare(
            "
            INSERT INTO item (code, name, currency, price, quantity, unit)
            VALUES ($1, $2, $3, $4, $5, $6)
        ",
        )
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
        .prepare(
            "
            UPDATE item SET
            name = $1, currency = $2, price = $3, quantity = $4, 
            unit = $5
            WHERE code = $6
        ",
        )
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
                &item.code,
            ],
        )
        .await?;

    Ok(())
}

pub async fn list(client: &Client) -> Result<Vec<Item>, Error> {
    let rows = client
        .query(
            "SELECT code, name, currency, price, quantity, unit FROM item ORDER BY code",
            &[],
        )
        .await?;

    let items = rows
        .iter()
        .map(|row| Item {
            code: row.get("code"),
            name: row.get("name"),
            currency: row.get("currency"),
            price: row.get("price"),
            quantity: row.get("quantity"),
            unit: row.get("unit"),
        })
        .collect();

    Ok(items)
}

pub async fn get(client: &Client, code: &str) -> Result<Option<Item>, Error> {
    let stmt = client
        .prepare("SELECT code, name, currency, price, quantity, unit FROM item WHERE code = $1")
        .await?;
    let row = client.query_opt(&stmt, &[&code]).await?;

    if let Some(row) = row {
        Ok(Some(Item {
            code: row.get("code"),
            name: row.get("name"),
            currency: row.get("currency"),
            price: row.get("price"),
            quantity: row.get("quantity"),
            unit: row.get("unit"),
        }))
    } else {
        Ok(None)
    }
}
