//! count_units_in_db.tsx

import { expect, test } from 'vitest'
import { KronosRequest } from '../../types/networking_types/KronosRequest'
import { kronosApiCall } from '../../helper_methods/ApiCall'
import { KronosResponse } from '../../types/networking_types/KronosResponse.tsx'
import { KronosApiMethod } from '../../types/networking_types/KronosApiMethodEnums'
import { AdminRequest } from '../../types/networking_types/AdminRequest.tsx'

export async function count_units_in_db(): Promise<number> {
    // Check how many plans are in the database:
    const admin_request: AdminRequest = {
        admin_action: "count_units",
    };
    
    const req: KronosRequest = {
        api_method: KronosApiMethod.admin_request,
        uic: "WJH8AA",
        plan_request: null,
        order_request: null,
        paragraph_request: null,
        task_request: null,
        admin_request: admin_request,
        unit_request: null,
    };

    let response : KronosResponse = await kronosApiCall(req);
    console.log(response)

    expect(response).not.toBeNull();
    expect(response.admin_vec).not.toBeNull();
    expect(response.admin_vec!.length).toBeGreaterThanOrEqual(1); // at least one result
    let admin_response = response.admin_vec![0];
    expect(admin_response.number_response).not.toBeNull();
    expect(admin_response.number_response).toBeGreaterThanOrEqual(0);
    let result : number = admin_response.number_response!;
    return result;
}