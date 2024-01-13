use rusqlite::{Connection, params, Result, Statement};
use uuid::Uuid;
use crate::database::task::Task;

static COLUMNS: &str = "id, description, created_at, done";
static TABLE: &str = "tasks";

pub fn insert(connection: &Connection, task: Task) -> Result<usize> {

    connection.execute(
        &*format!("insert into {} ({}) values (?1, ?2, ?3, ?4)", TABLE, COLUMNS),
        (&task.id, &task.description, &task.created_at, &task.done)
    )
}

pub fn delete(connection: &Connection, id: Uuid) -> Result<usize> {

    connection.execute(&*format!("delete from {} where id = ?1", TABLE), params!(id))
}

pub fn get_all(connection: &Connection) -> Result<Statement<'_>> {

    connection.prepare(&*format!("select {} from {}", COLUMNS, TABLE))
}

pub fn mark(connection: &Connection, id: Uuid, done: bool) -> Result<usize> {

    connection.execute(
        &*format!("update {} set done = {} where id = ?1", TABLE, if done { "1" } else { "0" } ),
        params!(id)
    )
}

