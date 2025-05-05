# This script generates an index.ts file that exports all TypeScript files in the bindings directory
# Loop through all .ts files in the bindings directory
for file in bindings/*.ts; do 
  # Extract just the filename without the path
  filename=$(basename "$file")
  # Skip the index.ts file itself
  if [ "$filename" != "index.ts" ]; then
    # Generate an export statement for each file, removing the .ts extension
    filename_no_ext=${filename%.ts}
    echo "export * from './$filename_no_ext';" 
  fi
# Write all export statements to index.ts
done > bindings/index.ts