-- Create slot_bookings table
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

-- Create indexes for better query performance
CREATE INDEX idx_slot_bookings_ev_id ON slot_bookings(ev_id);
CREATE INDEX idx_slot_bookings_arrival_time ON slot_bookings(arrival_time);
CREATE INDEX idx_slot_bookings_status ON slot_bookings(status);

-- Create trigger for updated_at
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