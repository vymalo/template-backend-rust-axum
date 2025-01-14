-- Create the todos table
CREATE TABLE todos
(
    id          TEXT PRIMARY KEY,
    created_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    metadata    JSONB                    DEFAULT '{}',
    title       TEXT NOT NULL,
    description TEXT
);

-- Add the indexes
CREATE INDEX idx_todos_created_at ON todos (created_at);