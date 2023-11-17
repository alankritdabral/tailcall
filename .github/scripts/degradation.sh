# Run the critcmp command and capture its output
output=$(critcmp before helo)

# Use awk to calculate percentage differences and store the comments
comments=$(awk 'BEGIN {
    comment=""
}
NR > 2 {
    group=$1
    base=$7
    changes=$3

    gsub(/[^\-0-9\.]/, "", changes)  # Extract only numerical values
    gsub(/[^\-0-9\.]/, "", base)     # Extract only numerical values

    # Ensure changes and base values are not empty
    if (changes != "" && base != "") {
        diff = changes - base  # Calculate the absolute difference
        percentage = (diff / base) * 100  # Calculate the percentage difference

        if (percentage > 10) {
            comment = comment sprintf("Percentage change for %s exceeds 10%%: %.2f%%\n", group, percentage)
        }
    }
}
END {
    print comment
}' <<< "$output")

# Print the captured output
echo "$output"

# Print comments and add them as a comment on the PR using GitHub API
if [ -n "$comments" ]; then
    echo "Adding comments to the pull request:"
    echo "$comments"

    # Extract the pull request number using environment variables provided by GitHub Actions
    pr_number="${{ github.event.number }}"
    
    # Add a comment to the pull request using GitHub API
    curl -X POST \
        -H "Authorization: Bearer ${{ secrets.GITHUB_TOKEN }}" \
        -H "Accept: application/vnd.github.v3+json" \
        "https://api.github.com/repos/${{ github.repository }}/issues/${pr_number}/comments" \
        -d "{\"body\":\"$comments\"}"
fi
