# blog-project

Учебный проект блога на Rust с несколькими клиентами и веб-сервером:

- HTTP + gRPC сервер на `actix-web` и `tonic`
- общая клиентская библиотека
- CLI-клиент
- браузерный WASM-клиент на `Dioxus`
---

## Содержание

- [Что входит в проект](#что-входит-в-проект)
- [Архитектура](#архитектура)
- [Зависимости и окружение](#зависимости-и-окружение)
- [Переменные окружения](#переменные-окружения)
- [Установка и первоначальная настройка](#установка-и-первоначальная-настройка)
- [Сборка и запуск компонентов](#сборка-и-запуск-компонентов)
- [Реальные сценарии использования](#реальные-сценарии-использования)
- [Краткая схема взаимодействия](#краткая-схема-взаимодействия)

---

## Что входит в проект

Workspace состоит из четырёх крейтов:

### `blog-server`

Основной backend.

Что делает:

- поднимает HTTP API на `actix-web`
- поднимает gRPC API на `tonic`
- подключается к PostgreSQL через `sqlx`
- автоматически применяет миграции при старте
- регистрирует пользователей и выдаёт JWT-токены
- создаёт, читает, обновляет и удаляет посты

HTTP API:

- публичные маршруты: `/api/public/...`
- защищённые маршруты: `/api/protected/...`

gRPC API описан в `blog-server/proto/blog.proto`.

### `blog-client`

Общая клиентская библиотека.

Что делает:

- хранит DTO для обмена данными
- содержит HTTP-клиент на `reqwest`
- содержит gRPC-клиент на `tonic`
- даёт единый интерфейс `BlogClientApi`

Особенность:

- по умолчанию включены native-клиенты (`reqwest`, `tonic`)
- для `blog-wasm` default features выключены, и из крейта используются в основном DTO
- общие версии части зависимостей централизованы в корневом `Cargo.toml` через `[workspace.dependencies]`

Это не отдельное приложение, а библиотека, которую используют `blog-cli` и `blog-wasm`.

### `blog-cli`

Консольный клиент на `clap` + `tokio`.

Что делает:

- регистрирует и логинит пользователя
- выводит список постов, а также позволяет создать, отредактировать или удалить пост с проверкой на авторство
- умеет работать как по HTTP, так и по gRPC
- сохраняет токен в локальный файл `.blog_token`

### `blog-wasm`

Браузерный клиент, использующий фреймворк `Dioxus`.

Что делает:

- показывает список постов
- позволяет зарегистрироваться и войти
- позволяет создавать, редактировать и удалять посты из браузера
- хранит токен и имя пользователя в `localStorage`

Этот клиент работает поверх HTTP API сервера.

Важно: в `blog-wasm/Cargo.toml` зависимость на `blog-client` оставлена локальной (`path = "../blog-client"`), потому что для браузерной сборки у неё отключены `default-features`. Это нужно, чтобы не подтягивать native HTTP/gRPC-клиенты в wasm-сборку.

---

## Архитектура

### Архитектура workspace

```text
blog-server  ---> PostgreSQL
	 ^
	 |
	 +--- HTTP --- blog-client --- blog-cli
	 |
	 +--- gRPC --- blog-client --- blog-cli
	 |
	 +--- HTTP --- blog-wasm
```

### Архитектура `blog-server`

Внутри `blog-server` код разделён по слоям:

#### `domain`

Доменная модель и ошибки:

- `user.rs` — сущность пользователя
- `post.rs` — сущность поста
- `error.rs` — доменные и прикладные ошибки

#### `data`

Доступ к данным и репозитории:

- `user_repository.rs` — работа с таблицей `users`
- `post_repository.rs` — работа с таблицей `posts`

Здесь инкапсулированы SQL-запросы к PostgreSQL.

#### `application`

Бизнес-логика:

- `auth_service.rs` — регистрация, логин, получение пользователя, хеширование пароля и выпуск токена
- `blog_service.rs` — CRUD-операции над постами

#### `infrastructure`

Технический слой:

- `config.rs` — чтение переменных окружения
- `database.rs` — подключение к БД и запуск миграций
- `jwt.rs` — генерация и проверка JWT (срок действия 1 час), а также хеширование паролей
- `logging.rs` — настройка логирования

#### `presentation`

Внешние интерфейсы приложения:

- `http_handlers/public.rs` — публичные HTTP endpoints
- `http_handlers/protected.rs` — защищённые HTTP endpoints
- `middleware.rs` — JWT middleware для HTTP и логирование ошибок middleware
- `grpc_service.rs` — gRPC-реализация сервиса
- `proto.rs` — сгенерированные gRPC-типы

### Схема данных

Миграции создают две таблицы:

- `users`
  - `id`
  - `username`
  - `email`
  - `password_hash`
  - `created_at`
- `posts`
  - `id`
  - `title`
  - `content`
  - `author_id`
  - `created_at`
  - `updated_at`

Связь: `posts.author_id -> users.id`.

### Как крейты связаны между собой

- `blog-server` предоставляет HTTP и gRPC API
- `blog-client` описывает общий клиентский слой для взаимодействия с сервером
- `blog-cli` использует `blog-client` и умеет переключаться между HTTP и gRPC
- `blog-wasm` использует DTO из `blog-client`, но сам ходит в HTTP API из браузера

Отдельно важно:

- proto-контракт лежит и в `blog-server/proto/blog.proto`, и в `blog-client/proto/blog.proto`
- в `build.rs` у `blog-server` и `blog-client` используется `protoc-bin-vendored`, поэтому отдельно устанавливать `protoc` не требуется

---

## Зависимости и окружение

Для локального запуска понадобятся:

1. Rust toolchain с `cargo`
2. PostgreSQL
3. Для браузерной части — `dx` (Dioxus CLI)
4. Для примеров через `curl` — любой HTTP-клиент, который умеет отправлять JSON

### Управление зависимостями в workspace

Часть повторяющихся зависимостей вынесена в корневой `Cargo.toml` в секцию `[workspace.dependencies]`. 
Сейчас из workspace наследуются версии для:

- `blog-client`
- `anyhow`
- `chrono`
- `prost`
- `protoc-bin-vendored`
- `serde`
- `thiserror`
- `tonic`
- `tonic-prost`
- `tonic-prost-build`
- `uuid`

Отдельное исключение — `blog-wasm`: он не наследует `blog-client` через `workspace = true`, потому что использует `default-features = false`. Такое переопределение нельзя наложить поверх workspace-наследования, поэтому для wasm-крейта эта зависимость оставлена отдельной строкой в его собственном `Cargo.toml`.
---

## Переменные окружения

Сервер читает настройки из `blog-server/.env`.

Основные переменные:

| Переменная | Обязательна | Пример | Назначение |
|---|---:|---|---|
| `HOST` | да | `127.0.0.1` | Адрес HTTP и gRPC сервера |
| `PORT` | да | `8080` | HTTP-порт |
| `GRPC_PORT` | да | `50051` | gRPC-порт |
| `DATABASE_URL` | да | `postgres://postgres:password@127.0.0.1:5432/blog` | Строка подключения к PostgreSQL |
| `JWT_SECRET` | да | `base64-or-random-secret` | Секрет для подписи JWT |
| `CORS_ORIGIN` | нет | `http://localhost:8081` | Origin фронтенда |

При этом в `blog-server/src/main.rs` сейчас включён `allow_any_origin()`, а значение `cors_origin` фактически не используется.
---

## Установка и первоначальная настройка

### 1. Установить Rust
### 2. Установить Dioxus CLI

Для запуска `blog-wasm` нужен `dx`.

```powershell
cargo install dioxus-cli
```

Проверка:

```powershell
dx --help
```

### 3. Добавить wasm target

```powershell
rustup target add wasm32-unknown-unknown
```

### 4. Установить PostgreSQL

Подойдёт локальный PostgreSQL любой актуальной версии, совместимой с `sqlx` и `postgres` драйвером.

После установки нужно:

- создать БД `blog`
- подготовить пользователя и пароль
- прописать корректный `DATABASE_URL`

### 5. Сгенерировать JWT-секрет

В проекте используется симметричный секрет `JWT_SECRET`, а не пара приватный/публичный ключ.

Пример генерации случайного секрета в PowerShell:

```powershell
$bytes = New-Object byte[] 32
[Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes)
[Convert]::ToBase64String($bytes)
```

Скопируйте результат и подставьте в `JWT_SECRET`.

### 6. Подготовить `blog-server/.env`

Пример рабочего файла:

```dotenv
HOST=127.0.0.1
PORT=8080
GRPC_PORT=50051
DATABASE_URL=postgres://blog_user:blog_password@127.0.0.1:5432/blog
JWT_SECRET=PUT_RANDOM_SECRET_HERE
CORS_ORIGINS=http://localhost:8081
```

### 7. Проверить сборку workspace

Из корня проекта:

```powershell
Set-Location "K:\Repositories\yandex_course\blog-project"
cargo check --workspace
```

Миграции отдельной командой запускать не нужно: сервер сам вызывает `sqlx::migrate!()` при старте.

---

## Сборка и запуск компонентов

Все команды ниже выполняются из корня workspace: `blog-project`.

### 1. `blog-server`

Сборка:

```powershell
cargo build -p blog-server
```

Запуск:

```powershell
cargo run -p blog-server
```

После старта сервер ожидаемо поднимает:

- HTTP: `http://127.0.0.1:8080`
- gRPC: `http://127.0.0.1:50051`

Проверка health endpoint:

```powershell
curl.exe http://127.0.0.1:8080/api/public/health
```

### 2. `blog-client`

Это библиотека, у неё нет собственного исполняемого файла.

Сборка библиотеки:

```powershell
cargo build -p blog-client
```

Проверка, что она компилируется в составе workspace:

```powershell
cargo check -p blog-client
```

Как использовать:

- через `blog-cli`
- из собственного Rust-приложения
- как набор DTO для `blog-wasm`

### 3. `blog-cli`

Сборка:

```powershell
cargo build -p blog-cli
```

Справка по CLI:

```powershell
cargo run -p blog-cli -- --help
```

Запуск через HTTP по умолчанию:

```powershell
cargo run -p blog-cli -- register --name alice --email alice@example.com --password secret123
```

Явный адрес HTTP-сервера:

```powershell
cargo run -p blog-cli -- --server http://127.0.0.1:8080 list --limit 10 --offset 0
```

Запуск через gRPC:

```powershell
cargo run -p blog-cli -- --grpc login --name alice --password secret123
cargo run -p blog-cli -- --grpc create --title "Первый пост" --content "Текст через gRPC"
```

CLI хранит токен в файле `.blog_token` в текущей рабочей директории.

### 4. `blog-wasm`

#### Сборка web-версии

```powershell
dx build --platform web -p blog-wasm
```

Результат сборки попадает в каталог вроде:

```text
target/dx/blog-wasm/debug/web/public
```

#### Запуск dev-сервера для браузера

```powershell
dx serve --platform web -p blog-wasm --port 8081 --open false
```

После этого откройте в браузере:

```text
http://127.0.0.1:8081
```

#### Что важно для `blog-wasm`

- в `blog-wasm/src/api.rs` базовый URL API сейчас захардкожен как `http://127.0.0.1:8080/api`
- значит сервер должен быть запущен именно на `127.0.0.1:8080`, если вы не меняли код
- обычный `cargo run -p blog-wasm` не запускает браузерный UI; он только выводит пояснение, что крейт предназначен для `wasm32-unknown-unknown`

Проверочный запуск native-заглушки:

```powershell
cargo run -p blog-wasm
```

---

## Реальные сценарии использования

Ниже примеры, которые отражают реальные маршруты и команды из кода проекта.

### Сценарий 1. Проверить, что сервер жив

```powershell
curl.exe http://127.0.0.1:8080/api/public/health
```

Ожидается JSON примерно такого вида:

```json
{
  "status": "ok",
  "timestamp": "2026-04-19T12:34:56.000Z"
}
```

### Сценарий 2. Зарегистрировать пользователя через `curl`

```powershell
curl.exe -X POST http://127.0.0.1:8080/api/public/register `
  -H "Content-Type: application/json" `
  -d '{"name":"alice","email":"alice@example.com","password":"secret123"}'
```

Ответ содержит `user` и `token`.

### Сценарий 3. Войти и сохранить токен в переменную PowerShell

```powershell
$login = Invoke-RestMethod -Method POST -Uri "http://127.0.0.1:8080/api/public/login" `
  -ContentType "application/json" `
  -Body '{"name":"alice","password":"secret123"}'

$token = $login.token
$token
```

### Сценарий 4. Создать пост через защищённый HTTP endpoint

```powershell
Invoke-RestMethod -Method POST -Uri "http://127.0.0.1:8080/api/protected/posts" `
  -Headers @{ Authorization = "Bearer $token" } `
  -ContentType "application/json" `
  -Body '{"title":"Первый пост","content":"Пост создан через PowerShell"}'
```

### Сценарий 5. Получить список постов

```powershell
curl.exe "http://127.0.0.1:8080/api/public/posts?limit=10&offset=0"
```

### Сценарий 6. Получить один пост по id

```powershell
curl.exe http://127.0.0.1:8080/api/public/posts/PUT_POST_ID_HERE
```

### Сценарий 7. Обновить пост

```powershell
Invoke-RestMethod -Method PUT -Uri "http://127.0.0.1:8080/api/protected/posts/PUT_POST_ID_HERE" `
  -Headers @{ Authorization = "Bearer $token" } `
  -ContentType "application/json" `
  -Body '{"title":"Обновлённый заголовок","content":"Обновлённый текст"}'
```

### Сценарий 8. Удалить пост

```powershell
Invoke-RestMethod -Method DELETE -Uri "http://127.0.0.1:8080/api/protected/posts/PUT_POST_ID_HERE" `
  -Headers @{ Authorization = "Bearer $token" }
```

### Сценарий 9. Работа через CLI по HTTP

#### Регистрация

```powershell
cargo run -p blog-cli -- register --name alice --email alice@example.com --password secret123
```

#### Логин

```powershell
cargo run -p blog-cli -- login --name alice --password secret123
```

#### Создание поста

```powershell
cargo run -p blog-cli -- create --title "CLI post" --content "Создано из CLI"
```

#### Список постов

```powershell
cargo run -p blog-cli -- list --limit 10 --offset 0
```

#### Получение поста

```powershell
cargo run -p blog-cli -- get --id PUT_POST_ID_HERE
```

#### Обновление поста

```powershell
cargo run -p blog-cli -- update --id PUT_POST_ID_HERE --title "New title" --content "New content"
```

#### Удаление поста

```powershell
cargo run -p blog-cli -- delete --id PUT_POST_ID_HERE
```

### Сценарий 10. Работа через CLI по gRPC

Те же сценарии, но с флагом `--grpc`:

```powershell
cargo run -p blog-cli -- --grpc register --name alice --email alice@example.com --password secret123
cargo run -p blog-cli -- --grpc login --name alice --password secret123
cargo run -p blog-cli -- --grpc create --title "gRPC post" --content "Создано через gRPC"
cargo run -p blog-cli -- --grpc list --limit 10 --offset 0
```

По умолчанию CLI ожидает gRPC сервер на `http://localhost:50051`.

### Сценарий 11. Работа через браузер (`blog-wasm`)

1. Запустите сервер:

   ```powershell
   cargo run -p blog-server
   ```

2. В отдельном терминале запустите web-клиент:

   ```powershell
   dx serve --platform web -p blog-wasm --port 8081 --open false
   ```

3. Откройте в браузере:

   ```text
   http://127.0.0.1:8081
   ```

4. На странице:
   - нажмите **Регистрация**
   - введите имя, email и пароль
   - после успешной регистрации токен сохранится в `localStorage`

5. Затем можно:
   - нажать **Вход**, если регистрировались раньше
   - нажать **Новый пост**
   - создать пост
   - отредактировать пост через форму редактирования
   - удалить пост кнопкой удаления

6. После перезагрузки страницы авторизация сохраняется, потому что `blog-wasm` хранит:
   - `blog_token`
   - `blog_user_id`
   - `blog_user_name`

---

## Краткая схема взаимодействия

### HTTP-маршруты

Публичные:

- `GET /api/public/health`
- `POST /api/public/register`
- `POST /api/public/login`
- `GET /api/public/posts`
- `GET /api/public/posts/{id}`

Защищённые:

- `POST /api/protected/posts`
- `PUT /api/protected/posts/{id}`
- `DELETE /api/protected/posts/{id}`

Для защищённых маршрутов нужен заголовок:

```text
Authorization: Bearer <JWT>
```

### gRPC-методы

Сервис `BlogService` предоставляет:

- `Register`
- `Login`
- `CreatePost`
- `GetPost`
- `UpdatePost`
- `DeletePost`
- `ListPosts`

---

## Быстрый старт

Если нужен самый короткий путь, то последовательность такая:

```powershell
Set-Location "K:\Repositories\yandex_course\blog-project"
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli
```

Подготовьте `blog-server/.env`, затем:

```powershell
cargo check --workspace
cargo run -p blog-server
```

В новом терминале:

```powershell
dx serve --platform web -p blog-wasm --port 8081 --open false
```

Или вместо браузера используйте CLI:

```powershell
cargo run -p blog-cli -- register --name alice --email alice@example.com --password secret123
cargo run -p blog-cli -- create --title "Hello" --content "From CLI"
```