# blog-cli

CLI-клиент для взаимодействия с Blog API.

## Сборка

```bash
cargo build -p blog-cli --release
```

Результат: `target/release/blog-cli.exe`

## Использование

Глобальные параметры указываются **перед** командой:

```bash
blog-cli [OPTIONS] <COMMAND>
```

### Глобальные параметры

| Параметр          | Описание                           | По умолчанию    |
| :---------------- | :--------------------------------- | :-------------- |
| `--grpc`          | Использовать gRPC вместо HTTP      | HTTP            |
| `--port`          | Порт сервера                       | `8080` (HTTP), `50051` (gRPC) |
| `-s, --server`    | Полный адрес сервера               | `localhost:8080` |

### Команды

#### Регистрация пользователя

```bash
blog-cli register --username "ivan" --email "ivan@example.com" --password "secret123"
```

> Токен **не сохраняется** при регистрации.

#### Вход в систему

```bash
blog-cli login --username "ivan" --password "secret123"
```

> Токен сохраняется в файл `.blog_token`.

#### Создание поста

```bash
blog-cli create --title "Мой первый пост" --content "Содержание поста"
```

#### Получение поста

```bash
blog-cli get --id 1
```

#### Обновление поста

```bash
blog-cli update --id 1 --title "Обновлённый заголовок" --content "Новое содержимое"
```

#### Удаление поста

```bash
blog-cli delete --id 1
```

#### Список постов

```bash
blog-cli list --limit 20 --offset 0
```

## Примеры использования

```bash
# Регистрация и вход (порт 8089)
blog-cli --port 8089 register --username "user1" --email "user@example.com" --password "pass123"
blog-cli --port 8089 login --username "user1" --password "pass123"

# Работа с постами
blog-cli --port 8089 create --title "Привет мир" --content "Это мой первый пост"
blog-cli --port 8089 list --limit 10 --offset 0
blog-cli --port 8089 get --id 1
blog-cli --port 8089 update --id 1 --title "Обновлённый заголовок" --content "Обновлённый текст"
blog-cli --port 8089 delete --id 1

# Использование gRPC
blog-cli --grpc create --title "gRPC пост" --content "Текст"
blog-cli --grpc list --limit 10 --offset 0

# Использование с кастомным портом
blog-cli --port 8089 list
blog-cli --grpc --port 50051 list
```

## Файлы

- `.blog_token` - файл с сохранённым токеном аутентификации (создаётся после команды `login`)

## Зависимости

- [blog-client](../blog-client) - библиотека для взаимодействия с API
