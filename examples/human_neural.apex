// === NEURAL LAYER: AUTONOMIC & MOTOR CONTROL ===

// --- AUTONOMIC: BREATHING ---
// Rhythmic oscillator for breathing (0.2 Hz = ~12 breaths per minute)
cpg Brain_Breathing {
    frequency = 0.2 Hz;
}

// Synaptic wiring to the Diaphragm
synapse Syn_Breath_L {
    source = Brain_Breathing;
    target = Diaphragm_L;
    weight = 1.0;
}

synapse Syn_Breath_R {
    source = Brain_Breathing;
    target = Diaphragm_R;
    weight = 1.0;
}

// --- MOTOR: LOCOMOTION (Simplified Gait Proxy) ---
cpg Brain_Gait {
    frequency = 0.5 Hz; // 1 step per 2 seconds
}

// Left Leg Swing
synapse Syn_Gait_L_Swing {
    source = Brain_Gait;
    target = L_RectusFem;
    weight = 1.2;
}

// Right Leg Swing (Using same CPG for now, would need phase offset in reality)
synapse Syn_Gait_R_Swing {
    source = Brain_Gait;
    target = R_RectusFem;
    weight = 1.2;
}

// Postural Stability (Constant firing)
cpg Brain_Core {
    frequency = 0.0 Hz; // DC signal
}

synapse Syn_Core_L {
    source = Brain_Core;
    target = L_Erector_L;
    weight = 0.5;
}

synapse Syn_Core_R {
    source = Brain_Core;
    target = R_Erector_L;
    weight = 0.5;
}
