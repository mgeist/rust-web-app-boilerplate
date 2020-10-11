mod password_reset_token;
mod user;

pub use password_reset_token::PasswordResetToken;
pub use user::User;

type Query = sqlx::Query<'static, sqlx::Sqlite>;
type QueryAs<T> = sqlx::QueryAs<'static, sqlx::Sqlite, T>;
