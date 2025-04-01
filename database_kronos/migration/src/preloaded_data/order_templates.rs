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
                        title: "NOTE TO PLANNERS",
                        text: "This format is pulled from FM 5-0 Planning and Orders Production, Appendix D. Not all paragraphs are relevant to your organization. Remove those that do not apply; do not fill them just to fill them. This paragraph can be deleted after it is read.",
                        subparagraphs: None,
                    },
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
                    MigrationParagraph {
                        title: "Concept of the Operation",
                        text: "Describe the combination of offense, defense, and stability operations (or defense support of civil authorities tasks) and the sequence of actions the force will use to achieve the operation's end state. Use components of the operational framework (deep, close, rear, and support areas; decisive, shaping, and sustaining operations; and main and supporting efforts) as required. If the concept of operations is phased, describe each phase in a subparagraph. Label these subparagraphs as \"Phase\" followed by the appropriate Roman numeral, for example, “Phase I.” If the operation is phased, all paragraphs and subparagraphs of the base order and all attachments must mirror the phasing established in the concept of operations. Refer to Appendix 2 (Operations Overlay) to Annex C (Operations) and other attachments to Annex C (Operations) as required.",
                        subparagraphs: Some(vec![
                            MigrationParagraph {
                                title: "NOTE TO PLANNERS",
                                text: "Not all operations deserve to have phases. Simple operations can be described in a single paragraph; i.e., the Company barbeque. Avoid the temptation to add unnecessary structure/paragraphs for the sake of paragraphs, which makes the order less clear, not more clear. This note can be deleted after it is read.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Phase I",
                                text: "Example: Preparation and Staging",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Phase II",
                                text: "Example: Infiltration",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Phase III",
                                text: "Example: Attack",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Phase IV",
                                text: "Example: Exploitation",
                                subparagraphs: None,
                            },
                        ]),
                    },
                    MigrationParagraph {
                        title: "Scheme of Maneuver",
                        text: "Describe the employment of maneuver units in accordance with the concept of operations. Identify the type of offensive or defensive operations (for example, movement to contact, area defense) and primary stability tasks when conducting stability operations. Provide the primary tasks of maneuver units, including security operations, and the purpose of each. Identify and include priorities for the reserve and reaction forces. If the operation is phased, identify the main effort by phase. Refer to Annex C (Operations) as required.",
                        subparagraphs: Some(vec![
                            MigrationParagraph {
                                title: "Phase I",
                                text: "Explain the specifics of this phase, and specifically transitions. Frequently, the simplest way to structure the phases is as Trigger/Action Sequences.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Phase II",
                                text: "Refer to the graphic control measures you have established. Good GCM's follow a scheme, frequently prescribed by the BDE Planning SOP",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Phase III",
                                text: "For phases with manuever involved, along with trigger/action, focus on sequencing across the unit. For example, shared prowords, GCM's, etc., that signal across the unit that an action has occurred.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Phase IV",
                                text: "As an operation continues, the level of certainty on the state of the battlefield decays. Good plans have logical answers to overwhelming success, moderate success, failure, and catastrophic failure. Building in these plans also helps the author logically consider the vulnerabilities inherent to the plan.",
                                subparagraphs: None,
                            },
                        ]),
                    },
                    MigrationParagraph {
                        title: "Scheme of Intelligence",
                        text: "Describe how the commander envisions intelligence supporting the concept of operations. Include the priority of effort for situation development, targeting, and assessment. State the priority of intelligence support to units and areas. Refer to Annex B (Intelligence) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Scheme of Information Collection",
                        text: "Describe how the commander intends to use reconnaissance missions and surveillance tasks to support the concept of operations. Include the primary reconnaissance objectives. Refer to Annex L (Information Collection) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Scheme of Fires",
                        text: "Describe how the commander intends to use fires to support the concept of operations. State the priorities for, allocation of, and restrictions on fires. Use subparagraphs for field artillery support, air support, air and missile defense, and cyberspace electromagnetic activities, as required. Refer to Annex C (Operations) and Annex D (Fires) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Scheme of Protection",
                        text: "Describe how the commander envisions protection supporting the concept of operations. Include the priorities of protection by unit and area, including survivability and local security measures. Address area security, including security for routes, bases, and critical infrastructure. Identify reaction forces and their priorities. Use subparagraphs for protection warfighting related tasks (for example, chemical, biological, radiological, and nuclear (CBRN) operations; explosive ordnance disposal support; personnel recovery; detention operations; antiterrorism measures; and police operations) as required. Refer to Annex E (Protection) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Scheme of Engineering",
                        text: "State the overall scheme of engineering in support of the concept of operations. Describe key mobility, countermobility, survivability, and general engineering tasks. Include priorities of support by unit or area. Refer to Annex G (Engineer) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Scheme of Information",
                        text: "Describe how the commander intends to use information and information operations to support the concept of operations. State the priorities for, allocation of, and restrictions on information capabilities. Use subparagraphs to describe key information operations, cyberspace electromagnetic activities, operations security, electromagnetic warfare, and military information support operations tasks as required. Refer to Annex C (Operations) as required.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Tasks to Subordinate Units",
                        text: "Note: This software provides an interface for assigning tasks. When utilized correctly, it will automatically create a task tracker and Troops-to-Task shell for use by the operations staff. Tasks must be explicitly allocated by unit. As the task populates, the software will aggregate tasks by unit.\n\n State the tasks assigned to each unit that reports directly to the headquarters issuing the order. Each task must include who (the subordinate unit assigned the task), what (the task itself), when, where, and why (purpose). Use a separate subparagraph for each unit. List units in task organization sequence.",
                        subparagraphs: None,
                    },
                    MigrationParagraph {
                        title: "Coordinating Instructions",
                        text: "Note: This software provides an interface for assigning tasks. When utilized correctly, it will automatically create a task tracker and Troops-to-Task shell for use by the operations staff. \n\n List only instructions and tasks applicable to two or more units not covered in unit standard operating procedures (SOPs).",
                        subparagraphs: Some(vec![
                            MigrationParagraph {
                                title: "Operational Timeline",
                                text: "List time or condition when the OPORD becomes effective. List critical times. Refer to Appendix 3 (Decision Support Products) to Annex C (Operations) as required.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Commander's Critical Information Requirements",
                                text: "CCIR is composed of PIR (Priority Information Requirements) and FFIR (Friendly Force Information Requirements). Per FM 3-98, there are standards for development of CCIR. \nCCIR are tied directly to the scheme of maneuver and decision points. CCIR that do not support decison making are trivial. \nCCIR is limited to critical intelligence and combat information needs. \nCCIR include the latest time the information is of value (LTIOV) to aid in timely reporting and decision making.\nCCIR is continuously updated, especially as LTIOV passes.",
                                subparagraphs: Some(vec![
                                    MigrationParagraph {
                                        title: "Priority Information Requirements",
                                        text: "PIR: What the Commander needs to know about the enemy and the environment.  Normally, PIRs are tied to either an NAI or a TAI. PIRs become the central focus for the Cavalry organizations conducting the BCT's reconnaissance and security operations. LTIOV is the time that the planning Staff needs information collected in order to answer PIR and inform the commander to make a decision. Therefore, LTIOV is the primary planning factor when determining the reconnaissance tempo for information collection operations and will dictate the level of detail to be collected and the stealth required",
                                        subparagraphs: None,
                                    },
                                    MigrationParagraph {
                                        title: "FFIR",
                                        text: "FFIR: What the Commander needs to know about his own force, and other friendly forces/supporting capabilities. FFIRs identify the information about the mission, troops and support available, and time available for friendly forces that the commander considers most important.",
                                        subparagraphs: None,
                                    },
                                ]),
                            },
                            MigrationParagraph {
                                title: "Essential Elements of Friendly Information",
                                text: "List essential elements of friendly information (EEFIs). Essential element of friendly information is a critical aspect of a friendly operation that, if known by a threat would subsequently compromise, lead to failure, or limit success of the operation and therefore should be protected from enemy detection (ADP 6-0).",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Fires Support Coordination Measures",
                                text: "List critical fire support coordination or control measures.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Airspace Coordinating Measures",
                                text: "List critical airspace coordinating or control measures.",
                                subparagraphs: None,
                            },
                            MigrationParagraph {
                                title: "Rules of Engagement",
                                text: "List rules of engagement. Refer to Appendix 11 (Rules of Engagement) to Annex C (Operations) as required.",
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
