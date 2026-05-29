// === INTEGUMENTARY SYSTEM: HIGH-FIDELITY SILHOUETTE ===

// HEAD & NECK
skin Skin_Head {
    anchor = Frontal;
    hull Hull_Skull { sphere (0.11); offset = (0.0, -0.05, 0.0); }
}
skin Skin_Neck {
    anchor = C4;
    hull Hull_Neck { cylinder (0.06, 0.15); offset = (0.0, 0.0, 0.0); }
}

// TORSO
skin Skin_Torso {
    anchor = Sternum;
    hull Hull_Chest { box (0.35, 0.35, 0.22); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Abdomen {
    anchor = L3;
    hull Hull_Belly { box (0.32, 0.25, 0.20); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Pelvis {
    anchor = Sacrum;
    hull Hull_Hips { box (0.38, 0.20, 0.22); offset = (0.0, 0.0, 0.0); }
}

// ARMS (Left)
skin Skin_Shoulder_L {
    anchor = L_Scapula;
    hull Hull_Deltoid { sphere (0.08); offset = (0.0, 0.1, 0.05); }
}
skin Skin_UpperArm_L {
    anchor = L_Humerus;
    hull Hull_Biceps { capsule (0.06, 0.25); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Forearm_L {
    anchor = L_Radius;
    hull Hull_Arm { capsule (0.045, 0.22); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Hand_L {
    anchor = L_Capitate;
    hull Hull_Palm { box (0.1, 0.03, 0.08); offset = (0.0, 0.0, 0.0); }
}

// ARMS (Right)
skin Skin_Shoulder_R {
    anchor = R_Scapula;
    hull Hull_Deltoid { sphere (0.08); offset = (0.0, 0.1, 0.05); }
}
skin Skin_UpperArm_R {
    anchor = R_Humerus;
    hull Hull_Biceps { capsule (0.06, 0.25); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Forearm_R {
    anchor = R_Radius;
    hull Hull_Arm { capsule (0.045, 0.22); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Hand_R {
    anchor = R_Capitate;
    hull Hull_Palm { box (0.1, 0.03, 0.08); offset = (0.0, 0.0, 0.0); }
}

// LEGS (Left)
skin Skin_Thigh_L {
    anchor = L_Femur;
    hull Hull_Quad { capsule (0.10, 0.4); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Calf_L {
    anchor = L_Tibia;
    hull Hull_Gastro { capsule (0.08, 0.35); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Foot_L {
    anchor = L_Navicular;
    hull Hull_Arch { box (0.08, 0.05, 0.2); offset = (0.0, -0.02, 0.05); }
}

// LEGS (Right)
skin Skin_Thigh_R {
    anchor = R_Femur;
    hull Hull_Quad { capsule (0.10, 0.4); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Calf_R {
    anchor = R_Tibia;
    hull Hull_Gastro { capsule (0.08, 0.35); offset = (0.0, 0.0, 0.0); }
}
skin Skin_Foot_R {
    anchor = R_Navicular;
    hull Hull_Arch { box (0.08, 0.05, 0.2); offset = (0.0, -0.02, 0.05); }
}
