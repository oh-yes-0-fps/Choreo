pub fn sqlx_stringify(err: sqlx::Error) -> String {
    format!("{}", err)
}
