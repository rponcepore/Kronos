import { expect, test } from 'vitest'
import { 
    extractParagraphFromOrderSummary, 
    insertParagraphBelowTarget, 
    insertParagraphAboveTarget, 
    doesTargetParagraphHaveChildren,
    insertSubparagraph, 
    checkSubparagraphsAreInOrder
} from './paragraph_test_helpers'
import { getTestOrder } from '../order_api.test/order_test_helpers';
import { KronosOrderSummary } from '../../../types/frontend_types/KronosOrderSummary';
import { ParagraphSummary } from '../../../types/frontend_types/ParagraphSummary';
import { deleteTestOrderComplete } from '../../test_helpers/delete_test_order';
import { InvariantParameters } from './insert_sibling_paragraph.test';



test('Inserting onto a subparagraph succeeds.', async () => {
    // Get an order
    let uic: string | undefined;
    try {
        const result = await getTestOrder();
        uic = result.uic;
        const { order } = result;
        
        // Get the situation paragraph
        const situationParagraph = extractParagraphFromOrderSummary(1, order);
        
        // Make sure the situation paragraph has subparagraphs
        if (doesTargetParagraphHaveChildren(situationParagraph) != true) {
            throw new Error(
                `Expected the paragraph id: ${situationParagraph.data.id}, ` +
                `${situationParagraph.data.title}:${situationParagraph.data.text} ` +
                `to have children, but it does not.`
            );
        }
        // Invariants:
        const invariant : InvariantParameters = {
            target : situationParagraph,
            new_title: "New Title",
            new_text: "New Text",
            number_of_children: situationParagraph.subparagraphs!.length,
        }
        // Insert a subparagraph 
        // Remember that this will return teh paragraph summary for the target
        // paragraph, or the parent of the inserted subparagraph.
        const sub_paragraph_response = await insertSubparagraph(situationParagraph, invariant.new_title, invariant.new_text);
        expect(sub_paragraph_response).not.toBeNull();
        expect(sub_paragraph_response.paragraphs_vec).not.toBeNull();
        expect(sub_paragraph_response.paragraphs_vec![0]).not.toBeNull();
        expect(sub_paragraph_response.paragraphs_vec!.length).toBe(1);
        
        const parent_of_new_subparagraph = sub_paragraph_response.paragraphs_vec![0];
        expect(parent_of_new_subparagraph.data.id).toBe(situationParagraph.data.id);
        expect(parent_of_new_subparagraph.data.title).toBe(situationParagraph.data.title);
        expect(parent_of_new_subparagraph.data.text).toBe(situationParagraph.data.text);
        
        expect(parent_of_new_subparagraph.subparagraphs).not.toBeNull();
        expect(parent_of_new_subparagraph.subparagraphs.length).toBeGreaterThanOrEqual
        expect(parent_of_new_subparagraph.subparagraphs).not.toBeNull();

        console.dir(parent_of_new_subparagraph, { depth: null });

        // This is the big enchilada; did we actually insert?
        expect(parent_of_new_subparagraph.subparagraphs!.length).toBe(invariant.number_of_children + 1);
        const last_index = parent_of_new_subparagraph.subparagraphs!.length - 1;
        const inserted_subparagraph = parent_of_new_subparagraph.subparagraphs![last_index];
        expect(inserted_subparagraph.data.text).eq(invariant.new_text);
        expect(inserted_subparagraph.data.title).eq(invariant.new_title)

        if (!checkSubparagraphsAreInOrder(parent_of_new_subparagraph)) {
            throw new Error(`Subparagraphs are out of order: ${JSON.stringify(parent_of_new_subparagraph, null, 2)}`);

        }

        // Insert a BELOW paragraph on the subparagraph
        
        // Insert an ABOVE paragraph on the BELOW paragraph
        
        // Check the order
        
        // Now, brass balls, delete TERRY and ensure that it cascade deletes.
        
        // Check that we're back to normal.
    } finally {
        if (uic) {
            await deleteTestOrderComplete(uic);
        }
    }
})