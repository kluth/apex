// === LEGS: HIGH-FIDELITY MODULAR ===

// LEFT LEG
bone L_Hip     { mass = 2.0 kg; position = (-0.15, 0.85, 0.0); mesh = "anatomy/human/hip_l"; }
bone L_Femur   { mass = 8.0 kg; position = (-0.15, 0.65, 0.0); mesh = "anatomy/human/femur_l"; }
bone L_Patella { mass = 0.2 kg; position = (-0.15, 0.47, 0.05); }
bone L_Tibia   { mass = 4.0 kg; position = (-0.15, 0.35, 0.0); mesh = "anatomy/human/tibia_l"; }
bone L_Fibula  { mass = 1.0 kg; position = (-0.18, 0.35, 0.0); }
bone L_Calc    { mass = 0.5 kg; position = (-0.15, 0.1, -0.05); mesh = "anatomy/human/calc_l"; }

joint JLL1 { source = Pelvis; target = L_Hip; }
joint JLL2 { source = L_Hip;   target = L_Femur; }
joint JLP  { source = L_Femur; target = L_Patella; }
joint JLL3 { source = L_Femur; target = L_Tibia; }
joint JLL4 { source = L_Femur; target = L_Fibula; }
joint JLL5 { source = L_Tibia; target = L_Calc; }

include "human_foot_l.apex";

// RIGHT LEG
bone R_Hip     { mass = 2.0 kg; position = (0.15, 0.85, 0.0); mesh = "anatomy/human/hip_r"; }
bone R_Femur   { mass = 8.0 kg; position = (0.15, 0.65, 0.0); mesh = "anatomy/human/femur_r"; }
bone R_Patella { mass = 0.2 kg; position = (0.15, 0.47, 0.05); }
bone R_Tibia   { mass = 4.0 kg; position = (0.15, 0.35, 0.0); mesh = "anatomy/human/tibia_r"; }
bone R_Fibula  { mass = 1.0 kg; position = (0.18, 0.35, 0.0); }
bone R_Calc    { mass = 0.5 kg; position = (0.15, 0.1, -0.05); mesh = "anatomy/human/calc_r"; }

joint JRL1 { source = Pelvis; target = R_Hip; }
joint JRL2 { source = R_Hip;   target = R_Femur; }
joint JRP  { source = R_Femur; target = R_Patella; }
joint JRL3 { source = R_Femur; target = R_Tibia; }
joint JRL4 { source = R_Femur; target = R_Fibula; }
joint JRL5 { source = R_Tibia; target = R_Calc; }

include "human_foot_r.apex";
