# 🌦️ Weather CLI

A blazing-fast and simple command-line application to fetch and display weather information for a given city using Rust. This tool utilizes `reqwest` to fetch weather data from an API and `serde` for efficient JSON parsing.

## ✨ Features
- 🚀 Fetches real-time weather data
- 🔍 Supports command-line input for city search
- 🌍 Displays temperature, weather conditions, and other relevant details

## 🛠️ Installation
### Prerequisites
- 🦀 Rust (latest stable version recommended)

### ▶️ Run the CLI

-  add this **export PATH="$HOME/.cargo/bin:$PATH"** to you shell configuration file e.g: ~/.zshrc ~/.bashrc

```sh
export PATH="$HOME/.cargo/bin:$PATH"
```

```sh
cargo run [CITY_NAME/COUNTRY]
```

Example:
```sh
cargo run tokyo
```


## 📦 Dependencies
- 🌐 `reqwest` - for making HTTP requests
- 📜 `serde` and `serde_json` - for parsing JSON data



## 🛡️ Error Handling
- The CLI gracefully handles network errors and invalid city inputs.
- If an error occurs, it provides a meaningful error message.

## 📜 License
This project is licensed under the **MIT License**.

## 🤝 Contribution
Feel free to **fork** the repository, **create issues**, or **submit pull requests**!

---
🌍 Stay informed with real-time weather updates from your terminal! 🚀

---
🚀 Also avaialable on crate.io with the name [**weather-app**](https://crates.io/crates/weather-app)
