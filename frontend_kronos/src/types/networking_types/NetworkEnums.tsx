//! NetworkEnums.tsx

export enum KronosApiMethod {
    // Plans CRUD
    get_plans = "get_plans", //Note; done in aggregate
    create_plan = "create_plan",
    edit_plan = "edit_plan",
    delete_plan = "delete_plan",
    // Orders CRUD
    get_order = "get_order",
    create_order = "create_order",
    edit_order = "edit_order", // Probably never used.
    delete_order = "delete_order",
    // Paragraphs CRUD
    get_paragraph = "get_paragraph", // Probably never used
    create_paragraph = "create_paragraph", // Only to create subparagraphs; major paragraphs cannot be deleted/added 
    edit_paragraph = "edit_paragraph",
    delete_paragraph = "delete_paragraph",
    // Tasks CRUD
    get_tasks = "get_tasks", // Aggregate
    create_task = "create_task",
    edit_task = "edit_task", 
    delete_Task = "delete_task"
}