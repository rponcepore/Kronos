//! oder_api.test.tsx
import { expect, test } from 'vitest'
import { getTestOrder } from './order_test_helpers'
import { deleteTestOrderComplete } from '../../test_helpers/delete_test_order';

test('Create an order API', async () => {
    // Create an order. 
    let {uic, order}  = await getTestOrder();
    expect(order).not.toBeNull();
    expect(order).not.toBeUndefined();

    // Now clean up the mess. 
    let result = await deleteTestOrderComplete(uic);
    let count : number = result.admin_vec![0].rows_affected!;
    console.log(`Order delete affected ${count} rows`);
})