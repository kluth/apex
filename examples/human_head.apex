// === HUMAN HEAD & NECK: 29 BONES ===
// Cranial Bones (8)
bone Frontal { mass = 0.5 kg; position = (0.0, 1.95, 0.1); }
bone Parietal_L { mass = 0.4 kg; position = (-0.08, 1.95, 0.0); }
bone Parietal_R { mass = 0.4 kg; position = (0.08, 1.95, 0.0); }
bone Temporal_L { mass = 0.3 kg; position = (-0.08, 1.85, 0.0); }
bone Temporal_R { mass = 0.3 kg; position = (0.08, 1.85, 0.0); }
bone Occipital { mass = 0.5 kg; position = (0.0, 1.85, -0.1); }
bone Sphenoid { mass = 0.2 kg; position = (0.0, 1.9, 0.0); }
bone Ethmoid { mass = 0.1 kg; position = (0.0, 1.88, 0.05); }

joint J_Skull1 { source = C1; target = Occipital; }
joint J_Skull2 { source = Occipital; target = Parietal_L; }
joint J_Skull3 { source = Occipital; target = Parietal_R; }
joint J_Skull4 { source = Occipital; target = Temporal_L; }
joint J_Skull5 { source = Occipital; target = Temporal_R; }
joint J_Skull6 { source = Occipital; target = Sphenoid; }
joint J_Skull7 { source = Sphenoid; target = Frontal; }
joint J_Skull8 { source = Sphenoid; target = Ethmoid; }

// Facial Bones (14)
bone Mandible { mass = 0.4 kg; position = (0.0, 1.75, 0.1); }
bone Maxilla_L { mass = 0.2 kg; position = (-0.05, 1.82, 0.12); }
bone Maxilla_R { mass = 0.2 kg; position = (0.05, 1.82, 0.12); }
bone Zygomatic_L { mass = 0.1 kg; position = (-0.08, 1.85, 0.1); }
bone Zygomatic_R { mass = 0.1 kg; position = (0.08, 1.85, 0.1); }
bone Nasal_L { mass = 0.05 kg; position = (-0.02, 1.88, 0.14); }
bone Nasal_R { mass = 0.05 kg; position = (0.02, 1.88, 0.14); }
bone Lacrimal_L { mass = 0.05 kg; position = (-0.03, 1.86, 0.13); }
bone Lacrimal_R { mass = 0.05 kg; position = (0.03, 1.86, 0.13); }
bone Palatine_L { mass = 0.05 kg; position = (-0.03, 1.80, 0.1); }
bone Palatine_R { mass = 0.05 kg; position = (0.03, 1.80, 0.1); }
bone InfNasalConcha_L { mass = 0.05 kg; position = (-0.02, 1.83, 0.13); }
bone InfNasalConcha_R { mass = 0.05 kg; position = (0.02, 1.83, 0.13); }
bone Vomer { mass = 0.05 kg; position = (0.0, 1.82, 0.13); }

joint J_Face1 { source = Temporal_L; target = Mandible; }
joint J_Face2 { source = Frontal; target = Maxilla_L; }
joint J_Face3 { source = Frontal; target = Maxilla_R; }
joint J_Face4 { source = Maxilla_L; target = Zygomatic_L; }
joint J_Face5 { source = Maxilla_R; target = Zygomatic_R; }
joint J_Face6 { source = Frontal; target = Nasal_L; }
joint J_Face7 { source = Frontal; target = Nasal_R; }
joint J_Face8 { source = Ethmoid; target = Lacrimal_L; }
joint J_Face9 { source = Ethmoid; target = Lacrimal_R; }
joint J_Face10 { source = Maxilla_L; target = Palatine_L; }
joint J_Face11 { source = Maxilla_R; target = Palatine_R; }
joint J_Face12 { source = Ethmoid; target = InfNasalConcha_L; }
joint J_Face13 { source = Ethmoid; target = InfNasalConcha_R; }
joint J_Face14 { source = Ethmoid; target = Vomer; }

// Ear Ossicles (6)
bone Malleus_L { mass = 0.01 kg; position = (-0.08, 1.85, 0.02); }
bone Incus_L { mass = 0.01 kg; position = (-0.08, 1.85, 0.03); }
bone Stapes_L { mass = 0.01 kg; position = (-0.08, 1.85, 0.04); }
bone Malleus_R { mass = 0.01 kg; position = (0.08, 1.85, 0.02); }
bone Incus_R { mass = 0.01 kg; position = (0.08, 1.85, 0.03); }
bone Stapes_R { mass = 0.01 kg; position = (0.08, 1.85, 0.04); }

joint J_Ear1 { source = Temporal_L; target = Malleus_L; }
joint J_Ear2 { source = Malleus_L; target = Incus_L; }
joint J_Ear3 { source = Incus_L; target = Stapes_L; }
joint J_Ear4 { source = Temporal_R; target = Malleus_R; }
joint J_Ear5 { source = Malleus_R; target = Incus_R; }
joint J_Ear6 { source = Incus_R; target = Stapes_R; }

// Hyoid (1)
bone Hyoid { mass = 0.1 kg; position = (0.0, 1.70, 0.08); }
joint J_Hyoid { source = C4; target = Hyoid; }
