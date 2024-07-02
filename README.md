# macOS Activity Tracker

## Overview

This Rust application tracks user activity on macOS, specifically capturing mouse clicks, keypresses, and mouse movements. It listens for these events, counts them, and saves the data to a JSON file named `activity_data.json`.

## Features

- Track left, right, and middle mouse button clicks.
- Count keyboard keypresses.
- Monitor mouse movements.
- Save activity data to a JSON file every 100 events and upon program termination.

## Prerequisites

- Rust and Cargo (Rust's package manager): [Install Rust](https://www.rust-lang.org/tools/install)
- macOS (as the application uses macOS-specific APIs for event listening)

## Setup

To set up the project on your local machine, follow these steps:

1. Clone the repository:

```bash
git clone https://github.com/nachoal/mac_activity_tracker
cd mac_activity_tracker
```

2. Build the project:

```bash
cargo build
```

## Running the Application

To run the application, use the following command:

```bash
cargo run
```

Ensure you have the necessary permissions on macOS to listen for keyboard and mouse events. You may need to enable your terminal or the specific executable in the System Preferences under Security & Privacy -> Privacy -> Accessibility.

## Data Format

The activity data is saved in the following JSON format:

```json
{
"left_clicks": 100,
"right_clicks": 150,
"middle_clicks": 50,
"keypresses": 200,
"mouse_movements": 500
}
```

## Usage

The application will run in the background and collect activity data. It will be saved to a JSON file named `activity_data.json` every 100 events or upon program termination.

## Contributing

Contributions to this project are welcome. Please fork the repository and submit a pull request with your changes.
