-- Add migration script here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE char_sheet_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID REFERENCES users(id) not null,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    schema JSONB NOT NULL,
    is_public BOOLEAN DEFAULT FALSE not null,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE characters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID REFERENCES users(id),
    template_id UUID REFERENCES char_sheet_templates(id),
    name VARCHAR(100) NOT NULL,
    data JSONB NOT NULL,
    is_public BOOLEAN DEFAULT FALSE not null,
    created_at TIMESTAMP DEFAULT NOW()
);
