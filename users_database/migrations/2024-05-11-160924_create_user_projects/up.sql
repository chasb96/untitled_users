CREATE TABLE user_projects (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    project_id VARCHAR(16) NOT NULL
);