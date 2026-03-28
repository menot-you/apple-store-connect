//! SCM (Source Control Management) model types for the App Store Connect API.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::Resource;

/// An SCM git reference (branch or tag).
pub type ScmGitReference = Resource<ScmGitReferenceAttributes>;

/// Attributes of an SCM git reference.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ScmGitReferenceAttributes {
    /// The reference name (e.g., "main", "v1.0.0").
    pub name: Option<String>,
    /// The kind of reference (e.g., "BRANCH", "TAG").
    pub kind: Option<String>,
    /// Whether the reference has been deleted.
    pub is_deleted: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::JsonApiResponse;

    #[test]
    fn test_deserialize_git_reference() {
        let json = r#"{
            "data": {
                "id": "ref-abc",
                "type": "scmGitReferences",
                "attributes": {
                    "name": "main",
                    "kind": "BRANCH",
                    "isDeleted": false
                }
            }
        }"#;
        let resp: JsonApiResponse<ScmGitReference> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.id, "ref-abc");
        assert_eq!(resp.data.attributes.name.as_deref(), Some("main"));
        assert_eq!(resp.data.attributes.kind.as_deref(), Some("BRANCH"));
        assert_eq!(resp.data.attributes.is_deleted, Some(false));
    }

    #[test]
    fn test_deserialize_git_reference_list() {
        let json = r#"{
            "data": [
                {
                    "id": "ref-1",
                    "type": "scmGitReferences",
                    "attributes": { "name": "main", "kind": "BRANCH" }
                },
                {
                    "id": "ref-2",
                    "type": "scmGitReferences",
                    "attributes": { "name": "v1.0.0", "kind": "TAG" }
                }
            ]
        }"#;
        let resp: JsonApiResponse<Vec<ScmGitReference>> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.data.len(), 2);
        assert_eq!(resp.data[0].attributes.name.as_deref(), Some("main"));
        assert_eq!(resp.data[1].attributes.kind.as_deref(), Some("TAG"));
    }
}
