pub fn sqlxStringify (err: sqlx::Error) -> String {
    format!("{}", err)
}