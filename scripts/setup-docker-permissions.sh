#!/bin/bash

# Setup Docker permissions for self-hosted runner
# This script should be run with sudo

echo "🔧 Setting up Docker permissions for self-hosted runner..."

# Add runner user to docker group
usermod -aG docker anil

# Create docker group if it doesn't exist
groupadd docker 2>/dev/null || true

# Set proper permissions on docker socket
chmod 666 /var/run/docker.sock 2>/dev/null || true

# Create docker daemon configuration for passwordless sudo
cat > /etc/sudoers.d/docker << EOF
# Allow anil to run docker without password
anil ALL=(ALL) NOPASSWD: /usr/bin/docker
EOF

echo "✅ Docker permissions configured!"
echo "📝 Note: You may need to restart the runner service for changes to take effect"
echo "🔄 Run: sudo systemctl restart actions.runner.*.service"
