// === MUSCLES: TORSO, SPINE & NECK (High Fidelity) ===

// --- ABDOMINAL WALL ---
muscle L_RectusAbd     { source = Sternum;   target = Pelvis;    max_force = 1200.0 Nm; }
muscle R_RectusAbd     { source = Sternum;   target = Pelvis;    max_force = 1200.0 Nm; }
muscle L_ExtOblique    { source = L_Rib6;    target = Pelvis;    max_force = 800.0 Nm; }
muscle R_ExtOblique    { source = R_Rib6;    target = Pelvis;    max_force = 800.0 Nm; }
muscle L_IntOblique    { source = L_Rib10;   target = Pelvis;    max_force = 700.0 Nm; }
muscle R_IntOblique    { source = R_Rib10;   target = Pelvis;    max_force = 700.0 Nm; }

// --- BACK & SPINE (Posterior Chain) ---
// Erector Spinae Cluster (Longissimus/Spinalis)
muscle L_Erector_L     { source = Sacrum;    target = L1;        max_force = 1500.0 Nm; }
muscle R_Erector_L     { source = Sacrum;    target = L1;        max_force = 1500.0 Nm; }
muscle L_Erector_T     { source = L1;        target = T6;        max_force = 1200.0 Nm; }
muscle R_Erector_T     { source = L1;        target = T6;        max_force = 1200.0 Nm; }
muscle L_Erector_C     { source = T6;        target = C4;        max_force = 800.0 Nm; }
muscle R_Erector_C     { source = T6;        target = C4;        max_force = 800.0 Nm; }

// Deep Stabilizers (Multifidus proxies)
muscle L_Multifid_1    { source = L5;        target = L3;        max_force = 400.0 Nm; }
muscle R_Multifid_1    { source = L5;        target = L3;        max_force = 400.0 Nm; }
muscle L_Multifid_2    { source = L3;        target = L1;        max_force = 400.0 Nm; }
muscle R_Multifid_2    { source = L3;        target = L1;        max_force = 400.0 Nm; }
muscle L_Multifid_3    { source = T12;       target = T9;        max_force = 300.0 Nm; }
muscle R_Multifid_3    { source = T12;       target = T9;        max_force = 300.0 Nm; }

// --- CHEST & SHOULDERS ---
muscle L_PectoralisMaj { source = Sternum;   target = L_Humerus; max_force = 2000.0 Nm; }
muscle R_PectoralisMaj { source = Sternum;   target = R_Humerus; max_force = 2000.0 Nm; }
muscle L_PectoralisMin { source = L_Rib3;    target = L_Scapula; max_force = 600.0 Nm; }
muscle R_PectoralisMin { source = R_Rib3;    target = R_Scapula; max_force = 600.0 Nm; }
muscle L_Latissimus    { source = T12;       target = L_Humerus; max_force = 2200.0 Nm; }
muscle R_Latissimus    { source = T12;       target = R_Humerus; max_force = 2200.0 Nm; }
muscle L_Trapezius_U   { source = Occipital; target = L_Clavicle;max_force = 1200.0 Nm; }
muscle R_Trapezius_U   { source = Occipital; target = R_Clavicle;max_force = 1200.0 Nm; }
muscle L_Trapezius_L   { source = T10;       target = L_Scapula; max_force = 1000.0 Nm; }
muscle R_Trapezius_L   { source = T10;       target = R_Scapula; max_force = 1000.0 Nm; }

// --- NECK & JAW ---
muscle L_SCM           { source = Sternum;   target = Temporal_L;max_force = 400.0 Nm; }
muscle R_SCM           { source = Sternum;   target = Temporal_R;max_force = 400.0 Nm; }
muscle L_Masseter      { source = Zygomatic_L; target = Mandible; max_force = 800.0 Nm; }
muscle R_Masseter      { source = Zygomatic_R; target = Mandible; max_force = 800.0 Nm; }
muscle L_Splenius      { source = T3;        target = Occipital; max_force = 500.0 Nm; }
muscle R_Splenius      { source = T3;        target = Occipital; max_force = 500.0 Nm; }

// --- BREATHING ---
muscle Diaphragm_L     { source = T12;       target = L_Rib6;    max_force = 500.0 Nm; }
muscle Diaphragm_R     { source = T12;       target = R_Rib6;    max_force = 500.0 Nm; }
