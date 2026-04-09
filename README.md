<div align="center">

[![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Windows-lightgrey)]()
[![Release](https://img.shields.io/github/v/release/asfrm/rustvision?label=latest%20release)](https://github.com/asfrm/rustvision/releases)

# RustVision

RustVision is a simple tool for adjusting gamma in games like Rust on the fly. Built with Rust.

You can manually change process name for any game.
Works well with any anti-cheat.

</div>

---

<details>
<summary><b>🇬🇧 English</b></summary>

### Why it is safe

- **No Hooks:** It does not hook into DirectX, Vulkan, or OpenGL.
- **No DLL Injection:** It operates as a standalone process.

### Installation and Setup

You can download `.exe` file and start using the program.

Download here: [GitHub Releases](https://github.com/asfrm/rustvision/releases)

### Or build it from source

If you prefer to compile the project yourself, follow these steps:

1. **Install Rust:** Ensure you have the Rust toolchain installed (get it at [rustup.rs](https://rustup.rs/)).
2. **Clone the repository:**
   ```bash
   git clone https://github.com/asfrm/rustvision.git
   cd rustvision
   ```
3. **Build:**
   ```bash
   cargo build --release
   ```

The binary will be at `target/release/rustvision.exe`.

</details>

<details>
<summary><b>🇷🇺 Русский</b></summary>

### Почему это безопасно

- **Без хуков:** Не внедряется в DirectX, Vulkan или OpenGL.
- **Без DLL-инъекций:** Работает как отдельный процесс.

### Установка

Скачай `.exe` из [Releases](https://github.com/asfrm/rustvision/releases) и запусти. Всё.

### Собрать самому

1. **Поставь Rust:** [rustup.rs](https://rustup.rs/)
2. **Склонируй репо:**
   ```bash
   git clone https://github.com/asfrm/rustvision.git
   cd rustvision
   ```
3. **Собери:**
   ```bash
   cargo build --release
   ```

Бинарник будет в `target/release/rustvision.exe`.

</details>

<img width="353" height="464" alt="image" src="https://github.com/user-attachments/assets/48f9111b-732a-436a-a060-f370243c2aba" />
