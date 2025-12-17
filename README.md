# WebSocket Server з Clean Architecture

WebSocket сервер на Rust з використанням Axum, Tokio, PostgreSQL та Redis, реалізований за принципами Clean Architecture.

## Архітектура

Проєкт організований у 4 шари:

- **Domain Layer** (`src/domain/`) - бізнес-логіка, entities та інтерфейси
- **Application Layer** (`src/application/`) - use cases та оркестрація
- **Infrastructure Layer** (`src/infrastructure/`) - імплементації БД, Redis, конфігурація
- **Presentation Layer** (`src/presentation/`) - HTTP handlers, WebSocket, routes

## Технології

- **Axum** - веб-фреймворк
- **Tokio** - асинхронний runtime
- **SQLx** - PostgreSQL client з compile-time перевіркою
- **Redis** - pub/sub для real-time messaging
- **Tracing** - структуроване логування
- **Docker** - контейнеризація

## API Endpoints

### WebSocket Connection
```
WS /ws
```

**Incoming message format:**
```json
{
  "content": "Hello, World!",
  "user_id": "user123"
}
```

**Outgoing message format:**
```json
{
  "id": "uuid-v4",
  "content": "Hello, World!",
  "user_id": "user123",
  "created_at": "2023-12-17T10:00:00Z"
}
```

### Health Check
```
GET /health
```

**Response (200 OK):**
```json
{
  "status": "healthy",
  "services": {
    "postgres": "up",
    "redis": "up"
  }
}
```

**Response (503 Service Unavailable):**
```json
{
  "status": "unhealthy",
  "services": {
    "postgres": "down",
    "redis": "up"
  }
}
```

## Запуск з Docker

### Повний стек (PostgreSQL + Redis + App)

```bash
# Створити .env файл
cp .env.example .env

# Запустити всі сервіси
docker-compose up -d

# Переглянути логи
docker-compose logs -f app

# Зупинити сервіси
docker-compose down
```

### Тільки інфраструктура (для локальної розробки)

```bash
# Запустити тільки PostgreSQL та Redis
docker-compose up -d postgres redis

# Створити .env файл
cp .env.example .env

# Запустити додаток локально
cargo run
```

## Локальна розробка

### Вимоги

- Rust 1.75+
- PostgreSQL 15+
- Redis 7+
- Docker (опціонально)

### Встановлення залежностей

```bash
cargo build
```

### Налаштування бази даних

```bash
# Встановити sqlx-cli
cargo install sqlx-cli --no-default-features --features postgres

# Створити базу даних
createdb tokio_test

# Запустити міграції
sqlx migrate run
```

### Запуск

```bash
# З .env файлом
cargo run

# Або з environment variables
DATABASE_URL=postgres://postgres:postgres@localhost:5432/tokio_test \
REDIS_URL=redis://localhost:6379 \
cargo run
```

## Тестування

### Тест WebSocket з'єднання

```bash
# Використати websocat
websocat ws://localhost:8080/ws
```

Відправити повідомлення:
```json
{"content": "Test message", "user_id": "user1"}
```

### Тест Health Check

```bash
curl http://localhost:8080/health
```

### JavaScript клієнт

```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = () => {
  console.log('Connected');
  ws.send(JSON.stringify({
    content: 'Hello from browser!',
    user_id: 'browser_user'
  }));
};

ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log('Received:', message);
};
```

## Структура проєкту

```
tokio-test/
├── src/
│   ├── domain/
│   │   ├── entities/
│   │   │   └── message.rs
│   │   ├── repositories/
│   │   │   └── message_repository.rs
│   │   └── services/
│   │       └── pubsub_service.rs
│   ├── application/
│   │   └── use_cases/
│   │       ├── send_message.rs
│   │       └── health_check.rs
│   ├── infrastructure/
│   │   ├── config.rs
│   │   ├── postgres.rs
│   │   └── redis_pubsub.rs
│   ├── presentation/
│   │   ├── handlers/
│   │   │   ├── websocket.rs
│   │   │   └── health.rs
│   │   └── routes.rs
│   └── main.rs
├── migrations/
│   └── 20231217000001_create_messages_table.sql
├── Cargo.toml
├── Dockerfile
├── docker-compose.yml
└── .env.example
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `postgres://postgres:postgres@localhost:5432/tokio_test` | PostgreSQL connection string |
| `REDIS_URL` | `redis://localhost:6379` | Redis connection string |
| `SERVER_HOST` | `0.0.0.0` | Server host |
| `SERVER_PORT` | `8080` | Server port |
| `RUST_LOG` | `info` | Log level (trace, debug, info, warn, error) |

## Особливості реалізації

### Clean Architecture

- **Domain Layer** не залежить від жодних зовнішніх фреймворків
- **Dependency Inversion** через traits (Repository, PubSubService)
- **Use Cases** інкапсулюють бізнес-логіку
- **Infrastructure** імплементує domain traits

### WebSocket Real-time Broadcasting

1. Клієнт відправляє повідомлення через WebSocket
2. Повідомлення зберігається в PostgreSQL
3. Повідомлення публікується в Redis pub/sub
4. Всі підключені клієнти отримують повідомлення через Redis

### Database Migrations

SQLx автоматично запускає міграції при старті додатку.

### Health Checks

Health endpoint перевіряє з'єднання з PostgreSQL та Redis і повертає статус кожного сервісу.

## Troubleshooting

### Помилка підключення до PostgreSQL

```bash
# Перевірити чи запущений PostgreSQL
docker-compose ps postgres

# Переглянути логи
docker-compose logs postgres
```

### Помилка підключення до Redis

```bash
# Перевірити чи запущений Redis
docker-compose ps redis

# Переглянути логи
docker-compose logs redis
```

## Ліцензія

MIT
