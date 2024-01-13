use rusqlite::Connection;

pub fn establish() -> Connection {

    let task_table_create_sql=
        "create table if not exists tasks (
            id blob primary key,
            description text not null,
            created_at integer not null,
            done integer not null
        )";

    let connection = Connection::open("todo.sqlite");

    connection.and_then(|connection| {
        connection.execute(task_table_create_sql, ())
            .map(|_| connection)
    }).expect("Cannot establish database connection!")
}