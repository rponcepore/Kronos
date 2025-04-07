//! NetworkEnums.tsx

/*
 * For convenience, this is also where we will define the API
 * contracts. Reference this when writing calls to the backend.
 */

export enum KronosApiMethod {
    // Plans CRUD
    get_plans = "get_plans", 
    /* get_plans is written. It requires:
        @params: 
        KronosRequest.api_method = api_method.get_plans
        uic: String of the unit's UIC
        @returns:
        KronosResponse with:
            plan_vec: An array of all plans, as PlanSummary objects, for the provided uic
    */
    create_plan = "create_plan",
    /* create_plan is written. It requires:
        @params: 
        KronosRequest.api_method = api_method.create_plan
        uic: String of the unit's UIC
        plan_request: not null, with the following fields:
            plan_name: String of the plan name, i.e., "Lightning Forge"
        @returns:
        KronosResponse with:
            plan_vec: An array of all plans, as PlanSummary objects, for the provided uic
    */
    edit_plan = "edit_plan",
    delete_plan = "delete_plan",

    // Orders CRUD
    get_order = "get_order",
    /* get_order is written. It requires:
        @params: 
        KronosRequest.api_method = api_method.get_order
        uic: String of the unit's UIC
        order_request: not null, with the following fields:
            parent_plan_id: integer id of the parent plan.
        @returns:
        KronosResponse with:
            order_vec: An array (in this case with one member) of OrderSummary object, containing all subparagraphs.
    */
    create_order = "create_order",
    edit_order = "edit_order", // Probably never used.
    delete_order = "delete_order",

    // Paragraphs CRUD
    get_paragraph = "get_paragraph", // Probably never used
    insert_paragraph = "insert_paragraph", // Only to create subparagraphs; major paragraphs cannot be deleted/added 
    edit_paragraph = "edit_paragraph",
    delete_paragraph = "delete_paragraph",

    // Tasks CRUD
    get_tasks = "get_tasks", // Aggregate
    create_task = "create_task",
    edit_task = "edit_task", 
    delete_Task = "delete_task"
}