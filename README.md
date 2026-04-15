# Sysen

**Understand your system.**

Sysen is a lightweight CLI tool that analyzes your system and presents meaningful insights instead of raw metrics.

---

## 🚀 Features

* 📊 CPU, memory, and swap usage
* 🔍 Top CPU-consuming process detection
* 🧠 Basic system load interpretation
* ⚡ Fast and minimal CLI

---

## 📦 Installation

Install via Cargo:

```bash
cargo install sysen
```

Or build locally:

```bash
git clone https://github.com/beingglitch/sysen.git
cd sysen
cargo build --release
```

---

## 🛠 Usage

### Show system status

```bash
sysen status
```

Example output:

```text
System Status:

CPU: 72%
Memory: 61%
Swap: 24%

Top Process:
  chrome (55%)

System Load: Moderate
```

---

### Show help

```bash
sysen help
```

---

## 🧠 Philosophy

Most system tools show raw numbers.

Sysen focuses on:

```
data → meaning → insight
```

The goal is to help you **understand what your system is doing**, not just display metrics.

---

## 🗺 Roadmap

* [ ] `sysen explain` — explain system slowdowns and causes
* [ ] Network usage insights
* [ ] Anomaly detection
* [ ] JSON output for integrations

---

## 🤝 Contributing

Contributions are welcome. Feel free to open issues or submit pull requests.

---

## 📄 License

MIT License
