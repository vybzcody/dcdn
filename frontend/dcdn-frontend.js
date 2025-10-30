// dCDN Frontend JavaScript Implementation

// Initialize the Linera client
let client;
let dcdnBackend;
let applicationId;

// DOM elements
const elements = {
    nodeCount: document.getElementById('node-count'),
    totalCapacity: document.getElementById('total-capacity'),
    dataServed: document.getElementById('data-served'),
    contentCount: document.getElementById('content-count'),
    uploadStatus: document.getElementById('upload-status'),
    contentFile: document.getElementById('content-file'),
    contentName: document.getElementById('content-name'),
    contentType: document.getElementById('content-type'),
    contentOwner: document.getElementById('content-owner'),
    uploadBtn: document.getElementById('upload-btn'),
    downloadStatus: document.getElementById('download-status'),
    contentId: document.getElementById('content-id'),
    downloadBtn: document.getElementById('download-btn'),
    metadataBtn: document.getElementById('metadata-btn'),
    nodeStatus: document.getElementById('node-status'),
    nodeId: document.getElementById('node-id'),
    nodeLocation: document.getElementById('node-location'),
    nodeCapacity: document.getElementById('node-capacity'),
    registerNodeBtn: document.getElementById('register-node-btn'),
    contentStatus: document.getElementById('content-status'),
    contentMetadata: document.getElementById('content-metadata'),
    availabilityStatus: document.getElementById('availability-status'),
    availabilityContentId: document.getElementById('availability-content-id'),
    availabilityBtn: document.getElementById('availability-btn'),
    availabilityResult: document.getElementById('availability-result')
};

// Format bytes to human-readable format
function formatBytes(bytes, decimals = 2) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
}

// Show status message with success or error styling
function showStatus(element, message, isSuccess = true) {
    element.textContent = message;
    element.className = `status ${isSuccess ? 'success' : 'error'}`;
}

// Initialize the application
async function initializeApp() {
    try {
        // Initialize the Linera client
        await linera.default();
        
        // Get the application ID from configuration
        applicationId = window.dcdnConfig ? window.dcdnConfig.applicationId : 'PLACEHOLDER_APP_ID'; 

        // Get the faucet URL from configuration (this will be used to get a wallet for local network)
        const faucetUrl = window.dcdnConfig ? window.dcdnConfig.faucetUrl : 'https://faucet.testnet-conway.linera.net';
        
        // Create a connection to the faucet to get a wallet and chain
        const faucet = await new linera.Faucet(faucetUrl);
        const wallet = await faucet.createWallet();
        client = await new linera.Client(wallet);
        
        // Claim a chain for our client
        const chainId = await faucet.claimChain(client);
        document.querySelector('#chain-id').textContent = chainId;
        
        // Get the dCDN application backend
        if (applicationId !== 'PLACEHOLDER_APP_ID') {
            dcdnBackend = await client.frontend().application(applicationId);
            showStatus(elements.uploadStatus, "dCDN frontend connected and ready!", true);
        } else {
            showStatus(elements.uploadStatus, "dCDN frontend connected but requires deployed app ID to function fully", true);
        }
        
        console.log("dCDN frontend initialized with chain:", chainId);
        
        // Load initial metrics
        await loadMetrics();
        
        // Add event listeners
        setupEventListeners();
        
    } catch (error) {
        console.error("Error initializing dCDN frontend:", error);
        showStatus(elements.uploadStatus, `Error connecting to dCDN: ${error.message}`, false);
    }
}

// Set up event listeners for UI elements
function setupEventListeners() {
    elements.uploadBtn.addEventListener('click', uploadContent);
    elements.downloadBtn.addEventListener('click', downloadContent);
    elements.metadataBtn.addEventListener('click', getContentMetadata);
    elements.registerNodeBtn.addEventListener('click', registerNode);
    elements.availabilityBtn.addEventListener('click', checkContentAvailability);
}

// Load network metrics
async function loadMetrics() {
    try {
        // For a real implementation with actual dCDN backend connection:
        // const metricsQuery = `{
        //   "query": "query { getNodeCount, getTotalCapacity, getTotalDataServed }"
        // }`;
        // const response = await dcdnBackend.query(metricsQuery);
        // const data = JSON.parse(response).data;
        
        // For now, simulate metrics loading
        elements.nodeCount.textContent = '0';
        elements.totalCapacity.textContent = '0 GB';
        elements.dataServed.textContent = '0 GB';
        elements.contentCount.textContent = '0';
    } catch (error) {
        console.error("Error loading metrics:", error);
    }
}

