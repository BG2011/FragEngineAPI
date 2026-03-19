# 🚀 FragEngine API Deployment Guide

This guide provides step-by-step instructions for hosting your **FragEngine API** for free. Since the project is built in Rust and uses a PostgreSQL backend, you can host the API server independently on a free platform.

---

## Option 1: Shuttle.rs (Recommended for Rust)

[Shuttle](https://www.shuttle.rs/) is a Cloud development platform for Rust. They have a very generous free tier and specialize in Axum servers.

### 1. Install Shuttle CLI
```bash
curl -sSfL https://www.shuttle.rs/install | bash
shuttle login
```

### 2. Prepare your project
Shuttle requires a small wrapper around your `main` function. For this project, you would modify your API source to include the `#[shuttle_runtime::main]` macro.

### 3. Deploy
```bash
shuttle deploy
```
Your API will be live at `https://your-project-name.shuttleapp.rs`.

---

## Option 2: Render (Easy Docker/Static)

[Render](https://render.com/) is a traditional cloud provider that supports Rust through their **Web Services** (Free tier).

### 1. Create a `Dockerfile`
Render works best with Docker. Use this optimized `Dockerfile` to avoid **Out-Of-Memory (OOM)** errors on the Free Tier:

```dockerfile
FROM rust:1.80 as builder
RUN apt-get update && apt-get install -y pkg-config libssl-dev
WORKDIR /usr/src/app
COPY . .
ENV CARGO_BUILD_JOBS=1
RUN cargo build --release --bin api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/app/target/release/api /usr/local/bin/api
EXPOSE 3000
CMD ["api"]
```

### 2. Connect your GitHub repository
1. Go to [Render Dashboard](https://dashboard.render.com/).
2. Select **New** -> **Web Service**.
3. Connect your GitHub Repo.
4. Set **Environment Variables**:
   - `DATABASE_URL`: Your PostgreSQL connection string.
5. Deploy!

---

## 🛠️ Environment Variables Checklist

Regardless of where you host, you **MUST** configure these variables in the host's dashboard (secrets):

| Variable | Description |
| --- | --- |
| `DATABASE_URL` | Your PostgreSQL connection string. |
| `RUST_LOG` | Set to `info` for logging (optional). |

---

## 🔑 Managing API Keys

After deploying, you can manage user access directly in your database via SQL:

### Create a new Free Key
```sql
INSERT INTO api_keys (key, tier, request_limit) 
VALUES ('user_abc_123', 'FREE', 1000);
```

### Create an Elite Key (Unlimited)
```sql
INSERT INTO api_keys (key, tier, request_limit) 
VALUES ('company_xyz_999', 'ELITE', NULL);
```

### Revoke a Key
```sql
DELETE FROM api_keys WHERE key = 'revoked_key_id';
```
