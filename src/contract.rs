#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};
use sha2::{Sha256, Digest};

use dcdn::Operation;
use dcdn::{DCDNResponse, ContentMetadata};

use self::state::{DCDNState, ContentData, NodeInfo};

pub struct DCDNContract {
    state: DCDNState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(DCDNContract);

impl WithContractAbi for DCDNContract {
    type Abi = dcdn::DCDNAbi;
}

impl Contract for DCDNContract {
    type Message = ();
    type Parameters = ();
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = DCDNState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        DCDNContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        // Initialize application parameters
        self.runtime.application_parameters();
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::Upload { content, metadata } => {
                self.upload_content(content, metadata).await
            }
            Operation::RequestCache { content_id, node_id } => {
                self.request_cache(content_id, node_id).await
            }
            Operation::UpdateAvailability { content_id, node_id, available } => {
                self.update_availability(content_id, node_id, available).await
            }
            Operation::Download { content_id } => {
                self.download_content(content_id).await
            }
            Operation::RegisterNode { node_id, location, capacity } => {
                self.register_node(node_id, location, capacity).await
            }
            Operation::ReportUsage { node_id, content_id, bytes_served } => {
                self.report_usage(node_id, content_id, bytes_served).await
            }
            Operation::UpdateMetadata { content_id, metadata } => {
                self.update_metadata(content_id, metadata).await
            }
        }
    }

    async fn execute_message(&mut self, _message: Self::Message) {}

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

impl DCDNContract {
    async fn upload_content(&mut self, content: Vec<u8>, mut metadata: ContentMetadata) -> DCDNResponse {
        // Generate content ID as SHA256 hash of the content
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let content_hash = format!("{:x}", hasher.finalize());
        let content_id = content_hash.clone();
        
        // Update metadata with the content hash
        metadata.content_hash = Some(content_hash);

        // Check if content already exists
        if self.state.content_map.get(&content_id).await.unwrap().is_some() {
            return DCDNResponse::Error { 
                message: "Content with this hash already exists".to_string() 
            };
        }

        let current_time = self.runtime.system_time();
        let current_time_ticks = current_time.micros();
        
        let content_data = ContentData {
            id: content_id.clone(),
            content,
            metadata,
            created_at: current_time_ticks,
            last_accessed: current_time_ticks,
            access_count: 0,
        };

        self.state.content_map.insert(&content_id, content_data).expect("Failed to insert content");
        
        DCDNResponse::UploadSuccess { content_id }
    }

    async fn download_content(&mut self, content_id: String) -> DCDNResponse {
        match self.state.content_map.get(&content_id).await.unwrap() {
            Some(mut content_data) => {
                // Update last accessed time and access count
                let current_time_ticks = self.runtime.system_time().micros();
                content_data.last_accessed = current_time_ticks;
                content_data.access_count += 1;
                
                self.state.content_map.insert(&content_id, content_data.clone()).expect("Failed to update content");
                
                DCDNResponse::DownloadSuccess { content: content_data.content }
            },
            None => DCDNResponse::Error { 
                message: "Content not found".to_string() 
            },
        }
    }

    async fn request_cache(&mut self, content_id: String, node_id: String) -> DCDNResponse {
        // Check if content exists
        if self.state.content_map.get(&content_id).await.unwrap().is_none() {
            return DCDNResponse::Error { 
                message: "Content does not exist".to_string() 
            };
        }

        // Check if node exists
        if self.state.nodes.get(&node_id).await.unwrap().is_none() {
            return DCDNResponse::Error { 
                message: "Node does not exist".to_string() 
            };
        }

        // Update content availability map
        let mut availability = self.state.content_availability.get(&content_id).await.unwrap().unwrap_or_default();
        if !availability.contains(&node_id) {
            availability.push(node_id);
        }
        self.state.content_availability.insert(&content_id, availability).expect("Failed to update availability");

        DCDNResponse::CacheRequestAccepted
    }

    async fn update_availability(&mut self, content_id: String, node_id: String, available: bool) -> DCDNResponse {
        match self.state.content_availability.get(&content_id).await.unwrap() {
            Some(mut availability) => {
                if available && !availability.contains(&node_id) {
                    availability.push(node_id);
                } else if !available {
                    availability.retain(|id| id != &node_id);
                }
                self.state.content_availability.insert(&content_id, availability).expect("Failed to update availability");
            },
            None => {
                if available {
                    self.state.content_availability.insert(&content_id, vec![node_id]).expect("Failed to update availability");
                }
            }
        }

        DCDNResponse::CacheRequestAccepted
    }

    async fn register_node(&mut self, node_id: String, location: String, capacity: u64) -> DCDNResponse {
        // Check if node already exists
        if self.state.nodes.get(&node_id).await.unwrap().is_some() {
            return DCDNResponse::Error { 
                message: "Node already registered".to_string() 
            };
        }

        let current_time_ticks = self.runtime.system_time().micros();
        let node_info = NodeInfo {
            id: node_id.clone(),
            location,
            capacity,
            used_capacity: 0,
            available: true,
            created_at: current_time_ticks,
            data_served: 0,
        };

        self.state.nodes.insert(&node_id, node_info).expect("Failed to insert node");
        
        let count = *self.state.node_count.get();
        self.state.node_count.set(count + 1);
        
        let total = *self.state.total_capacity.get();
        self.state.total_capacity.set(total + capacity);

        DCDNResponse::NodeRegistered
    }

    async fn report_usage(&mut self, node_id: String, content_id: String, bytes_served: u64) -> DCDNResponse {
        // Check if node exists
        if self.state.nodes.get(&node_id).await.unwrap().is_none() {
            return DCDNResponse::Error { 
                message: "Node does not exist".to_string() 
            };
        }

        // Check if content exists
        if self.state.content_map.get(&content_id).await.unwrap().is_none() {
            return DCDNResponse::Error { 
                message: "Content does not exist".to_string() 
            };
        }

        // Update node's served data
        if let Some(mut node_info) = self.state.nodes.get(&node_id).await.unwrap() {
            node_info.data_served += bytes_served;
            node_info.used_capacity = std::cmp::min(node_info.used_capacity + bytes_served, node_info.capacity);
            self.state.nodes.insert(&node_id, node_info).expect("Failed to update node");
        }

        // Update total data served
        let total = *self.state.total_data_served.get();
        self.state.total_data_served.set(total + bytes_served);

        DCDNResponse::UsageReported
    }

    async fn update_metadata(&mut self, content_id: String, metadata: ContentMetadata) -> DCDNResponse {
        match self.state.content_map.get(&content_id).await.unwrap() {
            Some(mut content_data) => {
                content_data.metadata = metadata;
                self.state.content_map.insert(&content_id, content_data).expect("Failed to update content");
                DCDNResponse::MetadataUpdated
            },
            None => DCDNResponse::Error {
                message: "Content not found".to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::FutureExt as _;
    use linera_sdk::{util::BlockingWait, views::View, Contract, ContractRuntime};

    use dcdn::{Operation, ContentMetadata};

    use super::{DCDNContract, DCDNState};



    fn create_and_instantiate_app() -> DCDNContract {
        let runtime = ContractRuntime::new().with_application_parameters(());
        let mut contract = DCDNContract {
            state: DCDNState::load(runtime.root_view_storage_context())
                .blocking_wait()
                .expect("Failed to read from mock key value store"),
            runtime,
        };

        contract
            .instantiate(())
            .now_or_never()
            .expect("Initialization of application state should not await anything");

        contract
    }
}