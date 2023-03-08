CREATE TABLE IF NOT EXISTS users (
    uuid uuid DEFAULT uuid_generate_v4 () PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    role uuid FOREIGN KEY REFERENCES roles(uuid)
);