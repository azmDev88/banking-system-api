# 📝 AZMA MARZUKI S.KOM
# 📝 COMPUTER SCIENCE UNIVERSITY PEMBANGUNAN JAYA
# 🏦 Rust Banking Core API

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Postgres](https://img.shields.io/badge/postgres-%23316192.svg?style=for-the-badge&logo=postgresql&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

Backend sistem perbankan (Core Banking) yang dibangun menggunakan **Rust**. Proyek ini dirancang untuk mensimulasikan pemrosesan transaksi finansial yang **High-Concurrency**, **Safe**, dan **Reliable**.

Fokus utama proyek ini adalah menerapkan standar keamanan data finansial menggunakan **Hexagonal Architecture** dan **ACID Database Transactions**.

## 🚀 Key Features

Sistem ini menangani masalah umum pada sistem terdistribusi (Distributed Systems):

-   **🏗 Hexagonal Architecture (Ports & Adapters):** Pemisahan total antara Domain Logic (Bisnis) dan Infrastructure (Database/API). Kode inti bersih dari framework.
-   **🔒 Pessimistic Locking (`SELECT FOR UPDATE`):** Mencegah *Race Condition* saat ribuan request transfer terjadi bersamaan pada akun yang sama.
-   **🛡️ Idempotency Mechanism:** Mencegah *Double Spending* (saldo terpotong 2x) jika terjadi *network timeout* atau user melakukan *retry* request.
-   **💰 Precision Money Handling:** Menggunakan `rust_decimal` untuk menghindari kesalahan pembulatan *floating point*.
-   **🧪 Integration Testing:** Full E2E testing dengan instance database nyata (bukan sekadar mock) untuk menjamin validitas transaksi.
-   **⚡ High Performance:** Dibangun di atas `Axum` dan `Tokio` (Asynchronous Runtime).

## 🛠 Tech Stack

-   **Language:** Rust 🦀
-   **Web Framework:** Axum
-   **Database:** PostgreSQL
-   **ORM/Query Builder:** SQLx (Compile-time checked queries)
-   **Architecture:** Domain Driven Design (DDD) & Hexagonal
-   **Containerization:** Docker & Docker Compose

## 📂 Project Structure

```text
src/
├── domain/     # Core Business Logic (Pure Rust)
├── ports/      # Interfaces (Traits) for Repository
├── services/   # Application Service / Use Cases
├── adapters/   # Implementation (API Handler, Postgres Repo)
├── shared/     # Config & Error Handling
└── tests/      # Integration & Unit Tests


# Rust Banking Core API

Backend sistem perbankan menggunakan Rust, Axum, SQLx, dan PostgreSQL.

## Fitur
- Hexagonal Architecture
- Atomic Transactions (ACID)
- Pessimistic Locking (`SELECT FOR UPDATE`)
- Idempotency (Anti Double-Spending)

## Cara Menjalankan

1. Pastikan Docker dan Rust terinstall.
2. Setup Database:
   ```bash
   docker-compose up -d
   sqlx migrate run