// === LEGS: HIGH-FIDELITY MODULAR ===

// LEFT LEG
bone L_Femur   { mass = 8.0 kg; position = (-0.15, 0.75, 0.0); mesh = "anatomy/human/femur_l"; }
bone L_Patella { mass = 0.2 kg; position = (-0.15, 0.47, 0.05); }
bone L_Tibia   { mass = 4.0 kg; position = (-0.15, 0.35, 0.0); mesh = "anatomy/human/tibia_l"; }
bone L_Fibula  { mass = 1.0 kg; position = (-0.18, 0.35, 0.0); }

joint JLL1 { source = L_Innominate; target = L_Femur; }
joint JLP  { source = L_Femur;      target = L_Patella; }
joint JLL2 { source = L_Femur;      target = L_Tibia; }
joint JLL3 { source = L_Femur;      target = L_Fibula; }

include "human_foot_l.apex";

// RIGHT LEG
bone R_Femur   { mass = 8.0 kg; position = (0.15, 0.75, 0.0); mesh = "anatomy/human/femur_r"; }
bone R_Patella { mass = 0.2 kg; position = (0.15, 0.47, 0.05); }
bone R_Tibia   { mass = 4.0 kg; position = (0.15, 0.35, 0.0); mesh = "anatomy/human/tibia_r"; }
bone R_Fibula  { mass = 1.0 kg; position = (0.18, 0.35, 0.0); }

joint JRL1 { source = R_Innominate; target = R_Femur; }
joint JRP  { source = R_Femur;      target = R_Patella; }
joint JRL2 { source = R_Femur;      target = R_Tibia; }
joint JRL3 { source = R_Femur;      target = R_Fibula; }

include "human_foot_r.apex";
