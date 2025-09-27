-- Drop dependencies first
DROP TRIGGER IF EXISTS update_slot_bookings_updated_at ON slot_bookings;
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop tables (ev_responses depends on slot_bookings, so drop it first)
DROP TABLE IF EXISTS ev_responses CASCADE;
DROP TABLE IF EXISTS slot_bookings CASCADE;

-- Now recreate everything from scratch
CREATE TABLE slot_bookings (
    id SERIAL PRIMARY KEY,
    slot_id VARCHAR(255) NOT NULL UNIQUE,
    ev_id VARCHAR(255) NOT NULL,
    arrival_time TIMESTAMPTZ NOT NULL,
    duration INTEGER NOT NULL CHECK (duration > 0),
    power_needed BIGINT NOT NULL CHECK (power_needed > 0),
    status VARCHAR(50) DEFAULT 'pending' NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

CREATE INDEX idx_slot_bookings_ev_id ON slot_bookings(ev_id);
CREATE INDEX idx_slot_bookings_arrival_time ON slot_bookings(arrival_time);
CREATE INDEX idx_slot_bookings_status ON slot_bookings(status);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_slot_bookings_updated_at
    BEFORE UPDATE ON slot_bookings
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TABLE ev_responses (
    id SERIAL PRIMARY KEY,
    slot_id VARCHAR(255) NOT NULL,
    status VARCHAR(100) NOT NULL,
    message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    FOREIGN KEY (slot_id) REFERENCES slot_bookings(slot_id) ON DELETE CASCADE
);

CREATE INDEX idx_ev_responses_slot_id ON ev_responses(slot_id);
CREATE INDEX idx_ev_responses_status ON ev_responses(status);

