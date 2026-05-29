// === INTEGUMENTARY SYSTEM: TOTAL ANATOMICAL FIDELITY SILHOUETTE ===

// --- HEAD & NECK ---
skin Skin_Head {
    anchor = Frontal;
    hull Skull_Vault { sphere (0.11); offset = (0.0, -0.05, 0.0); }
    hull Face_Front  { sphere (0.09); offset = (0.0, -0.08, 0.05); }
}
skin Skin_Neck {
    anchor = C4;
    hull Neck_Col { capsule (0.065, 0.15); offset = (0.0, 0.0, 0.0); }
}

// --- TORSO (Tapered & Rounded) ---
skin Skin_Thorax {
    anchor = T4;
    hull Upper_Chest { capsule (0.18, 0.35); offset = (0.0, 0.0, 0.05); }
}
skin Skin_Abdomen {
    anchor = L2;
    hull Belly { capsule (0.16, 0.30); offset = (0.0, 0.0, 0.02); }
}
skin Skin_Pelvis {
    anchor = Sacrum;
    hull Hips { capsule (0.19, 0.38); offset = (0.0, 0.0, 0.0); }
}

// --- ARMS (Left) ---
skin Skin_Shoulder_L {
    anchor = L_Scapula;
    hull Delt_L { sphere (0.10); offset = (0.0, 0.05, 0.05); }
}
skin Skin_UpperArm_L {
    anchor = L_Humerus;
    hull Bicep_L { capsule (0.065, 0.28); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Forearm_L {
    anchor = L_Radius;
    hull Arm_L { capsule (0.05, 0.25); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Hand_L {
    anchor = L_Capitate;
    hull Palm_L { box (0.1, 0.035, 0.09); offset = (0.0, 0.0, 0.0); }
}
// Finger Envelopes (Detailed)
skin Skin_L_Thumb { anchor = L_Thumb_P; hull H { capsule (0.015, 0.05); offset = (0.0, 0.0, 0.0); } }
skin Skin_L_Ind   { anchor = L_Ind_M;   hull H { capsule (0.012, 0.07); offset = (0.0, 0.0, 0.0); } }
skin Skin_L_Mid   { anchor = L_Mid_M;   hull H { capsule (0.012, 0.08); offset = (0.0, 0.0, 0.0); } }
skin Skin_L_Rng   { anchor = L_Rng_M;   hull H { capsule (0.012, 0.07); offset = (0.0, 0.0, 0.0); } }
skin Skin_L_Pnk   { anchor = L_Pnk_M;   hull H { capsule (0.010, 0.06); offset = (0.0, 0.0, 0.0); } }

// --- ARMS (Right) ---
skin Skin_Shoulder_R {
    anchor = R_Scapula;
    hull Delt_R { sphere (0.10); offset = (0.0, 0.05, 0.05); }
}
skin Skin_UpperArm_R {
    anchor = R_Humerus;
    hull Bicep_R { capsule (0.065, 0.28); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Forearm_R {
    anchor = R_Radius;
    hull Arm_R { capsule (0.05, 0.25); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Hand_R {
    anchor = R_Capitate;
    hull Palm_R { box (0.1, 0.035, 0.09); offset = (0.0, 0.0, 0.0); }
}
// Finger Envelopes
skin Skin_R_Thumb { anchor = R_Thumb_P; hull H { capsule (0.015, 0.05); offset = (0.0, 0.0, 0.0); } }
skin Skin_R_Ind   { anchor = R_Ind_M;   hull H { capsule (0.012, 0.07); offset = (0.0, 0.0, 0.0); } }
skin Skin_R_Mid   { anchor = R_Mid_M;   hull H { capsule (0.012, 0.08); offset = (0.0, 0.0, 0.0); } }
skin Skin_R_Rng   { anchor = R_Rng_M;   hull H { capsule (0.012, 0.07); offset = (0.0, 0.0, 0.0); } }
skin Skin_R_Pnk   { anchor = R_Pnk_M;   hull H { capsule (0.010, 0.06); offset = (0.0, 0.0, 0.0); } }

// --- LEGS (Left) ---
skin Skin_Thigh_L {
    anchor = L_Femur;
    hull Quad_L { capsule (0.11, 0.42); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Calf_L {
    anchor = L_Tibia;
    hull Gastro_L { capsule (0.085, 0.38); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Foot_L {
    anchor = L_Navicular;
    hull Arch_L { box (0.09, 0.06, 0.22); offset = (0.0, -0.02, 0.05); }
}

// --- LEGS (Right) ---
skin Skin_Thigh_R {
    anchor = R_Femur;
    hull Quad_R { capsule (0.11, 0.42); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Calf_R {
    anchor = R_Tibia;
    hull Gastro_R { capsule (0.085, 0.38); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Foot_R {
    anchor = R_Navicular;
    hull Arch_R { box (0.09, 0.06, 0.22); offset = (0.0, -0.02, 0.05); }
}
