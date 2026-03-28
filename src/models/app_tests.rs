//! Tests for App and Customer Review model deserialization.

use super::*;
use crate::models::JsonApiResponse;

#[test]
fn test_deserialize_app() {
    let json = r#"{
        "data": {
            "id": "app-123",
            "type": "apps",
            "attributes": {
                "name": "My Cool App",
                "bundleId": "com.example.coolapp",
                "sku": "COOL_APP_001",
                "primaryLocale": "en-US"
            }
        }
    }"#;
    let resp: JsonApiResponse<App> = serde_json::from_str(json).unwrap();
    assert_eq!(resp.data.id, "app-123");
    assert_eq!(resp.data.attributes.name.as_deref(), Some("My Cool App"));
    assert_eq!(
        resp.data.attributes.bundle_id.as_deref(),
        Some("com.example.coolapp")
    );
    assert_eq!(resp.data.attributes.sku.as_deref(), Some("COOL_APP_001"));
    assert_eq!(
        resp.data.attributes.primary_locale.as_deref(),
        Some("en-US")
    );
}

#[test]
fn test_deserialize_app_list() {
    let json = r#"{
        "data": [
            {"id": "a1", "type": "apps", "attributes": {"name": "App One"}},
            {"id": "a2", "type": "apps", "attributes": {"name": "App Two"}}
        ]
    }"#;
    let resp: JsonApiResponse<Vec<App>> = serde_json::from_str(json).unwrap();
    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.data[0].attributes.name.as_deref(), Some("App One"));
    assert_eq!(resp.data[1].attributes.name.as_deref(), Some("App Two"));
}

#[test]
fn test_deserialize_app_minimal() {
    let json = r#"{
        "data": {"id": "a3", "type": "apps", "attributes": {}}
    }"#;
    let resp: JsonApiResponse<App> = serde_json::from_str(json).unwrap();
    assert_eq!(resp.data.id, "a3");
    assert!(resp.data.attributes.name.is_none());
    assert!(resp.data.attributes.bundle_id.is_none());
}

#[test]
fn test_deserialize_customer_review() {
    let json = r#"{
        "data": {
            "id": "rev-001",
            "type": "customerReviews",
            "attributes": {
                "rating": 5,
                "title": "Awesome app!",
                "body": "Love everything about this app.",
                "reviewerNickname": "HappyUser42",
                "territory": "USA",
                "createdDate": "2026-03-15T10:30:00Z"
            }
        }
    }"#;
    let resp: JsonApiResponse<CustomerReview> = serde_json::from_str(json).unwrap();
    assert_eq!(resp.data.id, "rev-001");
    assert_eq!(resp.data.attributes.rating, Some(5));
    assert_eq!(resp.data.attributes.title.as_deref(), Some("Awesome app!"));
    assert_eq!(
        resp.data.attributes.reviewer_nickname.as_deref(),
        Some("HappyUser42")
    );
    assert!(resp.data.attributes.created_date.is_some());
}

#[test]
fn test_deserialize_customer_review_list() {
    let json = r#"{
        "data": [
            {
                "id": "rev-1", "type": "customerReviews",
                "attributes": {"rating": 4, "title": "Good"}
            },
            {
                "id": "rev-2", "type": "customerReviews",
                "attributes": {"rating": 2, "title": "Meh"}
            }
        ]
    }"#;
    let resp: JsonApiResponse<Vec<CustomerReview>> = serde_json::from_str(json).unwrap();
    assert_eq!(resp.data.len(), 2);
    assert_eq!(resp.data[0].attributes.rating, Some(4));
    assert_eq!(resp.data[1].attributes.rating, Some(2));
}

#[test]
fn test_deserialize_customer_review_response() {
    let json = r#"{
        "data": {
            "id": "resp-001",
            "type": "customerReviewResponses",
            "attributes": {
                "responseBody": "Thank you for the kind words!",
                "lastModifiedDate": "2026-03-16T08:00:00Z",
                "state": "PUBLISHED"
            }
        }
    }"#;
    let resp: JsonApiResponse<CustomerReviewResponse> = serde_json::from_str(json).unwrap();
    assert_eq!(resp.data.id, "resp-001");
    assert_eq!(
        resp.data.attributes.response_body.as_deref(),
        Some("Thank you for the kind words!")
    );
    assert_eq!(resp.data.attributes.state.as_deref(), Some("PUBLISHED"));
    assert!(resp.data.attributes.last_modified_date.is_some());
}
