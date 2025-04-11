//! count_units.test.tsx


import { expect, test } from 'vitest'
import { KronosRequest } from '../../../types/networking_types/KronosRequest.tsx'
import { kronosApiCall } from '../../../helper_methods/ApiCall.tsx'
import { KronosResponse } from '../../../types/networking_types/KronosResponse.tsx'
import { KronosApiMethod } from '../../../types/networking_types/KronosApiMethodEnums.tsx'
import { AdminRequest } from '../../../types/networking_types/AdminRequest.tsx'
import { count_units_in_db } from '../../test_helpers/count_units_in_db.tsx'

test('Check how many units are in the database', async () => {
    // Check how many plans are in the database:
    await count_units_in_db();
}) // src/tests/api_tests/unit_api.test/create_unit.test.tsx

