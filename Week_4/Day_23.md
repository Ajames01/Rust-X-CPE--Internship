# 🕒 Colorful Countdown Timer in Rust

## Overview

This is a simple yet engaging countdown timer implemented in Rust, featuring dynamic color-coded time display and robust user input handling. 
The timer provides a visually appealing way to track time, with colors that change based on the remaining duration.

## Features

- 🌈 Color-coded countdown:
  - 🟢 Green when plenty of time remains
  - 🟡 Yellow as time gets shorter
  - 🔴 Red when time is almost up
- ✨ Robust user input handling
- 🖥️ Simple terminal-based interface
- 🦀 Written in Rust

## How It Works

The countdown timer allows users to:
1. Input minutes and seconds
2. Watch a colorful, second-by-second countdown
3. Receive a vibrant "Time's up!" message when complete

### Color Progression Example
- Starting at 100 seconds: 🟢 Bright Green
- Halfway through: 🟡 Yellow
- Final moments: 🔴 Intense Red

## Dependencies

- `std::thread` for sleep functionality
- `std::time::Duration` for time management
- `colored` crate for terminal coloration