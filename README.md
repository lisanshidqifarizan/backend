## Table of Contents

- [Backend Documentation — Rust Axum API](#backend-documentation--rust-axum-api)
- [Dokumentasi Self Hosting Reverse Proxy dengan Docker + Cloudflare Tunnel](#dokumentasi-self-hosting-reverse-proxy-dengan-docker--cloudflare-tunnel)


# Backend Documentation — Rust Axum API

## Overview

Backend API berbasis Rust menggunakan framework Axum dengan arsitektur asynchronous Tokio runtime dan PostgreSQL sebagai database utama.

Project ini dirancang untuk:

- REST API
- Authentication system
- Blog backend
- Fullstack integration
- Self-hosting menggunakan Docker + Cloudflare Tunnel

---

# Tech Stack

| Technology | Purpose |
| --- | --- |
| Rust | Main programming language |
| Axum | Web framework |
| Tokio | Async runtime |
| Serde | Serialization & deserialization |
| SQLx | Database ORM/query |
| PostgreSQL | Main database |
| UUID | Unique identifiers |
| Chrono | Date & time |
| Dotenv | Environment loader |
| Docker | Containerization |
| Cloudflare Tunnel | Reverse proxy & public access |

---

# Project Structure

```
backend/
├── src/
│   ├── main.rs
│   ├── routes/
│   ├── handlers/
│   ├── models/
│   ├── database/
│   └── utils/
│
├── Cargo.toml
├── Dockerfile
├── .env
└── README.md
```

---

# Cargo.toml

## Dependencies

```
[package]
name = "backend"
version = "0.1.0"
edition = "2024"

[dependencies]
tokio = { version = "1.52.3", features = ["full"] }

axum = "0.8.9"

serde = { version = "1.0.228", features = ["derive"] }

dotenv = "0.15.0"

uuid = { version = "1.23.1", features = ["v4", "serde"] }

chrono = { version = "0.4", features = ["serde"] }

sqlx = {
    version = "0.8.6",
    features = [
        "runtime-tokio-rustls",
        "postgres",
        "uuid",
        "chrono"
    ]
}
```

---

# Dependency Explanation

## Tokio

Async runtime untuk menjalankan asynchronous tasks.

Digunakan oleh:

- Axum
- SQLx
- async networking
- concurrency

---

## Axum

Framework backend modern berbasis Tower ecosystem.

Digunakan untuk:

- routing
- middleware
- REST API
- JSON response
- extractor system

---

## Serde

Serialization & deserialization Rust.

Digunakan untuk:

- JSON request
- JSON response
- body parsing

---

## Dotenv

Membaca file `.env`.

Digunakan untuk:

- DATABASE_URL
- JWT_SECRET
- environment variables

---

## UUID

Unique identifier generator.

Digunakan untuk:

- user ID
- post ID
- token
- entity identifier

---

## Chrono

Date & time handling.

Digunakan untuk:

- created_at
- updated_at
- timestamp

---

## SQLx

Async SQL toolkit untuk Rust.

Digunakan untuk:

- PostgreSQL query
- async database
- type-safe query
- pool connection

---

# Environment Variables

## .env

```
DATABASE_URL=postgresql://postgres:password@localhost:5432/backend

PORT=3000

JWT_SECRET=your_secret_key
```

---

# Running Backend

## Development Mode

```
cargo run
```

---

# Build Release

```
cargo build--release
```

Binary hasil build:

```
target/release/backend
```

---

# Axum Server Configuration

## Example

```
letlistener= TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();

axum::serve(listener,app)
    .await
    .unwrap();
```

---

# Why 0.0.0.0?

Agar server dapat diakses:

- Docker
- reverse proxy
- Cloudflare Tunnel
- external network

---

# REST API Architecture

## Request Flow

```
Client
↓
Router
↓
Handler
↓
Database
↓
JSON Response
```

---

# Example Routes

| Method | Endpoint | Description |

|---|---|

| GET | /api/v1/posts | Get all posts |

| GET | /api/v1/posts/:id | Get single post |

| POST | /api/v1/posts | Create post |

| PUT | /api/v1/posts/:id | Update post |

| DELETE | /api/v1/posts/:id | Delete post |

---

# JSON Response Example

```
{
  "message":"Success",
  "data": []
}
```

===

# Dokumentasi Self Hosting Reverse Proxy dengan Docker + Cloudflare Tunnel

## Overview

Dokumentasi ini menjelaskan proses self-hosting backend/frontend menggunakan:

- Docker
- Cloudflare Tunnel (`cloudflared`)
- Reverse Proxy berbasis hostname
- Domain custom
- Docker container services

Arsitektur ini memungkinkan project berjalan online dari device pribadi tanpa VPS.

---

# Stack yang Digunakan

| Teknologi | Fungsi |
| --- | --- |
| Docker | Menjalankan aplikasi dalam container |
| Rust Axum | Backend API |
| Next.js | Frontend / Portfolio / Blog |
| PostgreSQL | Database |
| Cloudflare Tunnel | Reverse proxy & secure tunnel |
| Cloudflare DNS | Routing domain |
| Docker Compose | Manajemen multi-service |
| Custom Domain | Domain utama aplikasi |

---

# Arsitektur Sistem

```
Internet
↓
Cloudflare DNS
↓
Cloudflare Tunnel
↓
config.yml ingress routing
↓
localhost service mapping
↓
Docker Containers
```

---

# Konsep Reverse Proxy

Cloudflare Tunnel bekerja sebagai reverse proxy.

Request dari internet diarahkan berdasarkan hostname/domain menuju service lokal tertentu.

Contoh:

| Domain | Service |
| --- | --- |
| api.veoveneht.eu.org | localhost:3000 |
| blog.veoveneht.eu.org | localhost:3001 |
| veoveneht.eu.org | localhost:3002 |

---

# Struktur Project

```
project/
├── Dockerfile
├── docker-compose.yml
├── src/
├── Cargo.toml
└── .env
```

---

# Docker Backend Rust Axum

## Dockerfile

```
FROM rust:1.95.0as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/backend ./backend

EXPOSE3000

CMD ["./backend"]
```

---

# Penjelasan Dockerfile

| Bagian | Fungsi |
| --- | --- |
| FROM rust | Builder image Rust |
| WORKDIR /app | Folder kerja container |
| COPY . . | Menyalin source code |
| cargo build --release | Compile release binary |
| EXPOSE 3000 | Membuka port container |
| CMD | Menjalankan executable |

---

# Menjalankan Docker

## Build Image

```
docker build-t axum-backend .
```

## Run Container

```
docker run-p3000:3000 axum-backend
```

---

# Binding Axum

Agar container dapat diakses dari luar:

```
0.0.0.0:3000
```

Bukan:

```
127.0.0.1:3000
```

Contoh:

```
let listener= TcpListener::bind("0.0.0.0:3000")
    .await
    .unwrap();
// atau jika asynchronus
let listener= TcpListener::bind("0.0.0.0:3000")
    .await?;
```

---

# Cloudflare Tunnel

## Fungsi

Cloudflare Tunnel membuat koneksi aman dari internet menuju localhost tanpa port forwarding.

---

# Install cloudflared

Dokumentasi resmi:

[Cloudflare Tunnel Docs](https://developers.cloudflare.com/cloudflare-one/connections/connect-networks/?utm_source=chatgpt.com)

---

# Login Cloudflare

```
cloudflared tunnel login
```

Fungsi:

- autentikasi akun Cloudflare
- membuat `cert.pem`

Lokasi file:

```
C:\Users\<username>\.cloudflared\
```

---

# Membuat Tunnel

```
cloudflared tunnel create axum
```

Output:

```
Tunnel credentials written to:
C:\Users\<username>\.cloudflared\<UUID>.json
```

---

# UUID Tunnel

Tunnel memiliki UUID unik.

Contoh:

```
1513372b-7527-4010-9e6e-b10df0a59088
```

File credentials:

```
1513372b-7527-4010-9e6e-b10df0a59088.json
```

---

# Routing DNS ke Tunnel

## Menambahkan Subdomain

```
cloudflared tunnel route dns axum api.veoveneht.eu.org
```

Cloudflare otomatis membuat:

```
CNAME
→ <UUID>.cfargotunnel.com
```

---

# `<UUID>.cfargotunnel.com`

Contoh:

```
1513372b-7527-4010-9e6e-b10df0a59088.cfargotunnel.com
```

Fungsi:

- endpoint internal Cloudflare Tunnel
- penghubung domain dengan daemon `cloudflared`

User tidak mengakses ini secara langsung.

---

# File config.yml

Lokasi:

```
C:\Users\<username>\.cloudflared\config.yml
```

---

# Contoh config.yml

```
tunnel: 1513372b-7527-4010-9e6e-b10df0a59088

credentials-file: C:\Users\lisan\.cloudflared\1513372b-7527-4010-9e6e-b10df0a59088.json

ingress:
  - hostname: api.veoveneht.eu.org
    service: http://localhost:3000

  - hostname: blog.veoveneht.eu.org
    service: http://localhost:3001

  - hostname: veoveneht.eu.org
    service: http://localhost:3002

  - service: http_status:404
```

---

# Penjelasan ingress

## ingress

Berfungsi sebagai reverse proxy routing.

Cloudflare akan membaca hostname lalu meneruskan request ke service lokal tertentu.

---

# Contoh Routing

## API

```
- hostname: api.veoveneht.eu.org
  service: http://localhost:3000
```

→ menuju backend Axum.

---

## Blog

```
- hostname: blog.veoveneht.eu.org
  service: http://localhost:3001
```

→ menuju aplikasi blog.

---

## Portfolio

```
- hostname: veoveneht.eu.org
  service: http://localhost:3002
```

→ menuju website portfolio.

---

# Fallback 404

```
- service: http_status:404
```

HARUS selalu berada di paling bawah.

Fungsi:

- menangani hostname yang tidak cocok.

---

# Menjalankan Tunnel

```
cloudflared tunnel run axum
```

Jika berhasil:

```
Registered tunnel connection
```

---

# Auto Start Tunnel

## Install Service

```
cloudflaredservice install
```

Fungsi:

- tunnel otomatis berjalan saat Windows boot.

---

# Docker Compose (Opsional)

## docker-compose.yml

```
services:
  backend:
    build: .
    ports:
      -"3000:3000"
    restart: unless-stopped

  postgres:
    image: postgres:16
    ports:
      -"5432:5432"
    restart: unless-stopped
```

---

# Keuntungan Arsitektur Ini

## Self Hosted

Server berjalan di device pribadi.

## Tanpa VPS

Tidak perlu:

- Railway
- Render
- VPS cloud

## Full Control

Kontrol penuh terhadap:

- container
- database
- deployment
- domain

## Reverse Proxy Modern

Menggunakan hostname routing modern.

## Bisa Multi Service

Mendukung:

- backend
- frontend
- blog
- monitoring
- database UI
- game server

---

# Kekurangan

| Masalah | Penjelasan |
| --- | --- |
| Listrik mati | Server offline |
| Internet putus | Tidak bisa diakses |
| Laptop sleep | Service berhenti |
| Resource terbatas | RAM/CPU lokal |

---

# Best Practice

## Nonaktifkan Sleep

Agar server tetap online.

## Gunakan restart policy Docker

```
restart: unless-stopped
```

## Backup Database

Karena seluruh data berada di device pribadi.

## Gunakan Docker Compose

Untuk manajemen multi-service lebih mudah.

---

# Kesimpulan

Arsitektur:

```
Docker
+ Cloudflare Tunnel
+ Reverse Proxy
+ Domain Routing
+ Self Hosted
```

sudah merupakan pendekatan deployment modern yang sangat baik untuk:

- portfolio
- personal backend
- homelab
- learning infrastructure
- self-hosting ecosystem

dan dapat dikembangkan menjadi arsitektur production skala kecil hingga menengah.