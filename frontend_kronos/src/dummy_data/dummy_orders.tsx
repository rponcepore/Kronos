import { Order } from '../types/Order.tsx'
import { Plan } from '../types/Plan.tsx';
import { Paragraph } from '../types/Paragraph.tsx';

const plans: Plan[] = [
    {
        id: 1,
        unit: "1st Brigade",
        parent_plan: null,
        fiscal_year: 2025,
        serial_number: 101,
        classification: "SECRET",
        name: "Operation Iron Shield"
    }
];

const orders: Order[] = [
    {
        id: 1,
        parent_plan: 1,
        order_type: "OPORD",
        serial_number: 1,
        is_published: true,
        derived_from: null
    },
    {
        id: 2,
        parent_plan: 1,
        order_type: "FRAGO",
        serial_number: 2,
        is_published: false,
        derived_from: null
    }
];

const paragraphs: Paragraph[] = [
    {
        id: 1,
        order: 1,
        parent_paragraph: null,
        is_major: true,
        ordinal_sequence: 1,
        title: "Situation",
        text: "The 1st Brigade will defend sector A.",
        indent_level: 0
    },
    {
        id: 2,
        order: 1,
        parent_paragraph: null, 
        is_major: true,
        ordinal_sequence: 2,
        title: "Mission",
        text: "2-27 IN BN conducts mobile defense IVO OBJ BEARS IOT stop 3/357th MTZD BTG from encircling TF LIGHTNING.",
        indent_level: 0
    },
    {
        id: 11,
        order: 1,
        parent_paragraph: 1, 
        is_major: false,
        ordinal_sequence: 1,
        title: "Enemy Forces",
        text: "Intel indicates enemy movement along route B.",
        indent_level: 1
    },
    {
        id: 111,
        order: 1,
        parent_paragraph: 11, // Subparagraph of paragraph 11
        is_major: false,
        ordinal_sequence: 1,
        title: "Composition",
        text: "Enemy consists of mechanized and infantry units.",
        indent_level: 2
    },
    {
        id: 112,
        order: 1,
        parent_paragraph: 11, 
        is_major: false,
        ordinal_sequence: 2,
        title: "Disposition",
        text: "ENY lies along route BUDWEISER",
        indent_level: 0
    },
    {
        id: 3,
        order: 1,
        parent_paragraph: null, 
        is_major: true,
        ordinal_sequence: 3,
        title: "Execution",
        text: "This is the execution paragraph",
        indent_level: 0
    },
    {
        id: 4,
        order: 1,
        parent_paragraph: null, 
        is_major: true,
        ordinal_sequence: 4,
        title: "Sustainment",
        text: "Take CL 1, and 5 from the dead bodies of the enemy.",
        indent_level: 0
    },
    {
        id: 5,
        order: 1,
        parent_paragraph: null, 
        is_major: true,
        ordinal_sequence: 5,
        title: "Command and Signal",
        text: "The radios are broken, the CO is in the wrong grid square.",
        indent_level: 0
    },

];
