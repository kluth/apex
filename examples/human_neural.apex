// === NEURAL LAYER: BIPEDAL GAIT & RESPIRATION ===

// --- 1. RESPIRATION (Breathing) ---
// Rhythmic oscillator for breathing (0.25 Hz = 1 breath every 4s)
cpg Brain_Breathing {
    frequency = 0.25 Hz;
}

synapse Syn_Breath_L { source = Brain_Breathing; target = Diaphragm_L; weight = 1.5; }
synapse Syn_Breath_R { source = Brain_Breathing; target = Diaphragm_R; weight = 1.5; }

// --- 2. LOCOMOTION (Gait) ---
cpg Brain_Gait_Cycle {
    frequency = 0.8 Hz;
}

// Left Leg
synapse Syn_Gait_L_Hip    { source = Brain_Gait_Cycle; target = L_PsoasMaj;    weight = 1.0; }
synapse Syn_Gait_L_Knee   { source = Brain_Gait_Cycle; target = L_RectusFem;   weight = 1.2; }
synapse Syn_Gait_L_Push   { source = Brain_Gait_Cycle; target = L_Gastrocnem;  weight = -0.8; }

// Right Leg (Reciprocal)
synapse Syn_Gait_R_Hip    { source = Brain_Gait_Cycle; target = R_PsoasMaj;    weight = -1.0; }
synapse Syn_Gait_R_Knee   { source = Brain_Gait_Cycle; target = R_RectusFem;   weight = -1.2; }
synapse Syn_Gait_R_Push   { source = Brain_Gait_Cycle; target = R_Gastrocnem;  weight = 1.0; }

// --- 3. STABILIZATION ---
cpg Brain_Balance {
    frequency = 0.0 Hz;
}
synapse Syn_Bal_Back_L    { source = Brain_Balance; target = L_Erector_L; weight = 0.6; }
synapse Syn_Bal_Back_R    { source = Brain_Balance; target = R_Erector_L; weight = 0.6; }

// --- 4. SENSORY FEEDBACK ---
receptor Rec_Gait_L_Impact { target = L_Gastrocnem; type = GTO; }
receptor Rec_Gait_R_Impact { target = R_Gastrocnem; type = GTO; }
