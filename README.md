# CodeTanks
[![build](https://github.com/code-tanks/code-tanks/actions/workflows/build.yml/badge.svg)](https://github.com/code-tanks/code-tanks/actions/workflows/build.yml)[![devcontainer](https://github.com/code-tanks/code-tanks/actions/workflows/devcontainer.yml/badge.svg)](https://github.com/code-tanks/code-tanks/actions/workflows/devcontainer.yml)[![website](https://github.com/code-tanks/website/actions/workflows/website.yml/badge.svg)](https://github.com/code-tanks/website/actions/workflows/website.yml)

## Requirements
- [Docker](https://docs.docker.com/get-docker/)

## Installation

/etc/docker/daemon.json

{
  "insecure-registries" : ["localhost:5001"],
}

## Supported Languages
| Language | Dev Container |
| --- | --- |
| [Python](https://github.com/code-tanks/python-api) | [![Python](https://github.com/code-tanks/python-api/actions/workflows/devcontainer.yml/badge.svg)](https://github.com/code-tanks/python-api/actions/workflows/devcontainer.yml) |
| [Dart](https://github.com/code-tanks/dart-api) | [![devcontainer](https://github.com/code-tanks/dart-api/actions/workflows/devcontainer.yml/badge.svg)](https://github.com/code-tanks/dart-api/actions/workflows/devcontainer.yml) |


## Development

```
# for testing the in browser
cd viewer
./scripts/run.sh
```
