//! Tests for CI model deserialization.

use super::*;
use crate::models::JsonApiResponse;

#[test]
fn test_deserialize_product() {
    let json = r#"{
        "data": {
            "id": "prod-123",
            "type": "ciProducts",
            "attributes": {
                "name": "MyApp",
                "productType": "APP",
                "bundleId": "com.example.myapp",
                "createdDate": "2026-01-15T10:00:00Z"
            }
        }
    }"#;
    let resp: JsonApiResponse<CiProduct> = serde_json::from_str(json).unwrap();
    assert_eq!(resp.data.id, "prod-123");
    assert_eq!(resp.data.attributes.name.as_deref(), Some("MyApp"));
    assert_eq!(resp.data.attributes.product_type, Some(ProductType::App));
    assert!(resp.data.attributes.created_date.is_some());
}

#[test]
fn test_deserialize_product_unknown_type() {
    let json = r#"{
        "data": {
            "id": "prod-456",
            "type": "ciProducts",
            "attributes": { "productType": "WIDGET" }
        }
    }"#;
    let resp: JsonApiResponse<CiProduct> = serde_json::from_str(json).unwrap();
    assert_eq!(
        resp.data.attributes.product_type,
        Some(ProductType::Unknown)
    );
}

#[test]
fn test_deserialize_build_run() {
    let json = r#"{
        "data": {
            "id": "run-456",
            "type": "ciBuildRuns",
            "attributes": {
                "number": 42,
                "createdDate": "2026-03-01T12:00:00Z",
                "startedDate": "2026-03-01T12:01:00Z",
                "finishedDate": "2026-03-01T12:15:00Z",
                "sourceCommit": {
                    "commitSha": "abc123def",
                    "message": "fix: resolve build issue"
                },
                "executionProgress": "COMPLETE",
                "completionStatus": "SUCCEEDED"
            }
        }
    }"#;
    let resp: JsonApiResponse<CiBuildRun> = serde_json::from_str(json).unwrap();
    let attrs = &resp.data.attributes;
    assert_eq!(attrs.number, Some(42));
    assert_eq!(attrs.completion_status, Some(CompletionStatus::Succeeded));
    assert_eq!(attrs.execution_progress, Some(ExecutionProgress::Complete));
    let commit = attrs.source_commit.as_ref().unwrap();
    assert_eq!(commit.commit_sha.as_deref(), Some("abc123def"));
}

#[test]
fn test_deserialize_build_action() {
    let json = r#"{
        "data": {
            "id": "action-789",
            "type": "ciBuildActions",
            "attributes": {
                "name": "Build - iOS",
                "actionType": "BUILD",
                "startedDate": "2026-03-01T12:01:00Z",
                "finishedDate": "2026-03-01T12:10:00Z",
                "issueCounts": {
                    "analyzerWarnings": 0, "errors": 0,
                    "testFailures": 0, "warnings": 2
                },
                "executionProgress": "COMPLETE",
                "completionStatus": "SUCCEEDED"
            }
        }
    }"#;
    let resp: JsonApiResponse<CiBuildAction> = serde_json::from_str(json).unwrap();
    let attrs = &resp.data.attributes;
    assert_eq!(attrs.name.as_deref(), Some("Build - iOS"));
    assert_eq!(attrs.action_type, Some(ActionType::Build));
    assert_eq!(attrs.issue_counts.as_ref().unwrap().warnings, Some(2));
}

#[test]
fn test_deserialize_workflow() {
    let json = r#"{
        "data": {
            "id": "wf-001",
            "type": "ciWorkflows",
            "attributes": {
                "name": "Release Build",
                "description": "Builds for App Store",
                "isEnabled": true,
                "isLockedForEditing": false,
                "clean": true,
                "lastModifiedDate": "2026-02-20T08:00:00Z"
            }
        }
    }"#;
    let resp: JsonApiResponse<CiWorkflow> = serde_json::from_str(json).unwrap();
    assert_eq!(resp.data.attributes.name.as_deref(), Some("Release Build"));
    assert_eq!(resp.data.attributes.is_enabled, Some(true));
}

#[test]
fn test_deserialize_artifact() {
    let json = r#"{
        "data": {
            "id": "art-001",
            "type": "ciArtifacts",
            "attributes": {
                "name": "MyApp.ipa",
                "fileType": "application/octet-stream",
                "fileSize": 52428800,
                "downloadUrl": "https://example.com/download/art-001"
            }
        }
    }"#;
    let resp: JsonApiResponse<CiArtifact> = serde_json::from_str(json).unwrap();
    assert_eq!(resp.data.id, "art-001");
    assert_eq!(resp.data.attributes.file_size, Some(52428800));
}

#[test]
fn test_enum_unknown_variants() {
    assert_eq!(
        serde_json::from_str::<ExecutionProgress>(r#""BIZARRE""#).unwrap(),
        ExecutionProgress::Unknown
    );
    assert_eq!(
        serde_json::from_str::<CompletionStatus>(r#""BIZARRE""#).unwrap(),
        CompletionStatus::Unknown
    );
    assert_eq!(
        serde_json::from_str::<ActionType>(r#""BIZARRE""#).unwrap(),
        ActionType::Unknown
    );
}
