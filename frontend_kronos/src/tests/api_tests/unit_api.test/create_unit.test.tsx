//! create_unit.test.tsx

import { expect, test } from 'vitest'
import { KronosRequest } from '../../../types/networking_types/KronosRequest.tsx'
import { kronosApiCall } from '../../../helper_methods/ApiCall.tsx'
import { KronosResponse } from '../../../types/networking_types/KronosResponse.tsx'
import { KronosApiMethod } from '../../../types/networking_types/KronosApiMethodEnums.tsx'
import { AdminRequest } from '../../../types/networking_types/AdminRequest.tsx'

test('Check how many units are in the database', async () => {
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
    
    let response = await kronosApiCall(req);
    console.log(response)
}) // src/tests/api_tests/unit_api.test/create_unit.test.tsx