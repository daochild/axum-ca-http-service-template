# Quick Start Guide

## Prerequisites

- Rust 1.75+
- Docker and Docker Compose
- PostgreSQL 15+ (if running locally)
- Redis 7+ (if running locally)

## Option 1: Quick Start with Docker (Recommended)

```bash
# 1. Navigate to project directory
cd tokio-test

# 2. Copy environment file
cp .env.example .env

# 3. Start all services with Docker Compose
docker-compose up -d

# 4. Check logs
docker-compose logs -f app

# 5. Test health endpoint
curl http://localhost:8080/health

# 6. Stop all services
docker-compose down
```

## Option 2: Local Development

```bash
# 1. Start PostgreSQL and Redis with Docker
docker-compose up -d postgres redis

# 2. Copy environment file
cp .env.example .env

# 3. Install sqlx-cli (if not installed)
cargo install sqlx-cli --no-default-features --features postgres

# 4. Run database migrations
sqlx migrate run

# 5. Build the project
cargo build --release

# 6. Run the application
cargo run --release
```

## Testing

### Test Health Endpoint
```bash
curl http://localhost:8080/health
```

Expected output:
```json
{
  "status": "healthy",
  "services": {
    "postgres": "up",
    "redis": "up"
  }
}
```

### Test WebSocket Connection

#### Using websocat (CLI tool)
```bash
# Install websocat
cargo install websocat

# Connect to WebSocket
websocat ws://localhost:8080/ws

# Send a message (paste this JSON and press Enter)
{"content": "Hello, World!", "user_id": "test_user"}
```

#### Using HTML Test Client
Open `test-client.html` in your browser and it will automatically connect to the WebSocket server.

#### Using JavaScript
```javascript
const ws = new WebSocket('ws://localhost:8080/ws');

ws.onopen = () => {
    console.log('Connected');
    ws.send(JSON.stringify({
        content: 'Hello from JavaScript!',
        user_id: 'js_user'
    }));
};

ws.onmessage = (event) => {
    const message = JSON.parse(event.data);
    console.log('Received:', message);
};
```

## Common Commands

### Docker Commands
```bash
# View running containers
docker-compose ps

# View logs for specific service
docker-compose logs -f app
docker-compose logs -f postgres
docker-compose logs -f redis

# Restart a service
docker-compose restart app

# Stop all services
docker-compose down

# Stop and remove volumes
docker-compose down -v

# Rebuild and restart
docker-compose up -d --build
```

### Database Commands
```bash
# Connect to PostgreSQL
docker-compose exec postgres psql -U postgres -d tokio_test

# View tables
\dt

# View messages
SELECT * FROM messages ORDER BY created_at DESC LIMIT 10;

# Exit
\q
```

### Redis Commands
```bash
# Connect to Redis CLI
docker-compose exec redis redis-cli

# Subscribe to messages channel
SUBSCRIBE messages

# View all keys
KEYS *

# Exit
exit
```

## Troubleshooting

### Port Already in Use
If you get "address already in use" error:
```bash
# Windows
netstat -ano | findstr :8080
taskkill /PID <PID> /F

# Linux/Mac
lsof -i :8080
kill -9 <PID>
```

### Database Connection Issues
```bash
# Check if PostgreSQL is running
docker-compose ps postgres

# View PostgreSQL logs
docker-compose logs postgres

# Restart PostgreSQL
docker-compose restart postgres
```

### Redis Connection Issues
```bash
# Check if Redis is running
docker-compose ps redis

# Test Redis connection
docker-compose exec redis redis-cli ping

# Should return: PONG
```

### Clean Start
```bash
# Stop everything
docker-compose down -v

# Remove build artifacts
cargo clean

# Start fresh
docker-compose up -d --build
```

## Project Structure

```
tokio-test/
├── src/
│   ├── domain/           # Business logic & interfaces
│   ├── application/      # Use cases
│   ├── infrastructure/   # Database & Redis implementations
│   ├── presentation/     # HTTP handlers & routes
│   └── main.rs          # Application entry point
├── migrations/           # SQL migrations
├── Cargo.toml           # Dependencies
├── Dockerfile           # Container image
├── docker-compose.yml   # Multi-container setup
├── .env.example         # Environment template
├── test-client.html     # WebSocket test client
└── README.md            # Documentation
```

## Next Steps

1. **Add Authentication**: Implement JWT tokens for WebSocket connections
2. **Add Rooms**: Support multiple chat rooms/channels
3. **Add Persistence**: Store user sessions in Redis
4. **Add Metrics**: Integrate Prometheus metrics
5. **Add Tests**: Unit and integration tests
6. **Add Rate Limiting**: Prevent abuse
7. **Add CORS**: Configure for production

## Performance Tips

- Use connection pooling (already configured)
- Monitor Redis pub/sub performance
- Add indexes to frequently queried columns
- Use Redis for caching frequently accessed data
- Configure proper logging levels in production

## Security Considerations

- Always use TLS/SSL in production
- Implement proper authentication
- Validate and sanitize all user inputs
- Use environment variables for secrets
- Keep dependencies updated
- Enable rate limiting

## Links

- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Documentation](https://tokio.rs/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [Redis Documentation](https://redis.io/docs/)
