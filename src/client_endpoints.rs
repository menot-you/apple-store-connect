//! Domain-specific endpoint methods for [`AscClient`].
//!
//! Separated from core transport to keep file sizes manageable.

use crate::client::{ApiError, AscClient};
use crate::models::{
    App, CiBuildAction, CiBuildRun, CiProduct, CiWorkflow, CustomerReview, JsonApiResponse,
    SalesReportRow, parse_sales_tsv,
};

impl AscClient {
    // -----------------------------------------------------------------
    // CI endpoints
    // -----------------------------------------------------------------

    /// Lists all Xcode Cloud CI products.
    pub async fn list_products(&self) -> Result<JsonApiResponse<Vec<CiProduct>>, ApiError> {
        self.get("/ciProducts").await
    }

    /// Gets a specific CI product by ID.
    pub async fn get_product(&self, id: &str) -> Result<JsonApiResponse<CiProduct>, ApiError> {
        self.get(&format!("/ciProducts/{id}")).await
    }

    /// Lists workflows for a given CI product.
    pub async fn list_workflows(
        &self,
        product_id: &str,
    ) -> Result<JsonApiResponse<Vec<CiWorkflow>>, ApiError> {
        self.get(&format!("/ciProducts/{product_id}/workflows"))
            .await
    }

    /// Gets a specific workflow by ID.
    pub async fn get_workflow(&self, id: &str) -> Result<JsonApiResponse<CiWorkflow>, ApiError> {
        self.get(&format!("/ciWorkflows/{id}")).await
    }

    /// Lists build runs for a given workflow.
    pub async fn list_build_runs(
        &self,
        workflow_id: &str,
    ) -> Result<JsonApiResponse<Vec<CiBuildRun>>, ApiError> {
        self.get(&format!("/ciWorkflows/{workflow_id}/buildRuns"))
            .await
    }

    /// Gets a specific build run by ID.
    pub async fn get_build_run(&self, id: &str) -> Result<JsonApiResponse<CiBuildRun>, ApiError> {
        self.get(&format!("/ciBuildRuns/{id}")).await
    }

    /// Starts a new build run for a workflow.
    pub async fn start_build(
        &self,
        workflow_id: &str,
        git_reference_id: &str,
    ) -> Result<JsonApiResponse<CiBuildRun>, ApiError> {
        let body = serde_json::json!({
            "data": {
                "type": "ciBuildRuns",
                "relationships": {
                    "workflow": {
                        "data": { "type": "ciWorkflows", "id": workflow_id }
                    },
                    "sourceBranchOrTag": {
                        "data": { "type": "scmGitReferences", "id": git_reference_id }
                    }
                }
            }
        });
        self.post("/ciBuildRuns", &body).await
    }

    /// Lists build actions within a build run.
    pub async fn list_build_actions(
        &self,
        build_run_id: &str,
    ) -> Result<JsonApiResponse<Vec<CiBuildAction>>, ApiError> {
        self.get(&format!("/ciBuildRuns/{build_run_id}/actions"))
            .await
    }

    // -----------------------------------------------------------------
    // App endpoints
    // -----------------------------------------------------------------

    /// Lists all apps in App Store Connect.
    pub async fn list_apps(&self) -> Result<JsonApiResponse<Vec<App>>, ApiError> {
        self.get("/apps").await
    }

    /// Gets a specific app by ID.
    pub async fn get_app(&self, id: &str) -> Result<JsonApiResponse<App>, ApiError> {
        self.get(&format!("/apps/{id}")).await
    }

    // -----------------------------------------------------------------
    // Customer Review endpoints
    // -----------------------------------------------------------------

    /// Lists customer reviews for an app (first page).
    pub async fn list_customer_reviews(
        &self,
        app_id: &str,
    ) -> Result<JsonApiResponse<Vec<CustomerReview>>, ApiError> {
        self.get(&format!("/apps/{app_id}/customerReviews")).await
    }

    /// Lists all customer reviews for an app, following pagination.
    pub async fn list_all_customer_reviews(
        &self,
        app_id: &str,
    ) -> Result<Vec<CustomerReview>, ApiError> {
        self.get_all_pages(&format!("/apps/{app_id}/customerReviews"))
            .await
    }

    // -----------------------------------------------------------------
    // Sales Report endpoints
    // -----------------------------------------------------------------

    /// Downloads and parses a sales report from App Store Connect.
    ///
    /// The API returns gzip-compressed TSV data, which is decompressed
    /// and parsed into structured rows.
    pub async fn get_sales_report(
        &self,
        vendor_number: &str,
        report_type: &str,
        report_sub_type: &str,
        frequency: &str,
        report_date: &str,
    ) -> Result<Vec<SalesReportRow>, ApiError> {
        let query = vec![
            ("filter[vendorNumber]".into(), vendor_number.into()),
            ("filter[reportType]".into(), report_type.into()),
            ("filter[reportSubType]".into(), report_sub_type.into()),
            ("filter[frequency]".into(), frequency.into()),
            ("filter[reportDate]".into(), report_date.into()),
        ];
        let bytes = self
            .get_raw("/salesReports", &query, "application/a-gzip")
            .await?;
        parse_sales_tsv(&bytes).map_err(|e| ApiError::SalesReport(e.to_string()))
    }
}
