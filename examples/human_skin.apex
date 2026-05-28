// === INTEGUMENTARY SYSTEM: SKIN ENVELOPES ===

// TORSO
skin Skin_Torso {
    anchor = Sternum;
    hull Hull_Chest {
        box (0.4, 0.4, 0.25);
        offset = (0.0, 0.0, 0.0);
    }
}

// ARMS
skin Skin_Arm_L {
    anchor = L_Humerus;
    hull Hull_Biceps {
        sphere (0.1);
        offset = (0.0, 0.0, 0.0);
    }
}
skin Skin_Arm_R {
    anchor = R_Humerus;
    hull Hull_Biceps {
        sphere (0.1);
        offset = (0.0, 0.0, 0.0);
    }
}

// LEGS
skin Skin_Leg_L {
    anchor = L_Femur;
    hull Hull_Thigh {
        sphere (0.15);
        offset = (0.0, 0.0, 0.0);
    }
}
skin Skin_Leg_R {
    anchor = R_Femur;
    hull Hull_Thigh {
        sphere (0.15);
        offset = (0.0, 0.0, 0.0);
    }
}

// HEAD
skin Skin_Head {
    anchor = Frontal;
    hull Hull_Face {
        sphere (0.2);
        offset = (0.0, -0.05, 0.05);
    }
}
