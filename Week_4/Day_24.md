# Async Web Scraper in Rust

## Overview
A high-performance, concurrent web scraper built with Rust, featuring:
- Asynchronous URL fetching
- Configurable concurrency
- Error retry mechanism
- Structured data extraction

## Features
- Concurrent web scraping using `tokio` and `futures`
- Configurable request timeout and retry logic
- Extracts webpage title, meta description, and first paragraph
- Handles network errors gracefully
- Supports custom scraping configuration

## Dependencies
- `reqwest`: HTTP client
- `scraper`: HTML parsing
- `tokio`: Async runtime
- `serde`: Data serialization

## Usage
1. Configure scraper settings
2. Provide list of URLs
3. Run concurrent scraping
4. Receive structured scraping results

## Error Handling
- Configurable retry attempts
- Timeout management
- Graceful error propagation

## Performance Considerations
- Non-blocking async design
- Controlled concurrent requests
- Minimal overhead extractiong