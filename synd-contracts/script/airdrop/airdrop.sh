#!/bin/bash

# Airdrop script using bash and forge commands
# This script provides robust error handling and controlled flow

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CSV_FILE="${SCRIPT_DIR}/airdrop_data.csv"
LOG_FILE="${SCRIPT_DIR}/airdrop.log"
PROGRESS_FILE="${SCRIPT_DIR}/airdrop_progress.txt"
BATCH_SIZE=200
DRY_RUN=${DRY_RUN:-false}

# Required environment variables
: ${AIRDROP_CONTRACT:?Please set AIRDROP_CONTRACT address}
: ${TOKEN_ADDRESS:?Please set TOKEN_ADDRESS}
: ${RPC_URL:?Please set RPC_URL}
: ${ACCOUNT:?Please set ACCOUNT for signing}
: ${DEV_PUB_ADDRESS:?Please set DEV_PUB_ADDRESS}

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging function
log() {
    local level="$1"
    shift
    local message="$*"
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    echo -e "${timestamp} [${level}] ${message}" | tee -a "$LOG_FILE"
}

log_info() { log "INFO" "$@"; }
log_warn() { log "WARN" "$@"; }
log_error() { log "ERROR" "$@"; }
log_success() { log "SUCCESS" "$@"; }

# Function to save progress
save_progress() {
    local batch_index="$1"
    local address="$2"
    echo "LAST_BATCH_INDEX=${batch_index}" > "$PROGRESS_FILE"
    echo "LAST_ADDRESS=${address}" >> "$PROGRESS_FILE"
    log_info "Progress saved: batch ${batch_index}, last address ${address}"
}

# Function to load progress
load_progress() {
    if [[ -f "$PROGRESS_FILE" ]]; then
        source "$PROGRESS_FILE"
        log_info "Resuming from batch ${LAST_BATCH_INDEX}, last address ${LAST_ADDRESS}"
        echo "${LAST_BATCH_INDEX:-0}"
    else
        echo "0"
    fi
}

# Function to check if address has zero balance
check_balance() {
    local address="$1"
    local balance
    
    balance=$(cast call "$TOKEN_ADDRESS" "balanceOf(address)(uint256)" "$address" --rpc-url "$RPC_URL" 2>/dev/null || echo "error")
    
    if [[ "$balance" == "error" ]]; then
        log_error "Failed to check balance for address: $address"
        return 1
    fi
    
    if [[ "$balance" != "0" ]]; then
        log_warn "Address $address already has balance: $balance"
        return 1
    fi
    
    return 0
}

# Function to check contract allowance
check_allowance() {
    local total_amount="$1"
    local allowance
    
    allowance=$(cast call "$TOKEN_ADDRESS" "allowance(address,address)(uint256)" "$DEV_PUB_ADDRESS" "$AIRDROP_CONTRACT" --rpc-url "$RPC_URL" 2>/dev/null || echo "error")
    
    if [[ "$allowance" == "error" ]]; then
        log_error "Failed to check allowance"
        return 1
    fi
    
    # Convert to decimal for comparison
    local allowance_dec
    local total_dec
    allowance_dec=$(cast --to-dec "$allowance" 2>/dev/null || echo "0")
    total_dec=$(cast --to-dec "$total_amount" 2>/dev/null || echo "0")
    
    if (( allowance_dec < total_dec )); then
        log_error "Insufficient allowance. Required: $total_amount, Available: $allowance"
        return 1
    fi
    
    log_info "Allowance check passed. Required: $total_amount, Available: $allowance"
    return 0
}

