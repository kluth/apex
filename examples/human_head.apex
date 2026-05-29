// === HUMAN HEAD & NECK: 29 BONES ===
// Cranial Bones (8)
bone Frontal    { mass = 0.5 kg; position = (0.0, 1.95, 0.08); }
bone Parietal_L { mass = 0.4 kg; position = (-0.08, 1.95, 0.0); }
bone Parietal_R { mass = 0.4 kg; position = (0.08, 1.95, 0.0); }
bone Temporal_L { mass = 0.3 kg; position = (-0.08, 1.85, 0.0); }
bone Temporal_R { mass = 0.3 kg; position = (0.08, 1.85, 0.0); }
bone Occipital  { mass = 0.5 kg; position = (0.0, 1.85, -0.05); }
bone Sphenoid   { mass = 0.2 kg; position = (0.0, 1.90, 0.0); }
bone Ethmoid    { mass = 0.1 kg; position = (0.0, 1.88, 0.05); }

// ANATOMICAL WIRING: SKULL CAGE
joint J_Skull_Base { source = C1; target = Occipital; }
joint J_Skull_Occ_T_L { source = Occipital; target = Temporal_L; }
joint J_Skull_Occ_T_R { source = Occipital; target = Temporal_R; }
joint J_Skull_T_P_L { source = Temporal_L; target = Parietal_L; }
joint J_Skull_T_P_R { source = Temporal_R; target = Parietal_R; }
joint J_Skull_P_F_L { source = Parietal_L; target = Frontal; }
joint J_Skull_P_F_R { source = Parietal_R; target = Frontal; }
joint J_Skull_Front_Mid { source = Parietal_L; target = Parietal_R; }

// Facial Bones (14)
bone Mandible    { mass = 0.4 kg; position = (0.0, 1.75, 0.1); }
bone Maxilla_L   { mass = 0.2 kg; position = (-0.05, 1.82, 0.1); }
bone Maxilla_R   { mass = 0.2 kg; position = (0.05, 1.82, 0.1); }
bone Zygomatic_L { mass = 0.1 kg; position = (-0.08, 1.85, 0.08); }
bone Zygomatic_R { mass = 0.1 kg; position = (0.08, 1.85, 0.08); }
bone Nasal_L     { mass = 0.05 kg; position = (-0.02, 1.88, 0.12); }
bone Nasal_R     { mass = 0.05 kg; position = (0.02, 1.88, 0.12); }
bone Lacrimal_L  { mass = 0.05 kg; position = (-0.03, 1.86, 0.11); }
bone Lacrimal_R  { mass = 0.05 kg; position = (0.03, 1.86, 0.11); }
bone Palatine_L  { mass = 0.05 kg; position = (-0.03, 1.80, 0.1); }
bone Palatine_R  { mass = 0.05 kg; position = (0.03, 1.80, 0.1); }
bone InfNasal_L  { mass = 0.05 kg; position = (-0.02, 1.83, 0.11); }
bone InfNasal_R  { mass = 0.05 kg; position = (0.02, 1.83, 0.11); }
bone Vomer       { mass = 0.05 kg; position = (0.0, 1.82, 0.11); }

// ANATOMICAL WIRING: FACE
joint J_Face_F_N_L { source = Frontal; target = Nasal_L; }
joint J_Face_F_N_R { source = Frontal; target = Nasal_R; }
joint J_Face_T_Z_L { source = Temporal_L; target = Zygomatic_L; }
joint J_Face_T_Z_R { source = Temporal_R; target = Zygomatic_R; }
joint J_Face_Z_M_L { source = Zygomatic_L; target = Maxilla_L; }
joint J_Face_Z_M_R { source = Zygomatic_R; target = Maxilla_R; }
joint J_Face_Jaw_L { source = Temporal_L; target = Mandible; }
joint J_Face_Jaw_R { source = Temporal_R; target = Mandible; }
joint J_Face_M_M { source = Maxilla_L; target = Maxilla_R; }

// Ear Ossicles (6)
bone Malleus_L { mass = 0.01 kg; position = (-0.08, 1.85, 0.02); }
bone Incus_L   { mass = 0.01 kg; position = (-0.08, 1.85, 0.03); }
bone Stapes_L  { mass = 0.01 kg; position = (-0.08, 1.85, 0.04); }
bone Malleus_R { mass = 0.01 kg; position = (0.08, 1.85, 0.02); }
bone Incus_R   { mass = 0.01 kg; position = (0.08, 1.85, 0.03); }
bone Stapes_R  { mass = 0.01 kg; position = (0.08, 1.85, 0.04); }

joint J_Ear_L_1 { source = Temporal_L; target = Malleus_L; }
joint J_Ear_L_2 { source = Malleus_L; target = Incus_L; }
joint J_Ear_L_3 { source = Incus_L; target = Stapes_L; }
joint J_Ear_R_1 { source = Temporal_R; target = Malleus_R; }
joint J_Ear_R_2 { source = Malleus_R; target = Incus_R; }
joint J_Ear_R_3 { source = Incus_R; target = Stapes_R; }

// Hyoid (1)
bone Hyoid { mass = 0.1 kg; position = (0.0, 1.70, 0.08); }
joint J_Hyoid { source = C4; target = Hyoid; }
