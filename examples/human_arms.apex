// === ARMS: HIGH-FIDELITY MODULAR ===

// SCAPULAE (Shoulder Blades)
bone L_Scapula { mass = 1.0 kg; position = (-0.15, 1.45, -0.1); }
bone R_Scapula { mass = 1.0 kg; position = (0.15, 1.45, -0.1); }
joint JLS { source = T2; target = L_Scapula; }
joint JRS { source = T2; target = R_Scapula; }

// LEFT ARM
bone L_Clavicle { mass = 0.5 kg; position = (-0.1, 1.55, 0.05); mesh = "anatomy/human/clavicle_l"; }
bone L_Shoulder { mass = 1.0 kg; position = (-0.25, 1.55, 0.0); mesh = "anatomy/human/shoulder_l"; }
bone L_Humerus  { mass = 2.0 kg; position = (-0.45, 1.55, 0.0); mesh = "anatomy/human/humerus_l"; }
bone L_Radius   { mass = 0.6 kg; position = (-0.65, 1.55, 0.0); mesh = "anatomy/human/radius_l"; }
bone L_Ulna     { mass = 0.6 kg; position = (-0.65, 1.53, 0.0); mesh = "anatomy/human/ulna_l"; }

joint JLA1 { source = Sternum;    target = L_Clavicle; }
joint JLA2 { source = L_Clavicle; target = L_Shoulder; }
joint JLA3 { source = L_Shoulder; target = L_Humerus; }
joint JLA4 { source = L_Humerus;  target = L_Radius; }
joint JLA4b { source = L_Humerus; target = L_Ulna; }

include "human_hand_l.apex";

// RIGHT ARM
bone R_Clavicle { mass = 0.5 kg; position = (0.1, 1.55, 0.05); mesh = "anatomy/human/clavicle_r"; }
bone R_Shoulder { mass = 1.0 kg; position = (0.25, 1.55, 0.0); mesh = "anatomy/human/shoulder_r"; }
bone R_Humerus  { mass = 2.0 kg; position = (0.45, 1.55, 0.0); mesh = "anatomy/human/humerus_r"; }
bone R_Radius   { mass = 0.6 kg; position = (0.65, 1.55, 0.0); mesh = "anatomy/human/radius_r"; }
bone R_Ulna     { mass = 0.6 kg; position = (0.65, 1.53, 0.0); mesh = "anatomy/human/ulna_r"; }

joint JRA1 { source = Sternum;    target = R_Clavicle; }
joint JRA2 { source = R_Clavicle; target = R_Shoulder; }
joint JRA3 { source = R_Shoulder; target = R_Humerus; }
joint JRA4 { source = R_Humerus;  target = R_Radius; }
joint JRA4b { source = R_Humerus; target = R_Ulna; }

include "human_hand_r.apex";
