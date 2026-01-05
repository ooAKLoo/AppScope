#!/bin/bash

# AppScope Server Setup Script
# Run this on your server to set up Docker and deploy AppScope

set -e

echo "=== AppScope Server Setup ==="

# Install Docker if not present
if ! command -v docker &> /dev/null; then
    echo "Installing Docker..."
    curl -fsSL https://get.docker.com | sh
    systemctl enable docker
    systemctl start docker
fi

# Install Docker Compose if not present
if ! command -v docker-compose &> /dev/null && ! docker compose version &> /dev/null; then
    echo "Installing Docker Compose..."
    apt-get update
    apt-get install -y docker-compose-plugin
fi

# Create app directory
mkdir -p /opt/appscope
cd /opt/appscope

# Create .env file if not exists
if [ ! -f .env ]; then
    echo "Creating .env file..."
    cat > .env << 'EOF'
WRITE_KEY=wk_change_this_key
READ_KEY=rk_change_this_key
NEXT_PUBLIC_API_URL=http://YOUR_SERVER_IP:3001
EOF
    echo "Please edit /opt/appscope/.env with your keys"
fi

echo "=== Setup Complete ==="
echo "Next steps:"
echo "1. Edit /opt/appscope/.env with your keys"
echo "2. Run: cd /opt/appscope && docker compose up -d"
