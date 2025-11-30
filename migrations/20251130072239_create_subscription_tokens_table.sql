CREATE TABLE subscription_tokens(
    subscription_token TEXT PRIMARY KEY,
    subscriber_id uuid NOT NULL
        REFERENCES subscriptions (id)
);
