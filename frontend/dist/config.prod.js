// Production Configuration for dCDN
const dcdnConfig = {
    // Application ID - SET THIS TO YOUR DEPLOYED APPLICATION ID
    applicationId: '9a6140207dec406bb0f67fb98cda7cc925b90d84c2ef41e6c98cf2e38fda926f',
    
    // Network settings - adjust as needed for production
    faucetUrl: 'http://localhost:8080', // Local faucet for development
    
    // UI settings
    ui: {
        maxFileSize: 100 * 1024 * 1024, // 100MB max file size
        uploadTimeout: 30000, // 30 seconds timeout
        defaultPageSize: 20,
    },
    
    // Content settings
    content: {
        allowedTypes: [
            'text/plain',
            'text/html', 
            'text/css',
            'text/javascript',
            'application/javascript',
            'application/json',
            'image/jpeg',
            'image/png',
            'image/gif',
            'image/webp',
            'application/pdf',
            'application/octet-stream'
        ]
    }
};

// Export for use in the frontend
if (typeof module !== 'undefined' && module.exports) {
    module.exports = dcdnConfig;
} else {
    window.dcdnConfig = dcdnConfig;
}
