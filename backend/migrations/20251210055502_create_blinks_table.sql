-- Create blinks table

CREATE TABLE IF NOT EXISTS blinks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ DEFAULT now(),
    title TEXT NOT NULL,
    icon_url TEXT NOT NULL,
    description TEXT NOT NULL,
    label TEXT NOT NULL,
    wallet_address TEXT NOT NULL,
    amount_sol FLOAT8 DEFAULT 0.1
);

ALTER TABLE blinks ENABLE ROW LEVEL SECURITY;
