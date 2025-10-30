#!/bin/bash

# Build script for dCDN frontend
# This script prepares the frontend for production deployment

set -e

echo "Building dCDN frontend for production..."
echo "========================================"

# Check if we're in the frontend directory
if [ ! -f "index.html" ] || [ ! -f "dcdn-frontend.js" ]; then
    echo "Error: This script must be run from the frontend directory"
    exit 1
fi

# Install dependencies if not present
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    npm install
fi

# Create dist directory
DIST_DIR="dist"
if [ -d "$DIST_DIR" ]; then
    rm -rf "$DIST_DIR"
fi
mkdir -p "$DIST_DIR"

# Copy all necessary files to dist
echo "Copying files to dist directory..."
cp index.html config.js dcdn-frontend.js vercel.json _headers "$DIST_DIR/"

# Copy node_modules to ensure the Linera client is available
if [ -d "node_modules" ]; then
    echo "Copying node_modules to dist directory..."
    cp -r node_modules "$DIST_DIR/"
else
    echo "Error: node_modules not found. Run 'npm install' first."
    exit 1
fi

# Create a production config file
cat > "$DIST_DIR/config.prod.js" << 'EOF'
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
EOF

echo "Production config file created at dist/config.prod.js"

# Update the index.html in dist to use the production config
sed "s|config.js|config.prod.js|g" "$DIST_DIR/index.html" > "$DIST_DIR/index.html.tmp" && mv "$DIST_DIR/index.html.tmp" "$DIST_DIR/index.html"

echo "Updated index.html to use production config"

# Create a README for the dist directory
cat > "$DIST_DIR/README.md" << 'EOF'
# dCDN Frontend Distribution

This directory contains the production-ready build of the dCDN frontend application.

## Setup for Production

1. Ensure you have deployed the dCDN contract and have the application ID
2. Update the `applicationId` in `config.prod.js` with your deployed application ID
3. Deploy these files to your web server with the required security headers:
   - Cross-Origin-Embedder-Policy: require-corp
   - Cross-Origin-Opener-Policy: same-origin

## Required Headers

For the Linera client to work properly, your web server must serve these files with the following headers:

```
Cross-Origin-Embedder-Policy: require-corp
Cross-Origin-Opener-Policy: same-origin
```

These are configured in the vercel.json file for Vercel deployments.

## Serving Locally

To serve locally with proper headers:

npx http-party/http-server --header Cross-Origin-Embedder-Policy:require-corp --header Cross-Origin-Opener-Policy:same-origin --port 3000
EOF

echo "Build completed successfully!"
echo "Files are in the 'dist' directory"
echo "Serve using:"
echo "cd dist && npx http-party/http-server --header Cross-Origin-Embedder-Policy:require-corp --header Cross-Origin-Opener-Policy:same-origin --port 3000"