use self::error::AIServiceError;
use crate::db::Ad4mDb;
use crate::graphql::graphql_types::AITaskInput;
use crate::types::AITask;
use anyhow::anyhow;
use deno_core::error::AnyError;
use kalosm::language::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

mod error;

pub type Result<T> = std::result::Result<T, AnyError>;

lazy_static! {
    static ref AI_SERVICE: Arc<Mutex<Option<AIService>>> = Arc::new(Mutex::new(None));
}

#[derive(Clone)]
pub struct AIService {
    bert: Arc<Mutex<Bert>>,
    llama: Arc<Mutex<Llama>>,
    tasks: Arc<Mutex<HashMap<String, Task>>>,
}

impl AIService {
    pub async fn new() -> Result<Self> {
        let bert = Bert::builder().build().await?;
        let llama = Llama::builder()
            .with_source(LlamaSource::tiny_llama_1_1b())
            .build()
            .await?;

        let service = AIService {
            bert: Arc::new(Mutex::new(bert)),
            llama: Arc::new(Mutex::new(llama)),
            tasks: Arc::new(Mutex::new(HashMap::new())),
        };

        let tasks = Ad4mDb::with_global_instance(|db| db.get_tasks())
            .map_err(|e| AIServiceError::DatabaseError(e.to_string()))?;

        for task in &tasks {
            service.spawn_task(task).await?;
        }

        Ok(service)
    }

    pub async fn init_global_instance() -> Result<()> {
        let new_service = AIService::new().await?;
        let mut ai_service = AI_SERVICE.lock().await;
        *ai_service = Some(new_service);
        Ok(())
    }

    pub async fn global_instance() -> Result<AIService> {
        AI_SERVICE
            .lock()
            .await
            .clone()
            .ok_or(anyhow!(AIServiceError::ServiceNotInitialized))
    }

    pub async fn add_task(&mut self, task: AITaskInput) -> Result<AITask> {
        let task_id = Ad4mDb::with_global_instance(|db| {
            db.add_task(
                task.name.clone(),
                task.model_id.clone(),
                task.system_prompt.clone(),
                task.prompt_examples
                    .iter()
                    .map(|p| p.clone().into())
                    .collect(),
                task.meta_data.clone(),
            )
        })
        .map_err(|e| AIServiceError::DatabaseError(e.to_string()))?;

        let task = Ad4mDb::with_global_instance(|db| db.get_task(task_id))?
            .expect("to get task that we just created");

        self.spawn_task(&task).await?;
        Ok(task)
    }

    pub async fn delete_task(&mut self, task_id: String) -> Result<bool> {
        Ad4mDb::with_global_instance(|db| db.remove_task(task_id.clone()))
            .map_err(|e| AIServiceError::DatabaseError(e.to_string()))?;

        self.tasks.lock().await.remove(&task_id);

        Ok(true)
    }

    pub fn get_tasks() -> Result<Vec<AITask>> {
        let tasks = Ad4mDb::with_global_instance(|db| db.get_tasks())
            .map_err(|e| AIServiceError::DatabaseError(e.to_string()))?;
        Ok(tasks)
    }

    pub async fn update_task(&mut self, task: AITask) -> Result<AITask> {
        let task_id = task.task_id.clone();
        Ad4mDb::with_global_instance(|db| {
            db.update_task(
                task.task_id,
                task.name,
                task.model_id,
                task.system_prompt,
                task.prompt_examples,
                task.meta_data,
            )
        })
        .map_err(|e| AIServiceError::DatabaseError(e.to_string()))?;

        self.tasks.lock().await.remove(&task_id);

        let updated_task = Ad4mDb::with_global_instance(|db| db.get_task(task_id.clone()))
            .map_err(|e| AIServiceError::DatabaseError(e.to_string()))?
            .ok_or(AIServiceError::TaskNotFound)?;

        self.spawn_task(&updated_task).await?;

        Ok(updated_task)
    }

    pub async fn prompt(&self, task_id: String, prompt: String) -> Result<String> {
        let tasks_lock = self.tasks.lock().await;

        if let Some(task) = tasks_lock.get(&task_id) {
            Ok(self.run_task(task, prompt).await)
        } else {
            Err(AIServiceError::TaskNotFound.into())
        }
    }

    pub async fn embed(text: String) -> Result<Vec<f32>> {
        Ok(AIService::global_instance()
            .await?
            .bert
            .lock()
            .await
            .embed(text)
            .await?
            .to_vec())
    }

    async fn spawn_task(&self, task_description: &AITask) -> Result<()> {
        let task = Task::builder(task_description.system_prompt.clone())
            .with_examples(
                task_description
                    .prompt_examples
                    .clone()
                    .into_iter()
                    .map(|example| (example.input, example.output))
                    .collect::<Vec<(String, String)>>(),
            )
            .build();

        let _ = self
            .run_task(&task, "Test example prompt".to_string())
            .await;

        self.tasks
            .lock()
            .await
            .insert(task_description.task_id.clone(), task);

        Ok(())
    }

    async fn run_task(&self, task: &Task, prompt: String) -> String {
        let llama = self.llama.lock().await;
        task.run(prompt, &*llama).all_text().await
    }
}

#[cfg(test)]
mod tests {
    use crate::graphql::graphql_types::AIPromptExamplesInput;

    use super::*;

    #[tokio::test]
    async fn test_embedding() {
        AIService::init_global_instance()
            .await
            .expect("initialization to work");
        let vector = AIService::embed("Test string".into())
            .await
            .expect("embed to return a result");
        assert!(vector.len() > 300)
    }

    #[tokio::test]
    async fn test_prompt() {
        Ad4mDb::init_global_instance(":memory:").expect("Ad4mDb to initialize");
        AIService::init_global_instance()
            .await
            .expect("initialization to work");

        let task = AIService::with_mutable_global_instance(|service| {
            service.add_task(AITaskInput {
                name: "Test task".into(),
                model_id: "Llama tiny 1b".into(),
                system_prompt: "You are inside a test for tasks. Please make sure to create any non-zero length output".into(),
                prompt_examples: vec![AIPromptExamplesInput{
                    input: "Test string".into(),
                    output: "Yes, I'm working!".into()
                }],
                meta_data: None
            })
        }).await.expect("add_task to work without error");

        let response = AIService::prompt(task.task_id, "Test string".into())
            .await
            .expect("prompt to return a result");
        println!("Response: {}", response);
        assert!(response.len() > 0)
    }
}
