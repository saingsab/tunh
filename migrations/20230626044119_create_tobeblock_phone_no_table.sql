-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
    IF NOT EXISTS tbl_tobeblock_phone_no (
        id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
        phone_no VARCHAR(9) NOT NULL UNIQUE,
        created_by UUID,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );