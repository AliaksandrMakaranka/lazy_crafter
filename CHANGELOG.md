# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Добавлена поддержка `anyhow` для улучшенной обработки ошибок
- Добавлено логирование для отладки проблем
- Добавлены информативные сообщения об ошибках

### Changed
- Улучшена обработка ошибок в `main.rs`:
  - Заменены `unwrap()` на `lock_s()?` для безопасной работы с мьютексами
  - Добавлена обработка ошибок при инициализации базы данных
  - Улучшено логирование ошибок

- Улучшена обработка ошибок в `key_listener.rs`:
  - Добавлен `anyhow::Context` для более информативных сообщений об ошибках
  - Заменены `expect()` на `context()?` для операций с буфером обмена
  - Улучшена обработка ошибок при инициализации слушателя клавиатуры

- Улучшена обработка ошибок в `ui_app.rs`:
  - Добавлена обработка ошибок при отправке событий
  - Улучшена обработка ошибок при работе с мьютексами
  - Добавлен контекст для ошибок запуска UI

- Улучшена обработка ошибок в `local_db.rs`:
  - Заменены все `unwrap()` на `ok_or_else()?` с информативными сообщениями об ошибках
  - Улучшена обработка ошибок при работе с регулярными выражениями
  - Добавлены проверки на существование значений

- Улучшена обработка ошибок в `errors.rs`:
  - Улучшена безопасность при удалении сообщений об ошибках
  - Добавлены проверки индексов
  - Улучшено логирование

### Fixed
- Исправлена проблема с индексами при удалении сообщений об ошибках
- Исправлена проблема с обработкой ошибок при работе с буфером обмена
- Исправлена проблема с обработкой ошибок при инициализации базы данных

### Security
- Улучшена безопасность при работе с мьютексами
- Добавлены проверки на существование значений
- Улучшена обработка ошибок при работе с регулярными выражениями

### Performance
- Оптимизирована работа с мьютексами
- Улучшена производительность при удалении сообщений об ошибках

## [0.1.0] - 2025-03-13

### Added
- Initial release
- Basic crafting automation
- UI for mod selection
- Database for storing mods and crafting data
- Hotkey support for Windows
- Clipboard integration for Windows

## [0.2.0] - 2025-03-14

### Added
- Added support for multiple item bases
- Added new UI elements for item base selection
- Added new database fields for item base information

### Changed
- Updated database schema to support multiple item bases
- Improved UI layout to accommodate new features
- Enhanced error handling for database operations

### Fixed
- Fixed database initialization issues
- Fixed UI state management
- Fixed error handling in database operations

## [0.3.0] - 2025-03-15

### Added
- Added support for multiple item bases
- Added new UI elements for item base selection
- Added new database fields for item base information

### Changed
- Updated database schema to support multiple item bases
- Improved UI layout to accommodate new features
- Enhanced error handling for database operations

### Fixed
- Fixed database initialization issues
- Fixed UI state management
- Fixed error handling in database operations

## [0.4.0] - 2025-03-16

### Added
- Added support for multiple item bases
- Added new UI elements for item base selection
- Added new database fields for item base information

### Changed
- Updated database schema to support multiple item bases
- Improved UI layout to accommodate new features
- Enhanced error handling for database operations

### Fixed
- Fixed database initialization issues
- Fixed UI state management
- Fixed error handling in database operations

## [0.4.1] - 2025-03-17

### Added
- Added support for multiple item bases in crafting
- Added new UI elements for item base selection
- Added new database fields for item base information

### Changed
- Updated database schema to support multiple item bases
- Improved UI layout to accommodate new features
- Enhanced error handling for database operations

### Fixed
- Fixed database initialization issues
- Fixed UI state management
- Fixed error handling in database operations

## [0.4.2] - 2025-03-18

### Added
- Added Linux support for clipboard operations using `arboard` crate
- Added Linux support for hotkey detection and crafting automation
- Added feature flags for platform-specific dependencies (`windows` and `linux`)

### Changed
- Improved error handling with `anyhow` crate
- Replaced `unwrap()` and `expect()` with proper error handling
- Added better error context with `anyhow::Context`
- Updated dependency management in `Cargo.toml`

### Fixed
- Fixed error handling in `key_listener.rs` for clipboard operations
- Fixed error handling in `local_db.rs` for database operations
- Fixed error handling in `main.rs` for thread operations 