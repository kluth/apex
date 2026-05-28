// === NEURAL LAYER: THE 43 MAJOR NERVE PAIRS ===

// --- CRANIAL NERVES (12 Pairs) ---
cpg CN_I_Olfactory   { frequency = 0.0 Hz; }
cpg CN_II_Optic      { frequency = 0.0 Hz; }
cpg CN_III_Oculomot  { frequency = 0.0 Hz; }
cpg CN_IV_Trochlear  { frequency = 0.0 Hz; }
cpg CN_V_Trigemin    { frequency = 1.0 Hz; } // Jaw control
cpg CN_VI_Abducens   { frequency = 0.0 Hz; }
cpg CN_VII_Facial    { frequency = 0.5 Hz; }
cpg CN_VIII_Vestib   { frequency = 0.0 Hz; }
cpg CN_IX_Glossoph   { frequency = 0.0 Hz; }
cpg CN_X_Vagus       { frequency = 0.1 Hz; } // Autonomic/Heart/Lung
cpg CN_XI_Accessory  { frequency = 0.0 Hz; }
cpg CN_XII_Hypogloss { frequency = 0.0 Hz; }

// --- SPINAL NERVES (31 Pairs) ---
// Cervical (C1-C8)
cpg Nerve_C1 { frequency = 1.0 Hz; }
cpg Nerve_C2 { frequency = 1.0 Hz; }
cpg Nerve_C3 { frequency = 1.0 Hz; }
cpg Nerve_C4 { frequency = 1.0 Hz; }
cpg Nerve_C5 { frequency = 1.0 Hz; }
cpg Nerve_C6 { frequency = 1.0 Hz; }
cpg Nerve_C7 { frequency = 1.0 Hz; }
cpg Nerve_C8 { frequency = 1.0 Hz; }

// Thoracic (T1-T12)
cpg Nerve_T1 { frequency = 0.5 Hz; }
cpg Nerve_T2 { frequency = 0.5 Hz; }
cpg Nerve_T3 { frequency = 0.5 Hz; }
cpg Nerve_T4 { frequency = 0.5 Hz; }
cpg Nerve_T5 { frequency = 0.5 Hz; }
cpg Nerve_T6 { frequency = 0.5 Hz; }
cpg Nerve_T7 { frequency = 0.5 Hz; }
cpg Nerve_T8 { frequency = 0.5 Hz; }
cpg Nerve_T9 { frequency = 0.5 Hz; }
cpg Nerve_T10 { frequency = 0.5 Hz; }
cpg Nerve_T11 { frequency = 0.5 Hz; }
cpg Nerve_T12 { frequency = 0.5 Hz; }

// Lumbar (L1-L5)
cpg Nerve_L1 { frequency = 1.0 Hz; }
cpg Nerve_L2 { frequency = 1.0 Hz; }
cpg Nerve_L3 { frequency = 1.0 Hz; }
cpg Nerve_L4 { frequency = 1.0 Hz; }
cpg Nerve_L5 { frequency = 1.0 Hz; }

// Sacral & Coccygeal (S1-S5, Co1)
cpg Nerve_S1 { frequency = 1.0 Hz; }
cpg Nerve_S2 { frequency = 1.0 Hz; }
cpg Nerve_S3 { frequency = 1.0 Hz; }
cpg Nerve_S4 { frequency = 1.0 Hz; }
cpg Nerve_S5 { frequency = 1.0 Hz; }
cpg Nerve_Co1 { frequency = 0.0 Hz; }

// Wiring (Sample Synapses from Nerves to Muscles)
synapse Syn_Jaw { source = CN_V_Trigemin; target = L_Masseter; weight = 1.0; }
synapse Syn_Diaphragm { source = Nerve_C4; target = Diaphragm_L; weight = 1.0; }
synapse Syn_Quad { source = Nerve_L4; target = L_RectusFem; weight = 1.0; }
