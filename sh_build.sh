#!/bin/bash
set -e

cd ui
npm run build
cd ..

cargo build --release
