# Parking Discovery Platform

A production-ready parking discovery platform consisting of a Rust-powered web application and a Flutter mobile application. The platform helps users quickly find nearby parking, estimate which lots are most likely to have available spaces, and navigate directly to the selected parking location with a clean, premium user experience inspired by Uber.

## 🚀 Features

### Core Functionality
- **Automatic Location Detection**: GPS-based location with continuous updates
- **Nearby Parking Search**: Configurable radius search (250m - 5km)
- **Estimated Availability Engine**: Smart scoring based on historical data, community reports, and heuristics
- **Navigation Integration**: Turn-by-turn navigation with purple route highlighting
- **Community Reports**: Real-time user-submitted parking status updates
- **Smart Recommendations**: AI-powered parking suggestions
- **Favorites & History**: Save preferred locations and view search history
- **User Authentication**: JWT-based secure authentication

### Technical Features
- **Real-time Caching**: Redis-powered caching for fast responses
- **Rate Limiting**: API protection against abuse
- **Responsive Design**: Works seamlessly on desktop, tablet, and mobile
- **Clean Architecture**: Modular, scalable codebase
- **Type Safety**: Full type safety with Rust and Dart
- **OpenAPI Documentation**: Auto-generated API documentation

## 🛠 Tech Stack

### Backend
- **Language**: Rust
- **Framework**: Axum
- **Runtime**: Tokio
- **Database**: PostgreSQL with SQLx
- **Cache**: Redis
- **Authentication**: JWT with Argon2 password hashing
- **API Documentation**: OpenAPI/Swagger with Utoipa
- **Rate Limiting**: Tower Governor

### Web Frontend
- **Framework**: Leptos (Rust-based SSR framework)
- **Styling**: TailwindCSS
- **Maps**: Google Maps JavaScript SDK
- **State Management**: Leptos signals

### Mobile
- **Framework**: Flutter
- **State Management**: Riverpod
- **HTTP Client**: Dio
- **Navigation**: GoRouter
- **Maps**: Google Maps Flutter
- **Location**: Geolocator
- **Storage**: Flutter Secure Storage

### Data Source
- **Parking Data**: Apify Google Places Crawler

## 📁 Project Structure

```
parking-app/
├── backend/                 # Rust backend API
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── config.rs       # Configuration management
│   │   ├── db.rs           # Database connection
│   │   ├── cache.rs        # Redis cache
│   │   ├── auth.rs         # Authentication logic
│   │   ├── models.rs       # Data models
│   │   ├── handlers.rs     # HTTP handlers
│   │   ├── services.rs     # Business logic
│   │   ├── middleware.rs   # Middleware (rate limiting, auth)
│   │   └── error.rs        # Error handling
│   ├── migrations/         # Database migrations
│   ├── Cargo.toml          # Rust dependencies
│   ├── render.yaml         # Render deployment config
│   └── .env.example        # Environment variables template
├── frontend/               # Leptos web frontend
│   ├── src/
│   │   ├── lib.rs          # Application entry
│   │   ├── components/     # Reusable components
│   │   ├── pages/          # Page components
│   │   ├── services/       # API services
│   │   └── types/          # TypeScript-like types
│   ├── Cargo.toml          # Rust dependencies
│   └── vercel.json         # Vercel deployment config
├── mobile/                 # Flutter mobile app
│   ├── lib/
│   │   ├── main.dart       # Application entry
│   │   ├── models/         # Data models
│   │   ├── services/       # API services
│   │   ├── screens/        # UI screens
│   │   ├── widgets/        # Reusable widgets
│   │   ├── providers/      # Riverpod providers
│   │   └── utils/          # Utilities
│   ├── assets/             # Images, fonts, icons
│   └── pubspec.yaml        # Flutter dependencies
├── .github/
│   └── workflows/
│       └── ci.yml          # GitHub Actions CI/CD
├── docs/                   # Documentation
├── SETUP.md                # Setup instructions
└── README.md               # This file
```

## 🚦 Getting Started

### Prerequisites

1. **Rust** (1.70 or later)
   ```bash
   # Install from https://rustup.rs/
   rustc --version
   ```

2. **PostgreSQL** (14 or later)
   ```bash
   # Install from https://www.postgresql.org/download/
   # Default port: 5432
   ```

3. **Redis** (6 or later)
   ```bash
   # For Windows: Use Memurai
   # Default port: 6379
   ```

4. **Flutter** (3.16 or later)
   ```bash
   # Install from https://flutter.dev/docs/get-started/install
   flutter doctor
   ```

5. **Node.js** (18 or later)
   ```bash
   # Install from https://nodejs.org/
   node --version
   ```

### API Keys Required

