#!/bin/bash
file1="benches/iai-callgrind/old_benchmark.txt"
file2="benches/iai-callgrind/new_benchmark.txt"
config_file="benches/iai-callgrind/benchmarks.cfg"
readarray -t benchmarks < "$config_file"
attributes=("Instructions" "L1 Hits" "L2 Hits" "RAM Hits" "Total read+write" "Estimated Cycles")
fail_ci=0

calculate_value() {
    local file="$1"
    local bench="$2"
    local attribute="$3"
    case "$attribute" in
        "Total read+write")
            echo $(( $(grep -A5 "$bench" "$file" | grep -Po "L1 Hits:\s*\K\d+" || echo 0) +
                     $(grep -A5 "$bench" "$file" | grep -Po "L2 Hits:\s*\K\d+" || echo 0) +
                     $(grep -A5 "$bench" "$file" | grep -Po "RAM Hits:\s*\K\d+" || echo 0) ))
            ;;
        "Estimated Cycles")
            echo $(( $(grep -A5 "$bench" "$file" | grep -Po "L1 Hits:\s*\K\d+" || echo 0) +
                     5 * $(grep -A5 "$bench" "$file" | grep -Po "L2 Hits:\s*\K\d+" || echo 0) +
                     35 * $(grep -A5 "$bench" "$file" | grep -Po "RAM Hits:\s*\K\d+" || echo 0) ))
            ;;
        *)
            echo $(grep -A5 "$bench" "$file" | grep -Po "${attribute}:\s*\K\d+" || echo 0)
            ;;
    esac
}

for bench in "${benchmarks[@]}"; do
    for attribute in "${attributes[@]}"; do
        value1=$(calculate_value "$file1" "$bench" "$attribute")
        value2=$(calculate_value "$file2" "$bench" "$attribute")

        percent_change=$(( value1 ? ((value2 - value1) * 100) / value1 : 0 ))
       
        printf "s\n" "$bench" >> "benches/iai-callgrind/compare.txt"
        printf "| Attribute    | Base    | New      | % change |" >> "benches/iai-callgrind/compare.txt"
        printf "| -------------| ------- | ------- | -------- |" >> "benches/iai-callgrind/compare.txt"
        if ((percent_change > 10)); then
            echo "$bench $attribute has a change of $percent_change%, within CI limits. (Original values: $value1 -> $value2)"
            printf "| %-30s | %-20s | %-20s | %-10.2f |\n" "$attribute" "$value1" "$value2" "$percent_change" >> "benches/iai-callgrind/compare.txt"
            fail_ci=1
        else
            printf "| %-30s | %-20s | %-20s | %-10.2f |\n" "$attribute" "$value1" "$value2" "$percent_change" >> "benches/iai-callgrind/compare.txt"
        fi
    done
    echo "----------------------------------"
done
content=$(cat benches/iai-callgrind/compare.txt)
echo "$content"

exit "$fail_ci"
