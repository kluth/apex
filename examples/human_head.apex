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

// Wireframe Skull Cage
joint J_Sk1 { source = C1; target = Occipital; }
joint J_Sk2 { source = Occipital; target = Temporal_L; }
joint J_Sk3 { source = Occipital; target = Temporal_R; }
joint J_Sk4 { source = Temporal_L; target = Parietal_L; }
joint J_Sk5 { source = Temporal_R; target = Parietal_R; }
joint J_Sk6 { source = Parietal_L; target = Frontal; }
joint J_Sk7 { source = Parietal_R; target = Frontal; }
joint J_Sk8 { source = Occipital; target = Sphenoid; }
joint J_Sk9 { source = Sphenoid; target = Ethmoid; }
joint J_Sk10 { source = Ethmoid; target = Frontal; }

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

// Wireframe Face
joint J_Fa1 { source = Frontal; target = Nasal_L; }
joint J_Fa2 { source = Frontal; target = Nasal_R; }
joint J_Fa3 { source = Temporal_L; target = Zygomatic_L; }
joint J_Fa4 { source = Temporal_R; target = Zygomatic_R; }
joint J_Fa5 { source = Zygomatic_L; target = Maxilla_L; }
joint J_Fa6 { source = Zygomatic_R; target = Maxilla_R; }
joint J_Fa7 { source = Maxilla_L; target = Mandible; }
joint J_Fa8 { source = Maxilla_R; target = Mandible; }
// Jaw hinge
joint J_Fa9 { source = Temporal_L; target = Mandible; }
joint J_Fa10 { source = Temporal_R; target = Mandible; }
// Inner face details
joint J_Fa11 { source = Ethmoid; target = Lacrimal_L; }
joint J_Fa12 { source = Ethmoid; target = Lacrimal_R; }
joint J_Fa13 { source = Maxilla_L; target = Palatine_L; }
joint J_Fa14 { source = Maxilla_R; target = Palatine_R; }
joint J_Fa15 { source = Ethmoid; target = InfNasal_L; }
joint J_Fa16 { source = Ethmoid; target = InfNasal_R; }
joint J_Fa17 { source = Ethmoid; target = Vomer; }

// Ear Ossicles (6)
bone Malleus_L { mass = 0.01 kg; position = (-0.08, 1.85, 0.02); }
bone Incus_L   { mass = 0.01 kg; position = (-0.08, 1.85, 0.03); }
bone Stapes_L  { mass = 0.01 kg; position = (-0.08, 1.85, 0.04); }
bone Malleus_R { mass = 0.01 kg; position = (0.08, 1.85, 0.02); }
bone Incus_R   { mass = 0.01 kg; position = (0.08, 1.85, 0.03); }
bone Stapes_R  { mass = 0.01 kg; position = (0.08, 1.85, 0.04); }

joint J_Ear1 { source = Temporal_L; target = Malleus_L; }
joint J_Ear2 { source = Malleus_L; target = Incus_L; }
joint J_Ear3 { source = Incus_L; target = Stapes_L; }
joint J_Ear4 { source = Temporal_R; target = Malleus_R; }
joint J_Ear5 { source = Malleus_R; target = Incus_R; }
joint J_Ear6 { source = Incus_R; target = Stapes_R; }

// Hyoid (1)
bone Hyoid { mass = 0.1 kg; position = (0.0, 1.70, 0.08); }
joint J_Hyoid { source = C4; target = Hyoid; }
