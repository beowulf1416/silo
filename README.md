# Silo 🗄️

> A native, lightweight database management client and SQL query tool built with Rust, Relm4, and GTK4.

**Silo** is designed for developers and database administrators seeking a fast, memory-efficient, and responsive Linux desktop client for inspecting, querying, and managing databases. Built on top of GTK4, Silo leverages Rust's memory safety and performance to provide a seamless UI experience.

---

## Key Features

- ⚡ **Native Performance:** Built in Rust using GTK4 bindings for minimal resource footprint and low latency.
- 🎨 **Modern Linux UI:** Utilizes **GTK4** for an idiomatic, reactive GUI that integrates natively with modern desktop environments (GNOME/Libadwaita).
- 🔍 **SQL Query Workspace:** Execute custom SQL queries with syntax highlighting and structured tabular results.
- 🗂️ **Schema & Data Inspection:** Browse tables, views, and schemas through an intuitive sidebar tree interface.
- 🔒 **Type-Safe & Reliable:** Core database drivers and state management powered by Rust’s compile-time safety.

---

## Tech Stack

- **Language:** [Rust](https://www.rust-lang.org/)
- **UI Toolkit:** [GTK4](https://gtk.org/)

---

## System Prerequisites

To build and run **Silo** from source, ensure you have the following system dependencies installed:

### Dependencies (Debian / Ubuntu / Fedora)

#### Ubuntu / Debian:
```bash
sudo apt update
sudo apt install build-essential pkg-config libgtk-4-dev libadwaita-1-dev
