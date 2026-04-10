<p align="center">
  <img src="assets/axumkit_banner.png" alt="AxumKit" />
</p>

# AxumKit
Production-ready Rust web backend template.

## Features

- **Auth**:Session (Redis), email/password (Argon2), OAuth2 (Google, GitHub), TOTP 2FA
- **Users & Posts**:Profiles, image uploads (R2), CRUD with ownership
- **Search**:Full-text via MeiliSearch, auto-indexed by worker
- **Background Jobs**:NATS JetStream worker (email, indexing, cleanup, cron)
- **Email**:SMTP templates (Lettre + MRML + MiniJinja)
- **Rate Limiting**:Sliding window (Redis Lua), per-route
- **API Docs**:Auto-generated Swagger UI (debug builds)
- **Deploy**:Docker multi-stage, Helm charts, GitHub Actions CI/CD

## Quick Start

```bash
git clone https://github.com/levish0/AxumKit.git && cd AxumKit
cp .env.example .env  # edit with your config

cd crates/migration && cargo run && cd ../..  # migrations
cargo run -p kit_server                   # API server
cargo run -p kit_worker                   # worker (separate terminal)
```

## Project Structure

```
crates/
├── kit-server     # API (handlers → services → repositories → entities)
├── kit-worker     # Background jobs (NATS consumers, cron)
├── kit-config     # Env config
├── kit-constants  # Shared constants
├── kit-dto        # Request / response types
├── kit-entity     # SeaORM models
├── kit-errors     # Centralized error handling
├── migration      # DB migrations
└── e2e            # E2E tests
```

## DDD Architecture

AxumKit follows Domain-Driven Design (DDD) principles with a modular architecture:

### Core Domain Modules

| Domain Module | Description | Core Features |
|--------------|-------------|---------------|
| `kit-auth`   | Authentication Domain | Login, logout, registration, password management, TOTP 2FA |
| `kit-user`   | User Domain | User profile management, role management, user banning |
| `kit-oauth`  | Third-party Authentication Domain | Google, GitHub login and connection |
| `kit-search` | Search Domain | Full-text search functionality |
| `kit-action-log` | Action Log Domain | System action and event recording |
| `kit-moderation` | Moderation Domain | Content moderation and management |
| `kit-health` | Health Check Domain | System health status checking |

### Shared Modules

| Shared Module | Description |
|--------------|-------------|
| `kit-config`     | Global configuration management |
| `kit-constants`  | Shared constants definition |
| `kit-errors`     | Centralized error handling system |
| `kit-worker`     | Background task processing |

## Configuration

Env vars from `.env`, validated at startup. See [`.env.example`](.env.example) for the full list.

## License

[MIT](LICENSE)
