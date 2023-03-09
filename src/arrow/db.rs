use crate::job;
use async_trait::async_trait;

#[derive(Clone)]
pub struct DatabaseResetStep {}

impl DatabaseResetStep {
    pub fn new() -> Box<dyn job::Step> {
        Box::new(DatabaseResetStep {})
    }
}

#[async_trait]
impl job::Step for DatabaseResetStep {
    fn description(&self) -> String {
        "Restart the the PSQL database and clear all tables.".into()
    }

    async fn action(&self) -> job::StepResult {
        let psql_cmd = "
        BEGIN;
        DELETE FROM arrow.vehicles;
        DELETE FROM arrow.vertiports;
        DELETE FROM arrow.vertipads;
        COMMIT;
        ";

        let status = std::process::Command::new("docker")
                     .arg("exec -it arrow-cockroachdb")
                     .arg("cockroach sql --cert-dir /cockroach/ssl/certs")
                     .arg(format!("-e {psql_cmd}"))
                     .status();

        let Ok(status) = status else {
            return Err(status.unwrap_err().to_string())
        };

        if !status.success() {
            return Err("Failed to clear PSQL tables.".into());
        }

        Ok(())
    }
}
