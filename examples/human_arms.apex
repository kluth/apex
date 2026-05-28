// === ARMS: HIGH-FIDELITY MODULAR ===

// LEFT ARM
bone L_Scapula  { mass = 1.0 kg; position = (-0.15, 1.45, -0.1); mesh = "anatomy/human/scapula_l"; }
bone L_Clavicle { mass = 0.5 kg; position = (-0.1, 1.55, 0.05); mesh = "anatomy/human/clavicle_l"; }
bone L_Humerus  { mass = 2.0 kg; position = (-0.3, 1.45, 0.0); mesh = "anatomy/human/humerus_l"; }
bone L_Radius   { mass = 0.6 kg; position = (-0.5, 1.45, 0.0); mesh = "anatomy/human/radius_l"; }
bone L_Ulna     { mass = 0.6 kg; position = (-0.5, 1.43, 0.0); mesh = "anatomy/human/ulna_l"; }

joint JLA1 { source = Sternum;    target = L_Clavicle; }
joint JLA2 { source = L_Clavicle; target = L_Scapula; }
joint JLA3 { source = L_Scapula;  target = L_Humerus; }
joint JLA4 { source = L_Humerus;  target = L_Radius; }
joint JLA4b { source = L_Humerus; target = L_Ulna; }

include "human_hand_l.apex";

// RIGHT ARM
bone R_Scapula  { mass = 1.0 kg; position = (0.15, 1.45, -0.1); mesh = "anatomy/human/scapula_r"; }
bone R_Clavicle { mass = 0.5 kg; position = (0.1, 1.55, 0.05); mesh = "anatomy/human/clavicle_r"; }
bone R_Humerus  { mass = 2.0 kg; position = (0.3, 1.45, 0.0); mesh = "anatomy/human/humerus_r"; }
bone R_Radius   { mass = 0.6 kg; position = (0.5, 1.45, 0.0); mesh = "anatomy/human/radius_r"; }
bone R_Ulna     { mass = 0.6 kg; position = (0.5, 1.43, 0.0); mesh = "anatomy/human/ulna_r"; }

joint JRA1 { source = Sternum;    target = R_Clavicle; }
joint JRA2 { source = R_Clavicle; target = R_Scapula; }
joint JRA3 { source = R_Scapula;  target = R_Humerus; }
joint JRA4 { source = R_Humerus;  target = R_Radius; }
joint JRA4b { source = R_Humerus; target = R_Ulna; }

include "human_hand_r.apex";
