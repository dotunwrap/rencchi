use mysql::*;

pub fn init_dnd_db() -> PooledConn {
    Pool::new(
        OptsBuilder::from_opts(
            Opts::from_url(&std::env::var("DND_DATABASE_URL").expect("Database URL not found"))
                .expect("Could not parse database URL"),
        )
        .ssl_opts(SslOpts::default()),
    )
    .expect("Could not connect to database")
    .get_conn()
    .expect("Could not get connection")
}

pub async fn init_dnd_db_async() -> Result<PooledConn, mysql::Error> {
    Pool::new(
        OptsBuilder::from_opts(
            Opts::from_url(&std::env::var("DND_DATABASE_URL").expect("Database URL not found"))
                .expect("Could not parse database URL"),
        )
        .ssl_opts(SslOpts::default()),
    )
    .expect("Could not connect to database")
    .get_conn()
}
