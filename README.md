# Link Shortener in Rust

## Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Endpoints](#endpoints)
- [Configuration](#configuration)
- [Database Schema](#database-schema)
- [Directory Structure](#directory-structure)
- [Contributing](#contributing)
- [License](#license)

## Introduction
This project is a URL shortening service written in Rust using the Axum framework. It allows users to shorten long URLs and track usage statistics for each shortened link.

## Features
- Shorten long URLs
- Redirect to original URLs
- Track usage statistics
- Health check endpoint
- Prometheus metrics integration

## Installation
### Prerequisites
- Rust (latest stable version)
- Docker (for running the database)

### Clone the Repository
```sh
git clone https://github.com/saulhs12/link-shortener-rust.git
cd link-shortener-rust
```

## Usage
Once the application is running, you can use the endpoints to shorten URLs, redirect to original URLs, and get usage statistics.

## Endpoints
- POST /create: Create a shortened URL.
- GET /:id: Redirect to the original URL.
- PATCH /:id: Update a shortened URL.
- GET /:id/statistics: Get usage statistics for a shortened URL.
- GET /health: Health check endpoint.
- GET /metrics: Prometheus metrics endpoint.
### Configuration
Configuration is managed using the settings.rs file. You can set the application parameters such as database connection URL, application host, and port.


Database Schema
The project uses SQLx for database migrations. The database schema is defined in the migrations folder.


