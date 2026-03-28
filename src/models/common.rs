//! Shared JSON:API envelope types for App Store Connect responses.

use serde::{Deserialize, Serialize};

/// JSON:API response envelope wrapping any data type.
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonApiResponse<T> {
    /// The primary data for the response.
    pub data: T,
    /// Pagination links, if present.
    #[serde(default)]
    pub links: Option<PagedDocumentLinks>,
}

/// Pagination links from a JSON:API document.
#[derive(Debug, Deserialize, Serialize)]
pub struct PagedDocumentLinks {
    /// Link to the current page.
    #[serde(rename = "self")]
    pub self_link: Option<String>,
    /// Link to the next page, if any.
    pub next: Option<String>,
}

/// A JSON:API resource object with typed attributes.
#[derive(Debug, Deserialize, Serialize)]
pub struct Resource<A> {
    /// The unique identifier for this resource.
    pub id: String,
    /// The JSON:API resource type string.
    #[serde(rename = "type")]
    pub resource_type: String,
    /// The resource-specific attributes.
    pub attributes: A,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_response_with_links() {
        let json = r#"{
            "data": {"id": "1", "type": "test", "attributes": {"name": "hello"}},
            "links": {"self": "https://a.com/v1/test/1", "next": "https://a.com/v1/test?cursor=2"}
        }"#;

        #[derive(Debug, Deserialize, Serialize)]
        struct Attrs {
            name: String,
        }

        let resp: JsonApiResponse<Resource<Attrs>> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.id, "1");
        assert_eq!(resp.data.attributes.name, "hello");
        let links = resp.links.unwrap();
        assert!(links.self_link.unwrap().contains("test/1"));
        assert!(links.next.unwrap().contains("cursor=2"));
    }

    #[test]
    fn test_deserialize_response_without_links() {
        let json = r#"{"data": {"id": "2", "type": "x", "attributes": {}}}"#;

        #[derive(Debug, Deserialize, Serialize)]
        struct Empty {}

        let resp: JsonApiResponse<Resource<Empty>> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.id, "2");
        assert!(resp.links.is_none());
    }

    #[test]
    fn test_deserialize_list_response() {
        let json = r#"{"data": [
            {"id": "1", "type": "t", "attributes": {"v": 10}},
            {"id": "2", "type": "t", "attributes": {"v": 20}}
        ]}"#;

        #[derive(Debug, Deserialize, Serialize)]
        struct Val {
            v: i32,
        }

        let resp: JsonApiResponse<Vec<Resource<Val>>> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].attributes.v, 10);
        assert_eq!(resp.data[1].attributes.v, 20);
    }
}
