use anyhow::Result;
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use protobuf::SpecialFields;
use sqlx::PgPool;
use crate::proto::account::Account;
use crate::proto::stats::Stats;

#[derive(Clone, Debug)]
pub struct Store {
    db: PgPool
}

impl Store {
    pub fn new(db: PgPool) -> Self {
        Store { db }
    }

    pub async fn create_account(&self, account: &Account) -> Result<Account> {

        let birthday = match account.birthday {
            Some(birthday) => {
                let date = DateTime::from_timestamp(birthday, 0).unwrap();
                Some(date.date_naive())
            },
            None => None,
        };

        struct T {
            pub id: i64,
            first_name: Option<String>,
            birthday: Option<NaiveDate>,
            discord_id: Option<String>,
        }
        let re : sqlx::Result<T> = sqlx::query_as!(
            T,
            r#"
            INSERT INTO accounts (
                first_name, birthday, discord_id
            ) VALUES ($1, $2, $3)
            RETURNING id, first_name, birthday, discord_id
            ;"#,
            account.first_name,
            birthday,
            account.discord_id,
        )
            .fetch_one(&self.db)
            .await;


        let re = re?;

        let birthday  = match re.birthday {
            Some(birthday) => {
                Some(birthday.and_hms_opt(0,0,0,).unwrap().and_utc().timestamp_millis())
            },
            None => None,
        };

        Ok(Account{
            id: re.id.to_string(),
            first_name: re.first_name,
            discord_id: re.discord_id,
            birthday,
            special_fields: SpecialFields::default(),
        })
    }
}