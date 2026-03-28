//! # asc-mcp
//!
//! MCP server for the Apple App Store Connect API.
//!
//! This crate provides:
//! - JWT authentication for the ASC API ([`auth`])
//! - A typed HTTP client with rate-limiting and pagination ([`client`])
//! - Domain endpoint methods ([`client_endpoints`])
//! - JSON:API data models ([`models`])
//! - An MCP tool server ([`tools`])

#![forbid(unsafe_code)]

pub mod auth;
pub mod client;
mod client_endpoints;
pub mod models;
pub mod tools;
