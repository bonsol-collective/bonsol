use solana_transaction_status::TransactionStatus as TransactionConfirmationStatus;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending { expiry: u64 },
    Confirmed(TransactionConfirmationStatus),
}
