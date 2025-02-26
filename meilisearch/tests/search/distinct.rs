use meili_snap::snapshot;
use once_cell::sync::Lazy;

use crate::common::{Server, Value};
use crate::json;

pub(self) static DOCUMENTS: Lazy<Value> = Lazy::new(|| {
    json!([
      {
        "id": 1,
        "description": "Leather Jacket",
        "brand": "Lee Jeans",
        "product_id": "123456",
        "color": "Brown"
      },
      {
        "id": 2,
        "description": "Leather Jacket",
        "brand": "Lee Jeans",
        "product_id": "123456",
        "color": "Black"
      },
      {
        "id": 3,
        "description": "Leather Jacket",
        "brand": "Lee Jeans",
        "product_id": "123456",
        "color": "Blue"
      },
      {
        "id": 4,
        "description": "T-Shirt",
        "brand": "Nike",
        "product_id": "789012",
        "color": "Red"
      },
      {
        "id": 5,
        "description": "T-Shirt",
        "brand": "Nike",
        "product_id": "789012",
        "color": "Blue"
      },
      {
        "id": 6,
        "description": "Running Shoes",
        "brand": "Adidas",
        "product_id": "456789",
        "color": "Black"
      },
      {
        "id": 7,
        "description": "Running Shoes",
        "brand": "Adidas",
        "product_id": "456789",
        "color": "White"
      },
      {
        "id": 8,
        "description": "Hoodie",
        "brand": "Puma",
        "product_id": "987654",
        "color": "Gray"
      },
      {
        "id": 9,
        "description": "Sweater",
        "brand": "Gap",
        "product_id": "234567",
        "color": "Green"
      },
      {
        "id": 10,
        "description": "Sweater",
        "brand": "Gap",
        "product_id": "234567",
        "color": "Red"
      },
      {
        "id": 11,
        "description": "Sweater",
        "brand": "Gap",
        "product_id": "234567",
        "color": "Blue"
      },
      {
        "id": 12,
        "description": "Jeans",
        "brand": "Levi's",
        "product_id": "345678",
        "color": "Indigo"
      },
      {
        "id": 13,
        "description": "Jeans",
        "brand": "Levi's",
        "product_id": "345678",
        "color": "Black"
      },
      {
        "id": 14,
        "description": "Jeans",
        "brand": "Levi's",
        "product_id": "345678",
        "color": "Stone Wash"
      }
    ])
});

pub(self) static DOCUMENT_PRIMARY_KEY: &str = "id";
pub(self) static DOCUMENT_DISTINCT_KEY: &str = "product_id";

/// testing: https://github.com/meilisearch/meilisearch/issues/4078
#[actix_rt::test]
async fn distinct_search_with_offset_no_ranking() {
    let server = Server::new().await;
    let index = server.index("test");

    let documents = DOCUMENTS.clone();
    index.add_documents(documents, Some(DOCUMENT_PRIMARY_KEY)).await;
    index.update_distinct_attribute(json!(DOCUMENT_DISTINCT_KEY)).await;
    index.wait_task(1).await;

    fn get_hits(response: &Value) -> Vec<&str> {
        let hits_array = response["hits"].as_array().unwrap();
        hits_array.iter().map(|h| h[DOCUMENT_DISTINCT_KEY].as_str().unwrap()).collect::<Vec<_>>()
    }

    let (response, code) = index.search_post(json!({"offset": 0, "limit": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"2");
    snapshot!(format!("{:?}", hits), @r#"["123456", "789012"]"#);
    snapshot!(response["estimatedTotalHits"] , @"11");

    let (response, code) = index.search_post(json!({"offset": 2, "limit": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"2");
    snapshot!(format!("{:?}", hits), @r#"["456789", "987654"]"#);
    snapshot!(response["estimatedTotalHits"], @"10");

    let (response, code) = index.search_post(json!({"offset": 4, "limit": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"2");
    snapshot!(format!("{:?}", hits), @r#"["234567", "345678"]"#);
    snapshot!(response["estimatedTotalHits"], @"6");

    let (response, code) = index.search_post(json!({"offset": 5, "limit": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"1");
    snapshot!(format!("{:?}", hits), @r#"["345678"]"#);
    snapshot!(response["estimatedTotalHits"], @"6");

    let (response, code) = index.search_post(json!({"offset": 6, "limit": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"0");
    snapshot!(format!("{:?}", hits), @r#"[]"#);
    snapshot!(response["estimatedTotalHits"], @"6");

    let (response, code) = index.search_post(json!({"offset": 7, "limit": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"0");
    snapshot!(format!("{:?}", hits), @r#"[]"#);
    snapshot!(response["estimatedTotalHits"], @"6");
}

/// testing: https://github.com/meilisearch/meilisearch/issues/4130
#[actix_rt::test]
async fn distinct_search_with_pagination_no_ranking() {
    let server = Server::new().await;
    let index = server.index("test");

    let documents = DOCUMENTS.clone();
    index.add_documents(documents, Some(DOCUMENT_PRIMARY_KEY)).await;
    index.update_distinct_attribute(json!(DOCUMENT_DISTINCT_KEY)).await;
    index.wait_task(1).await;

    fn get_hits(response: &Value) -> Vec<&str> {
        let hits_array = response["hits"].as_array().unwrap();
        hits_array.iter().map(|h| h[DOCUMENT_DISTINCT_KEY].as_str().unwrap()).collect::<Vec<_>>()
    }

    let (response, code) = index.search_post(json!({"page": 0, "hitsPerPage": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"0");
    snapshot!(format!("{:?}", hits), @r#"[]"#);
    snapshot!(response["page"], @"0");
    snapshot!(response["totalPages"], @"3");
    snapshot!(response["totalHits"], @"6");

    let (response, code) = index.search_post(json!({"page": 1, "hitsPerPage": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"2");
    snapshot!(format!("{:?}", hits), @r#"["123456", "789012"]"#);
    snapshot!(response["page"], @"1");
    snapshot!(response["totalPages"], @"3");
    snapshot!(response["totalHits"], @"6");

    let (response, code) = index.search_post(json!({"page": 2, "hitsPerPage": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"2");
    snapshot!(format!("{:?}", hits), @r#"["456789", "987654"]"#);
    snapshot!(response["page"], @"2");
    snapshot!(response["totalPages"], @"3");
    snapshot!(response["totalHits"], @"6");

    let (response, code) = index.search_post(json!({"page": 3, "hitsPerPage": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"2");
    snapshot!(format!("{:?}", hits), @r#"["234567", "345678"]"#);
    snapshot!(response["page"], @"3");
    snapshot!(response["totalPages"], @"3");
    snapshot!(response["totalHits"], @"6");

    let (response, code) = index.search_post(json!({"page": 4, "hitsPerPage": 2})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"0");
    snapshot!(format!("{:?}", hits), @r#"[]"#);
    snapshot!(response["page"], @"4");
    snapshot!(response["totalPages"], @"3");
    snapshot!(response["totalHits"], @"6");

    let (response, code) = index.search_post(json!({"page": 2, "hitsPerPage": 3})).await;
    let hits = get_hits(&response);
    snapshot!(code, @"200 OK");
    snapshot!(hits.len(), @"3");
    snapshot!(format!("{:?}", hits), @r#"["987654", "234567", "345678"]"#);
    snapshot!(response["page"], @"2");
    snapshot!(response["totalPages"], @"2");
    snapshot!(response["totalHits"], @"6");
}
