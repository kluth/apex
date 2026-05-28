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

joint JLR1 { source = T1; target = L_Rib1; }
joint JLR2 { source = T2; target = L_Rib2; }
joint JLR3 { source = T3; target = L_Rib3; }
joint JLR4 { source = T4; target = L_Rib4; }
joint JLR5 { source = T5; target = L_Rib5; }
joint JLR6 { source = T6; target = L_Rib6; }
joint JLR7 { source = T7; target = L_Rib7; }
joint JLR8 { source = T8; target = L_Rib8; }
joint JLR9 { source = T9; target = L_Rib9; }
joint JLR10 { source = T10; target = L_Rib10; }
joint JLR11 { source = T11; target = L_Rib11; }
joint JLR12 { source = T12; target = L_Rib12; }

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

joint JRR1 { source = T1; target = R_Rib1; }
joint JRR2 { source = T2; target = R_Rib2; }
joint JRR3 { source = T3; target = R_Rib3; }
joint JRR4 { source = T4; target = R_Rib4; }
joint JRR5 { source = T5; target = R_Rib5; }
joint JRR6 { source = T6; target = R_Rib6; }
joint JRR7 { source = T7; target = R_Rib7; }
joint JRR8 { source = T8; target = R_Rib8; }
joint JRR9 { source = T9; target = R_Rib9; }
joint JRR10 { source = T10; target = R_Rib10; }
joint JRR11 { source = T11; target = R_Rib11; }
joint JRR12 { source = T12; target = R_Rib12; }
