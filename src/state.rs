use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};
use serde::{Deserialize, Serialize};

use dcdn::ContentMetadata;

#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct DCDNState {
    /// Map of content ID to content data
    pub content_map: MapView<String, ContentData>,
    /// Map of content ID to its availability across nodes
    pub content_availability: MapView<String, Vec<String>>,
    /// Map of node ID to node information
    pub nodes: MapView<String, NodeInfo>,
    /// Total number of registered nodes
    pub node_count: RegisterView<u64>,
    /// Total storage capacity across all nodes
    pub total_capacity: RegisterView<u64>,
    /// Total amount of data served (for payment calculations)
    pub total_data_served: RegisterView<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, async_graphql::SimpleObject)]
pub struct ContentData {
    pub id: String,
    #[graphql(skip)]
    pub content: Vec<u8>,
    pub metadata: ContentMetadata,
    pub created_at: u64,
    pub last_accessed: u64,
    /// Number of times this content has been accessed
    pub access_count: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, async_graphql::SimpleObject)]
pub struct NodeInfo {
    pub id: String,
    pub location: String,
    pub capacity: u64,
    pub used_capacity: u64,
    pub available: bool,
    pub created_at: u64,
    /// Amount of data this node has served
    pub data_served: u64,
}