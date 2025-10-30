use async_graphql::{Request, Response};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};

pub struct DCDNAbi;

impl ContractAbi for DCDNAbi {
    type Operation = Operation;
    type Response = DCDNResponse;
}

impl ServiceAbi for DCDNAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum Operation {
    /// Upload content to the CDN
    Upload {
        content: Vec<u8>,
        metadata: ContentMetadata,
    },
    /// Request to cache content on a specific node
    RequestCache {
        content_id: String,
        node_id: String,
    },
    /// Update content availability information
    UpdateAvailability {
        content_id: String,
        node_id: String,
        available: bool,
    },
    /// Request content download
    Download {
        content_id: String,
    },
    /// Register a new CDN node
    RegisterNode {
        node_id: String,
        location: String,
        capacity: u64,
    },
    /// Report bandwidth usage for payment
    ReportUsage {
        node_id: String,
        content_id: String,
        bytes_served: u64,
    },
    /// Update content metadata
    UpdateMetadata {
        content_id: String,
        metadata: ContentMetadata,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DCDNResponse {
    UploadSuccess { content_id: String },
    DownloadSuccess { content: Vec<u8> },
    CacheRequestAccepted,
    NodeRegistered,
    UsageReported,
    MetadataUpdated,
    Error { message: String },
}

#[derive(Debug, Deserialize, Serialize, Clone, async_graphql::InputObject, async_graphql::SimpleObject)]
pub struct ContentMetadata {
    pub name: String,
    pub size: u64,
    pub content_type: String,
    pub owner: String,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    /// Hash of the content for integrity verification
    pub content_hash: Option<String>,
}