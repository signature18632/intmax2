async fn send_get_request(url: &str) -> anyhow::Result<reqwest::Response> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    Ok(response)
}

// export async function syncValidityProof(baseUrl: string,): Promise<void> {
//     const url = `${baseUrl}/block-validity-prover/sync`;
//     await callServer(url);
// }

// export async function postEmptyBlock(baseUrl: string,): Promise<void> {
//     const url = `${baseUrl}/block-builder/post-empty-block`;
//     await callServer(url);
// }

// export async function constructBlock(baseUrl: string,): Promise<void> {
//     const url = `${baseUrl}/block-builder/construct-block`;
//     await callServer(url);
// }

// export async function postBlock(baseUrl: string,): Promise<void> {
//     const url = `${baseUrl}/block-builder/post-block`;
//     await callServer(url);
// }

pub async fn sync_validity_proof(base_url: &str) -> anyhow::Result<()> {
    let url = format!("{}/block-validity-prover/sync", base_url);
    send_get_request(&url).await?;
    Ok(())
}

pub async fn post_empty_block(base_url: &str) -> anyhow::Result<()> {
    let url = format!("{}/block-builder/post-empty-block", base_url);
    send_get_request(&url).await?;
    Ok(())
}

pub async fn construct_block(base_url: &str) -> anyhow::Result<()> {
    let url = format!("{}/block-builder/construct-block", base_url);
    send_get_request(&url).await?;
    Ok(())
}

pub async fn post_block(base_url: &str) -> anyhow::Result<()> {
    let url = format!("{}/block-builder/post-block", base_url);
    send_get_request(&url).await?;
    Ok(())
}
