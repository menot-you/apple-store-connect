//! Data models for the App Store Connect API v1 (JSON:API format).

mod app;
mod ci;
mod common;
mod sales;
mod scm;

pub use app::{
    App, AppAttributes, CustomerReview, CustomerReviewAttributes, CustomerReviewResponse,
    CustomerReviewResponseAttributes,
};
pub use ci::{
    ActionType, CiArtifact, CiArtifactAttributes, CiBuildAction, CiBuildActionAttributes,
    CiBuildRun, CiBuildRunAttributes, CiProduct, CiProductAttributes, CiWorkflow,
    CiWorkflowAttributes, CompletionStatus, ExecutionProgress, ProductType,
};
pub use common::{JsonApiResponse, PagedDocumentLinks, Resource};
pub use sales::{SalesReportRow, SalesTsvError, parse_sales_tsv};
pub use scm::{ScmGitReference, ScmGitReferenceAttributes};
