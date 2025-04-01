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
    pub title: &'static str,
    pub text: &'static str,
    pub subparagraphs: Option<Vec<MigrationParagraph>>,
}

pub struct OrderTemplate {
    pub paragraphs: Vec<MigrationParagraph>,
}

pub fn default_order_template() -> OrderTemplate {
    OrderTemplate {
        paragraphs: vec![
            MigrationParagraph {
                title: "Situation",
                text: "",
                subparagraphs: Some(vec![
                    MigrationParagraph {
                        title: "Area of Interest",
                        text: "Describe the area of interest which includes the area of influence in all five domains and information aspects. Refer to Annex B (Intelligence) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Area of Operations",
                        text: "Describe the area of operations. Refer to the appropriate map by its subparagraph under references, for example, ‘Map, reference (a).’ See Appendix 2 (Operation Overlay) to Annex C (Operations) as required.",
                        subparagraphs: Some(vec![
                            MigrationParagraph {
                                title: "Terrain",
                                text: "Describe the aspects of terrain that impact operations. Refer to Annex B (Intelligence) as required.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Weather",
                                text: "Describe the aspects of weather that impact operations. Refer to Annex B (Intelligence) as required.",
                                subparagraphs: None,
                            },
                        ]),
                    },
                    MigrationParagraph {
                        title: "Enemy Forces",
                        text: "Identify enemy forces and appraise their general capabilities. Describe the enemy's composition, disposition, location, strength, and probable courses of action. Identify adversaries and known or potential threats within the area of operations. Refer to Annex B (Intelligence) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Friendly Forces",
                        text: "Briefly identify the mission of friendly forces in the following subparagraphs:",
                        subparagraphs: Some(vec![
                            MigrationParagraph {
                                title: "Higher Headquarters Two Levels Up",
                                text: "The higher headquarters mission and commander's intent two echelons above.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Higher Headquarters One Level Up",
                                text: "Identify the higher headquarters mission, commander's intent, and concept of operations one echelon above.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Missions of Adjacent Units",
                                text: "Identify and state the missions of adjacent units and other units whose actions have a significant impact on the issuing headquarters.",
                                subparagraphs: None,
                            },
                        ]),
                    },
                    MigrationParagraph {
                        title: "Interagency, Intergovernmental, and Nongovernmental Organizations",
                        text: "Identify and state the objectives or goals of those non-Department of Defense organizations that have a significant role within the area of operations. Refer to Annex V (Interagency Coordination) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Civil Considerations",
                        text: "Describe the critical aspects of the civil situation that impact operations. Refer to Appendix 1 (Intelligence Estimate) to Annex B (Intelligence) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Assumptions",
                        text: "List assumptions used in the development of the OPLAN or OPORD.",
                        subparagraphs: None,
                    }                    
                ]),
            },
            MigrationParagraph {
                title: "Mission",
                text: "State the unit's mission—a short description of who, what (task), when, where, and why (purpose) that clearly indicates the action to be taken and the reason for doing so.",
                subparagraphs: None,
            },
            MigrationParagraph {
                title: "Execution",
                text: "",
                subparagraphs: Some(vec![
                    MigrationParagraph {
                        title: "Commander\'s Intent",
                        text: "Describe what the force must do and conditions the force must establish with respect to the enemy, terrain, and civil considerations that represent the desired end state. The commander's intent normally includes:",
                        subparagraphs: Some(vec![
                            MigrationParagraph {
                                title: "Purpose",
                                text: "An expanded description of the operation's purpose.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Key Tasks",
                                text: "Significant activities the force as a whole must perform to achieve the desired end state.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "End State",
                                text: "A description of the desired future conditions that represent success.",
                                subparagraphs: None,
                            },
                        ]),
                    },
                ]),
            },
            MigrationParagraph {
                title: "Sustainment",
                text: "",
                subparagraphs: Some(vec![
                    MigrationParagraph {
                        title: "Logistics",
                        text: "Refer to Annex F (Sustainment) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Personnel",
                        text: "Refer to Annex F (Sustainment) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Health Service Support",
                        text: "Refer to Annex F (Sustainment) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Financial Management",
                        text: "Refer to Annex F (Sustainment) as required.",
                        subparagraphs: None,
                    },
                ]),
            },
            MigrationParagraph {
                title: "Command and Signal",
                text: "",
                subparagraphs: Some(vec![
                    MigrationParagraph {
                        title: "Command",
                        text: "",
                        subparagraphs: Some(vec![
                            MigrationParagraph {
                                title: "Location of Commander and Key Leaders",
                                text: "State where the commander and key leaders intend to be during the operation, by phase if the operation is phased.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Succession of Command",
                                text: "State the succession of command if not covered in the unit's SOPs.",
                                subparagraphs: None,
                            },
                        ]),
                    },
                    MigrationParagraph {
                        title: "Command Posts",
                        text: "Describe the employment of command posts, including the location of each command post and its time of opening and closing, as appropriate.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Signal",
                        text: "Describe the concept of signal support, including location and movement of key signal nodes and critical electromagnetic spectrum considerations throughout the operation.",
                        subparagraphs: Some(vec![
                            MigrationParagraph {
                                title: "Communications PACE Plan",
                                text: "State the Primary, Alternate, Contingency, and Emergency methods for communication. If all these methods fail, you initiate the lost comms plan, in the next paragraph. ",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Lost Communications PACE Plan",
                                text: "State the Primary, Alternate, Contingency, and Emergency methods for recovering from lost communications. This plan assuems your normal PACE plan has failed. Generally, this plan involves (1) moving back to last location where communications wewre assured, (2) moving to predetermined RV for this plan, or other variants, to include continuing the mission, if appropriate. (i.e., if comms are lost halfway through a breach, you continue to execute the breach rather than attempt to reestablish communications.)",
                                subparagraphs: None,
                            },
                        ]),
                    },
                ]),
            },
        ],
    }
}
