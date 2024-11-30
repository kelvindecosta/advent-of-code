#!/bin/bash
# This script adds a new day entry to the README.md file for the given year.

# Ensure year and day arguments are provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <year> <day>"
    exit 1
fi

YEAR=$1
DAY=$(printf "%02d" $2)  # Ensure day is zero-padded (e.g., 01, 02, ...)
DAY_NON_PADDED=$((10#$2)) # Remove zero-padding for the link
YEAR_README="events/y${YEAR}/README.md"

# Validate that year and day are numeric
if ! [[ "$YEAR" =~ ^[0-9]{4}$ ]] || ! [[ "$DAY" =~ ^[0-9]{2}$ ]]; then
    echo "Error: Year must be a four-digit number and day must be a two-digit number."
    exit 1
fi

# Run the aoc CLI command and capture the title
AOC_OUTPUT=$(aoc read --year "$YEAR" --day "$DAY" 2>&1)
if [ $? -ne 0 ]; then
    echo "Error running 'aoc read'. Please check the year and day."
    echo "$AOC_OUTPUT"
    exit 1
fi

# Extract the title from the output using awk
TITLE=$(echo "$AOC_OUTPUT" | awk -v day="$DAY" '/^--- Day [0-9]+: / { sub(/^--- Day [0-9]+: /, ""); sub(/ ---$/, ""); print; exit }')

if [ -z "$TITLE" ]; then
    echo "Error: Could not extract title from the aoc output."
    exit 1
fi

NEW_ENTRY="| [$DAY](./src/d${DAY}.rs) | [$TITLE](https://adventofcode.com/${YEAR}/day/${DAY_NON_PADDED}) |"

# Check if the day entry already exists
if grep -qF "$NEW_ENTRY" "$YEAR_README"; then
    echo "Entry for Day $DAY is already in the README."
    exit 0
fi

# Insert the new day entry in sorted order
awk -v new_entry="$NEW_ENTRY" -v day="$DAY" '
  BEGIN { entry_added = 0 }
  /^\| [0-9]{2} \|/ {
    entry_day = substr($2, 2, 2) + 0
    if (entry_day >= day && !entry_added) {
      print new_entry
      entry_added = 1
    }
    print
    next
  }
  {
    print
  }
  END {
    if (!entry_added) {
      print new_entry
    }
  }
' "$YEAR_README" > "${YEAR_README}.tmp" && mv "${YEAR_README}.tmp" "$YEAR_README"

echo "Day $DAY added successfully to $YEAR_README."
