#!/bin/bash

CURRENT_PARTITION=0
echo "g" # Create new GPT

while read -r partition; do
	TYPE=$(echo $partition | jq ".type" | tr -d '"')
	SIZE=$(echo $partition | jq ".size" | tr -d '"')

	echo "n" # Create new partition
	CURRENT_PARTITION=$((CURRENT_PARTITION+1))

	echo "" # Use next partition number
	echo "" # Use next available sector
	echo "+$SIZE" # Set size of partition
	echo "t" # Specify type
	if [ $CURRENT_PARTITION -gt 1 ]; then # If multiple partitions exist, specify which one
		echo $CURRENT_PARTITION
	fi
	if [ $TYPE = "BOOT" ]; then # Set type
		echo "1"
	else
		echo $TYPE
	fi
done < <(jq -c ".partitions[]" $1)

echo "w" # Write changes