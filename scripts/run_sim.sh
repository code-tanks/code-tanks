#!/bin/bash
curl -d "$*" -X POST http://localhost:8089/run --http1.1
echo ""
