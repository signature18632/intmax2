import axios from "axios";

async function callServer(url: string): Promise<any> {
    try {
        const response = await axios.get<any>(url);
        return response.data;
    } catch (error) {
        if (axios.isAxiosError(error)) {
            throw new Error(`API Error: ${error.message}`);
        }
        throw error;
    }
}

// The following functions are not used in production. 
// They are used to explicitly change the state of the test server.

export async function syncValidityProof(baseUrl: string,): Promise<void> {
    const url = `${baseUrl}/block-validity-prover/sync`;
    await callServer(url);
}

export async function postEmptyBlock(baseUrl: string,): Promise<void> {
    const url = `${baseUrl}/block-builder/post-empty-block`;
    await callServer(url);
}

export async function constructBlock(baseUrl: string,): Promise<void> {
    const url = `${baseUrl}/block-builder/construct-block`;
    await callServer(url);
}

export async function postBlock(baseUrl: string,): Promise<void> {
    const url = `${baseUrl}/block-builder/post-block`;
    await callServer(url);
}