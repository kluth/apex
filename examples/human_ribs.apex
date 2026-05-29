// === HUMAN CORE: THE FULL RIBCAGE & STERNUM ===

bone Sternum { mass = 1.0 kg; position = (0.0, 1.45, 0.15); }
joint JStern { source = T1; target = Sternum; }

// LEFT RIBS (12)
bone L_Rib1  { mass = 0.2 kg; position = (-0.1, 1.61, 0.05); }
bone L_Rib2  { mass = 0.2 kg; position = (-0.12, 1.57, 0.08); }
bone L_Rib3  { mass = 0.2 kg; position = (-0.14, 1.53, 0.1); }
bone L_Rib4  { mass = 0.2 kg; position = (-0.16, 1.49, 0.12); }
bone L_Rib5  { mass = 0.2 kg; position = (-0.18, 1.45, 0.14); }
bone L_Rib6  { mass = 0.2 kg; position = (-0.2, 1.41, 0.15); }
bone L_Rib7  { mass = 0.2 kg; position = (-0.18, 1.37, 0.14); }
bone L_Rib8  { mass = 0.2 kg; position = (-0.16, 1.33, 0.12); }
bone L_Rib9  { mass = 0.2 kg; position = (-0.14, 1.29, 0.1); }
bone L_Rib10 { mass = 0.2 kg; position = (-0.12, 1.25, 0.08); }
bone L_Rib11 { mass = 0.2 kg; position = (-0.1, 1.21, 0.06); }
bone L_Rib12 { mass = 0.2 kg; position = (-0.08, 1.17, 0.04); }

// Posterior connections (to Spine)
joint JLP1 { source = T1; target = L_Rib1; }
joint JLP2 { source = T2; target = L_Rib2; }
joint JLP3 { source = T3; target = L_Rib3; }
joint JLP4 { source = T4; target = L_Rib4; }
joint JLP5 { source = T5; target = L_Rib5; }
joint JLP6 { source = T6; target = L_Rib6; }
joint JLP7 { source = T7; target = L_Rib7; }
joint JLP8 { source = T8; target = L_Rib8; }
joint JLP9 { source = T9; target = L_Rib9; }
joint JLP10 { source = T10; target = L_Rib10; }
joint JLP11 { source = T11; target = L_Rib11; }
joint JLP12 { source = T12; target = L_Rib12; }

// Anterior connections (forming the Basket)
joint JLA1 { source = L_Rib1; target = Sternum; }
joint JLA2 { source = L_Rib2; target = Sternum; }
joint JLA3 { source = L_Rib3; target = Sternum; }
joint JLA4 { source = L_Rib4; target = Sternum; }
joint JLA5 { source = L_Rib5; target = Sternum; }
joint JLA6 { source = L_Rib6; target = Sternum; }
joint JLA7 { source = L_Rib7; target = Sternum; }
joint JLA8 { source = L_Rib8; target = L_Rib7; } // False ribs
joint JLA9 { source = L_Rib9; target = L_Rib8; }
joint JLA10 { source = L_Rib10; target = L_Rib9; }

// RIGHT RIBS (12)
bone R_Rib1  { mass = 0.2 kg; position = (0.1, 1.61, 0.05); }
bone R_Rib2  { mass = 0.2 kg; position = (0.12, 1.57, 0.08); }
bone R_Rib3  { mass = 0.2 kg; position = (0.14, 1.53, 0.1); }
bone R_Rib4  { mass = 0.2 kg; position = (0.16, 1.49, 0.12); }
bone R_Rib5  { mass = 0.2 kg; position = (0.18, 1.45, 0.14); }
bone R_Rib6  { mass = 0.2 kg; position = (0.2, 1.41, 0.15); }
bone R_Rib7  { mass = 0.2 kg; position = (0.18, 1.37, 0.14); }
bone R_Rib8  { mass = 0.2 kg; position = (0.16, 1.33, 0.12); }
bone R_Rib9  { mass = 0.2 kg; position = (0.14, 1.29, 0.1); }
bone R_Rib10 { mass = 0.2 kg; position = (0.12, 1.25, 0.08); }
bone R_Rib11 { mass = 0.2 kg; position = (0.1, 1.21, 0.06); }
bone R_Rib12 { mass = 0.2 kg; position = (0.08, 1.17, 0.04); }

// Posterior
joint JRP1 { source = T1; target = R_Rib1; }
joint JRP2 { source = T2; target = R_Rib2; }
joint JRP3 { source = T3; target = R_Rib3; }
joint JRP4 { source = T4; target = R_Rib4; }
joint JRP5 { source = T5; target = R_Rib5; }
joint JRP6 { source = T6; target = R_Rib6; }
joint JRP7 { source = T7; target = R_Rib7; }
joint JRP8 { source = T8; target = R_Rib8; }
joint JRP9 { source = T9; target = R_Rib9; }
joint JRP10 { source = T10; target = R_Rib10; }
joint JRP11 { source = T11; target = R_Rib11; }
joint JRP12 { source = T12; target = R_Rib12; }

// Anterior
joint JRA1 { source = R_Rib1; target = Sternum; }
joint JRA2 { source = R_Rib2; target = Sternum; }
joint JRA3 { source = R_Rib3; target = Sternum; }
joint JRA4 { source = R_Rib4; target = Sternum; }
joint JRA5 { source = R_Rib5; target = Sternum; }
joint JRA6 { source = R_Rib6; target = Sternum; }
joint JRA7 { source = R_Rib7; target = Sternum; }
joint JRA8 { source = R_Rib8; target = R_Rib7; }
joint JRA9 { source = R_Rib9; target = R_Rib8; }
joint JRA10 { source = R_Rib10; target = R_Rib9; }
