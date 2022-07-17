use sea_orm::sea_query::extension::postgres::TypeCreateStatement;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                TypeCreateStatement::new()
                    .as_enum(ReceiptState::Type)
                    .values(vec![
                        ReceiptState::Inbox,
                        ReceiptState::Valid,
                        ReceiptState::Payed,
                        ReceiptState::Declined,
                        ReceiptState::Process,
                        ReceiptState::Done,
                    ])
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Receipts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Receipts::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Receipts::Name).string().not_null())
                    .col(
                        ColumnDef::new(Receipts::State)
                            .enumeration(
                                ReceiptState::Type,
                                vec![
                                    ReceiptState::Inbox,
                                    ReceiptState::Valid,
                                    ReceiptState::Payed,
                                    ReceiptState::Declined,
                                    ReceiptState::Process,
                                    ReceiptState::Done,
                                ],
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Receipts::FileHash).string().not_null())
                    .col(ColumnDef::new(Receipts::Category).string().null())
                    .col(
                        ColumnDef::new(Receipts::PaymentDate)
                            .date_time()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Receipts::Table).to_owned())
            .await
    }
}

enum ReceiptState {
    Type,
    Inbox,
    Valid,
    Payed,
    Declined,
    Process,
    Done,
}

impl Iden for ReceiptState {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                ReceiptState::Inbox => "inbox",
                ReceiptState::Valid => "valid",
                ReceiptState::Payed => "payed",
                ReceiptState::Declined => "declined",
                ReceiptState::Process => "process",
                ReceiptState::Done => "done",
                ReceiptState::Type => "receipt_state",
            }
        )
        .unwrap();
    }
}

impl std::fmt::Display for ReceiptState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReceiptState::Inbox => "inbox",
                ReceiptState::Valid => "valid",
                ReceiptState::Payed => "payed",
                ReceiptState::Declined => "declined",
                ReceiptState::Process => "process",
                ReceiptState::Done => "done",
                ReceiptState::Type => "receipt_state",
            }
        )
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Receipts {
    Table,
    Id,
    Name,
    State,
    FileHash,
    Category,
    PaymentDate,
}
