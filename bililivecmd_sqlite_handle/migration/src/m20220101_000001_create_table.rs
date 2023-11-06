use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DM::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DM::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(DM::UName).string())
                    .col(ColumnDef::new(DM::UId).big_integer())
                    .col(ColumnDef::new(DM::UFace).string())
                    .col(ColumnDef::new(DM::Timestamp).big_integer())
                    .col(ColumnDef::new(DM::RoomId).big_integer())
                    .col(ColumnDef::new(DM::Msg).string())
                    .col(ColumnDef::new(DM::MsgId).string())
                    .col(ColumnDef::new(DM::GuardLevel).big_integer())
                    .col(ColumnDef::new(DM::FansMedalWearingStatus).boolean())
                    .col(ColumnDef::new(DM::FansMedalName).string())
                    .col(ColumnDef::new(DM::FansMedalLevel).big_integer())
                    .col(ColumnDef::new(DM::EmojiImgUrl).string())
                    .col(ColumnDef::new(DM::DmType).big_integer())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DM::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}

#[derive(DeriveIden)]
enum DM {
    Table,
    Id,
    UName,
    UId,
    UFace,
    Timestamp,
    RoomId,
    Msg,
    MsgId,
    GuardLevel,
    FansMedalWearingStatus,
    FansMedalName,
    FansMedalLevel,
    EmojiImgUrl,
    DmType,
}