// Upload content to the dCDN
async function uploadContent() {
    const file = elements.contentFile.files[0];
    const name = elements.contentName.value.trim();
    const contentType = elements.contentType.value.trim();
    const owner = elements.contentOwner.value.trim();
    
    if (!file) {
        showStatus(elements.uploadStatus, "Please select a file to upload", false);
        return;
    }
    
    // Validate file size
    if (window.dcdnConfig && file.size > window.dcdnConfig.ui.maxFileSize) {
        showStatus(elements.uploadStatus, `File size exceeds maximum allowed size of ${formatBytes(window.dcdnConfig.ui.maxFileSize)}`, false);
        return;
    }
    
    // Validate content type
    if (window.dcdnConfig && window.dcdnConfig.content.allowedTypes && 
        !window.dcdnConfig.content.allowedTypes.includes(contentType)) {
        showStatus(elements.uploadStatus, `Content type '${contentType}' is not allowed. Allowed types: ${window.dcdnConfig.content.allowedTypes.join(', ')}`, false);
        return;
    }
    
    if (!name || !contentType || !owner) {
        showStatus(elements.uploadStatus, "Please fill in all required fields", false);
        return;
    }
    
    try {
        showStatus(elements.uploadStatus, "Reading file...", true);
        
        // Read the file content
        const contentArrayBuffer = await file.arrayBuffer();
        const content = Array.from(new Uint8Array(contentArrayBuffer));
        
        // Create metadata object
        const metadata = {
            name: name,
            size: file.size,
            content_type: contentType,
            owner: owner,
            created_at: Math.floor(Date.now() / 1000), // Unix timestamp in seconds
            expires_at: null,
            content_hash: null
        };
        
        showStatus(elements.uploadStatus, "Uploading content to dCDN...", true);
        
        // For a real implementation with actual dCDN backend connection:
        // const uploadMutation = {
        //   query: `mutation {
        //     upload(content: ${JSON.stringify(content)}, metadata: ${JSON.stringify(metadata)})
        //   }`
        // };
        // const response = await dcdnBackend.query(JSON.stringify(uploadMutation));
        // const result = JSON.parse(response);
        
        // Simulate upload with timeout from config
        const timeout = window.dcdnConfig ? window.dcdnConfig.ui.uploadTimeout : 30000;
        setTimeout(() => {
            // Generate a mock content ID (in reality, this would be the SHA256 hash)
            const mockContentId = 'mock_' + Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
            showStatus(elements.uploadStatus, `Content uploaded successfully! Content ID: ${mockContentId}`, true);
            
            // Reset form
            elements.contentFile.value = '';
            elements.contentName.value = '';
            elements.contentType.value = '';
            elements.contentOwner.value = '';
            
            // Reload metrics to reflect the new content
            loadMetrics();
        }, timeout / 20); // Use a shorter timeout for simulation
    } catch (error) {
        console.error("Error uploading content:", error);
        showStatus(elements.uploadStatus, `Upload failed: ${error.message}`, false);
    }
}

// Download content from the dCDN
async function downloadContent() {
    const contentId = elements.contentId.value.trim();
    
    if (!contentId) {
        showStatus(elements.downloadStatus, "Please enter a content ID", false);
        return;
    }
    
    try {
        showStatus(elements.downloadStatus, "Fetching content from dCDN...", true);
        
        // For a real implementation with actual dCDN backend connection:
        // const downloadMutation = {
        //   query: `mutation { download(contentId: "${contentId}") }`
        // };
        // const response = await dcdnBackend.query(JSON.stringify(downloadMutation));
        // const result = JSON.parse(response);
        
        // Simulate download
        setTimeout(() => {
            showStatus(elements.downloadStatus, `Content with ID ${contentId} would be downloaded in a real implementation`, true);
        }, 1500);
    } catch (error) {
        console.error("Error downloading content:", error);
        showStatus(elements.downloadStatus, `Download failed: ${error.message}`, false);
    }
}

