// === MUSCLES: TORSO & SPINE ===

// Core (Flexion/Stability)
muscle L_RectusAbd     { source = Sternum;   target = Pelvis;    max_force = 1200.0 Nm; }
muscle R_RectusAbd     { source = Sternum;   target = Pelvis;    max_force = 1200.0 Nm; }

// Back (Extension/Posture)
muscle L_ErectorSpinae { source = Sacrum;    target = T1;        max_force = 2000.0 Nm; }
muscle R_ErectorSpinae { source = Sacrum;    target = T1;        max_force = 2000.0 Nm; }

// Shoulder/Neck support
muscle L_Trapezius     { source = T1;        target = L_Scapula; max_force = 1000.0 Nm; }
muscle R_Trapezius     { source = T1;        target = R_Scapula; max_force = 1000.0 Nm; }

// Breathing (Simplified Diaphragm proxy)
muscle Diaphragm_L     { source = T12;       target = L_Rib6;    max_force = 500.0 Nm; }
muscle Diaphragm_R     { source = T12;       target = R_Rib6;    max_force = 500.0 Nm; }
