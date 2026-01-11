# PHASE 0 IMPLEMENTATION PLAN: Server Infrastructure & Database

## Executive Summary

**Purpose**: Establish server-custodial NFT infrastructure with instant, zero-cost transfers before implementing game logic.

**Timeline**: 10-14 days for complete Phase 0 implementation

**Prerequisites**:
- MySQL 8.0+ installed
- Rust 1.75+ with cargo
- Basic understanding of async Rust (tokio)
- Environment for development (Windows/Linux)

**Cost**:
- Development: One-time setup (10-14 days)
- Monthly Hosting: $50-500 (scales with traffic)
- Blockchain: $0 until Phase 9

---

## Part 1: Prerequisites & Environment Setup

### 1.1 Development Environment (Day 1, Morning - 2 hours)

**Install Required Software:**

```powershell
# Windows PowerShell

# 1. MySQL 8.0+
# Download from: https://dev.mysql.com/downloads/windows/installer/
# Or use chocolatey:
choco install mysql

# 2. Verify Rust installation
rustc --version  # Should be 1.75+
cargo --version

# 3. Install sqlx-cli for migrations
cargo install sqlx-cli --features mysql

# 4. Install development tools
cargo install cargo-watch  # For auto-recompilation during development
```

**Estimated Time**: 2 hours (including downloads)

**Verification:**
```bash
mysql --version  # Should show MySQL 8.0.x or higher
cargo --version  # Should show 1.75+
sqlx --version   # Should show 0.7.x
```

---

### 1.2 Project Structure Creation (Day 1, Afternoon - 1 hour)

**Create Directory Structure:**

```bash
cd H:\RustGames\kaiju_sim

# Create server project
mkdir kaiju_server
cd kaiju_server

# Initialize Rust workspace
cargo init --name kaiju_server

# Create subdirectories
mkdir -p src/crypto
mkdir -p src/api
mkdir -p src/bin
mkdir migrations
mkdir tests
```

**Project Layout:**
```
kaiju_server/
├── Cargo.toml                 # Dependencies
├── .env.example              # Environment template
├── src/
│   ├── main.rs               # Server entry point
│   ├── lib.rs                # Library exports
│   ├── types.rs              # Core data types
│   ├── error.rs              # Error types
│   ├── transfer_service.rs   # Transfer logic
│   ├── crypto/
│   │   ├── mod.rs
│   │   ├── keygen.rs         # Key generation
│   │   ├── signer.rs         # Signature creation
│   │   ├── verifier.rs       # Signature verification
│   │   └── state_hash.rs     # State hashing
│   ├── api/
│   │   ├── mod.rs
│   │   ├── transfer.rs       # Transfer endpoints
│   │   └── verification.rs   # Verification endpoints
│   └── bin/
│       └── verify_proof.rs   # Standalone verification CLI
├── migrations/               # SQL migration files
│   ├── 01_create_users.sql
│   ├── 02_create_kaiju.sql
│   ├── ... (10 total)
├── tests/
│   ├── integration_tests.rs
│   └── fixtures/
└── README.md
```

**Estimated Time**: 1 hour

---

## Part 2: Database Setup

### 2.1 MySQL Database Creation (Day 1, Afternoon - 1 hour)

**Create Database and User:**

```sql
-- Connect to MySQL as root
-- Windows: Open MySQL Command Line Client or use mysql -u root -p

-- 1. Create database
CREATE DATABASE kaiju_game CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- 2. Create dedicated user
CREATE USER 'kaiju_user'@'localhost' IDENTIFIED BY 'STRONG_PASSWORD_HERE';

-- 3. Grant privileges
GRANT ALL PRIVILEGES ON kaiju_game.* TO 'kaiju_user'@'localhost';

-- 4. Flush privileges
FLUSH PRIVILEGES;

-- 5. Verify
USE kaiju_game;
SHOW GRANTS FOR 'kaiju_user'@'localhost';
```

**Security Best Practices:**
- Use a password manager to generate 32+ character password
- Never commit passwords to git
- Store in `.env` file (add to `.gitignore`)

**Environment Configuration:**

Create `H:\RustGames\kaiju_sim\kaiju_server\.env`:
```env
DATABASE_URL=mysql://kaiju_user:STRONG_PASSWORD_HERE@localhost:3306/kaiju_game
SERVER_SECRET_KEY=  # Will generate in step 3.1
SERVER_PORT=3000
RUST_LOG=info,sqlx=warn
```

