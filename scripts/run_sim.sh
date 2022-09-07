#!/bin/bash
curl -d "$*" -X POST http://localhost:8089/run
echo ""
