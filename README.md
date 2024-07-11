# believex

## Сборка и запуск

### Контейнер

Выполнить в корне репозитория:
1. `docker build -t believex .`
2. `docker run --rm -it believex`

### Без Docker

Требования:
- `Visual Studio C++ Build Tools` / `MinGW` на Windows или `build-essential` на Ubuntu (`base-devel` на Arch)
- Rust *(toolchain)*, например установленная с помощью [rustup](https://rustup.rs/).
- [*Подробная инструкция по установке Rust*](https://doc.rust-lang.org/stable/book/ch01-01-installation.html)

Выполнить в корне репозитория:
1. `cargo run` или `cargo run --release`
