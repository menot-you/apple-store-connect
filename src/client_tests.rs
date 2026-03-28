//! Tests for the App Store Connect API client.

use super::*;
use std::path::PathBuf;
use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_key_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("test-key.p8")
}

fn test_credentials() -> Arc<Credentials> {
    let pem = std::fs::read(test_key_path()).expect("test fixture missing");
    Arc::new(Credentials::new("KEY".into(), "ISS".into(), pem))
}

fn client_for(server: &MockServer) -> AscClient {
    AscClient::with_base_url(test_credentials(), server.uri())
}

#[tokio::test]
async fn test_list_products() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ciProducts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{
                "id": "p1",
                "type": "ciProducts",
                "attributes": { "name": "MyApp", "productType": "APP" }
            }]
        })))
        .mount(&server)
        .await;

    let resp = client_for(&server).list_products().await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].id, "p1");
}

#[tokio::test]
async fn test_get_product() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ciProducts/p1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": { "id": "p1", "type": "ciProducts", "attributes": { "name": "MyApp" } }
        })))
        .mount(&server)
        .await;

    let resp = client_for(&server).get_product("p1").await.unwrap();
    assert_eq!(resp.data.attributes.name.as_deref(), Some("MyApp"));
}

#[tokio::test]
async fn test_list_workflows_with_pagination() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ciProducts/p1/workflows"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {"id": "w1", "type": "ciWorkflows", "attributes": {"name": "CI"}},
                {"id": "w2", "type": "ciWorkflows", "attributes": {"name": "Release"}}
            ],
            "links": {"self": "https://x/v1/test", "next": "https://x/v1/test?c=2"}
        })))
        .mount(&server)
        .await;

    let resp = client_for(&server).list_workflows("p1").await.unwrap();
    assert_eq!(resp.data.len(), 2);
    assert!(resp.links.unwrap().next.is_some());
}

#[tokio::test]
async fn test_start_build() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/ciBuildRuns"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "data": {
                "id": "run-new", "type": "ciBuildRuns",
                "attributes": { "number": 99, "executionProgress": "PENDING" }
            }
        })))
        .mount(&server)
        .await;

    let resp = client_for(&server)
        .start_build("w1", "ref-main")
        .await
        .unwrap();
    assert_eq!(resp.data.id, "run-new");
    assert_eq!(resp.data.attributes.number, Some(99));
}

#[tokio::test]
async fn test_auth_header_present() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ciProducts"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"data": []})))
        .mount(&server)
        .await;

    let resp = client_for(&server).list_products().await.unwrap();
    assert!(resp.data.is_empty());
}

#[tokio::test]
async fn test_api_error_401() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ciProducts"))
        .respond_with(ResponseTemplate::new(401).set_body_string("Unauthorized"))
        .mount(&server)
        .await;

    let err = client_for(&server).list_products().await.unwrap_err();
    match err {
        ApiError::Status { status, body } => {
            assert_eq!(status, 401);
            assert_eq!(body, "Unauthorized");
        }
        other => panic!("expected Status error, got: {other}"),
    }
}

#[tokio::test]
async fn test_api_error_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/ciProducts/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_string("Not Found"))
        .mount(&server)
        .await;

    let err = client_for(&server)
        .get_product("missing")
        .await
        .unwrap_err();
    assert!(matches!(err, ApiError::Status { status: 404, .. }));
}

// -----------------------------------------------------------------
// App endpoint tests
// -----------------------------------------------------------------

#[tokio::test]
async fn test_list_apps() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/apps"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {"id": "app-1", "type": "apps", "attributes": {"name": "Cool App", "bundleId": "com.ex.cool"}},
                {"id": "app-2", "type": "apps", "attributes": {"name": "Other App"}}
            ]
        })))
        .mount(&server)
        .await;

    let resp = client_for(&server).list_apps().await.unwrap();
    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.data[0].id, "app-1");
    assert_eq!(resp.data[0].attributes.name.as_deref(), Some("Cool App"));
}

#[tokio::test]
async fn test_get_app() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": {"id": "app-1", "type": "apps", "attributes": {"name": "Cool App", "sku": "SKU1"}}
        })))
        .mount(&server)
        .await;

    let resp = client_for(&server).get_app("app-1").await.unwrap();
    assert_eq!(resp.data.attributes.name.as_deref(), Some("Cool App"));
    assert_eq!(resp.data.attributes.sku.as_deref(), Some("SKU1"));
}

// -----------------------------------------------------------------
// Customer Review endpoint tests
// -----------------------------------------------------------------

