-- Users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(255),
    phone VARCHAR(20),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index on email for fast lookups
CREATE INDEX idx_users_email ON users(email);

-- Parking locations table
CREATE TABLE IF NOT EXISTS parking_locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    address TEXT NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    place_id VARCHAR(255) UNIQUE,
    parking_type VARCHAR(50) NOT NULL CHECK (parking_type IN ('lot', 'garage', 'street', 'private')),
    is_covered BOOLEAN,
    has_ev_charging BOOLEAN,
    is_free BOOLEAN,
    is_wheelchair_accessible BOOLEAN,
    height_restriction_m DOUBLE PRECISION,
    rating DOUBLE PRECISION CHECK (rating >= 0 AND rating <= 5),
    review_count INTEGER DEFAULT 0,
    phone VARCHAR(20),
    website TEXT,
    opening_hours JSONB,
    photos TEXT[],
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Geospatial index for location queries
CREATE INDEX idx_parking_locations_coords ON parking_locations USING GIST (point(longitude, latitude));

-- Index on place_id
CREATE INDEX idx_parking_locations_place_id ON parking_locations(place_id);

-- Parking reports table
CREATE TABLE IF NOT EXISTS parking_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parking_id UUID NOT NULL REFERENCES parking_locations(id) ON DELETE CASCADE,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    reporter_latitude DOUBLE PRECISION NOT NULL,
    reporter_longitude DOUBLE PRECISION NOT NULL,
    status VARCHAR(50) NOT NULL CHECK (status IN ('found_parking', 'almost_full', 'completely_full', 'closed', 'incorrect_info', 'temporary_closure')),
    comment TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Index for querying recent reports
CREATE INDEX idx_parking_reports_parking_id ON parking_reports(parking_id);
CREATE INDEX idx_parking_reports_created_at ON parking_reports(created_at DESC);
CREATE INDEX idx_parking_reports_expires_at ON parking_reports(expires_at);

-- Favorites table
CREATE TABLE IF NOT EXISTS favorites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parking_id UUID NOT NULL REFERENCES parking_locations(id) ON DELETE CASCADE,
    name VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, parking_id)
);

-- Index for user favorites
CREATE INDEX idx_favorites_user_id ON favorites(user_id);

-- Search history table
CREATE TABLE IF NOT EXISTS search_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    query TEXT NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    radius_m INTEGER NOT NULL,
    results_count INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for user search history
CREATE INDEX idx_search_history_user_id ON search_history(user_id);
CREATE INDEX idx_search_history_created_at ON search_history(created_at DESC);

-- Availability history table (for tracking availability over time)
CREATE TABLE IF NOT EXISTS availability_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parking_id UUID NOT NULL REFERENCES parking_locations(id) ON DELETE CASCADE,
    score INTEGER NOT NULL CHECK (score >= 0 AND score <= 100),
    confidence INTEGER NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    level VARCHAR(20) NOT NULL CHECK (level IN ('high', 'moderate', 'low')),
    calculated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for availability history
CREATE INDEX idx_availability_history_parking_id ON availability_history(parking_id);
CREATE INDEX idx_availability_history_calculated_at ON availability_history(calculated_at DESC);

-- Cached searches table
CREATE TABLE IF NOT EXISTS cached_searches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cache_key VARCHAR(512) UNIQUE NOT NULL,
    search_params JSONB NOT NULL,
    results JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Index for cached searches
CREATE INDEX idx_cached_searches_cache_key ON cached_searches(cache_key);
CREATE INDEX idx_cached_searches_expires_at ON cached_searches(expires_at);

-- Sessions table (for JWT token management if needed)
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Index for sessions
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_token_hash ON sessions(token_hash);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers to automatically update updated_at
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_parking_locations_updated_at BEFORE UPDATE ON parking_locations
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Function to clean up expired reports and cached searches
CREATE OR REPLACE FUNCTION cleanup_expired_data()
RETURNS void AS $$
BEGIN
    DELETE FROM parking_reports WHERE expires_at < NOW();
    DELETE FROM cached_searches WHERE expires_at < NOW();
    DELETE FROM sessions WHERE expires_at < NOW();
END;
$$ LANGUAGE plpgsql;

-- Schedule cleanup to run periodically (requires pg_cron extension)
-- SELECT cron.schedule('cleanup-expired-data', '0 * * * *', 'SELECT cleanup_expired_data()');
