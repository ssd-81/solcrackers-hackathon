-- Create ev_responses table for logging responses
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