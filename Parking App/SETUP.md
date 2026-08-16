# Parking Discovery Platform - Setup Guide

## Prerequisites

### Required Tools

1. **Rust** (1.70 or later)
   - Install from: https://rustup.rs/
   - After installation, run: `rustc --version` to verify

2. **PostgreSQL** (14 or later)
   - Install from: https://www.postgresql.org/download/
   - For Windows: Use the installer from EnterpriseDB
   - Default port: 5432

3. **Redis** (6 or later)
   - Install from: https://redis.io/download
   - For Windows: Use Memurai or Redis for Windows
   - Default port: 6379

4. **Flutter** (3.16 or later)
   - Install from: https://flutter.dev/docs/get-started/install
   - Run: `flutter doctor` to verify setup
   - For mobile development, you'll also need:
     - Android Studio (for Android)
     - Xcode (for iOS, macOS only)

5. **Node.js** (18 or later) - for frontend tooling
   - Install from: https://nodejs.org/

6. **Git** - for version control
   - Install from: https://git-scm.com/

### API Keys Required

You'll need to obtain the following API keys:

1. **Google Maps JavaScript API** - for web frontend
2. **Google Maps Flutter SDK** - for mobile app
3. **Google Directions API** - for navigation
4. **Google Geocoding API** - for address search
5. **Apify API Key** - for Google Places Crawler

Get Google APIs from: https://console.cloud.google.com/
Get Apify API from: https://apify.com/

## Installation Steps

### 1. Install Rust

```powershell
# Visit https://rustup.rs/ and download the installer
# Or run this in PowerShell:
Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe
```

### 2. Install PostgreSQL

Download and install from: https://www.postgresql.org/download/windows/

During installation:
- Set a password for the postgres user (remember it!)
- Keep default port 5432
- Install pgAdmin (optional but helpful)

### 3. Install Redis

For Windows, we recommend Memurai:
- Download from: https://www.memurai.com/get-memurai
- Install with default settings
- It will run as a Windows service

### 4. Install Flutter

```powershell
# Download Flutter SDK from https://flutter.dev/docs/get-started/install/windows
# Extract to C:\flutter
# Add to PATH: C:\flutter\bin

flutter doctor
```

### 5. Install Node.js

Download and install from: https://nodejs.org/

## Project Setup

Once all prerequisites are installed:

### 1. Initialize Backend

```powershell
cd backend
cargo init --name parking-api
```

### 2. Initialize Frontend

```powershell
cd frontend
cargo leptos new --project-name parking-web
```

### 3. Initialize Mobile App

```powershell
cd mobile
flutter create parking_app
```

### 4. Create Environment File

Copy `.env.example` to `.env` and fill in your API keys:

```env
# Database
DATABASE_URL=postgresql://postgres:your_password@localhost:5432/parking_db
REDIS_URL=redis://localhost:6379

# JWT
JWT_SECRET=your_super_secret_jwt_key
JWT_EXPIRATION_HOURS=24

# Google APIs
GOOGLE_MAPS_API_KEY=your_google_maps_api_key
GOOGLE_DIRECTIONS_API_KEY=your_google_directions_api_key
GEOCODING_API_KEY=your_geocoding_api_key

# Apify
APIFY_API_KEY=your_apify_api_key
APIFY_GOOGLE_PLACES_CRAWLER_ID=compass/crawler-google-places

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
```

### 5. Setup Database

```powershell
cd backend
# Create database
psql -U postgres -c "CREATE DATABASE parking_db;"

# Run migrations (once implemented)
cargo run -- migrate
```

### 6. Start Redis

```powershell
# Redis/Memurai should be running as a service
# Verify with:
redis-cli ping
# Should return: PONG
```

## Development

### Backend Development

```powershell
cd backend
cargo run
```

### Frontend Development

```powershell
cd frontend
cargo leptos watch
```

### Mobile Development

```powershell
cd mobile
flutter run
```

## Testing

```powershell
# Backend tests
cd backend
cargo test

# Frontend tests
cd frontend
cargo test

# Mobile tests
cd mobile
flutter test
```

## Troubleshooting

### Rust not found
- Restart your terminal after installing Rust
- Ensure `%USERPROFILE%\.cargo\bin` is in your PATH

### PostgreSQL connection errors
- Verify PostgreSQL service is running
- Check password in DATABASE_URL
- Ensure port 5432 is not blocked by firewall

### Redis connection errors
- Verify Redis/Memurai service is running
- Check port 6379 is available

### Flutter doctor issues
- Run `flutter doctor` to see missing dependencies
- Install Android Studio for Android development
- Install Xcode for iOS development (macOS only)

## Next Steps

After completing setup:
1. Run `cargo build` in backend to verify Rust setup
2. Run `flutter doctor` to verify Flutter setup
3. Obtain API keys from Google and Apify
4. Configure `.env` file
5. Start building!

## Additional Resources

- Rust Book: https://doc.rust-lang.org/book/
- Axum Documentation: https://docs.rs/axum/
- Leptos Documentation: https://leptos.dev/
- Flutter Documentation: https://flutter.dev/docs
- PostgreSQL Documentation: https://www.postgresql.org/docs/
- Redis Documentation: https://redis.io/docs/
