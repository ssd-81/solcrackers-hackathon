-- 20250913_create_ev_requests.sql
CREATE TABLE ev_requests (
    id SERIAL PRIMARY KEY,
    user_id TEXT NOT NULL,
    slot_start TIMESTAMP NOT NULL,
    slot_end TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);
