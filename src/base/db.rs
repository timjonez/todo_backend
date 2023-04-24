use surrealdb::{Datastore, Session};

pub async fn setup_db() -> (Datastore, Session) {
    let db = get_database().await;
    let session = get_session().await;

    (db, session)
}

pub async fn get_database() -> Datastore {
    Datastore::new("file:./test.db")
        .await
        .unwrap()
}

pub async fn get_session() -> Session {
    Session::for_kv().with_ns("test_ns").with_db("test_db")
}
