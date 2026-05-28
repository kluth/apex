// === MUSCLES: LEGS & FEET (High Fidelity) ===

// --- HIP POWER & ROTATION ---
muscle L_GluteusMax    { source = Sacrum;       target = L_Femur;   max_force = 3000.0 Nm; }
muscle R_GluteusMax    { source = Sacrum;       target = R_Femur;   max_force = 3000.0 Nm; }
muscle L_GluteusMed    { source = L_Innominate; target = L_Femur;   max_force = 1500.0 Nm; }
muscle R_GluteusMed    { source = R_Innominate; target = R_Femur;   max_force = 1500.0 Nm; }
muscle L_AdductorMag   { source = L_Innominate; target = L_Femur;   max_force = 1800.0 Nm; }
muscle R_AdductorMag   { source = R_Innominate; target = R_Femur;   max_force = 1800.0 Nm; }
muscle L_PsoasMaj      { source = L1;           target = L_Femur;   max_force = 1200.0 Nm; }
muscle R_PsoasMaj      { source = L1;           target = R_Femur;   max_force = 1200.0 Nm; }

// --- THIGH (Knee Extensors/Flexors) ---
// Quadriceps
muscle L_RectusFem     { source = L_Innominate; target = L_Patella; max_force = 1500.0 Nm; }
muscle R_RectusFem     { source = R_Innominate; target = R_Patella; max_force = 1500.0 Nm; }
muscle L_VastusLat     { source = L_Femur;      target = L_Patella; max_force = 2000.0 Nm; }
muscle R_VastusLat     { source = R_Femur;      target = R_Patella; max_force = 2000.0 Nm; }
muscle L_VastusMed     { source = L_Femur;      target = L_Patella; max_force = 1800.0 Nm; }
muscle R_VastusMed     { source = R_Femur;      target = R_Patella; max_force = 1800.0 Nm; }
// Hamstrings
muscle L_BicepsFem     { source = L_Innominate; target = L_Fibula;  max_force = 1200.0 Nm; }
muscle R_BicepsFem     { source = R_Innominate; target = R_Fibula;  max_force = 1200.0 Nm; }
muscle L_Semitendin    { source = L_Innominate; target = L_Tibia;   max_force = 1100.0 Nm; }
muscle R_Semitendin    { source = R_Innominate; target = R_Tibia;   max_force = 1100.0 Nm; }

// --- LOWER LEG (Ankle & Toe Power) ---
muscle L_Gastrocnem    { source = L_Femur;      target = L_Calcaneus;max_force = 2500.0 Nm; }
muscle R_Gastrocnem    { source = R_Femur;      target = R_Calcaneus;max_force = 2500.0 Nm; }
muscle L_Soleus        { source = L_Tibia;      target = L_Calcaneus;max_force = 2200.0 Nm; }
muscle R_Soleus        { source = R_Tibia;      target = R_Calcaneus;max_force = 2200.0 Nm; }
muscle L_TibialisAnt   { source = L_Tibia;      target = L_Meta_1;   max_force = 800.0 Nm; }
muscle R_TibialisAnt   { source = R_Tibia;      target = R_Meta_1;   max_force = 800.0 Nm; }

// --- INTRINSIC FEET (Arch & Grip) ---
muscle L_AbductorHall  { source = L_Calcaneus;  target = L_Toe1_P;   max_force = 300.0 Nm; }
muscle R_AbductorHall  { source = R_Calcaneus;  target = R_Toe1_P;   max_force = 300.0 Nm; }
muscle L_FlexDigBrev   { source = L_Calcaneus;  target = L_Toe3_P;   max_force = 250.0 Nm; }
muscle R_FlexDigBrev   { source = R_Calcaneus;  target = R_Toe3_P;   max_force = 250.0 Nm; }
