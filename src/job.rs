pub type StepResult = Result<(), String>;

#[async_trait::async_trait]
pub trait Step {
    fn description(&self) -> String;
    async fn action(&self) -> StepResult;
}

pub struct Job<S: Step + Clone> {
    pub name: String,
    pub description: String,
    pub requirements: Vec<String>,

    // This should point to a common series of setup steps
    pub setup: Vec<S>,

    // This is the main test body
    pub steps: Vec<S>,

    // This should point to a common series of cleanup steps
    pub cleanup: Vec<S>,
}

impl<S: Step + Clone> Job<S> {
    // pub fn steps(&self) -> &Vec<S> {
    //     &self.steps
    // }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn requirements(&self) -> &Vec<String> {
        &self.requirements
    }

    // pub fn setup_steps(&self) -> Vec<S> {
    //     match &self.setup {
    //         Some(s) => s.to_vec(),
    //         None => vec![]
    //     }
    // }

    // pub fn cleanup_steps(&self) -> Vec<S> {
    //     match &self.cleanup {
    //         Some(s) => s.to_vec(),
    //         None => vec![]
    //     }
    // }

    pub async fn execute(&mut self) -> Result<(), String> {
        // TODO check if plan only

        let mut ok = true;
        for s in &self.setup {
            let result = s.action().await;
            if result.is_err() {
                ok = false;
                break;
            }
        }
        
        if ok {
            for s in &self.steps {
                let result = s.action().await;

                if result.is_err() {
                    break;
                }
            }
        }

        for s in &self.cleanup {
            let _ = s.action().await;
        }

        Ok(())
    }
}
