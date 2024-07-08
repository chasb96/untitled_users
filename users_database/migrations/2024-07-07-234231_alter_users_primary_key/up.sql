ALTER TABLE users 
DROP COLUMN id;

ALTER TABLE users 
ADD COLUMN id VARCHAR(16) PRIMARY KEY
SET DEFAULT (
    select
        string_agg(substr(characters, (random() * length(characters) + 1)::integer, 1), '') as random_word
    from (values('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789')) as symbols(characters)
    join generate_series(1, 16) on 1 = 1
);