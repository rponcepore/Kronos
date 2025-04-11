//! create_unit.test.tsx

import { expect, test } from 'vitest'
import { count_units_in_db } from '../../test_helpers/count_units_in_db.tsx'
import { create_test_unit } from '../../test_helpers/create_test_unit.tsx'
import { delete_test_unit } from '../../test_helpers/delete_test_unit.tsx';
import { UnitSummary } from '../../../types/frontend_types/UnitSummary.tsx';
test('Create a unit', async () => {
    //let startingCount = await count_units_in_db();
    // ignore the result, but create a unit
    const test_unit : UnitSummary = await create_test_unit();
    //let endingCount = await count_units_in_db();

    //expect(startingCount + 1).toBe(endingCount); //!This only works if no other db access is going on concurrently.

    const response2 = await delete_test_unit(test_unit.data.uic);
    expect(response2).not.toBeNull();
    console.log(response2);
    expect(response2.rows_affected).not.toBeNull();
    expect(response2.rows_affected).toBe(1);
}) // src/tests/api_tests/unit_api.test/create_unit.test.tsx