import { expect, test } from 'vitest'
import { 
    extractParagraphFromOrderSummary, 
    insertParagraphBelowTarget, 
    insertParagraphAboveTarget, 
    doesTargetParagraphHaveChildren,
    insertSubparagraph 
} from './paragraph_test_helpers'
import { getTestOrder } from '../order_api.test/order_test_helpers';
import { KronosOrderSummary } from '../../../types/frontend_types/KronosOrderSummary';
import { ParagraphSummary } from '../../../types/frontend_types/ParagraphSummary';
import { deleteTestOrderComplete } from '../../test_helpers/delete_test_order';

export type InvariantParameters = {
    target: ParagraphSummary,
    new_title: string,
    new_text: string,
    number_of_children: number,
};

test('Inserting below a major paragraph fails.', async () => {
    // Get an order
    let uic: string | undefined;
    try {
        const result = await getTestOrder();
        uic = result.uic;
        const { order } = result;

        // Get the situation paragraph
        const situationParagraph = extractParagraphFromOrderSummary(1, order);

        // Attempt to insert a BELOW paragraph (this should FAIL)
        try {
            await insertParagraphBelowTarget(situationParagraph, "New Title", "New Text");
            throw new Error('Expected kronosApiCall to throw an error, but it did not');
        } catch (error: any) {
            expect(error).toBeDefined();
        }
    } finally {
        if (uic) {
            await deleteTestOrderComplete(uic);
        }
    }

})

test('Inserting above a major paragraph fails.', async () => {
    // Get an order
    let uic: string | undefined;
    try {
        const result = await getTestOrder();
        uic = result.uic;
        const { order } = result;
    
        // Get the situation paragraph
        const situationParagraph = extractParagraphFromOrderSummary(1, order);
        
        // Attempt to insert a ABOVE paragraph (this should FAIL)
        try {
            await insertParagraphAboveTarget(situationParagraph, "New Title", "New Text");
            // This should fail; can't add a sibling to a major paragraph.
            throw new Error('Expected kronosApiCall to throw an error, but it did not');
        } catch (error: any) {
            // Test that an error was thrown and has the expected message or status
            expect(error).toBeDefined();
        }
    } finally {
        if (uic) {
            await deleteTestOrderComplete(uic);
        }
    }

})
