#!/bin/bash
set -e

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$DIR"

echo "📦 Compiling Yew UI with wasm-pack..."
wasm-pack build --release --target web --out-dir ../server/public

echo "🎨 Bundling all component CSS files into ui.css..."
echo "/* Auto-generated component styles */" > ../server/public/ui.css
find src -name "*.css" -type f | sort | while read -r f; do
    echo -e "\n/* === $f === */" >> ../server/public/ui.css
    cat "$f" >> ../server/public/ui.css
done

echo "📋 Copying index.html to server public..."
cp index.html ../server/public/

echo "✅ Build Complete -> ../server/public"
