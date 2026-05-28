// === NEURAL LAYER: THE 43 MAJOR NERVE PAIRS (Bilateral) ===

// --- CRANIAL NERVES (12 Pairs = 24 Nodes) ---
cpg CN_I_L { frequency = 0.0 Hz; } cpg CN_I_R { frequency = 0.0 Hz; }
cpg CN_V_L { frequency = 1.0 Hz; } cpg CN_V_R { frequency = 1.0 Hz; } // Trigemin
cpg CN_VII_L { frequency = 0.5 Hz; } cpg CN_VII_R { frequency = 0.5 Hz; } // Facial
cpg CN_X_L { frequency = 0.1 Hz; } cpg CN_X_R { frequency = 0.1 Hz; } // Vagus

// --- SPINAL NERVES (31 Pairs = 62 Nodes) ---
// Cervical (C1-C8)
cpg Nerve_C1_L { frequency = 1.0 Hz; } cpg Nerve_C1_R { frequency = 1.0 Hz; }
cpg Nerve_C2_L { frequency = 1.0 Hz; } cpg Nerve_C2_R { frequency = 1.0 Hz; }
cpg Nerve_C3_L { frequency = 1.0 Hz; } cpg Nerve_C3_R { frequency = 1.0 Hz; }
cpg Nerve_C4_L { frequency = 1.0 Hz; } cpg Nerve_C4_R { frequency = 1.0 Hz; }
cpg Nerve_C5_L { frequency = 1.0 Hz; } cpg Nerve_C5_R { frequency = 1.0 Hz; }
cpg Nerve_C6_L { frequency = 1.0 Hz; } cpg Nerve_C6_R { frequency = 1.0 Hz; }
cpg Nerve_C7_L { frequency = 1.0 Hz; } cpg Nerve_C7_R { frequency = 1.0 Hz; }
cpg Nerve_C8_L { frequency = 1.0 Hz; } cpg Nerve_C8_R { frequency = 1.0 Hz; }

// Thoracic (T1-T12)
cpg Nerve_T1_L { frequency = 0.5 Hz; } cpg Nerve_T1_R { frequency = 0.5 Hz; }
cpg Nerve_T2_L { frequency = 0.5 Hz; } cpg Nerve_T2_R { frequency = 0.5 Hz; }
cpg Nerve_T3_L { frequency = 0.5 Hz; } cpg Nerve_T3_R { frequency = 0.5 Hz; }
cpg Nerve_T4_L { frequency = 0.5 Hz; } cpg Nerve_T4_R { frequency = 0.5 Hz; }
cpg Nerve_T5_L { frequency = 0.5 Hz; } cpg Nerve_T5_R { frequency = 0.5 Hz; }
cpg Nerve_T6_L { frequency = 0.5 Hz; } cpg Nerve_T6_R { frequency = 0.5 Hz; }
cpg Nerve_T7_L { frequency = 0.5 Hz; } cpg Nerve_T7_R { frequency = 0.5 Hz; }
cpg Nerve_T8_L { frequency = 0.5 Hz; } cpg Nerve_T8_R { frequency = 0.5 Hz; }
cpg Nerve_T9_L { frequency = 0.5 Hz; } cpg Nerve_T9_R { frequency = 0.5 Hz; }
cpg Nerve_T10_L { frequency = 0.5 Hz; } cpg Nerve_T10_R { frequency = 0.5 Hz; }
cpg Nerve_T11_L { frequency = 0.5 Hz; } cpg Nerve_T11_R { frequency = 0.5 Hz; }
cpg Nerve_T12_L { frequency = 0.5 Hz; } cpg Nerve_T12_R { frequency = 0.5 Hz; }

// Lumbar (L1-L5)
cpg Nerve_L1_L { frequency = 1.0 Hz; } cpg Nerve_L1_R { frequency = 1.0 Hz; }
cpg Nerve_L2_L { frequency = 1.0 Hz; } cpg Nerve_L2_R { frequency = 1.0 Hz; }
cpg Nerve_L3_L { frequency = 1.0 Hz; } cpg Nerve_L3_R { frequency = 1.0 Hz; }
cpg Nerve_L4_L { frequency = 1.0 Hz; } cpg Nerve_L4_R { frequency = 1.0 Hz; }
cpg Nerve_L5_L { frequency = 1.0 Hz; } cpg Nerve_L5_R { frequency = 1.0 Hz; }

// Sacral
cpg Nerve_S1_L { frequency = 1.0 Hz; } cpg Nerve_S1_R { frequency = 1.0 Hz; }

// Wiring
synapse Syn_Jaw_L { source = CN_V_L; target = L_Masseter; weight = 1.0; }
synapse Syn_Jaw_R { source = CN_V_R; target = R_Masseter; weight = 1.0; }
synapse Syn_Diaphragm_L { source = Nerve_C4_L; target = Diaphragm_L; weight = 1.0; }
synapse Syn_Diaphragm_R { source = Nerve_C4_R; target = Diaphragm_R; weight = 1.0; }
synapse Syn_Quad_L { source = Nerve_L4_L; target = L_RectusFem; weight = 1.0; }
synapse Syn_Quad_R { source = Nerve_L4_R; target = R_RectusFem; weight = 1.0; }
