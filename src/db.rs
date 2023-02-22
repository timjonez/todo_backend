use surrealdb::{Datastore, Session };

pub async fn setup_db() -> (Datastore, Session) {
    let db = get_database().await;
    let session = get_session().await;

    (db, session)
}

pub async fn get_database() -> Datastore {
    Datastore::new("file:/home/tim/Coding/Projects/rusty_todo/todo_backend/test.db")
        .await
        .unwrap()
}

pub async fn get_session() -> Session {
    Session::for_kv().with_ns("test_ns").with_db("test_db")
}


//let query = "CREATE company:surrealdb SET name = 'SurrealDB', cofounders = [person:tobie, person:jaime];";
// let query = "Select * from account;";
// let add_query = "CREATE account SET name = 'ACME Inc', created_at = time::now();";
// let ses = Session::for_kv().with_ns("test_ns").with_db("test_db");
// ds.execute(add_query, &ses, None, false)
//     .await
//     .unwrap();
// let res = &ds.execute(query, &ses, None, false)
//     .await
//     .unwrap();

// for r in res {
//     let result = r.result.as_ref().unwrap();
//     println!("Test: {}", result);
// }

// res.first().unwrap().result.unwrap()
