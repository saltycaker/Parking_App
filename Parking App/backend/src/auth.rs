use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Utc, Duration};
use uuid::Uuid;

use crate::models::{User, RegisterRequest, LoginRequest, AuthResponse, UserResponse};
use crate::error::{AppError, AppResult};
use crate::config::Config;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
}

pub struct AuthService {
    config: Config,
}

impl AuthService {
    pub fn new(config: Config) -> Self {
        AuthService { config }
    }

    pub fn hash_password(&self, password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> AppResult<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(format!("Invalid password hash: {}", e)))?;
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    pub fn generate_token(&self, user_id: &str) -> AppResult<String> {
        let now = Utc::now();
        let exp = now + Duration::hours(self.config.jwt_expiration_hours);
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.jwt_secret.as_bytes()),
        ).map_err(|e| AppError::Jwt(e))
    }

    pub fn verify_token(&self, token: &str) -> AppResult<String> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.jwt_secret.as_bytes()),
            &Validation::default(),
        ).map_err(|e| AppError::Jwt(e))?;

        Ok(token_data.claims.sub)
    }

    pub async fn register(
        &self,
        db: &crate::db::Database,
        request: RegisterRequest,
    ) -> AppResult<AuthResponse> {
        // Check if user already exists
        let existing = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM users WHERE email = $1"
        )
        .bind(&request.email)
        .fetch_one(db.pool())
        .await?;

        if existing > 0 {
            return Err(AppError::Validation("Email already registered".to_string()));
        }

        // Hash password
        let password_hash = self.hash_password(&request.password)?;

        // Create user
        let user_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO users (id, email, password_hash, name, phone, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(user_id)
        .bind(&request.email)
        .bind(&password_hash)
        .bind(&request.name)
        .bind(&request.phone)
        .bind(now)
        .bind(now)
        .execute(db.pool())
        .await?;

        // Generate token
        let token = self.generate_token(&user_id.to_string())?;

        // Fetch user
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_one(db.pool())
        .await?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                email: user.email,
                name: user.name,
                phone: user.phone,
                created_at: user.created_at,
            },
        })
    }

    pub async fn login(
        &self,
        db: &crate::db::Database,
        request: LoginRequest,
    ) -> AppResult<AuthResponse> {
        // Find user
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(&request.email)
        .fetch_optional(db.pool())
        .await?
        .ok_or_else(|| AppError::Auth("Invalid credentials".to_string()))?;

        // Verify password
        if !self.verify_password(&request.password, &user.password_hash)? {
            return Err(AppError::Auth("Invalid credentials".to_string()));
        }

        // Generate token
        let token = self.generate_token(&user.id.to_string())?;

        Ok(AuthResponse {
            token,
            user: UserResponse {
                id: user.id,
                email: user.email,
                name: user.name,
                phone: user.phone,
                created_at: user.created_at,
            },
        })
    }
}
