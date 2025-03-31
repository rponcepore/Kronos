//! ApiCall.tsx

import { KronosRequest } from "../types/networking_types/KronosRequest";
import { KronosResponse } from "../types/networking_types/KronosResponse";

// This file defines the API calls and responses that we can expect

export async function kronosApiCall(request: KronosRequest): Promise<KronosResponse> {
    try {
        const response = await fetch('http://localhost:8000/api', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(request),
        });

        if (!response.ok) {
            const errorMessage = await response.text(); // or response.json() if expecting JSON
            throw new Error(errorMessage);
        }

        const kronosResponse: KronosResponse = await response.json(); 
        console.log(kronosResponse); 

        return kronosResponse;

    } catch (error) {
        console.error('There was an error fetching the plans:', error);
        throw error; // Rethrow so calling functions can handle it
    }
}