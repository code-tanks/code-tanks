#!/bin/bash

curl -d "@$1" -X POST http://localhost:8089/upload
echo ""