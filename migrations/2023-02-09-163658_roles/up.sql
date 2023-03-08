CREATE TABLE IF NOT EXISTS roles (
    uuid uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    name TEXT NOT NULL
);
