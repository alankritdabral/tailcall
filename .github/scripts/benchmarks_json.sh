#!/bin/bash

# Process command-line arguments
while [ "$#" -gt 0 ]; do
    case "$1" in
        --name) 
            shift
            if [ "$#" -gt 0 ]; then
                word_to_append="$1"
            else
                echo "Error: Missing value for --name argument."
                exit 1
            fi
            ;;
        --file)
            shift
            if [ "$#" -gt 0 ]; then
                input_file="$1"
            else
                echo "Error: Missing value for --input file argument."
                exit 1
            fi
            ;;
        *)
            echo "Unknown argument: $1"
            exit 1
            ;;
    esac
    shift
done

# Check if the input file exists
if [ ! -f "$input_file" ]; then
    echo "Input file '$input_file' not found."
    exit 1
fi

# Read JSON data from the input file line by line
modified_json=""
while IFS= read -r line; do
    # Update IDs in each JSON object
    modified_line=$(echo "$line" | jq ". |= if(has(\"id\")) then .id += \"/$word_to_append\" else . end")
    modified_json+="$modified_line"$'\n'
done < "$input_file"

# Print the modified JSON
echo "$modified_json" > $input_file