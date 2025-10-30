# Decentralized Content Delivery Network (dCDN)

A high-performance, decentralized content delivery network built on Linera's microchain architecture.

## Overview

The dCDN project aims to create a distributed content delivery system that leverages Linera's unique microchain capabilities to provide:
- Ultra-low latency content delivery
- Resistance to censorship and single points of failure
- Cost-effective content distribution through community participation
- Real-time content availability across a global network
- Content integrity verification through cryptographic hashing

## Architecture

The dCDN system operates using Linera's multi-chain architecture where:

- **Content Providers**: Each content creator operates on their own microchain
- **Edge Nodes**: Independent operators run caching nodes across multiple chains
- **Request Routing**: Smart routing across chains based on proximity and availability
- **Payment System**: Microtransactions for content delivery, settled instantly across chains

### Data Structure
- **ContentMap**: Maps content IDs (SHA256 hashes) to content data with metadata
- **NodeMap**: Tracks all registered CDN nodes with location, capacity, and performance metrics
- **AvailabilityMap**: Tracks which content is cached on which nodes
- **Metrics**: Network-wide statistics (total capacity, data served, node count)

### Key Architecture Benefits
- **Scalability**: Parallel processing across chains
- **Integrity**: Automatic SHA256 verification
- **Performance**: Real-time content availability and node metrics
- **Decentralization**: No single points of failure
- **Economic Incentives**: Built-in usage tracking for payments

## Key Improvements

### 1. Content Addressing & Integrity
- Content ID generated from SHA256 hash of content
- Automatic verification of content integrity
- Deduplication of identical content across the network

### 2. Enhanced Metrics & Analytics
- Content access tracking
- Node performance metrics
- Data served statistics
- Capacity utilization monitoring

### 3. Node Performance Management
- Data served tracking per node
- Capacity utilization metrics
- Reliability scoring system
- Geographic location awareness

### 4. Updated Operations
- Content hash verification during upload/download
- Metadata update functionality
- Performance monitoring queries

## Features

- **Content Upload/Download**: Secure and efficient content storage and retrieval with integrity verification
- **Content Addressing**: Automatic generation of content IDs based on cryptographic hashes
- **Node Registration**: Anyone can join the network as a CDN node operator
- **Content Caching**: Intelligent caching strategies across distributed nodes
- **Usage Tracking**: Real-time metrics and payment reporting with per-node accounting
- **Availability Management**: Dynamic content availability across the network
- **Performance Analytics**: Node and content performance metrics

## Operations

### Core Operations:
- `Upload(content, metadata)`: Upload content (ID auto-generated from hash) with metadata
- `Download(content_id)`: Request and retrieve content by ID
- `RegisterNode(node_id, location, capacity)`: Register a new CDN node with location and capacity
- `RequestCache(content_id, node_id)`: Request to cache content on a specific node
- `UpdateAvailability(content_id, node_id, available)`: Update content availability status
- `ReportUsage(node_id, content_id, bytes_served)`: Report bandwidth usage for payment calculations
- `UpdateMetadata(content_id, metadata)`: Update content metadata

### Queries:
- `contentExists(content_id)`: Check if content is available
- `getNodeCount()`: Get total number of registered nodes
- `getTotalCapacity()`: Get aggregate storage capacity
- `getTotalDataServed()`: Get total bytes served across network
- `getNode(node_id)`: Get information about a specific node
- `getContentMetadata(content_id)`: Retrieve content details including hash
- `getContentNodes(content_id)`: List nodes where content is cached
- `getNodePerformance(node_id)`: Get performance metrics for a specific node
- `getPopularContent(limit)`: Get most accessed content (limited implementation)

## Why Linera?

dCDN leverages Linera's unique advantages:
- **Parallel Processing**: Handle thousands of simultaneous content requests
- **Low Latency**: Fast finality ensures minimal delay for content delivery
- **Cross-Chain Communication**: Seamless content sharing and redundancy
- **Real-time Scaling**: Dynamic addition of new edge nodes based on demand

## Getting Started

### Prerequisites
- Rust (version specified in rust-toolchain.toml)
- Linera SDK

### Building
```bash
cargo build
# For WebAssembly target (Linera's requirement)
cargo build --target wasm32-unknown-unknown --release
```

### Testing
```bash
cargo test
```

### Deployment to Linera Testnet
```bash
# After installing Linera CLI
linera net up
cd /path/to/dcdn
linera project publish-and-create dcdn
```

### Key Features
- **Content Integrity**: Automatic SHA256 content verification
- **Performance Monitoring**: Node performance and reliability metrics
- **Decentralized Caching**: Content availability across multiple nodes
- **Payment Ready**: Usage reporting for economic incentives
- **Real-time Queries**: GraphQL interface for fast data retrieval
```

### Example Usage Flow
1. A content creator uploads a video file using the `Upload` operation
2. The system generates a SHA256 hash as the content ID for integrity
3. CDN nodes register with location and capacity information
4. Request routing algorithms determine optimal caching locations
5. Content gets cached across multiple nodes for redundancy
6. End users download content from the nearest available node
7. Usage metrics are tracked for node operators' compensation

## Use Cases

- **Web3 Applications**: Decentralized hosting for dApps with content integrity
- **Content Creators**: Direct distribution without centralized platforms
- **Enterprises**: Cost-effective alternative to traditional CDNs
- **Developers**: Infrastructure for building on-chain applications
- **Media Companies**: Secure distribution with tamper-proof content verification

## Advanced Features

### Content Integrity
The system uses SHA256 hashing to ensure content integrity:
1. Content ID is generated from the SHA256 hash of the content
2. This prevents duplicate content from being stored
3. Content authenticity can be verified by comparing hashes
4. Eliminates the possibility of content tampering

### Performance Tracking
Advanced metrics for network optimization:
1. Content access frequency tracking
2. Node capacity utilization
3. Data served per node
4. Performance scoring system

## Implementation Highlights

The dCDN implementation showcases advanced Linera concepts:

- **Custom View Implementation**: Uses MapView, RegisterView for efficient state storage
- **GraphQL Integration**: Sophisticated query/mutation system with complex data types
- **Multi-chain State Management**: Handles state across multiple chains efficiently
- **Content Integrity Algorithms**: SHA256 hashing for content verification
- **Performance Tracking**: Real-time metrics and node scoring
- **Async Architecture**: Proper async handling across all components

## Future Enhancements

- Advanced caching algorithms (LRU, LFU, TTL-based)
- Geographic and latency-based routing optimization
- Content encryption and access control
- Advanced analytics and monitoring
- Integration with other Web3 protocols
- Tokenomics and payment system implementation
- CDN governance token for network participation