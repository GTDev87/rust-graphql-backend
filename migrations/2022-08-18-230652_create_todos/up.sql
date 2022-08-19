CREATE TABLE todos (
    id serial PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    done BOOLEAN NOT NULL DEFAULT false
);
