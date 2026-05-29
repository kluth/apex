// === NEURAL LAYER: BIPEDAL GAIT CONTROLLER ===

// --- CENTRAL PATTERN GENERATORS (CPGs) ---
// Rhythmic oscillator for the gait cycle (0.8 Hz = ~1 step per 1.25s)
cpg Brain_Gait_Cycle {
    frequency = 0.8 Hz;
}

// --- SYNPATIC WIRING: LEFT LEG (Phase A) ---
// Swing Phase: Hip flexors and Knee extensors
synapse Syn_Gait_L_Hip    { source = Brain_Gait_Cycle; target = L_PsoasMaj;    weight = 1.0; }
synapse Syn_Gait_L_Knee   { source = Brain_Gait_Cycle; target = L_RectusFem;   weight = 1.2; }
// Stance Phase: Glutes and Calf (using negative weight for reciprocal inhibition proxy)
synapse Syn_Gait_L_Push   { source = Brain_Gait_Cycle; target = L_Gastrocnem;  weight = -0.8; }

// --- SYNPATIC WIRING: RIGHT LEG (Phase B - Reciprocal) ---
// Using reciprocal inhibition: Right leg fires when Left leg relaxes
synapse Syn_Gait_R_Hip    { source = Brain_Gait_Cycle; target = R_PsoasMaj;    weight = -1.0; }
synapse Syn_Gait_R_Knee   { source = Brain_Gait_Cycle; target = R_RectusFem;   weight = -1.2; }
synapse Syn_Gait_R_Push   { source = Brain_Gait_Cycle; target = R_Gastrocnem;  weight = 1.0; }

// --- POSTURAL STABILIZATION ---
cpg Brain_Balance {
    frequency = 0.0 Hz; // Steady firing
}
synapse Syn_Bal_Back_L    { source = Brain_Balance; target = L_Erector_L; weight = 0.6; }
synapse Syn_Bal_Back_R    { source = Brain_Balance; target = R_Erector_L; weight = 0.6; }
synapse Syn_Bal_Core_L    { source = Brain_Balance; target = L_RectusAbd; weight = 0.4; }
synapse Syn_Bal_Core_R    { source = Brain_Balance; target = R_RectusAbd; weight = 0.4; }

// --- SENSORY FEEDBACK ---
receptor Rec_Gait_L_Impact { target = L_Gastrocnem; type = GTO; }
receptor Rec_Gait_R_Impact { target = R_Gastrocnem; type = GTO; }
