#!/bin/bash

# Rust Pi Zero Example - Automated Deployment Script
# This script handles cross-compilation and deployment to Raspberry Pi

set -e  # Exit on any error

# Configuration
PI_HOST="${1:-raspberrypi.local}"
PI_USER="${2:-pi}"
BINARY_NAME="rust_pi_zero_example"
SERVICE_NAME="rust-pi-app"
TARGET="arm-unknown-linux-gnueabihf"
REMOTE_PATH="/home/${PI_USER}/${BINARY_NAME}"
SERVICE_PATH="/etc/systemd/system/${SERVICE_NAME}.service"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Help function
show_help() {
    cat << EOF
Rust Pi Zero Example - Deployment Script

Usage: $0 [PI_HOST] [PI_USER]

Arguments:
  PI_HOST    Raspberry Pi hostname or IP (default: raspberrypi.local)
  PI_USER    Username on Pi (default: pi)

Examples:
  $0                                    # Deploy to raspberrypi.local as pi
  $0 192.168.1.100                     # Deploy to specific IP as pi
  $0 mypi.local myuser                  # Deploy to custom host and user

Requirements:
  - Rust with ARM cross-compilation target
  - SSH access to Raspberry Pi
  - sudo privileges on Pi for service management

EOF
}

# Check if help is requested
if [[ "$1" == "-h" || "$1" == "--help" ]]; then
    show_help
    exit 0
fi

# Banner
echo "🚀 Rust Pi Zero Example - Deployment Script"
echo "=============================================="
echo "Target: ${PI_USER}@${PI_HOST}"
echo "Binary: ${BINARY_NAME}"
echo ""

# Check prerequisites
log_info "Checking prerequisites..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    log_error "Rust/Cargo not found. Please install Rust: https://rustup.rs/"
    exit 1
fi

# Check if ARM target is installed
if ! rustup target list --installed | grep -q "${TARGET}"; then
    log_warning "ARM target not installed. Installing ${TARGET}..."
    rustup target add "${TARGET}"
fi

# Check if cross-compilation tools are available
if ! command -v arm-linux-gnueabihf-gcc &> /dev/null; then
    log_warning "ARM cross-compilation tools not found."
    log_info "Install with: sudo apt-get install gcc-arm-linux-gnueabihf"
    log_info "Continuing anyway - cargo might handle this..."
fi

# Test SSH connectivity
log_info "Testing SSH connectivity to ${PI_USER}@${PI_HOST}..."
if ! ssh -o ConnectTimeout=10 -o BatchMode=yes "${PI_USER}@${PI_HOST}" exit 2>/dev/null; then
    log_error "Cannot connect to ${PI_USER}@${PI_HOST}"
    log_info "Please ensure:"
    log_info "  1. SSH is enabled on the Pi"
    log_info "  2. SSH keys are set up or password authentication is enabled"
    log_info "  3. The hostname/IP is correct"
    exit 1
fi

log_success "SSH connectivity verified"

# Build for ARM
log_info "Building for ARM target (${TARGET})..."
export CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc
cargo build --release --target "${TARGET}" --features hardware --no-default-features

if [ ! -f "target/${TARGET}/release/${BINARY_NAME}" ]; then
    log_error "Build failed - binary not found"
    exit 1
fi

log_success "Build completed successfully"

# Stop existing service if running
log_info "Stopping existing service on Pi..."
ssh "${PI_USER}@${PI_HOST}" "sudo systemctl stop ${SERVICE_NAME} 2>/dev/null || true"

# Copy binary to Pi
log_info "Copying binary to Pi..."
scp "target/${TARGET}/release/${BINARY_NAME}" "${PI_USER}@${PI_HOST}:${REMOTE_PATH}"

# Make binary executable
ssh "${PI_USER}@${PI_HOST}" "chmod +x ${REMOTE_PATH}"

log_success "Binary deployed successfully"

# Copy and install service file
log_info "Installing systemd service..."
scp "${SERVICE_NAME}.service" "${PI_USER}@${PI_HOST}:/tmp/"

ssh "${PI_USER}@${PI_HOST}" << EOF
    sudo cp /tmp/${SERVICE_NAME}.service ${SERVICE_PATH}
    sudo systemctl daemon-reload
    sudo systemctl enable ${SERVICE_NAME}
EOF

log_success "Service installed and enabled"

# Start the service
log_info "Starting service..."
ssh "${PI_USER}@${PI_HOST}" "sudo systemctl start ${SERVICE_NAME}"

# Wait a moment for service to start
sleep 3

# Check service status
log_info "Checking service status..."
if ssh "${PI_USER}@${PI_HOST}" "sudo systemctl is-active --quiet ${SERVICE_NAME}"; then
    log_success "Service is running successfully!"
else
    log_error "Service failed to start. Checking logs..."
    ssh "${PI_USER}@${PI_HOST}" "sudo journalctl -u ${SERVICE_NAME} --no-pager -n 20"
    exit 1
fi

# Test API endpoint
log_info "Testing API endpoint..."
sleep 2  # Give the service a moment to fully start

if ssh "${PI_USER}@${PI_HOST}" "curl -s http://localhost:3030/ > /dev/null"; then
    log_success "API endpoint is responding!"
else
    log_warning "API endpoint test failed - service might still be starting"
fi

# Show service information
echo ""
echo "🎉 Deployment completed successfully!"
echo ""
echo "Service Information:"
echo "  Name: ${SERVICE_NAME}"
echo "  Status: $(ssh "${PI_USER}@${PI_HOST}" "sudo systemctl is-active ${SERVICE_NAME}")"
echo "  Binary: ${REMOTE_PATH}"
echo ""
echo "Useful Commands:"
echo "  Check status:  ssh ${PI_USER}@${PI_HOST} 'sudo systemctl status ${SERVICE_NAME}'"
echo "  View logs:     ssh ${PI_USER}@${PI_HOST} 'sudo journalctl -u ${SERVICE_NAME} -f'"
echo "  Restart:       ssh ${PI_USER}@${PI_HOST} 'sudo systemctl restart ${SERVICE_NAME}'"
echo "  Stop:          ssh ${PI_USER}@${PI_HOST} 'sudo systemctl stop ${SERVICE_NAME}'"
echo ""
echo "API Endpoints:"
echo "  Status:        curl http://${PI_HOST}:3030/status"
echo "  Sensor:        curl http://${PI_HOST}:3030/sensor"
echo "  LED Control:   curl -X POST -H 'Content-Type: application/json' -d '{\"state\": true}' http://${PI_HOST}:3030/led"
echo ""
echo "🔗 Access the web interface at: http://${PI_HOST}:3030/"

log_success "Deployment script completed!"
