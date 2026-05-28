// === MUSCLES: TORSO, SPINE & NECK (Deep Fidelity) ===

// --- ABDOMINAL WALL ---
muscle L_RectusAbd     { source = Sternum;      target = L_Innominate; max_force = 1200.0 Nm; }
muscle R_RectusAbd     { source = Sternum;      target = R_Innominate; max_force = 1200.0 Nm; }
muscle L_ExtOblique    { source = L_Rib6;       target = L_Innominate; max_force = 800.0 Nm; }
muscle R_ExtOblique    { source = R_Rib6;       target = R_Innominate; max_force = 800.0 Nm; }
muscle L_IntOblique    { source = L_Rib10;      target = L_Innominate; max_force = 700.0 Nm; }
muscle R_IntOblique    { source = R_Rib10;      target = R_Innominate; max_force = 700.0 Nm; }
muscle L_TransAbd      { source = L1;           target = L_Innominate; max_force = 600.0 Nm; }
muscle R_TransAbd      { source = L1;           target = R_Innominate; max_force = 600.0 Nm; }

// --- INTERCOSTALS (Breathing Stability - 11 pairs) ---
muscle L_IntCost_1  { source = L_Rib1;  target = L_Rib2;  max_force = 100.0 Nm; }
muscle L_IntCost_2  { source = L_Rib2;  target = L_Rib3;  max_force = 100.0 Nm; }
muscle L_IntCost_3  { source = L_Rib3;  target = L_Rib4;  max_force = 100.0 Nm; }
muscle L_IntCost_4  { source = L_Rib4;  target = L_Rib5;  max_force = 100.0 Nm; }
muscle L_IntCost_5  { source = L_Rib5;  target = L_Rib6;  max_force = 100.0 Nm; }
muscle L_IntCost_6  { source = L_Rib6;  target = L_Rib7;  max_force = 100.0 Nm; }
muscle L_IntCost_7  { source = L_Rib7;  target = L_Rib8;  max_force = 100.0 Nm; }
muscle L_IntCost_8  { source = L_Rib8;  target = L_Rib9;  max_force = 100.0 Nm; }
muscle L_IntCost_9  { source = L_Rib9;  target = L_Rib10; max_force = 100.0 Nm; }
muscle L_IntCost_10 { source = L_Rib10; target = L_Rib11; max_force = 100.0 Nm; }
muscle L_IntCost_11 { source = L_Rib11; target = L_Rib12; max_force = 100.0 Nm; }

muscle R_IntCost_1  { source = R_Rib1;  target = R_Rib2;  max_force = 100.0 Nm; }
muscle R_IntCost_2  { source = R_Rib2;  target = R_Rib3;  max_force = 100.0 Nm; }
muscle R_IntCost_3  { source = R_Rib3;  target = R_Rib4;  max_force = 100.0 Nm; }
muscle R_IntCost_4  { source = R_Rib4;  target = R_Rib5;  max_force = 100.0 Nm; }
muscle R_IntCost_5  { source = R_Rib5;  target = R_Rib6;  max_force = 100.0 Nm; }
muscle R_IntCost_6  { source = R_Rib6;  target = R_Rib7;  max_force = 100.0 Nm; }
muscle R_IntCost_7  { source = R_Rib7;  target = R_Rib8;  max_force = 100.0 Nm; }
muscle R_IntCost_8  { source = R_Rib8;  target = R_Rib9;  max_force = 100.0 Nm; }
muscle R_IntCost_9  { source = R_Rib9;  target = R_Rib10; max_force = 100.0 Nm; }
muscle R_IntCost_10 { source = R_Rib10; target = R_Rib11; max_force = 100.0 Nm; }
muscle R_IntCost_11 { source = R_Rib11; target = R_Rib12; max_force = 100.0 Nm; }

// --- DEEP BACK & NECK ---
muscle L_Erector_L     { source = Sacrum;    target = L1;        max_force = 1500.0 Nm; }
muscle R_Erector_L     { source = Sacrum;    target = L1;        max_force = 1500.0 Nm; }
muscle L_Erector_T     { source = L1;        target = T6;        max_force = 1200.0 Nm; }
muscle R_Erector_T     { source = L1;        target = T6;        max_force = 1200.0 Nm; }
muscle L_Erector_C     { source = T6;        target = C4;        max_force = 800.0 Nm; }
muscle R_Erector_C     { source = T6;        target = C4;        max_force = 800.0 Nm; }

muscle L_Masseter      { source = Zygomatic_L; target = Mandible; max_force = 800.0 Nm; }
muscle R_Masseter      { source = Zygomatic_R; target = Mandible; max_force = 800.0 Nm; }
muscle L_Temporalis    { source = Temporal_L;  target = Mandible; max_force = 600.0 Nm; }
muscle R_Temporalis    { source = Temporal_R;  target = Mandible; max_force = 600.0 Nm; }

// --- CHEST & SHOULDERS ---
muscle L_PectoralisMaj { source = Sternum;   target = L_Humerus; max_force = 2000.0 Nm; }
muscle R_PectoralisMaj { source = Sternum;   target = R_Humerus; max_force = 2000.0 Nm; }
muscle L_Latissimus    { source = T12;       target = L_Humerus; max_force = 2200.0 Nm; }
muscle R_Latissimus    { source = T12;       target = R_Humerus; max_force = 2200.0 Nm; }

// --- BREATHING ---
muscle Diaphragm_L     { source = T12;       target = L_Rib6;    max_force = 500.0 Nm; }
muscle Diaphragm_R     { source = T12;       target = R_Rib6;    max_force = 500.0 Nm; }
