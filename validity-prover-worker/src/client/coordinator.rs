use reqwest::Client;

pub struct CoordinatorClient {
    client: Client,
    base_url: String,
}

impl CoordinatorClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn assign_task(&self) -> Result<AssignResponse> {
        let response = self
            .client
            .post(format!("{}/coordinator/assign", self.base_url))
            .send()
            .await?
            .json::<AssignResponse>()
            .await?;
        Ok(response)
    }

    pub async fn complete_task(&self, block_number: u64, transition_proof: String) -> Result<()> {
        let request = CompleteRequest {
            block_number,
            transition_proof,
        };

        self.client
            .post(format!("{}/coordinator/complete", self.base_url))
            .json(&request)
            .send()
            .await?;
        Ok(())
    }

    pub async fn heartbeat(&self, block_number: u64) -> Result<()> {
        let request = HeartBeatRequest { block_number };

        self.client
            .post(format!("{}/coordinator/heartbeat", self.base_url))
            .json(&request)
            .send()
            .await?;
        Ok(())
    }
}
