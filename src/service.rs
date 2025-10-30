#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use std::sync::Arc;

use async_graphql::{EmptySubscription, Object, Schema};
use linera_sdk::{
    graphql::GraphQLMutationRoot, linera_base_types::WithServiceAbi, views::View, Service,
    ServiceRuntime,
};

use dcdn::Operation;

use self::state::DCDNState;

pub struct DCDNService {
    state: Arc<DCDNState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(DCDNService);

impl WithServiceAbi for DCDNService {
    type Abi = dcdn::DCDNAbi;
}

impl Service for DCDNService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = DCDNState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        DCDNService {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, query: Self::Query) -> Self::QueryResponse {
        Schema::build(
            QueryRoot,
            Operation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .data(Arc::clone(&self.state))
        .finish()
        .execute(query)
        .await
    }
}

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn content_exists(&self, ctx: &async_graphql::Context<'_>, content_id: String) -> bool {
        let state = ctx.data::<Arc<DCDNState>>().unwrap();
        state.content_map.get(&content_id).await.unwrap().is_some()
    }

    async fn get_node_count(&self, ctx: &async_graphql::Context<'_>) -> u64 {
        let state = ctx.data::<Arc<DCDNState>>().unwrap();
        *state.node_count.get()
    }

    async fn get_total_capacity(&self, ctx: &async_graphql::Context<'_>) -> u64 {
        let state = ctx.data::<Arc<DCDNState>>().unwrap();
        *state.total_capacity.get()
    }

    async fn get_total_data_served(&self, ctx: &async_graphql::Context<'_>) -> u64 {
        let state = ctx.data::<Arc<DCDNState>>().unwrap();
        *state.total_data_served.get()
    }

    async fn get_node(&self, ctx: &async_graphql::Context<'_>, node_id: String) -> Option<NodeQuery> {
        let state = ctx.data::<Arc<DCDNState>>().unwrap();
        match state.nodes.get(&node_id).await.unwrap() {
            Some(node_info) => Some(NodeQuery {
                id: node_info.id,
                location: node_info.location,
                capacity: node_info.capacity,
                used_capacity: node_info.used_capacity,
                available: node_info.available,
                data_served: node_info.data_served,
            }),
            None => None,
        }
    }

    async fn get_content_metadata(&self, ctx: &async_graphql::Context<'_>, content_id: String) -> Option<ContentMetadataQuery> {
        let state = ctx.data::<Arc<DCDNState>>().unwrap();
        match state.content_map.get(&content_id).await.unwrap() {
            Some(content_data) => Some(ContentMetadataQuery {
                id: content_data.id,
                name: content_data.metadata.name,
                size: content_data.metadata.size,
                content_type: content_data.metadata.content_type,
                owner: content_data.metadata.owner,
                created_at: content_data.metadata.created_at,
                expires_at: content_data.metadata.expires_at,
                content_hash: content_data.metadata.content_hash,
            }),
            None => None,
        }
    }

    async fn get_content_nodes(&self, ctx: &async_graphql::Context<'_>, content_id: String) -> Vec<String> {
        let state = ctx.data::<Arc<DCDNState>>().unwrap();
        state.content_availability.get(&content_id).await.unwrap().unwrap_or_default()
    }

    async fn get_popular_content(&self, ctx: &async_graphql::Context<'_>, limit: Option<i32>) -> Vec<ContentMetadataQuery> {
        let _state = ctx.data::<Arc<DCDNState>>().unwrap();
        let _limit = limit.unwrap_or(10) as usize;
        let content_list = Vec::new();
        
        // Get all content and sort by access count
        // This is a simplified version - in a real implementation you'd need to iterate through all content
        // For this demo, we'll return an empty list as we can't efficiently iterate all content in this query
        content_list
    }

    async fn get_node_performance(&self, ctx: &async_graphql::Context<'_>, node_id: String) -> Option<NodePerformanceQuery> {
        let state = ctx.data::<Arc<DCDNState>>().unwrap();
        match state.nodes.get(&node_id).await.unwrap() {
            Some(node_info) => {
                // Calculate performance metrics based on data served vs capacity
                let utilization = if node_info.capacity > 0 {
                    (node_info.used_capacity as f64 / node_info.capacity as f64) * 100.0
                } else {
                    0.0
                };
                
                Some(NodePerformanceQuery {
                    node_id,
                    data_served: node_info.data_served,
                    capacity_utilization: utilization,
                    reliability_score: 100.0, // Placeholder - would be calculated based on availability history
                })
            },
            None => None,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
struct NodeQuery {
    id: String,
    location: String,
    capacity: u64,
    used_capacity: u64,
    available: bool,
    data_served: u64,
}

#[derive(async_graphql::SimpleObject)]
struct ContentMetadataQuery {
    id: String,
    name: String,
    size: u64,
    content_type: String,
    owner: String,
    created_at: u64,
    expires_at: Option<u64>,
    content_hash: Option<String>,
}

#[derive(async_graphql::SimpleObject)]
struct NodePerformanceQuery {
    node_id: String,
    data_served: u64,
    capacity_utilization: f64,
    reliability_score: f64,
}