# Function to process a single batch
process_batch() {
    local batch_index="$1"
    local addresses=("${@:2}")
    local batch_size=${#addresses[@]}
    
    log_info "${GREEN}Processing batch $((batch_index + 1)) with $batch_size addresses${NC}"
    
    # Check balances for all addresses in batch
    log_info "Checking balances for batch $((batch_index + 1))..."
    local amounts=()
    local total_amount="0"
    local line_num=1
    
    for address in "${addresses[@]}"; do
        # Skip header line
        if [[ "$address" == "address,amount" ]]; then
            continue
        fi
        
        IFS=',' read -r addr amount <<< "$address"
        
        # Validate address format
        if [[ ! "$addr" =~ ^0x[a-fA-F0-9]{40}$ ]]; then
            log_error "Invalid address format at line $line_num: $addr"
            return 1
        fi
        
        # Check balance
        if ! check_balance "$addr"; then
            log_error "Balance check failed for address: $addr (batch $((batch_index + 1)), line $line_num)"
            return 1
        fi
        
        amounts+=("$amount")
        total_amount=$(cast --to-wei "$total_amount" ether)
        amount_wei=$(cast --to-wei "$amount" ether)
        total_amount=$(cast --to-dec $(cast add "$total_amount" "$amount_wei"))
        
        ((line_num++))
    done
    
    # Convert total back to hex
    total_amount=$(cast --to-hex "$total_amount")
    
    # Check allowance
    if ! check_allowance "$total_amount"; then
        log_error "Allowance check failed for batch $((batch_index + 1))"
        return 1
    fi
    
    # Prepare forge script call
    local addresses_str=""
    local amounts_str=""
    local addr_array=()
    local amount_array=()
    
    for address in "${addresses[@]}"; do
        if [[ "$address" == "address,amount" ]]; then
            continue
        fi
        
        IFS=',' read -r addr amount <<< "$address"
        addr_array+=("$addr")
        amount_wei=$(cast --to-hex $(cast --to-wei "$amount" ether))
        amount_array+=("$amount_wei")
    done
    
    # Build arrays for the call
    addresses_str=$(printf '"%s",' "${addr_array[@]}")
    addresses_str="[${addresses_str%,}]"
    
    amounts_str=$(printf '%s,' "${amount_array[@]}")
    amounts_str="[${amounts_str%,}]"
    
    log_info "Executing airdrop for batch $((batch_index + 1))..."
    log_info "Addresses: ${#addr_array[@]}, Total amount: $total_amount"
    
    if [[ "$DRY_RUN" == "true" ]]; then
        log_info "${YELLOW}DRY RUN: Would execute airdrop batch $((batch_index + 1))${NC}"
        log_info "Addresses: $addresses_str"
        log_info "Amounts: $amounts_str"
        log_info "Total: $total_amount"
    else
        # Execute the airdrop using forge
        if cast send "$AIRDROP_CONTRACT" "airdropERC20(address,address[],uint256[],uint256)" \
            "$TOKEN_ADDRESS" "$addresses_str" "$amounts_str" "$total_amount" \
            --rpc-url "$RPC_URL" --account "$ACCOUNT" --gas-limit 3000000; then
            
            log_success "Batch $((batch_index + 1)) completed successfully"
            save_progress "$batch_index" "${addr_array[-1]}"
        else
            log_error "Batch $((batch_index + 1)) failed to execute"
            save_progress "$batch_index" "${addr_array[0]}"
            return 1
        fi
    fi
}

# Main execution function
main() {
    log_info "Starting airdrop script"
    log_info "Contract: $AIRDROP_CONTRACT"
    log_info "Token: $TOKEN_ADDRESS"
    log_info "CSV file: $CSV_FILE"
    log_info "Batch size: $BATCH_SIZE"
    log_info "Dry run: $DRY_RUN"
    
    # Check if CSV file exists
    if [[ ! -f "$CSV_FILE" ]]; then
        log_error "CSV file not found: $CSV_FILE"
        exit 1
    fi
    
    # Read CSV file and skip header
    local lines=()
    while IFS= read -r line || [[ -n "$line" ]]; do
        if [[ "$line" != "address,amount" && -n "$line" ]]; then
            lines+=("$line")
        fi
    done < "$CSV_FILE"
    
    local total_lines=${#lines[@]}
    local total_batches=$(( (total_lines + BATCH_SIZE - 1) / BATCH_SIZE ))
    
    log_info "Total recipients: $total_lines"
    log_info "Total batches: $total_batches"
    
    # Load progress
    local start_batch
    start_batch=$(load_progress)
    
    # Process batches
    for ((batch_index = start_batch; batch_index < total_batches; batch_index++)); do
        local start_idx=$((batch_index * BATCH_SIZE))
        local end_idx=$((start_idx + BATCH_SIZE))
        
        if ((end_idx > total_lines)); then
            end_idx=$total_lines
        fi
        
        # Extract batch
        local batch=()
        for ((i = start_idx; i < end_idx; i++)); do
            batch+=("${lines[i]}")
        done
        
        log_info "Processing batch $((batch_index + 1))/$total_batches (addresses $((start_idx + 1))-$end_idx)"
        
        if ! process_batch "$batch_index" "${batch[@]}"; then
            log_error "Batch $((batch_index + 1)) failed. Last processed index: $batch_index"
            exit 1
        fi
        
        # Small delay between batches
        if [[ "$DRY_RUN" != "true" ]]; then
            sleep 2
        fi
    done
    
    log_success "${GREEN}All batches completed successfully!${NC}"
    
    # Clean up progress file on successful completion
    if [[ -f "$PROGRESS_FILE" ]]; then
        rm "$PROGRESS_FILE"
        log_info "Progress file cleaned up"
    fi
}

# Help function
show_help() {
    cat << EOF
Airdrop Script

USAGE:
    $0 [OPTIONS]

REQUIRED ENVIRONMENT VARIABLES:
    AIRDROP_CONTRACT    - Address of the deployed Airdrop contract
    TOKEN_ADDRESS       - Address of the ERC20 token to airdrop
    RPC_URL            - RPC endpoint URL
    ACCOUNT            - Account name for signing transactions  
    DEV_PUB_ADDRESS    - Public address of the deployer/sender

OPTIONAL ENVIRONMENT VARIABLES:
    DRY_RUN=true       - Run without executing transactions (default: false)

OPTIONS:
    -h, --help         - Show this help message

EXAMPLES:
    # Normal execution
    AIRDROP_CONTRACT=0x123... TOKEN_ADDRESS=0x456... RPC_URL=https://... ACCOUNT=deployer DEV_PUB_ADDRESS=0x789... $0
    
    # Dry run
    DRY_RUN=true AIRDROP_CONTRACT=0x123... TOKEN_ADDRESS=0x456... RPC_URL=https://... ACCOUNT=deployer DEV_PUB_ADDRESS=0x789... $0

FILES:
    airdrop_data.csv     - Input CSV file with address,amount format
    airdrop.log          - Log file with detailed execution information
    airdrop_progress.txt - Progress tracking file (auto-generated)

EOF
}

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Execute main function
main "$@"