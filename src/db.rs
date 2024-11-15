use crate::model::*;
use crate::prelude::*;
use crate::DB;

#[allow(dead_code)]
const TASK: &str = "task";

pub async fn add_task(description: String) -> Result<Task> {
    let created: Option<Task> = DB
        .create(TASK)
        .content(Task {
            id: None,
            description,
            completed: false,
        })
        .await?;

    created.ok_or_else(|| Error::Generic("Failed to create task".into()))
}

pub async fn get_task(id: String) -> Result<Task> {
    let rec: Option<Task> = DB.select((TASK, id)).await?;

    Ok(rec.unwrap())
}

pub async fn delete_task(id: String) -> Result<AffectedRows> {
    let _rec: Option<Record> = DB.delete((TASK, id)).await?;
    match _rec {
        Some(record) => {
            if record.id.to_string().is_empty() {
                Ok(AffectedRows { rows_affected: 0 })
            } else {
                Ok(AffectedRows { rows_affected: 1 })
            }
        },
        None => Ok(AffectedRows { rows_affected: 0 }),
    }
}

pub async fn toggle_task(id: String) -> Result<Task> {
    // First get the current task
    let current_task: Option<Task> = DB.select((TASK, &id)).await?;
    let task = current_task.ok_or_else(|| Error::Generic("Task not found".into()))?;
    
    // Now update with the opposite of current completed status
    let sql = "UPDATE type::thing($tb, $id) SET completed = $new_status";
    
    let mut response = DB
        .query(sql)
        .bind(("tb", TASK))
        .bind(("id", id))
        .bind(("new_status", !task.completed))
        .await?;

    let task_updated = response
        .take::<Vec<Task>>(0)?
        .into_iter()
        .next()
        .ok_or(Error::Generic("Failed to update record".into()))?;

    Ok(task_updated)
}

pub async fn get_all_tasks() -> Result<Vec<Task>> {
    let tasks: Vec<Task> = DB.select(TASK).await?;

    Ok(tasks)
    // let sql = "SELECT * FROM type::table($table) ORDER BY created_at DESC;";

    // let mut response = DB.query(sql).bind(("table", TASK)).await?;

    // let tasks: Vec<Task> = response.take(0)?;

    // Ok(tasks)
}