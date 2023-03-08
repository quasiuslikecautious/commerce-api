CREATE TABLE IF NOT EXISTS deals (
    uuid uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    name TEXT NOT NULL,
    image TEXT NOT NULL,
    price INTEGER NOT NULL,
    description TEXT NOT NULL
);
