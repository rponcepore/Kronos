//! create_plan.test.tsx

import { assert, expect, test } from 'vitest'
import { KronosRequest, KronosRequestBuilder } from '../../../types/networking_types/KronosRequest.tsx'
import { kronosApiCall } from '../../../helper_methods/ApiCall.tsx'
import { KronosResponse } from '../../../types/networking_types/KronosResponse.tsx'
import { KronosApiMethod } from '../../../types/networking_types/KronosApiMethodEnums.tsx'
import { AdminRequest } from '../../../types/networking_types/AdminRequest.tsx'
import { count_units_in_db } from '../../test_helpers/count_units_in_db.tsx'
import { create_test_unit, generate_name } from '../../test_helpers/create_test_unit.tsx'
import { delete_test_unit } from '../../test_helpers/delete_test_unit.tsx'
import { UnitSummary } from '../../../types/frontend_types/UnitSummary.tsx'
import { PlanRequest } from '../../../types/networking_types/PlanRequest.tsx'

test('create_plan', async () => {

    const test_unit : UnitSummary = await create_test_unit();

    const plan_req : PlanRequest = {
        plan_name : generate_name("test_plan_", 10),
        classification: null,
        plan_id: null
    };

    const kronos_request = new KronosRequestBuilder()
        .setUic(test_unit.data.uic)
        .setApiMethod(KronosApiMethod.create_plan)
        .setPlanRequest(plan_req)
        .build()
    
    const response = await kronosApiCall(kronos_request);
    expect(response).not.toBeNull();
    expect(response.plans_vec).not.toBeNull();
    expect(response.plans_vec!.length).toBe(1);

    const plan_summary = response.plans_vec![0];
    
    const response2 = await delete_test_unit(test_unit.data.uic);
    expect(response2).not.toBeNull();
    console.log(response2);
    expect(response2.rows_affected).not.toBeNull();
    expect(response2.rows_affected).toBe(1);
}) // src/tests/api_tests/unit_api.test/create_unit.test.tsx