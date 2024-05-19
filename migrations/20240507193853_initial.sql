CREATE TABLE IF NOT EXISTS client_info (
    client_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    token TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY,
    send_id TEXT NOT NULL,
    balance BIGINT NOT NULL,
    credit_limit BIGINT NOT NULL,
    account_type TEXT NOT NULL,
    currency_code INTEGER NOT NULL,
    cashback_type TEXT,
    iban TEXT,
    last_sync_at TIMESTAMP WITHOUT TIME ZONE,
    client_id TEXT NOT NULL,
    FOREIGN KEY (client_id) REFERENCES client_info(client_id)
);

CREATE TABLE IF NOT EXISTS statement_items (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL,
    time TIMESTAMP NOT NULL,
    description TEXT NOT NULL,
    mcc INTEGER NOT NULL,
    original_mcc INTEGER NOT NULL,
    hold BOOLEAN NOT NULL,
    amount BIGINT NOT NULL,
    operation_amount BIGINT NOT NULL,
    currency_code INTEGER NOT NULL,
    commission_rate BIGINT NOT NULL,
    cashback_amount BIGINT NOT NULL,
    balance BIGINT NOT NULL,
    comment TEXT,
    receipt_id TEXT,
    invoice_id TEXT,
    counter_edrpou TEXT,
    counter_iban TEXT,
    counter_name TEXT,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
);
