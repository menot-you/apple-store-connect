//! Xcode Cloud CI model types for App Store Connect API.

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::common::Resource;

// ---------------------------------------------------------------------------
// Enums for stringly-typed fields
// ---------------------------------------------------------------------------

/// Execution progress of a build run or action.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionProgress {
    Pending,
    Running,
    Complete,
    #[serde(other)]
    Unknown,
}

/// Final completion status of a build run or action.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CompletionStatus {
    Succeeded,
    Failed,
    Errored,
    Canceled,
    Skipped,
    #[serde(other)]
    Unknown,
}

/// Type of a CI build action.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionType {
    Build,
    Analyze,
    Test,
    Archive,
    #[serde(other)]
    Unknown,
}

/// Product type of a CI product.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    App,
    Framework,
    #[serde(other)]
    Unknown,
}

// ---------------------------------------------------------------------------
// Resource types
// ---------------------------------------------------------------------------

/// A CI product (corresponds to an Xcode project/workspace).
pub type CiProduct = Resource<CiProductAttributes>;

/// Attributes of a CI product.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CiProductAttributes {
    /// Display name of the product.
    pub name: Option<String>,
    /// Product type (e.g., APP, FRAMEWORK).
    pub product_type: Option<ProductType>,
    /// Bundle identifier.
    pub bundle_id: Option<String>,
    /// Creation date.
    pub created_date: Option<DateTime<Utc>>,
}

/// A CI workflow defining build/test/archive actions.
pub type CiWorkflow = Resource<CiWorkflowAttributes>;

/// Attributes of a CI workflow.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CiWorkflowAttributes {
    /// Workflow display name.
    pub name: Option<String>,
    /// Human-readable description.
    pub description: Option<String>,
    /// Last-modified date.
    pub last_modified_date: Option<DateTime<Utc>>,
    /// Whether the workflow is enabled.
    pub is_enabled: Option<bool>,
    /// Whether editing is locked.
    pub is_locked_for_editing: Option<bool>,
    /// Whether to perform a clean build.
    pub clean: Option<bool>,
}

/// A CI build run (one execution of a workflow).
pub type CiBuildRun = Resource<CiBuildRunAttributes>;

/// Attributes of a CI build run.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CiBuildRunAttributes {
    /// Build run number.
    pub number: Option<i64>,
    /// Creation timestamp.
    pub created_date: Option<DateTime<Utc>>,
    /// Start timestamp.
    pub started_date: Option<DateTime<Utc>>,
    /// Finish timestamp.
    pub finished_date: Option<DateTime<Utc>>,
    /// Source commit information.
    pub source_commit: Option<SourceCommit>,
    /// Current execution progress.
    pub execution_progress: Option<ExecutionProgress>,
    /// Final completion status.
    pub completion_status: Option<CompletionStatus>,
}

/// Commit information embedded in a build run.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceCommit {
    /// The commit SHA.
    pub commit_sha: Option<String>,
    /// The commit message.
    pub message: Option<String>,
}

/// A single action within a CI build run (build, test, archive, etc.).
pub type CiBuildAction = Resource<CiBuildActionAttributes>;

/// Attributes of a CI build action.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CiBuildActionAttributes {
    /// Action display name.
    pub name: Option<String>,
    /// Action type (e.g., BUILD, TEST, ARCHIVE).
    pub action_type: Option<ActionType>,
    /// Start timestamp.
    pub started_date: Option<DateTime<Utc>>,
    /// Finish timestamp.
    pub finished_date: Option<DateTime<Utc>>,
    /// Issue counts summary.
    pub issue_counts: Option<IssueCounts>,
    /// Current execution progress.
    pub execution_progress: Option<ExecutionProgress>,
    /// Final completion status.
    pub completion_status: Option<CompletionStatus>,
}

/// Issue count summary for a build action.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueCounts {
    /// Number of analyzer warnings.
    pub analyzer_warnings: Option<i64>,
    /// Number of errors.
    pub errors: Option<i64>,
    /// Number of test failures.
    pub test_failures: Option<i64>,
    /// Number of warnings.
    pub warnings: Option<i64>,
}

/// A CI artifact produced by a build action.
pub type CiArtifact = Resource<CiArtifactAttributes>;

/// Attributes of a CI artifact.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CiArtifactAttributes {
    /// Display name of the artifact.
    pub name: Option<String>,
    /// MIME type of the artifact file.
    pub file_type: Option<String>,
    /// Size in bytes.
    pub file_size: Option<i64>,
    /// Direct download URL.
    pub download_url: Option<String>,
}

#[cfg(test)]
#[path = "ci_tests.rs"]
mod tests;
