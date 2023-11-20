use std::collections::HashMap;

use testcontainers::{core::WaitFor, Image};

const NAME: &str = "postgres";
const TAG: &str = "11-alpine";

/// Module to work with [`Postgres`] inside of tests.
///
/// Starts an instance of Postgres.
/// This module is based on the official [`Postgres docker image`].
///
/// # Example
/// ```
/// use testcontainers::clients;
/// use testcontainers_modules::postgres;
///
/// let docker = clients::Cli::default();
/// let postgres_instance = docker.run(postgres::Postgres::default());
///
/// let connection_string = format!(
///     "postgres://postgres:postgres@127.0.0.1:{}/postgres",
///     postgres_instance.get_host_port_ipv4(5432)
/// );
/// ```
///
/// [`Postgres`]: https://www.postgresql.org/
/// [`Postgres docker image`]: https://hub.docker.com/_/postgres
#[derive(Debug)]
pub struct Postgres {
    env_vars: HashMap<String, String>,
}

impl Default for Postgres {
    fn default() -> Self {
        let mut env_vars = HashMap::new();
        env_vars.insert("POSTGRES_DB".to_owned(), "postgres".to_owned());
        env_vars.insert("POSTGRES_HOST_AUTH_METHOD".into(), "trust".into());

        Self { env_vars }
    }
}

impl Image for Postgres {
    type Args = ();

    fn name(&self) -> String {
        NAME.to_owned()
    }

    fn tag(&self) -> String {
        TAG.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        )]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item=(&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }
}

#[cfg(test)]
mod users {
    use axum_test::TestServer;
    use http::StatusCode;
    use sea_orm::{ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement};
    use testcontainers::clients::Cli;

    use crate::app;
    use crate::tests::users::Postgres;

    #[tokio::test]
    async fn when_calling_users_endpoint_should_return_200() {
        let docker = Cli::default();
        let node = docker.run(Postgres::default());

        // prepare connection string
        let connection_string = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432)
        );

        let db: DatabaseConnection = Database::connect(connection_string).await.expect("Cannot find posts in page");
        let statement = Statement {
            sql: "
                CREATE TABLE IF NOT EXISTS user_base
                (
                    id         INTEGER GENERATED ALWAYS AS IDENTITY,
                    first_name VARCHAR(255) NULL,
                    last_name  VARCHAR(255) NULL,
                    email      VARCHAR(255) NOT NULL,
                    phone      VARCHAR(25)  NULL,
                    created_on TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                    updated_on TIMESTAMP WITH TIME ZONE,
                    PRIMARY KEY (email),
                    UNIQUE (phone),
                    UNIQUE (id)
                );
            ".to_string(),
            values: None,
            db_backend: DatabaseBackend::Postgres,
        };
        db.execute(statement).await.expect("TODO: panic message");
        let statement = Statement {
            sql: "
                INSERT INTO user_base (first_name, last_name, email, phone)
                VALUES ('User', 'Internal', 'user@internal.io', '555-555-5555')
                ON CONFLICT DO NOTHING;
            ".to_string(),
            values: None,
            db_backend: DatabaseBackend::Postgres,
        };
        db.execute(statement).await.expect("TODO: panic message");

        let app = app(connection_string.clone())
            .await
            .into_make_service();
        let server = TestServer::new(app).unwrap();
        let response = server
            .get("/users")
            .await;

        assert_eq!(response.status_code(), StatusCode::OK);
    }
}
