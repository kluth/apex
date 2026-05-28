// === SENSORY LAYER: RECEPTORS (Muscle Spindles & GTOs) ===

// Autonomic Feedback (Breathing regulation)
receptor Rec_Breath_L {
    target = Diaphragm_L;
    type = Spindle;
}

receptor Rec_Breath_R {
    target = Diaphragm_R;
    type = Spindle;
}

// Locomotion Feedback (Proprioception)
receptor Rec_Leg_L_Stretch {
    target = L_RectusFem;
    type = Spindle;
}

receptor Rec_Leg_R_Stretch {
    target = R_RectusFem;
    type = Spindle;
}

receptor Rec_Leg_L_Tension {
    target = L_Gastrocnem;
    type = GTO;
}

receptor Rec_Leg_R_Tension {
    target = R_Gastrocnem;
    type = GTO;
}
