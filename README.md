# Blog System (Rust Fullstack)

Проект представляет собой систему управления блогом, реализованную на языке Rust с использованием различных архитектурных подходов (HTTP, gRPC, WASM).

## Архитектура проекта

Проект разбит на несколько независимых компонентов, которые взаимодействуют через общую библиотеку клиента.

### Компоненты:

1.  **`blog-server` (Backend)**
    *   **Роль:** Центральный узел системы.
    *   **Технологии:** `Actix-web` (HTTP API), `Tonic` (gRPC), `SQLx` (PostgreSQL), `Tokio` (Runtime).
    *   **Функционал:** Предоставляет два интерфейса одновременно:
        *   **REST API:** Для веб-клиентов (WASM) и простых HTTP запросов.
        *   **gRPC API:** Для высокопроизводительного взаимодействия (CLI или микросервисы).
    *   **Безопасность:** Реализована аутентификация через JWT.

2.  **`blog-client` (Core Library)**
    *   **Роль:** Общая библиотека для всех клиентов.
    *   **Функционал:** Содержит логику выбора транспорта (HTTP или gRPC), модели данных (`Post`, `User`) и реализации запросов.
    *   **Особенность:** Поддерживает условную компиляцию (`#[cfg(target_arch = "wasm32")]`), что позволяет одной и той же библиотеке работать и в обычном окружении, и в браузере.

3.  **`blog-cli` (Command Line Interface)**
    *   **Роль:** Инструмент управления блогом через терминал.
    *   **Технологии:** `Clap` (парсинг аргументов), `reqwest` (HTTP), `tonic` (gRPC).
    *   **Функционал:** Позволяет регистрировать пользователей, логиниться, создавать, читать, обновлять и удалять посты. Поддерживает сохранение JWT-токена в локальный файл `.blog_token`.

4.  **`blog-wasm` (Web Frontend)**
    *   **Роль:** Веб-интерфейс блога.
    *   **Технологии:** `Yew` (фреймворк), `wasm-bindgen`, `gloo-net` (HTTP для WASM).
    *   **Функционал:** Компилируется в WebAssembly и работает непосредственно в браузере, взаимодействуя с сервером через REST API.

---

## Установка и настройка

### Предварительные требования
*   **Rust:** `rustup` (установленный `wasm32-unknown-unknown` target для WASM).
*   **PostgreSQL:** Установленный и запущенный сервер БД.
*   **Protobuf Compiler:** `protoc` (необходим для генерации gRPC кода).

### 1. Настройка базы данных
Создайте базу данных в PostgreSQL:
```sql
CREATE DATABASE blog_db;
```

### 2. Настройка окружения
Создайте файл `.env` в директории `blog-server/`:
```env
DATABASE_URL=postgres://username:password@localhost:5432/blog_db
PORT=8080
DEBUG=true
JWT_SECRET=your_super_secret_random_string_here
```

> **Важно:** Сгенерируйте надежный секретный ключ для `JWT_SECRET`.

---

## Запуск компонентов

### 1. Запуск сервера
```bash
cd blog-server
cargo run
```
*Сервер запустит HTTP на порту 8080 и gRPC на порту 50051.*

### 2. Запуск CLI
Сначала соберите CLI:
```bash
cd blog-cli
cargo build --release
```
Использование (примеры):
```bash
# Регистрация
./target/release/blog-cli register --username alice --email alice@example.com --password password123

# Вход (сохранит токен)
./target/release/blog-cli login --username alice --password password123

# Создание поста
./target/release/blog-cli create --title "Привет, Rust!" --content "Это мой первый пост."

# Список постов
./target/release/blog-cli list
```

### 3. Запуск WASM (Web)
Для запуска WASM-компонента рекомендуется использовать `trunk`:
```bash
cd blog-wasm
trunk serve
```
Откройте `http://localhost:8080` (или порт, указанный trunk) в браузере.

---

## Примеры использования (Сценарии)

### Сценарий 1: Полный цикл через CLI (gRPC)
Использование флага `--grpc` заставляет клиент использовать протокол gRPC вместо HTTP.

1.  **Регистрация:**
    `./blog-cli register --username bob --email bob@test.com --password secret`
2.  **Вход:**
    `./blog-cli login --username bob --password secret`
3.  **Создание поста (через gRPC):**
    `./blog-cli --grpc create --title "gRPC Post" --content "Hello via gRPC"`
4.  **Проверка списка:**
    `./blog-cli --grpc list`

### Сценарий 2: Прямые HTTP запросы (curl)
Если вы хотите проверить REST API напрямую:

1.  **Регистрация:**
    ```bash
    curl -X POST http://localhost:8080/api/auth/register \
         -H "Content-Type: application/json" \
         -d '{"username": "curl_user", "email": "curl@test.com", "password": "password"}'
    ```
2.  **Логин (получение токена):**
    ```bash
    curl -X POST http://localhost:8080/api/auth/login \
         -H "Content-Type: application/json" \
         -d '{"username": "curl_user", "password": "password"}'
    ```
3.  **Создание поста (с JWT):**
    ```bash
    curl -X POST http://localhost:8080/api/posts \
         -H "Authorization: Bearer <<<YOURYOUR_TOKEN_HERE>>" \
         -H "Content-Type: application/json" \
         -d '{"title": "Curl Post", "content": "Testing REST API"}'
    ```