#[tokio::test]
async fn test_list_customer_reviews() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/apps/app-1/customerReviews"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {"id": "rev-1", "type": "customerReviews", "attributes": {"rating": 5, "title": "Great!"}},
                {"id": "rev-2", "type": "customerReviews", "attributes": {"rating": 3, "title": "OK"}}
            ]
        })))
        .mount(&server)
        .await;

    let resp = client_for(&server)
        .list_customer_reviews("app-1")
        .await
        .unwrap();
    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.data[0].attributes.rating, Some(5));
}

// -----------------------------------------------------------------
// Pagination tests
// -----------------------------------------------------------------

#[tokio::test]
async fn test_get_all_pages_follows_next() {
    let server = MockServer::start().await;
    let page2_url = format!("{}/page2/customerReviews", server.uri());

    // Page 1 — has a next link pointing to a distinct page-2 path
    Mock::given(method("GET"))
        .and(path("/apps/app-1/customerReviews"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {"id": "rev-1", "type": "customerReviews", "attributes": {"rating": 5}}
            ],
            "links": {
                "self": format!("{}/apps/app-1/customerReviews", server.uri()),
                "next": page2_url
            }
        })))
        .mount(&server)
        .await;

    // Page 2 — distinct path, no next link (last page)
    Mock::given(method("GET"))
        .and(path("/page2/customerReviews"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [
                {"id": "rev-2", "type": "customerReviews", "attributes": {"rating": 3}}
            ]
        })))
        .mount(&server)
        .await;

    let reviews = client_for(&server)
        .list_all_customer_reviews("app-1")
        .await
        .unwrap();
    assert_eq!(reviews.len(), 2);
    assert_eq!(reviews[0].id, "rev-1");
    assert_eq!(reviews[1].id, "rev-2");
}

// -----------------------------------------------------------------
// Rate limiting tests
// -----------------------------------------------------------------

#[tokio::test]
async fn test_rate_limit_retry_then_success() {
    let server = MockServer::start().await;

    // First call returns 429 with Retry-After: 0 (instant retry for test speed)
    Mock::given(method("GET"))
        .and(path("/apps"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "0"))
        .up_to_n_times(1)
        .expect(1)
        .mount(&server)
        .await;

    // Second call succeeds
    Mock::given(method("GET"))
        .and(path("/apps"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{"id": "app-1", "type": "apps", "attributes": {"name": "Recovered"}}]
        })))
        .expect(1)
        .mount(&server)
        .await;

    let resp = client_for(&server).list_apps().await.unwrap();
    assert_eq!(resp.data.len(), 1);
    assert_eq!(resp.data[0].attributes.name.as_deref(), Some("Recovered"));
}

#[tokio::test]
async fn test_rate_limit_exhausted() {
    let server = MockServer::start().await;

    // Always return 429
    Mock::given(method("GET"))
        .and(path("/apps"))
        .respond_with(ResponseTemplate::new(429).insert_header("retry-after", "0"))
        .mount(&server)
        .await;

    let err = client_for(&server).list_apps().await.unwrap_err();
    assert!(matches!(err, ApiError::RateLimited));
}

// -----------------------------------------------------------------
// Sales report endpoint tests
// -----------------------------------------------------------------

#[tokio::test]
async fn test_get_sales_report() {
    use flate2::Compression;
    use flate2::write::GzEncoder;
    use std::io::Write;

    let header = "Provider\tProvider Country\tSKU\tDeveloper\tTitle\tVersion\t\
                  Product Type Identifier\tUnits\tDeveloper Proceeds\t\
                  Currency of Proceeds\tBegin Date\tEnd Date\tCustomer Currency\t\
                  Customer Price\tPromo Code\tParent Identifier\tSubscription\t\
                  Period\tCategory\tCMB\tDevice\tSupported Platforms\t\
                  Proceeds Reason\tPreserved Pricing\tClient\tOrder Type";
    let row = "APPLE\tUS\tSKU1\tDev\tApp1\t1.0\t1F\t10\t7.00\t\
               USD\t03/01/2026\t03/31/2026\tUSD\t0.99\t\t\t\t\tGames\t\t\
               iPhone\tiOS\t\t\tApp Store\tBuy";
    let tsv = format!("{header}\n{row}\n");

    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(tsv.as_bytes()).unwrap();
    let compressed = encoder.finish().unwrap();

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/salesReports"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(compressed, "application/a-gzip"))
        .mount(&server)
        .await;

    let rows = client_for(&server)
        .get_sales_report("85000", "SALES", "SUMMARY", "DAILY", "2026-03-01")
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].sku, "SKU1");
    assert_eq!(rows[0].units, "10");
}
