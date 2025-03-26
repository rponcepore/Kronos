//! ApiCall.tsx

import { KronosRequest } from "../types/KronosRequest";
import { KronosResponse } from "../types/KronosResponse";

// This file defines the API calls and responses that we can expect

export async function kronosApiCall(request: KronosRequest): Promise<KronosResponse> {
    try {
        console.log('Making API request to:', 'http://localhost:8000/api');
        console.log('Request body:', JSON.stringify(request));
        
        const response = await fetch('http://localhost:8000/api', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Accept': 'application/json'
            },
            body: JSON.stringify(request),
        });

        console.log('Response status:', response.status);
        console.log('Response headers:', [...response.headers.entries()]);

        if (!response.ok) {
            const errorText = await response.text();
            console.error('Error response body:', errorText);
            throw new Error(`Network response was not ok: ${response.status} ${response.statusText}`);
        }

        const kronosResponse: KronosResponse = await response.json(); 
        console.log('Successful response:', kronosResponse); 

        return kronosResponse;

    } catch (error) {
        console.error('There was an error fetching the plans:', error);
        throw error; // Rethrow so calling functions can handle it
    }
}