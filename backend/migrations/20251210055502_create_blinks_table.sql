-- Create blinks table
CREATE TYPE blink_type AS ENUM ('donation', 'payment', 'vote');

CREATE TABLE blinks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMPTZ DEFAULT now(),
    title TEXT NOT NULL,
    icon_url TEXT NOT NULL,
    description TEXT NOT NULL,
    label TEXT NOT NULL,
    wallet_address TEXT NOT NULL,
    
    type blink_type NOT NULL DEFAULT 'donation',
    config JSONB NOT NULL DEFAULT '{}'::jsonb
);

ALTER TABLE blinks ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Allow all" ON blinks FOR ALL USING (true);