#[cfg(test)]
mod tests {
    use chrono::Utc;
    use database::item::{add, delete, get, list, update, Item};
    use tokio_postgres::{Client, NoTls};

    async fn get_test_client() -> Client {
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

    async fn create_dummy_item(client: &Client, test_code: &str) -> Item {
        let item = Item {
            code: test_code.to_string(),
            name: "Test Item".to_string(),
            currency: "USD".to_string(),
            price: 10.0,
            quantity: 100.0,
            unit: Some("pcs".to_string()),
            created: Utc::now(),
            updated: Utc::now(),
            created_by: Some("tester".to_string()),
            updated_by: Some("tester".to_string()),
        };
        add(client, &item).await.unwrap();
        item
    }

    async fn remove_dummy_item(client: &Client, code: &str) {
        delete(client, code).await.unwrap();
    }

    #[tokio::test]
    async fn test_add() {
        let client = get_test_client().await;
        let item = create_dummy_item(&client, "ABC001").await;

        let fetched_item = get(&client, &item.code).await.unwrap();
        assert!(fetched_item.is_some());

        remove_dummy_item(&client, &item.code).await;
    }

    #[tokio::test]
    async fn test_delete() {
        let client = get_test_client().await;
        let item = create_dummy_item(&client, "ABC002").await;

        remove_dummy_item(&client, &item.code).await;

        let fetched_item = get(&client, &item.code).await.unwrap();
        assert!(fetched_item.is_none());
    }

    #[tokio::test]
    async fn test_update() {
        let client = get_test_client().await;
        let mut item = create_dummy_item(&client, "ABC003").await;

        item.name = "Updated Test Item".to_string();
        update(&client, &item).await.unwrap();

        let fetched_item = get(&client, &item.code).await.unwrap();
        assert!(fetched_item.is_some());
        assert_eq!(fetched_item.unwrap().name, "Updated Test Item");

        remove_dummy_item(&client, &item.code).await;
    }

    #[tokio::test]
    async fn test_list() {
        let client = get_test_client().await;
        let item = create_dummy_item(&client, "ABC004").await;

        let items = list(&client).await.unwrap();
        assert!(items.iter().any(|i| i.code == item.code));

        remove_dummy_item(&client, &item.code).await;
    }

    #[tokio::test]
    async fn test_get() {
        let client = get_test_client().await;
        let item = create_dummy_item(&client, "ABC005").await;

        let fetched_item = get(&client, &item.code).await.unwrap();
        assert!(fetched_item.is_some());
        assert_eq!(fetched_item.unwrap().code, item.code);

        remove_dummy_item(&client, &item.code).await;
    }
}
