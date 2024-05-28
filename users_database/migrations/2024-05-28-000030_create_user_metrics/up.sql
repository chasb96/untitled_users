CREATE TABLE user_metrics (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL UNIQUE,
    view_count INTEGER NOT NULL DEFAULT 0
)