You'll need to obtain API keys from:
- **Apify**: Google Places Crawler (for parking data)

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd parking-app
   ```

2. **Set up the backend**
   ```bash
   cd backend
   cargo build
   cp .env.example .env
   # Edit .env with your API keys and database credentials
   ```

3. **Set up the database**
   ```bash
   # Create database
   psql -U postgres -c "CREATE DATABASE parking_db;"
   
   # Run migrations
   cargo run -- migrate
   ```

4. **Set up the frontend**
   ```bash
   cd frontend
   cargo build
   ```

5. **Set up the mobile app**
   ```bash
   cd mobile
   flutter pub get
   ```

### Running the Applications

#### Backend
```bash
cd backend
cargo run
```
The API will be available at `http://localhost:8080`
API documentation: `http://localhost:8080/swagger-ui`

#### Frontend
```bash
cd frontend
cargo leptos watch
```
The web app will be available at `http://localhost:3000`

#### Mobile
```bash
cd mobile
flutter run
```

## 🧪 Testing

### Backend Tests
```bash
cd backend
cargo test
```

### Frontend Tests
```bash
cd frontend
cargo test
```

### Mobile Tests
```bash
cd mobile
flutter test
```

## 🚢 Deployment

### Backend (Render)

The backend is configured for automatic deployment to Render via `render.yaml`.

1. Connect your GitHub repository to Render
2. Render will automatically deploy on push to main branch
3. Set environment variables in Render dashboard

### Frontend (Vercel)

The frontend is configured for Vercel deployment via `vercel.json`.

1. Connect your GitHub repository to Vercel
2. Vercel will automatically deploy on push to main branch
3. Set `API_BASE_URL` environment variable

### Mobile

#### Android
```bash
cd mobile
flutter build apk --release
```

#### iOS
```bash
cd mobile
flutter build ios --release
```

## 🔧 Configuration

### Environment Variables

See `.env.example` in the backend directory for all required environment variables:

```env
DATABASE_URL=postgresql://...
REDIS_URL=redis://...
JWT_SECRET=your_secret_key
APIFY_API_KEY=your_key
# ... and more
```

## 📊 Database Schema

The application uses PostgreSQL with the following main tables:

- **users**: User accounts and authentication
- **parking_locations**: Parking lot information
- **parking_reports**: Community-submitted status reports
- **favorites**: User's saved parking locations
- **search_history**: User's search history
- **availability_history**: Historical availability data
- **cached_searches**: Cached search results
- **sessions**: JWT session management

See `backend/migrations/001_initial.sql` for the complete schema.

## 🔐 Security

- **Password Hashing**: Argon2
- **JWT Authentication**: Secure token-based auth
- **Rate Limiting**: API rate limiting
- **SQL Injection Protection**: Parameterized queries
- **CORS**: Configured for production domains
- **Environment Variables**: Sensitive data in env vars

## 🎨 Design System

The application follows an Uber-inspired design philosophy:

### Colors
- **Primary Background**: #FFFFFF (Pure White)
- **Secondary Surface**: #F6F6F6 (Light Gray)
- **Primary Text**: #111111 (Black)
- **Secondary Text**: #666666 (Gray)
- **Accent**: #7C3AED (Purple)
- **Success**: #10B981 (Green)
- **Warning**: #F59E0B (Orange)
- **Error**: #EF4444 (Red)

### Typography
- Clean, minimal fonts
- Hierarchical sizing
- High readability

### Components
- Rounded corners (16px-24px)
- Subtle shadows
- Smooth animations
- Bottom sheets for mobile

## 📝 API Documentation

API documentation is automatically generated using OpenAPI/Swagger and available at:
- Development: `http://localhost:8080/swagger-ui`
- Production: `https://your-api.onrender.com/swagger-ui`

### Main Endpoints

- `POST /search` - Search for nearby parking
- `GET /parking/:id` - Get parking details
- `POST /reports` - Submit parking status report
- `GET /favorites` - Get user's favorites
- `POST /favorites` - Add favorite
- `DELETE /favorites/:id` - Remove favorite
- `GET /recommendations` - Get smart recommendations
- `POST /auth/register` - Register user
- `POST /auth/login` - Login user
- `GET /profile` - Get user profile
- `PATCH /profile` - Update user profile

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License.

## 🙏 Acknowledgments

- Google Maps Platform for mapping services
- Apify for Google Places Crawler
- Rust community for excellent tooling
- Flutter team for cross-platform framework

## 📞 Support

For support, please open an issue in the GitHub repository or contact the development team.

## 🗺 Roadmap

- [ ] Add real-time parking data integration
- [ ] Implement payment processing
- [ ] Add parking reservation feature
- [ ] Support for additional cities
- [ ] AR navigation integration
- [ ] Machine learning for better predictions
- [ ] Multi-language support
- [ ] Apple Watch companion app

---

**Built with ❤️ using Rust, Leptos, and Flutter**
