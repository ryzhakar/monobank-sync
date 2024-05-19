use crate::models;
use serde_with::chrono::NaiveDateTime;
use sqlx::SqlitePool;

pub async fn insert_client_info(
    pool: SqlitePool,
    client_info: models::ClientInfo,
) -> Result<(), sqlx::Error> {
    tracing::debug!("Trying to write client info into DB...");
    let result = sqlx::query!(
        "
        INSERT OR IGNORE INTO client_info (client_id, name, token)
        VALUES (?, ?, ?)
        ",
        client_info.client_id,
        client_info.name,
        client_info.token,
    )
    .execute(&pool)
    .await;
    if let Err(e) = result {
        Err(e)
    } else {
        Ok(())
    }
}

pub async fn update_last_sync_time(
    pool: SqlitePool,
    account_id: String,
    last_sync_at: Option<NaiveDateTime>,
) -> Result<(), sqlx::Error> {
    tracing::debug!("Attempting to update an account...");
    let result = sqlx::query!(
        "
        UPDATE accounts
        SET last_sync_at = ?
        WHERE id = ?
        ",
        last_sync_at,
        account_id,
    )
    .execute(&pool)
    .await;
    if let Err(e) = result {
        Err(e)
    } else {
        Ok(())
    }
}

pub async fn get_last_sync_time(
    pool: SqlitePool,
    account_id: String,
) -> Result<Option<NaiveDateTime>, sqlx::Error> {
    tracing::debug!("Retrieving last sync time from DB...");
    let result = sqlx::query_as!(
        models::LastSync,
        r#"
        SELECT last_sync_at as "last_sync_at: NaiveDateTime"
        FROM accounts
        WHERE id = ?
        "#,
        account_id,
    )
    .fetch_optional(&pool)
    .await;
    match result {
        Ok(Some(time_struct)) => Ok(time_struct.last_sync_at),
        Ok(None) => Ok(None),
        Err(e) => Err(e),
    }
}

pub async fn insert_account(pool: SqlitePool, account: models::Account) -> Result<(), sqlx::Error> {
    tracing::debug!("Trying to write account info into DB...");
    let result = sqlx::query!(
        "
        INSERT OR IGNORE INTO accounts (
            id,
            client_id,
            send_id,
            balance,
            credit_limit,
            account_type,
            currency_code,
            cashback_type,
            iban,
            last_sync_at
        )
        VALUES (?,?,?,?,?,?,?,?,?,?)
        ",
        account.id,
        account.client_id,
        account.send_id,
        account.balance,
        account.credit_limit,
        account.account_type,
        account.currency_code,
        account.cashback_type,
        account.iban,
        account.last_sync_at,
    )
    .execute(&pool)
    .await;
    if let Err(e) = result {
        Err(e)
    } else {
        Ok(())
    }
}

pub async fn insert_statement_item(
    pool: SqlitePool,
    statement_item: models::StatementItem,
) -> Result<(), sqlx::Error> {
    tracing::debug!("Trying to write statement item...");
    let result = sqlx::query!(
        "
        INSERT OR IGNORE INTO statement_items (
            id,
            account_id,
            time,
            description,
            mcc,
            original_mcc,
            hold,
            amount,
            operation_amount,
            currency_code,
            commission_rate,
            cashback_amount,
            balance,
            comment,
            receipt_id,
            invoice_id,
            counter_edrpou,
            counter_iban,
            counter_name
        )
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
        ",
        statement_item.id,
        statement_item.account_id,
        statement_item.time,
        statement_item.description,
        statement_item.mcc,
        statement_item.original_mcc,
        statement_item.hold,
        statement_item.amount,
        statement_item.operation_amount,
        statement_item.currency_code,
        statement_item.commission_rate,
        statement_item.cashback_amount,
        statement_item.balance,
        statement_item.comment,
        statement_item.receipt_id,
        statement_item.invoice_id,
        statement_item.counter_edrpou,
        statement_item.counter_iban,
        statement_item.counter_name,
    )
    .execute(&pool)
    .await;
    if let Err(e) = result {
        Err(e)
    } else {
        Ok(())
    }
}
