#!/bin/bash
# This script adds a new year entry to the events README.md file.

# Ensure a year argument is passed
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <year>"
    exit 1
fi

YEAR=$1
README="events/README.md"
NEW_ENTRY="| [${YEAR}](./y${YEAR}) |"

# Validate that the year is a positive integer
if ! [[ "$YEAR" =~ ^[0-9]{4}$ ]]; then
    echo "Error: Year must be a four-digit number."
    exit 1
fi

# Create the README.md if it doesn't exist with initial content
if [ ! -f "$README" ]; then
    echo -e "# Advent of Code Events\n\n| Event           |\n| --------------- |" > "$README"
fi

# Check if the year entry already exists
if grep -qF "$NEW_ENTRY" "$README"; then
    echo "Year ${YEAR} is already in the README."
    exit 0
fi

# Insert the new year entry in sorted order
awk -v new_entry="$NEW_ENTRY" '
  BEGIN { entry_added = 0 }
  /^\| \[[0-9]{4}\]\(.*\) \|$/ {
    # Extract the year number using substr and index
    year_start = index($2, "[") + 1
    year_end = index($2, "]") - 1
    current_year = substr($2, year_start, year_end - year_start)

    if (current_year >= '"$YEAR"' && !entry_added) {
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
' "$README" > "${README}.tmp" && mv "${README}.tmp" "$README"

echo "Year ${YEAR} added successfully to README."
