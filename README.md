# 🗺️ Kenshi Trade Map
## 🌍 ENGLISH VERSION GUIDE BELOW / ВЕРСИЯ ГАЙДА НА АНГЛИЙСКОМ НИЖЕ

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com)

Интерактивная карта торговых маршрутов для игры **Kenshi** с возможностью создания городов, планирования торговых путей и расчета наценки товаров.

![Kenshi Trade Map](https://img.shields.io/badge/version-3.0-brightgreen.svg)

## 📋 Содержание

- [Возможности](#-возможности)
- [Версии](#-версии)
- [Установка](#-установка)
- [Использование](#-использование)
- [Скриншоты](#-скриншоты)
- [Технологии](#-технологии)
- [Разработка](#-разработка)
- [Лицензия](#-лицензия)

## ✨ Возможности
### возможности Desktop-версии (v3.0)

- 🗺️ **Интерактивная карта** - загрузите изображение карты Kenshi
- 🏙️ **Создание городов** - размещайте города в любом месте карты
- 🛣️ **Торговые маршруты** - создавайте маршруты между городами
- 📦 **Управление товарами** - добавляйте товары с наценкой
- 💾 **Сохранение/загрузка** - экспорт и импорт данных в JSON
- 🔍 **Масштабирование** - плавное приближение и отдаление карты
- 🎨 **Визуализация** - цветовая кодировка маршрутов
- 🌙 **Темная тема** - современный темный интерфейс
- ∞ **Неограниченные маршруты** - создавайте сколько угодно маршрутов от одного города
- 🎨 **Выбор цвета** - полный color picker + 8 предустановленных цветов
- ➡️ **Стрелки направления** - визуальное обозначение начала и конца маршрута
- ⚡ **Высокая производительность** - нативное приложение на Rust
- 🖥️ **Автономная работа** - не требует браузера

## 🎯 Версии

### 🦀 Desktop-версия (Rust) - v3.0 ⭐ Рекомендуется

**Расположение:** `kenshi-trade-map-rust/`

**Особенности:**
- Нативное приложение на Rust
- Темная тема
- Неограниченное количество маршрутов от одного города
- Один маршрут = 2 точки (начало → конец)
- Выбор цвета маршрута (color picker + предустановленные)
- Стрелки направления
- Высокая производительность
- Плавное приближение карты

**Системные требования:**
- Windows 10+ / Linux / macOS
- 4 ГБ RAM
- ~500 МБ свободного места (для компиляции)

**Размер:** ~5.7 МБ (исполняемый файл)

## 📥 Установка

### Desktop-версия (Rust)

#### Вариант 1: Скачать готовый релиз (рекомендуется)

1. Перейдите в [Releases](https://github.com/HeBECT/kenshi-trade-map/releases)
2. Скачайте последнюю версию для вашей ОС
3. Распакуйте архив
4. Запустите `kenshi-trade-map.exe` (Windows) или `kenshi-trade-map` (Linux/macOS)

#### Вариант 2: Компиляция из исходников

**Требования:**
- [Rust](https://rustup.rs/) 1.70 или выше

**Шаги:**

```bash
# Клонируйте репозиторий
git clone https://github.com/yourusername/kenshi-trade-map.git
cd kenshi-trade-map/kenshi-trade-map-rust

# Скомпилируйте проект
cargo build --release

# Запустите приложение
cargo run --release
```

**Или используйте скрипты:**

Windows (PowerShell):
```powershell
.\build.ps1  # Компиляция
.\run.ps1    # Запуск
```

Linux/macOS:
```bash
chmod +x build.sh run.sh
./build.sh   # Компиляция
./run.sh     # Запуск
```

## 🎮 Использование

### Desktop-версия (v3.0)

#### Быстрый старт

1. **Загрузите карту**
   - Нажмите `📁 Загрузить карту`
   - Выберите изображение карты Kenshi

2. **Создайте города**
   - Нажмите `🏙️ Добавить город`
   - Кликните на карте в нужном месте
   - Введите название города
   - Нажмите `✓ Подтвердить`

3. **Создайте маршрут**
   - ПКМ по городу (начало маршрута)
   - ЛКМ по другому городу (конец маршрута)
   - Выберите цвет маршрута
   - Введите название (или оставьте автоматическое)
   - Нажмите `✓ Создать`

4. **Добавьте товары**
   - Кликните по линии маршрута
   - Нажмите `+ Добавить товар`
   - Введите название товара и наценку (%)
   - Товар сохранится автоматически

5. **Сохраните данные**
   - Нажмите `💾 Сохранить`
   - Выберите место для сохранения JSON файла

#### Управление

**Навигация:**
- `Колесо мыши` - плавное приближение/отдаление
- `ЛКМ + перетаскивание` - перемещение карты
- `Наведение на город` - показать название

**Создание маршрута:**
- `ПКМ по городу` - начать создание маршрута
- `ЛКМ по городу` - выбрать конец маршрута
- `ESC` - отменить создание

**Работа с маршрутами:**
- `ЛКМ по линии` - открыть панель маршрута
- `🗑️ Удалить маршрут` - удалить выбранный маршрут

**Горячие клавиши:**
- `ESC` - отмена действия / закрыть панель

#### Выбор цвета маршрута

При создании маршрута доступны:

**Color Picker:** Выберите любой цвет RGB

**Предустановленные цвета:**
- 🟢 Зеленый - прибыльные маршруты
- 🔵 Синий - основные маршруты
- 🟠 Оранжевый - средние маршруты
- 🔴 Розовый/Красный - убыточные маршруты
- 🔷 Голубой - водные маршруты
- 🟡 Желтый - экспериментальные
- 🟣 Фиолетовый - специальные
- 🔺 Красный - опасные

#### Примеры использования

**Пример 1: Торговая сеть из центрального города**

```
Хаб → Стоат (🟢 зеленый, железо +50%)
Хаб → Скваин (🔵 синий, ткань +30%)
Хаб → Блистер Хилл (🟠 оранжевый, еда +20%)
Хаб → Монгрел (🔴 красный, робототехника +100%)
```

**Пример 2: Двусторонняя торговля**

```
Хаб → Стоат (🟢 зеленый, железо туда)
Стоат → Хаб (🔵 синий, ткань обратно)
```

**Пример 3: Цветовая организация**

- 🟢 Зеленые маршруты = прибыльные (наценка > 40%)
- 🔵 Синие маршруты = нейтральные (наценка 10-40%)
- 🔴 Красные маршруты = убыточные (наценка < 10%)

### Desktop-версия (v3.0)

```
[Здесь будет скриншот темной темы с маршрутами]
```

**Особенности:**
- Темная тема
- Стрелки направления
- Множественные маршруты от одного города
- Color picker для выбора цвета

## 🛠️ Технологии

### Desktop-версия (Rust)

- **Язык:** Rust 1.70+
- **GUI Framework:** [egui](https://github.com/emilk/egui) 0.27.2
- **Application Framework:** [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) 0.27.2
- **Сериализация:** [serde](https://serde.rs/) 1.0 + [serde_json](https://github.com/serde-rs/json) 1.0
- **Работа с изображениями:** [image](https://github.com/image-rs/image) 0.24
- **Диалоги файлов:** [rfd](https://github.com/PolyMeilex/rfd) 0.14

**Архитектура:**
- Объектно-ориентированный подход
- Структура `KenshiTradeMap` - главный контроллер
- Enum `AppState` - управление состояниями
- Struct `Camera` - управление видом карты
- Плавная интерполяция зума (lerp factor: 0.15)

## 🔧 Разработка

### Структура проекта

```
kenshi-trade-map/
├── kenshi-trade-map-rust/      # Desktop-версия (Rust)
│   ├── src/
│   │   └── main.rs             # Главный файл (~800 строк)
│   ├── Cargo.toml              # Зависимости Rust
│   ├── Cargo.lock              # Версии зависимостей
│   ├── build.ps1               # Скрипт сборки (Windows)
│   ├── run.ps1                 # Скрипт запуска (Windows)
│   ├── build.sh                # Скрипт сборки (Linux/macOS)
│   ├── run.sh                  # Скрипт запуска (Linux/macOS)
│   └── README.md               # Документация Rust-версии
├── README.md                   # Этот файл
├── LICENSE                     # Лицензия
└── .gitignore                  # Git ignore
```

### Сборка Desktop-версии

**Требования:**
- Rust 1.70+
- Cargo (устанавливается с Rust)

**Команды:**

```bash
# Разработка (быстрая сборка)
cargo build

# Релиз (оптимизированная сборка)
cargo build --release

# Запуск
cargo run --release

# Тесты
cargo test

# Проверка кода
cargo clippy

# Форматирование
cargo fmt
```

## 📝 История версий

### v3.0 (Desktop) - Текущая версия ⭐

**Дата:** 2026 год

**Изменения:**
- ✅ Неограниченное количество маршрутов от одного города
- ✅ Один маршрут = 2 точки (начало → конец)
- ✅ Выбор цвета маршрута (color picker + предустановленные)
- ✅ Стрелки направления на маршрутах
- ✅ Темная тема
- ✅ Упрощенная структура данных
- ✅ Удаление базовой и итоговой цены (только наценка)

### v2.0 (Desktop)

**Изменения:**
- ✅ Убраны предустановленные города
- ✅ Добавлена возможность создавать свои города
- ✅ Плавное приближение карты
- ✅ Названия городов только при наведении
- ✅ Сохранение городов вместе с маршрутами

## 🤝 Вклад в проект

Мы приветствуем вклад в проект! Вот как вы можете помочь:

1. **Fork** репозитория
2. Создайте **feature branch** (`git checkout -b feature/AmazingFeature`)
3. **Commit** изменения (`git commit -m 'Add some AmazingFeature'`)
4. **Push** в branch (`git push origin feature/AmazingFeature`)
5. Откройте **Pull Request**

### Идеи для улучшения

- [ ] Импорт данных напрямую из игры
- [ ] Расчет оптимальных маршрутов
- [ ] Учет опасности зон
- [ ] Расчет времени пути
- [ ] Мультиплеер (синхронизация карт)
- [ ] Экспорт карты в изображение
- [ ] Поддержка модов Kenshi
- [ ] Мобильная версия
- [ ] Темы оформления
- [ ] Локализация на другие языки

## 📄 Лицензия

Этот проект распространяется под лицензией MIT. См. файл [LICENSE](LICENSE) для подробностей.

## 🙏 Благодарности

- **Lo-Fi Games** - за создание игры Kenshi
- **egui** - за отличный GUI фреймворк для Rust
- **Сообщество Kenshi** - за вдохновение и поддержку

## 📞 Контакты

- **GitHub Issues:** [Создать issue](https://github.com/HeBECT/kenshi-trade-map/issues)
- **Discussions:** [Обсуждения](https://github.com/HeBECT/kenshi-trade-map/discussions)

## 🌟 Поддержите проект

Если вам нравится этот проект, поставьте ⭐ на GitHub!

---

**Сделано с ❤️ для сообщества Kenshi**

🎮 Приятной торговли в мире Kenshi! 🗺️

════════════════════════════════════════════════════════════════════════════════════════════════════
════════════════════════════════════════════════════════════════════════════════════════════════════
════════════════════════════════════════════════════════════════════════════════════════════════════

# 🗺️ Kenshi Trade Map

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com)

An interactive trade route map for the game **Kenshi** with support for creating cities, planning trade routes, and calculating product markups.

![Kenshi Trade Map](https://img.shields.io/badge/version-3.0-brightgreen.svg)

## 📋 Contents

- [Features](#-features)
- [Versions](#-versions)
- [Installation](#-installation)
- [Usage](#-usage)
- [Screenshots](#-screenshots)
- [Technologies](#-technologies)
- [Development](#-development)
- [License](#-license)

## ✨ Features

### Desktop Version Features (v3.0)

- 🗺️ **Interactive Map** - load a Kenshi map image
- 🏙️ **City Creation** - place cities anywhere on the map
- 🛣️ **Trade Routes** - create routes between cities
- 📦 **Product Management** - add products with markups
- 💾 **Save/Load** - export and import data in JSON format
- 🔍 **Zooming** - smooth map zoom in/out
- 🎨 **Visualization** - color-coded routes
- 🌙 **Dark Theme** - modern dark interface
- ∞ **Unlimited Routes** - create as many routes as you want from one city
- 🎨 **Color Selection** - full color picker + 8 preset colors
- ➡️ **Direction Arrows** - visual indication of route direction
- ⚡ **High Performance** - native Rust application
- 🖥️ **Offline Usage** - no browser required

## 🎯 Versions

### 🦀 Desktop Version (Rust) - v3.0 ⭐ Recommended

**Location:** `kenshi-trade-map-rust/`

**Features:**
- Native Rust application
- Dark theme
- Unlimited routes from a single city
- One route = 2 points (start → end)
- Route color selection (color picker + presets)
- Direction arrows
- High performance
- Smooth map zooming

**System Requirements:**
- Windows 10+ / Linux / macOS
- 4 GB RAM
- ~500 MB free space (for compilation)

**Size:** ~5.7 MB (executable file)

## 📥 Installation

### Desktop Version (Rust)

#### Option 1: Download a Ready Release (Recommended)

1. Go to [Releases](https://github.com/HeBECT/kenshi-trade-map/releases)
2. Download the latest version for your OS
3. Extract the archive
4. Run `kenshi-trade-map.exe` (Windows) or `kenshi-trade-map` (Linux/macOS)

#### Option 2: Build from Source

**Requirements:**
- [Rust](https://rustup.rs/) 1.70 or higher

**Steps:**

```bash
# Clone the repository
git clone https://github.com/yourusername/kenshi-trade-map.git
cd kenshi-trade-map/kenshi-trade-map-rust

# Build the project
cargo build --release

# Run the application
cargo run --release
```

**Or use the scripts:**

Windows (PowerShell):
```powershell
.\build.ps1  # Build
.\run.ps1    # Run
```

Linux/macOS:
```bash
chmod +x build.sh run.sh
./build.sh   # Build
./run.sh     # Run
```

## 🎮 Usage

### Desktop Version (v3.0)

#### Quick Start

1. **Load the map**
   - Click `📁 Load Map`
   - Select a Kenshi map image

2. **Create cities**
   - Click `🏙️ Add City`
   - Click on the desired location on the map
   - Enter the city name
   - Click `✓ Confirm`

3. **Create a route**
   - Right-click a city (route start)
   - Left-click another city (route destination)
   - Choose a route color
   - Enter a name (or leave the automatic one)
   - Click `✓ Create`

4. **Add products**
   - Click the route line
   - Click `+ Add Product`
   - Enter the product name and markup (%)
   - The product will be saved automatically

5. **Save your data**
   - Click `💾 Save`
   - Choose where to save the JSON file

#### Controls

**Navigation:**
- `Mouse wheel` - smooth zoom in/out
- `LMB + drag` - move the map
- `Hover over city` - show city name

**Route Creation:**
- `RMB on city` - start creating a route
- `LMB on city` - select route destination
- `ESC` - cancel creation

**Working with Routes:**
- `LMB on line` - open route panel
- `🗑️ Delete Route` - remove selected route

**Hotkeys:**
- `ESC` - cancel action / close panel

#### Route Color Selection

When creating a route, the following are available:

**Color Picker:** Choose any RGB color

**Preset Colors:**
- 🟢 Green - profitable routes
- 🔵 Blue - main routes
- 🟠 Orange - medium routes
- 🔴 Pink/Red - unprofitable routes
- 🔷 Cyan - water routes
- 🟡 Yellow - experimental
- 🟣 Purple - special
- 🔺 Red - dangerous

#### Usage Examples

**Example 1: Trade Network from a Central City**

```text
Hub → Stoat (🟢 green, iron +50%)
Hub → Squin (🔵 blue, fabric +30%)
Hub → Blister Hill (🟠 orange, food +20%)
Hub → Mongrel (🔴 red, robotics +100%)
```

**Example 2: Two-Way Trading**

```text
Hub → Stoat (🟢 green, iron outbound)
Stoat → Hub (🔵 blue, fabric return)
```

**Example 3: Color Organization**

- 🟢 Green routes = profitable (markup > 40%)
- 🔵 Blue routes = neutral (markup 10-40%)
- 🔴 Red routes = unprofitable (markup < 10%)

## 📸 Screenshots

### Desktop Version (v3.0)

```text
[Dark theme screenshot with routes will be here]
```

**Features:**
- Dark theme
- Direction arrows
- Multiple routes from one city
- Color picker for route selection

## 🛠️ Technologies

### Desktop Version (Rust)

- **Language:** Rust 1.70+
- **GUI Framework:** [egui](https://github.com/emilk/egui) 0.27.2
- **Application Framework:** [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) 0.27.2
- **Serialization:** [serde](https://serde.rs/) 1.0 + [serde_json](https://github.com/serde-rs/json) 1.0
- **Image Processing:** [image](https://github.com/image-rs/image) 0.24
- **File Dialogs:** [rfd](https://github.com/PolyMeilex/rfd) 0.14

**Architecture:**
- Object-oriented approach
- `KenshiTradeMap` struct - main controller
- `AppState` enum - state management
- `Camera` struct - map view control
- Smooth zoom interpolation (lerp factor: 0.15)

## 🔧 Development

### Project Structure

```text
kenshi-trade-map/
├── kenshi-trade-map-rust/      # Desktop version (Rust)
│   ├── src/
│   │   └── main.rs             # Main file (~800 lines)
│   ├── Cargo.toml              # Rust dependencies
│   ├── Cargo.lock              # Dependency versions
│   ├── build.ps1               # Build script (Windows)
│   ├── run.ps1                 # Run script (Windows)
│   ├── build.sh                # Build script (Linux/macOS)
│   ├── run.sh                  # Run script (Linux/macOS)
│   └── README.md               # Rust version documentation
├── README.md                   # This file
├── LICENSE                     # License
└── .gitignore                  # Git ignore
```

### Building the Desktop Version

**Requirements:**
- Rust 1.70+
- Cargo (installed with Rust)

**Commands:**

```bash
# Development build (fast build)
cargo build

# Release build (optimized)
cargo build --release

# Run
cargo run --release

# Tests
cargo test

# Linting
cargo clippy

# Formatting
cargo fmt
```

## 📝 Version History

### v3.0 (Desktop) - Current Version ⭐

**Date:** 2026

**Changes:**
- ✅ Unlimited routes from one city
- ✅ One route = 2 points (start → end)
- ✅ Route color selection (color picker + presets)
- ✅ Direction arrows on routes
- ✅ Dark theme
- ✅ Simplified data structure
- ✅ Removed base/final price (markup only)

### v2.0 (Desktop)

**Changes:**
- ✅ Removed preset cities
- ✅ Added custom city creation
- ✅ Smooth map zooming
- ✅ City names only on hover
- ✅ Saving cities together with routes

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. Create a **feature branch** (`git checkout -b feature/AmazingFeature`)
3. **Commit** your changes (`git commit -m 'Add some AmazingFeature'`)
4. **Push** to the branch (`git push origin feature/AmazingFeature`)
5. Open a **Pull Request**

### Ideas for Improvements

- [ ] Import data directly from the game
- [ ] Optimal route calculations
- [ ] Danger zone tracking
- [ ] Travel time calculation
- [ ] Multiplayer (map synchronization)
- [ ] Export map as image
- [ ] Kenshi mod support
- [ ] Mobile version
- [ ] UI themes
- [ ] Localization for more languages

## 📄 License

This project is distributed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgements

- **Lo-Fi Games** - for creating Kenshi
- **egui** - for the excellent Rust GUI framework
- **Kenshi Community** - for inspiration and support

## 📞 Contacts

- **GitHub Issues:** [Create issue](https://github.com/HeBECT/kenshi-trade-map/issues)
- **Discussions:** [Discussions](https://github.com/HeBECT/kenshi-trade-map/discussions)

## 🌟 Support the Project

If you like this project, give it a ⭐ on GitHub!

---

**Made with ❤️ for the Kenshi community**

🎮 Happy trading in the world of Kenshi! 🗺️