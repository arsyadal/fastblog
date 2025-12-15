# Panduan Setup & Menjalankan FastBlog

## 📋 Prasyarat

Sebelum memulai, pastikan Anda telah menginstall:

1. **Rust** (versi 1.70+)
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **PostgreSQL** (versi 14+)
   ```bash
   # macOS (dengan Homebrew)
   brew install postgresql@14
   brew services start postgresql@14
   
   # Ubuntu/Debian
   sudo apt-get install postgresql postgresql-contrib
   sudo systemctl start postgresql
   
   # Windows
   # Download dari https://www.postgresql.org/download/windows/
   ```

3. **Node.js** (versi 18+)
   ```bash
   # Install Node.js via nvm (recommended)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 18
   nvm use 18
   ```

---

## 🗄️ Setup Database

### 1. Buat Database PostgreSQL

```bash
# Login ke PostgreSQL
psql postgres

# Atau dengan user tertentu
psql -U postgres
```

Di dalam PostgreSQL shell:

```sql
-- Buat database
CREATE DATABASE fastblog;

-- Buat user (opsional)
CREATE USER fastblog_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE fastblog TO fastblog_user;

-- Keluar
\q
```

### 2. Verifikasi Database

```bash
# Test koneksi
psql -U postgres -d fastblog -c "SELECT version();"
```

---

## 🔧 Setup Backend (Rust)

### 1. Masuk ke direktori backend

```bash
cd backend
```

### 2. Setup Environment Variables

```bash
# Copy file env.example ke .env
cp env.example .env
```

Edit file `.env` dengan konfigurasi Anda:

```env
# Database Configuration
DATABASE_URL=postgresql://postgres@localhost/fastblog
# Atau dengan password:
# DATABASE_URL=postgresql://fastblog_user:your_password@localhost/fastblog

# JWT Configuration (ubah untuk production!)
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production

# Server Configuration
PORT=3001
ENVIRONMENT=development

# CORS Configuration
CORS_ORIGINS=http://localhost:3000,http://localhost:3001

# File Upload Configuration
MAX_FILE_SIZE=10485760
```

### 3. Install Dependencies & Build

```bash
# Install dependencies (akan compile semua crate)
cargo build

# Atau untuk release build (lebih cepat)
cargo build --release
```

### 4. Jalankan Backend Server

```bash
# Development mode
cargo run

# Atau release mode (lebih cepat)
cargo run --release
```

Backend akan:
- Otomatis menjalankan database migrations
- Start server di `http://localhost:3001`
- Health check tersedia di `http://localhost:3001/health`

**Catatan:** Pastikan PostgreSQL sudah berjalan sebelum menjalankan backend!

---

## 🎨 Setup Frontend (Astro)

### 1. Masuk ke direktori frontend

```bash
cd frontend
```

### 2. Install Dependencies

```bash
npm install
```

### 3. Setup Environment Variables

```bash
# Copy file env.example ke .env
cp env.example .env
```

File `.env` sudah benar jika berisi:

```env
BACKEND_URL=http://localhost:3001
PUBLIC_BACKEND_URL=http://localhost:3001
```

### 4. Jalankan Frontend Development Server

```bash
# Development mode
npm run dev

# Atau
npm start
```

Frontend akan berjalan di `http://localhost:4321` (default Astro port)

---

## 🚀 Menjalankan Program Lengkap

### Terminal 1: Backend Server

```bash
cd backend
cargo run
```

Anda akan melihat output seperti:
```
🚀 FastBlog server starting on port 3001
📖 API Documentation: http://localhost:3001/docs
```

### Terminal 2: Frontend Server

```bash
cd frontend
npm run dev
```

Anda akan melihat output seperti:
```
  ➜  Local:   http://localhost:4321/
  ➜  Network: use --host to expose
```

### Akses Aplikasi

- **Frontend**: http://localhost:4321
- **Backend API**: http://localhost:3001
- **Health Check**: http://localhost:3001/health

---

## ✅ Verifikasi Setup

### 1. Test Backend Health

```bash
curl http://localhost:3001/health
```

Harus mengembalikan:
```json
{
  "status": "ok",
  "service": "fastblog-backend",
  "version": "0.1.0",
  "timestamp": "..."
}
```

### 2. Test Database Connection

Backend akan otomatis menjalankan migrations saat pertama kali dijalankan. Cek log untuk memastikan tidak ada error.

### 3. Test Frontend

Buka browser dan akses http://localhost:4321. Anda harus melihat homepage FastBlog.

---

## 🐛 Troubleshooting

### Error: Database connection failed

**Solusi:**
1. Pastikan PostgreSQL berjalan:
   ```bash
   # macOS
   brew services list
   
   # Linux
   sudo systemctl status postgresql
   ```

2. Cek DATABASE_URL di `.env` file
3. Pastikan database `fastblog` sudah dibuat

### Error: Port already in use

**Solusi:**
1. Ubah PORT di `.env` file (backend)
2. Atau kill process yang menggunakan port:
   ```bash
   # Cari process
   lsof -i :3001
   
   # Kill process
   kill -9 <PID>
   ```

### Error: Migration failed

**Solusi:**
1. Pastikan database sudah dibuat
2. Pastikan user memiliki permission
3. Cek log error untuk detail

### Error: npm install failed

**Solusi:**
1. Update Node.js ke versi terbaru
2. Clear cache:
   ```bash
   npm cache clean --force
   rm -rf node_modules package-lock.json
   npm install
   ```

### Error: Cargo build failed

**Solusi:**
1. Update Rust:
   ```bash
   rustup update
   ```

2. Clean build:
   ```bash
   cargo clean
   cargo build
   ```

---

## 📝 Catatan Penting

1. **JWT_SECRET**: Pastikan mengubah JWT_SECRET untuk production!
2. **Database**: Migrations akan otomatis berjalan saat backend pertama kali dijalankan
3. **Ports**: 
   - Backend: 3001 (default)
   - Frontend: 4321 (default Astro)
4. **File Uploads**: Folder `backend/uploads/` akan dibuat otomatis

---

## 🎯 Quick Start Commands

```bash
# Setup database
createdb fastblog

# Backend (Terminal 1)
cd backend
cp env.example .env
# Edit .env dengan DATABASE_URL yang benar
cargo run

# Frontend (Terminal 2)
cd frontend
cp env.example .env
npm install
npm run dev
```

---

## 📚 Next Steps

Setelah aplikasi berjalan:

1. **Register user baru** di http://localhost:4321/register
2. **Login** di http://localhost:4321/login
3. **Tulis artikel pertama** di http://localhost:4321/write
4. **Lihat statistics** di http://localhost:4321/stats

Selamat coding! 🚀

