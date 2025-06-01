# Lazy Crafter

Инструмент для автоматизации крафта в Path of Exile.

## ⚠️ Предупреждение

Этот инструмент нарушает правила игры и может привести к бану аккаунта. Используйте на свой страх и риск.

## Возможности

- Графический интерфейс для управления крафтом
- Отслеживание нажатий клавиш (Ctrl+Shift+E для автоматического крафта)
- Работа с буфером обмена для чтения/записи данных предметов
- Фильтрация модов по:
  - Классу предмета
  - Базе предмета
  - Уровню предмета
  - Текстовому поиску
- Расчет шансов крафта (в разработке)
- Автоматический крафт (нестабильный)

## Требования

- Rust 1.70 или выше
- X11 (для Linux)
- Windows API (для Windows)

## Установка

1. Клонируйте репозиторий:
```bash
git clone https://github.com/yourusername/lazy_crafter.git
cd lazy_crafter
```

2. Соберите проект:
```bash
cargo build --release
```

## Использование

1. Запустите приложение:
```bash
cargo run --release
```

2. Выберите класс предмета и базу
3. Выберите желаемые моды
4. Нажмите Ctrl+Shift+E для автоматического крафта

## Структура проекта

- `src/main.rs` - Точка входа в приложение
- `src/key_listener.rs` - Обработка нажатий клавиш
- `src/ui/` - Графический интерфейс
- `src/storage/` - Работа с данными
- `src/usecases/` - Бизнес-логика

## Изменения

Подробная информация об изменениях в проекте доступна в [CHANGELOG.md](CHANGELOG.md).

## Лицензия

MIT

## Features

- Mods filtering by item class, item base, item level and text search. (Not tested enought, there are some represenation mods mistakes)
- Crafting chance calculation is not ready
- Auto crafting is not stable, but you can try it. (Ctrl+Shift+E on item with "currency in hand")

## Disclaimer

This application doesn't follow GGG's ToS. GGG would ban you if you use that application.

The app doesn't change game files. It works with your clipboard buffer and control your clicks only.

It may be quite hard and expensive to reveal the usage of that kind of app. However, I can't give you any guaranties.

### Planned features

- Filtering stabilization
- Auto-crafting stabilization
- Auto-colorization
- Auto-linking
- Improve test coverage
- Add currency choices for estimation
- Implement estimation for selected mods and average/median cost
- CI

### lower priority plans 

- Telemetry for bugs
- Telemetry for statistic

## Download

earlier version x86

https://github.com/antonguzun/lazy_crafter/releases/tag/0.4.2

## Demo

[![demo](https://img.youtube.com/vi/tH3UOBZh0-w/0.jpg)](https://www.youtube.com/watch?v=tH3UOBZh0-w "Demo")

## Build
it requires rustc 1.65 or newer

```sh
cargo build
```

## Run as debug

### unix

```sh
RUST_LOG=DEBUG cargo run
```

### windows

```PowerShell
$env:RUST_LOG='DEBUG'
cargo run
```

## Run tests

### unit tests

```sh
cargo run tests
```

### integration tests

```sh
cargo test --test test_item_parser
```

### Dependences
used prepaired data by RePoe https://github.com/brather1ng/RePoE

used font https://www.exljbris.com/fontin.html
