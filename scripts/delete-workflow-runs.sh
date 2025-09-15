#!/bin/bash

# Script to delete all workflow runs from GitHub repository
# This will delete all completed, failed, and cancelled workflow runs

set -e

echo "🗑️  Starting workflow run cleanup..."

# Get all workflow run IDs (excluding in_progress runs)
echo "📋 Fetching workflow run IDs..."
RUN_IDS=$(gh run list --limit 1000 --json databaseId,status | jq -r '.[] | select(.status != "in_progress") | .databaseId')

if [ -z "$RUN_IDS" ]; then
    echo "✅ No workflow runs found to delete."
    exit 0
fi

# Count total runs
TOTAL_RUNS=$(echo "$RUN_IDS" | wc -l)
echo "📊 Found $TOTAL_RUNS workflow runs to delete."

# Delete runs in batches
echo "🗑️  Deleting workflow runs..."
DELETED=0
FAILED=0

for run_id in $RUN_IDS; do
    if echo "y" | gh run delete "$run_id" 2>/dev/null; then
        ((DELETED++))
        echo "✅ Deleted run $run_id"
    else
        ((FAILED++))
        echo "❌ Failed to delete run $run_id"
    fi
    
    # Add small delay to avoid rate limiting
    sleep 0.2
done

echo ""
echo "📊 Cleanup Summary:"
echo "   ✅ Successfully deleted: $DELETED runs"
echo "   ❌ Failed to delete: $FAILED runs"
echo "   📈 Total processed: $((DELETED + FAILED)) runs"

if [ $FAILED -eq 0 ]; then
    echo "🎉 All workflow runs deleted successfully!"
else
    echo "⚠️  Some runs could not be deleted. You may need to delete them manually."
fi
