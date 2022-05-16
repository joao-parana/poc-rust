#!/bin/bash

set -e

# echo "************** Removing modules from NPM and pkg files"
# rm -rf node_modules
# rm -rf pkg
echo "************** Installing modules from NPM"
npm install
echo "************** Building for test on the WEB"
npm run serve