Create `.env.example` (for documentation):
```env
DATABASE_URL=mysql://kaiju_user:PASSWORD@localhost:3306/kaiju_game
SERVER_SECRET_KEY=<hex_encoded_secret_key>
SERVER_PORT=3000
RUST_LOG=info
```

**Estimated Time**: 1 hour

**Common Pitfalls:**
- ❌ Using weak passwords
- ❌ Forgetting to add `.env` to `.gitignore`
- ❌ Not using utf8mb4 character set (required for full Unicode support)

---

### 2.2 SQL Migration Files (Day 2, Full Day - 6-8 hours)

**Migration Order (CRITICAL - must be executed in order):**

Refer to [DATABASE_SCHEMA.md](DATABASE_SCHEMA.md) for complete SQL for all migrations:

1. `migrations/01_create_users.sql` - User accounts table
2. `migrations/02_create_kaiju.sql` - Main kaiju entity table
3. `migrations/03_create_ownership_history.sql` - Complete audit trail
4. `migrations/04_create_breeding_rights.sql` - Breeding rights system
5. `migrations/05_create_breeding_history.sql` - Breeding records
6. `migrations/06_create_tournaments.sql` - Tournament system tables
7. `migrations/07_create_security_tables.sql` - Security infrastructure
8. `migrations/08_create_triggers.sql` - Automated triggers
9. `migrations/09_create_views.sql` - Query optimization views
10. `migrations/10_create_stored_procedures.sql` - Business logic procedures
11. `migrations/11_initial_data.sql` - Bootstrap data

**Run Migrations:**

```bash
cd H:\RustGames\kaiju_sim\kaiju_server

# Create migration files (one at a time)
sqlx migrate add create_users
sqlx migrate add create_kaiju
# ... (create all 11 migrations)

# Run all migrations
sqlx migrate run

# Verify
sqlx migrate info
```

**Estimated Time**: 6-8 hours (includes testing each migration)

**Common Pitfalls:**
- ❌ Running migrations out of order
- ❌ Foreign key errors from missing parent tables
- ❌ Syntax errors in CHECK constraints
- ❌ Forgetting to add indexes

**Recovery Steps (if migration fails):**
```bash
# Rollback last migration
sqlx migrate revert

# Check database state
mysql -u kaiju_user -p kaiju_game -e "SHOW TABLES;"

# Re-run after fixing
sqlx migrate run
```

---

## Part 3: Cryptographic System Implementation

### 3.1 Key Generation (Day 3, Morning - 3 hours)

Refer to [SIGNATURE_SYSTEM.md](SIGNATURE_SYSTEM.md) for complete implementation.

**Generate Production Keys:**

Create `src/bin/generate_keys.rs` and run:
```bash
cargo run --bin generate_keys > keys_backup.txt

# Update .env with generated key
# CRITICAL: Store keys_backup.txt in secure vault (1Password, etc.)
```

**Estimated Time**: 3 hours

---

### 3.2 Signature & Verification System (Day 3, Afternoon - 4 hours)

Implement all crypto modules as detailed in [SIGNATURE_SYSTEM.md](SIGNATURE_SYSTEM.md):
- `src/crypto/signer.rs`
- `src/crypto/verifier.rs`
- `src/crypto/state_hash.rs`
- `src/crypto/mod.rs`

**Estimated Time**: 4 hours

---

## Part 4: Transfer Service Implementation

### 4.1 Core Types (Day 4, Morning - 2 hours)

Configure `Cargo.toml` with all dependencies (see [SERVER_TRANSFER_SYSTEM.md](SERVER_TRANSFER_SYSTEM.md)).

Implement:
- `src/types.rs`
- `src/error.rs`

**Estimated Time**: 2 hours

---

### 4.2 Transfer Service Logic (Day 4, Afternoon - 5 hours)

Implement `src/transfer_service.rs` as detailed in [SERVER_TRANSFER_SYSTEM.md](SERVER_TRANSFER_SYSTEM.md).

**Critical Implementation Notes:**

1. **Transaction Safety**: Always use `BEGIN/COMMIT/ROLLBACK`
2. **Nonce Management**: Atomic increment in database
3. **Error Handling**: Rollback on any error

**Integration Tests**: Create `tests/transfer_tests.rs`

**Estimated Time**: 5 hours

---

## Part 5: API Layer

### 5.1 REST API Endpoints (Day 5, Full Day - 6 hours)

Implement:
- `src/api/transfer.rs`
- `src/api/verification.rs`
- `src/api/mod.rs`
- `src/main.rs`