// Get content metadata
async function getContentMetadata() {
    const contentId = elements.contentId.value.trim();
    
    if (!contentId) {
        showStatus(elements.contentStatus, "Please enter a content ID", false);
        return;
    }
    
    try {
        showStatus(elements.contentStatus, "Fetching content metadata...", true);
        
        // For a real implementation with actual dCDN backend connection:
        // const metadataQuery = {
        //   query: `query { getContentMetadata(contentId: "${contentId}") }`
        // };
        // const response = await dcdnBackend.query(JSON.stringify(metadataQuery));
        // const result = JSON.parse(response);
        
        // Simulate response
        setTimeout(() => {
            const mockMetadata = {
                id: contentId,
                name: "Example Content",
                size: 1024000, // 1MB
                content_type: "application/octet-stream",
                owner: "example-owner",
                created_at: Math.floor((Date.now() - 86400000) / 1000), // 1 day ago in seconds
                expires_at: null,
                content_hash: "sha256_mock_hash_value"
            };
            
            elements.contentMetadata.innerHTML = `
                <h3>Content Metadata</h3>
                <p><strong>ID:</strong> ${mockMetadata.id}</p>
                <p><strong>Name:</strong> ${mockMetadata.name}</p>
                <p><strong>Size:</strong> ${formatBytes(mockMetadata.size)}</p>
                <p><strong>Type:</strong> ${mockMetadata.content_type}</p>
                <p><strong>Owner:</strong> ${mockMetadata.owner}</p>
                <p><strong>Created:</strong> ${new Date(mockMetadata.created_at * 1000).toLocaleString()}</p>
                <p><strong>Hash:</strong> ${mockMetadata.content_hash}</p>
            `;
            
            showStatus(elements.contentStatus, "Metadata loaded successfully", true);
        }, 1000);
    } catch (error) {
        console.error("Error getting content metadata:", error);
        showStatus(elements.contentStatus, `Failed to get metadata: ${error.message}`, false);
    }
}

// Register a new CDN node
async function registerNode() {
    const nodeId = elements.nodeId.value.trim();
    const location = elements.nodeLocation.value.trim();
    const capacity = parseInt(elements.nodeCapacity.value);
    
    if (!nodeId || !location || isNaN(capacity) || capacity <= 0) {
        showStatus(elements.nodeStatus, "Please fill in all fields with valid values", false);
        return;
    }
    
    try {
        showStatus(elements.nodeStatus, "Registering node with dCDN...", true);
        
        // For a real implementation with actual dCDN backend connection:
        // const registerMutation = {
        //   query: `mutation { 
        //     registerNode(nodeId: "${nodeId}", location: "${location}", capacity: ${capacity}) 
        //   }`
        // };
        // const response = await dcdnBackend.query(JSON.stringify(registerMutation));
        // const result = JSON.parse(response);
        
        // Simulate registration
        setTimeout(() => {
            showStatus(elements.nodeStatus, `Node ${nodeId} registered successfully with ${formatBytes(capacity)} capacity`, true);
            
            // Reset form
            elements.nodeId.value = '';
            elements.nodeLocation.value = '';
            elements.nodeCapacity.value = '';
            
            // Reload metrics to reflect the new node
            loadMetrics();
        }, 1500);
    } catch (error) {
        console.error("Error registering node:", error);
        showStatus(elements.nodeStatus, `Node registration failed: ${error.message}`, false);
    }
}

// Check content availability across nodes
async function checkContentAvailability() {
    const contentId = elements.availabilityContentId.value.trim();
    
    if (!contentId) {
        showStatus(elements.availabilityStatus, "Please enter a content ID", false);
        return;
    }
    
    try {
        showStatus(elements.availabilityStatus, "Checking content availability...", true);
        
        // For a real implementation with actual dCDN backend connection:
        // const availabilityQuery = {
        //   query: `query { getContentNodes(contentId: "${contentId}") }`
        // };
        // const response = await dcdnBackend.query(JSON.stringify(availabilityQuery));
        // const result = JSON.parse(response);
        
        // Simulate response
        setTimeout(() => {
            const mockNodes = [
                `node_${Math.random().toString(36).substring(2, 10)}`,
                `node_${Math.random().toString(36).substring(2, 10)}`,
                `node_${Math.random().toString(36).substring(2, 10)}`
            ];
            
            elements.availabilityResult.innerHTML = `
                <h3>Content Availability</h3>
                <p><strong>Content ID:</strong> ${contentId}</p>
                <p><strong>Available on nodes:</strong></p>
                <ul>
                    ${mockNodes.map(node => `<li>${node}</li>`).join('')}
                </ul>
            `;
            
            showStatus(elements.availabilityStatus, `Content available on ${mockNodes.length} nodes`, true);
        }, 1000);
    } catch (error) {
        console.error("Error checking content availability:", error);
        showStatus(elements.availabilityStatus, `Failed to check availability: ${error.message}`, false);
    }
}

// Initialize the app when the page loads
document.addEventListener('DOMContentLoaded', initializeApp);