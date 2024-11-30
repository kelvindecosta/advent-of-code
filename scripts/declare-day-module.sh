#!/bin/bash
# This script simply adds a new module declaration for a given day in the lib.rs file for the corresponding year.

# Check if the correct number of arguments is provided
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <year> <day>"
    exit 1
fi

YEAR=$1
DAY=$2

FILE="events/y${YEAR}/src/lib.rs"
MODULE="pub mod d${DAY};"

# Check if the module already exists
if grep -qF "$MODULE" "$FILE"; then
    echo "Module for day $DAY already exists in the file."
    exit 0
fi

# Create a temporary file for the new content
TEMP_FILE=$(mktemp)

# Process the file to insert the module in sorted order
awk -v day="$DAY" -v module="$MODULE" '
  function extract_day(line) {
    # Extract the number after "d" using substr and index
    idx = index(line, "d") + 3
    return substr(line, idx) + 0  # Convert to number for comparison
  }
  BEGIN { module_inserted = 0 }
  /pub mod day[0-9]+;/ {
    current_day = extract_day($0)
    if (!module_inserted && current_day >= day) {
      print module
      module_inserted = 1
    }
    print
    next
  }
  {
    print
  }
  END {
    if (!module_inserted) {
      print module
    }
  }
' "$FILE" > "$TEMP_FILE"

# Replace the original file with the updated content
mv "$TEMP_FILE" "$FILE"
echo "Module '$MODULE' added successfully."