**Test API Endpoints:**
```bash
# Start server
cargo run

# Test transfer endpoint
curl -X POST http://localhost:3000/api/transfer \
  -H "Content-Type: application/json" \
  -d '{...}'
```

**Estimated Time**: 6 hours

---

## Part 6: Standalone Verification Tool

### 6.1 Command-Line Verifier (Day 6, Morning - 2 hours)

Create `src/bin/verify_proof.rs` as shown in [SIGNATURE_SYSTEM.md](SIGNATURE_SYSTEM.md).

**Estimated Time**: 2 hours

---

## Part 7: Testing & Quality Assurance

### 7.1 Comprehensive Test Suite (Day 6, Afternoon - 4 hours)

**Test Categories:**
1. Unit Tests (in each module)
2. Integration Tests (in `tests/`)
3. Database Tests (using sqlx::test)
4. API Tests (using reqwest)

**Run Full Test Suite:**
```bash
cargo test --all
cargo tarpaulin --out Html --output-dir coverage/
```

**Test Coverage Goals:**
- Core logic: 90%+
- API endpoints: 80%+
- Error paths: 100%
- Database operations: 85%+

**Estimated Time**: 4 hours

---

## Part 8: Performance Optimization

### 8.1 Connection Pooling & Indexing (Day 7, Morning - 3 hours)

Optimize database pool and add performance indexes.

**Estimated Time**: 3 hours

---

### 8.2 Monitoring & Observability (Day 7, Afternoon - 3 hours)

Add metrics, logging, and health checks.

**Estimated Time**: 3 hours

---

## Part 9: Security Hardening

### 9.1 Security Checklist (Day 8, Full Day - 6 hours)

1. ✅ Secret keys in environment variables
2. ✅ Backup keys in secure vault
3. ✅ Authentication & Authorization
4. ✅ Rate Limiting
5. ✅ Input Validation
6. ✅ SQL Injection Prevention
7. ✅ CORS Configuration
8. ✅ Audit Logging

**Estimated Time**: 6 hours

---

## Part 10: Deployment & Documentation

### 10.1 Production Deployment Checklist (Day 9, Full Day - 6 hours)

1. Environment Configuration
2. Docker Containerization (Optional)
3. Backup Strategy
4. Monitoring Setup
5. Documentation

**Estimated Time**: 6 hours

---

## Part 11: Final Validation & Launch

### 11.1 Pre-Launch Checklist (Day 10, Full Day - 6 hours)

**Functional Testing:**
- [ ] Create test user via API
- [ ] Execute transfer between users
- [ ] Verify ownership proof generation
- [ ] Test offline verification tool
- [ ] Verify all audit logs

**Performance Testing:**
- [ ] Load test with artillery
- [ ] Stress test transfers

**Security Testing:**
- [ ] Run cargo audit
- [ ] Test rate limiting
- [ ] Verify authentication

**Estimated Time**: 6 hours

---

## Timeline Summary

| Day | Task | Hours |
|-----|------|-------|
| Day 1 | Environment & DB setup | 4 |
| Day 2 | SQL migrations | 8 |
| Day 3 | Crypto implementation | 7 |
| Day 4 | Transfer service | 7 |
| Day 5 | API layer | 6 |
| Day 6 | Verification tool & tests | 6 |
| Day 7 | Performance & monitoring | 6 |
| Day 8 | Security hardening | 6 |
| Day 9 | Deployment prep | 6 |
| Day 10 | Final validation | 6 |
| **Total** | **62 hours** | **10 days** |

---

## Success Criteria

Phase 0 is complete when:

✅ Mysql database running with all tables
✅ Server starts without errors
✅ Can execute transfer via API
✅ Ownership proofs verify correctly
✅ All unit tests pass (90%+ coverage)
✅ Performance: <50ms transfer latency
✅ Security: No vulnerabilities in cargo audit
✅ Documentation complete

---

## Reference Documents

- [DATABASE_SCHEMA.md](DATABASE_SCHEMA.md) - Complete MySQL schema
- [SERVER_TRANSFER_SYSTEM.md](SERVER_TRANSFER_SYSTEM.md) - Rust transfer service implementation
- [SIGNATURE_SYSTEM.md](SIGNATURE_SYSTEM.md) - Cryptographic proof system
- [SERVER_CUSTODIAL_NFT_DESIGN.md](SERVER_CUSTODIAL_NFT_DESIGN.md) - Architecture overview
