//! fragord_data.rs

// FRAGORD Paragraphs:

/*
pub enum Paragraph {
    Table,
    Id,
    KronosOrder,
    ParentParagraph,
    IsMajor,
    OrdinalSequence,
    Title,
    Text,
    IndentLevel, // 0 is base for paragraphs 1-5.
}
*/

const MAJOR_PARAGRAPH_ARRAY: &[(i32, i32, &str, &str)] = &[
    (0, 1, "Situation", "The situation paragraph describes the conditions of the operational environment that impact operations in the following subparagraphs:"),
    (0, 2, "Mission", "State the unit\'s mission—a short description of who, what (task), when, where, and why (purpose) that clearly indicates the action to be taken and the reason for doing so."),
    (0, 3, "Execution", "Describe how the commander intends to accomplish the mission in terms of the commander\'s intent, concept of operations, schemes of support, tasks to subordinate units, and coordinating instructions in the subparagraphs below"),
    (0, 4, "Sustainment", "Describe the concept of sustainment, including priorities of sustainment by unit or area. Include instructions for administrative movements, deployments, and transportation—or references to applicable appendixes—if appropriate. Use the following subparagraphs to provide the scheme of support for logistics, financial management, personnel, and health service support. Provide detailed instructions for each sustainment subfunctions in appendices to Annex F (Sustainment)."),
    (0, 5, "Command and Signal", "Include, at a minimum, succession of command."),
];

pub fn get_header_paragraph_vec() -> Vec<(i32, i32, &'static str, &'static str)> {
    MAJOR_PARAGRAPH_ARRAY.to_vec()
}

pub struct MigrationParagraph {
    title: &'static str,
    text: &'static str,
    subparagraphs: Option<Vec<MigrationParagraph>>,
}

pub struct OrderTemplate {
    paragraphs: Vec<MigrationParagraph>,
}

pub fn default_order_template() -> OrderTemplate {
    OrderTemplate {
        paragraphs: vec![
            MigrationParagraph {
                title: "Situation",
                text: "The situation paragraph describes the conditions of the operational environment that impact operations in the following subparagraphs:",
                subparagraphs: None,
            },
            MigrationParagraph {
                title: "Mission",
                text: "State the unit\'s mission—a short description of who, what (task), when, where, and why (purpose) that clearly indicates the action to be taken and the reason for doing so.",
                subparagraphs: None,
            },
            MigrationParagraph {
                title: "Execution",
                text: "Describe how the commander intends to accomplish the mission in terms of the commander\'s intent, concept of operations, schemes of support, tasks to subordinate units, and coordinating instructions in the subparagraphs below",
                subparagraphs: Some(vec![
                    MigrationParagraph {
                        title: "Commander's Intent",
                        text: "Describe what the force must do and conditions the force must establish with respect to the enemy, terrain, and civil considerations that represent the desired end state. The commander's intent normally includes:",
                        subparagraphs: None,
                    }
                ]),
            },
            MigrationParagraph {
                title: "Sustainment",
                text: "Describe the concept of sustainment, including priorities of sustainment by unit or area. Include instructions for administrative movements, deployments, and transportation—or references to applicable appendixes—if appropriate. Use the following subparagraphs to provide the scheme of support for logistics, financial management, personnel, and health service support. Provide detailed instructions for each sustainment subfunctions in appendices to Annex F (Sustainment).",
                subparagraphs: None,
            },
            MigrationParagraph {
                title: "Command and Signal",
                text: "At a minimum, include succession of command.",
                subparagraphs: None,
            },
        ],
    }
}
