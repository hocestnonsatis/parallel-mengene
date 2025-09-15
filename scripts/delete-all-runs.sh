#!/bin/bash

# Script to delete all workflow runs using GitHub API
# This is more efficient than using gh CLI for bulk operations

set -e

echo "🗑️  Starting bulk workflow run cleanup using GitHub API..."

# Get repository info
REPO_OWNER="hocestnonsatis"
REPO_NAME="parallel-mengene"

# Check if GITHUB_TOKEN is set
if [ -z "$GITHUB_TOKEN" ]; then
    echo "❌ GITHUB_TOKEN environment variable is not set."
    echo "Please set it with: export GITHUB_TOKEN=your_token"
    exit 1
fi

# Function to delete workflow runs
delete_workflow_runs() {
    local page=1
    local per_page=100
    local total_deleted=0
    
    while true; do
        echo "📄 Processing page $page..."
        
        # Get workflow runs for this page
        local response=$(curl -s -H "Authorization: token $GITHUB_TOKEN" \
            "https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/actions/runs?per_page=$per_page&page=$page")
        
        # Check if we got any runs
        local run_count=$(echo "$response" | jq '.workflow_runs | length')
        
        if [ "$run_count" -eq 0 ]; then
            echo "✅ No more runs to process."
            break
        fi
        
        echo "📊 Found $run_count runs on page $page"
        
        # Extract run IDs and delete them
        local run_ids=$(echo "$response" | jq -r '.workflow_runs[].id')
        
        for run_id in $run_ids; do
            echo "🗑️  Deleting run $run_id..."
            
            if curl -s -X DELETE -H "Authorization: token $GITHUB_TOKEN" \
                "https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/actions/runs/$run_id" > /dev/null; then
                ((total_deleted++))
                echo "✅ Deleted run $run_id"
            else
                echo "❌ Failed to delete run $run_id"
            fi
            
            # Small delay to avoid rate limiting
            sleep 0.1
        done
        
        ((page++))
    done
    
    echo ""
    echo "🎉 Bulk cleanup completed!"
    echo "📊 Total runs deleted: $total_deleted"
}

# Run the cleanup
delete_workflow_runs
