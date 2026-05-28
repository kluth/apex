// === ARMS: HIGH-FIDELITY MODULAR ===

// SCAPULAE (Shoulder Blades)
bone L_Scapula { mass = 1.0 kg; position = (-0.15, 1.45, -0.1); }
bone R_Scapula { mass = 1.0 kg; position = (0.15, 1.45, -0.1); }
joint JLS { source = T2; target = L_Scapula; }
joint JRS { source = T2; target = R_Scapula; }

// LEFT ARM
bone L_Clavicle { mass = 0.5 kg; position = (-0.1, 1.55, 0.05); }
bone L_Shoulder { mass = 1.0 kg; position = (-0.25, 1.55, 0.0); }
bone L_Humerus  { mass = 2.0 kg; position = (-0.45, 1.55, 0.0); }
bone L_Radius   { mass = 1.2 kg; position = (-0.65, 1.55, 0.0); }
bone L_Carpal   { mass = 0.5 kg; position = (-0.75, 1.55, 0.0); }

joint JLA1 { source = Sternum;    target = L_Clavicle; }
joint JLA2 { source = L_Clavicle; target = L_Shoulder; }
joint JLA3 { source = L_Shoulder; target = L_Humerus; }
joint JLA4 { source = L_Humerus;  target = L_Radius; }
joint JLA5 { source = L_Radius;   target = L_Carpal; }

include "human_hand_l.apex";

// RIGHT ARM
bone R_Clavicle { mass = 0.5 kg; position = (0.1, 1.55, 0.05); }
bone R_Shoulder { mass = 1.0 kg; position = (0.25, 1.55, 0.0); }
bone R_Humerus  { mass = 2.0 kg; position = (0.45, 1.55, 0.0); }
bone R_Radius   { mass = 1.2 kg; position = (0.65, 1.55, 0.0); }
bone R_Carpal   { mass = 0.5 kg; position = (0.75, 1.55, 0.0); }

joint JRA1 { source = Sternum;    target = R_Clavicle; }
joint JRA2 { source = R_Clavicle; target = R_Shoulder; }
joint JRA3 { source = R_Shoulder; target = R_Humerus; }
joint JRA4 { source = R_Humerus;  target = R_Radius; }
joint JRA5 { source = R_Radius;   target = R_Carpal; }

include "human_hand_r.apex";
