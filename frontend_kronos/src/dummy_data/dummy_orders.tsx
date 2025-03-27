import { Plan } from "../types/Plan";
import { KronosOrder } from "../types/KronosOrder";
import { Paragraph } from "../types/Paragraph";
import { KronosResponse } from "../types/KronosResponse";
import { OrderKind } from "../types/enums/OrderKind";

const dummyData: KronosResponse = {
  kronos_request: {
    action: "load",
    unit: "1st Brigade",
    plan_id: null,
    order_id: null,
    paragraph_id: null,
    task_id: null
  },
  plans_vec: [
    {
      id: 101,
      unit: "1st Brigade",
      parent_plan: null,
      fiscal_year: 2025,
      serial_number: 1,
      classification: "SECRET",
      name: "Operation Iron Shield"
    }
  ],
  orders_vec: [
    {
      id: 1,
      parent_plan: 101,
      order_type: OrderKind.OPORD,
      serial_number: 1,
      is_published: false,
      derived_from: null
    },
    {
      id: 2,
      parent_plan: 101,
      order_type: OrderKind.WARNORD,
      serial_number: 1,
      is_published: true,
      derived_from: null
    },
    {
      id: 3,
      parent_plan: 101,
      order_type: OrderKind.FRAGORD,
      serial_number: 2,
      is_published: true,
      derived_from: 1
    },
    {
      id: 4,
      parent_plan: 101,
      order_type: OrderKind.FRAGORD,
      serial_number: 3,
      is_published: true,
      derived_from: 1
    }
  ],
  paragraphs_vec: [
    // OPORD 25-01-01
    { id: 1, order: 1, parent_paragraph: null, is_major: true, ordinal_sequence: 1, title: "Situation", text: "Enemy forces are active near AO Falcon. Coalition forces must deploy and deter aggression.", indent_level: 0 },
    { id: 2, order: 1, parent_paragraph: null, is_major: true, ordinal_sequence: 2, title: "Mission", text: "Deploy all brigade elements to AO Falcon NLT 15JAN2025 to begin deterrence operations.", indent_level: 0 },
    { id: 3, order: 1, parent_paragraph: null, is_major: true, ordinal_sequence: 3, title: "Execution", text: "Movement begins 07JAN2025. Use rail and air transport. Priority to 1st BCT.", indent_level: 0 },
    { id: 4, order: 1, parent_paragraph: null, is_major: true, ordinal_sequence: 4, title: "Sustainment", text: "Sustainment BDE provides all logistical support. Establish FOBs at designated coordinates.", indent_level: 0 },
    { id: 5, order: 1, parent_paragraph: null, is_major: true, ordinal_sequence: 5, title: "Command and Signal", text: "Command post established at Fort Campbell. Use encrypted comms via SINCGARS.", indent_level: 0 },

    // WARNO 25-01-01
    { id: 6, order: 2, parent_paragraph: null, is_major: true, ordinal_sequence: 1, title: "Situation", text: "Increased enemy movement detected near AO Falcon.", indent_level: 0 },
    { id: 7, order: 2, parent_paragraph: null, is_major: true, ordinal_sequence: 2, title: "Mission", text: "Brigade elements prepare for rapid deployment within 72 hours notice.", indent_level: 0 },
    { id: 8, order: 2, parent_paragraph: null, is_major: true, ordinal_sequence: 3, title: "Execution", text: "Unit prep starts immediately. Rehearsals 04JAN2025.", indent_level: 0 },
    { id: 9, order: 2, parent_paragraph: null, is_major: true, ordinal_sequence: 4, title: "Sustainment", text: "Coordinate with Sustainment BDE for pre-deployment supplies.", indent_level: 0 },
    { id: 10, order: 2, parent_paragraph: null, is_major: true, ordinal_sequence: 5, title: "Command and Signal", text: "S3 operations will distribute further instructions via OPORD 25-01-01.", indent_level: 0 },

    // FRAGO 25-01-02
    { id: 11, order: 3, parent_paragraph: null, is_major: true, ordinal_sequence: 1, title: "Situation", text: "Severe weather has delayed rail logistics by 48 hours.", indent_level: 0 },
    { id: 12, order: 3, parent_paragraph: null, is_major: true, ordinal_sequence: 2, title: "Mission", text: "Adjust sustainment delivery schedule accordingly.", indent_level: 0 },
    { id: 13, order: 3, parent_paragraph: null, is_major: true, ordinal_sequence: 3, title: "Execution", text: "Deliver supplies NLT 12JAN2025. Air transport authorized for high priority items.", indent_level: 0 },
    { id: 14, order: 3, parent_paragraph: null, is_major: true, ordinal_sequence: 4, title: "Sustainment", text: "All battalion S4s report updates to SPO by 11JAN2025.", indent_level: 0 },
    { id: 15, order: 3, parent_paragraph: null, is_major: true, ordinal_sequence: 5, title: "Command and Signal", text: "All changes coordinated through brigade SPO.", indent_level: 0 },

    // FRAGO 25-01-03
    { id: 16, order: 4, parent_paragraph: null, is_major: true, ordinal_sequence: 1, title: "Situation", text: "MSR Copperhead is impassable due to structural damage.", indent_level: 0 },
    { id: 17, order: 4, parent_paragraph: null, is_major: true, ordinal_sequence: 2, title: "Mission", text: "Reroute 1st BCT convoy via alternate MSR Python.", indent_level: 0 },
    { id: 18, order: 4, parent_paragraph: null, is_major: true, ordinal_sequence: 3, title: "Execution", text: "Convoy departs 13JAN2025 0500. Security escorts updated.", indent_level: 0 },
    { id: 19, order: 4, parent_paragraph: null, is_major: true, ordinal_sequence: 4, title: "Sustainment", text: "Fuel and rations reallocated at RP Alpha.", indent_level: 0 },
    { id: 20, order: 4, parent_paragraph: null, is_major: true, ordinal_sequence: 5, title: "Command and Signal", text: "Report position hourly via satellite radio.", indent_level: 0 }
  ],
  units_vec: null
};

export default dummyData;


