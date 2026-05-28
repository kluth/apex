// === LEFT HAND: METACARPALS & PHALANGES ===

// Metacarpals
bone L_Meta1 { mass = 0.1 kg; position = (-0.72, 1.52, 0.05); }
bone L_Meta2 { mass = 0.1 kg; position = (-0.75, 1.5, 0.03); }
bone L_Meta3 { mass = 0.1 kg; position = (-0.76, 1.48, 0.0); }
bone L_Meta4 { mass = 0.1 kg; position = (-0.75, 1.46, -0.03); }
bone L_Meta5 { mass = 0.1 kg; position = (-0.73, 1.44, -0.05); }

joint JLM1 { source = L_Carpal; target = L_Meta1; }
joint JLM2 { source = L_Carpal; target = L_Meta2; }
joint JLM3 { source = L_Carpal; target = L_Meta3; }
joint JLM4 { source = L_Carpal; target = L_Meta4; }
joint JLM5 { source = L_Carpal; target = L_Meta5; }

// Thumb (2 Phalanges)
bone L_Thumb_P { mass = 0.05 kg; position = (-0.75, 1.55, 0.08); }
bone L_Thumb_D { mass = 0.05 kg; position = (-0.78, 1.58, 0.1); }
joint JLT1 { source = L_Meta1;   target = L_Thumb_P; }
joint JLT2 { source = L_Thumb_P; target = L_Thumb_D; }

// Fingers (3 Phalanges each)
bone L_Ind_P { mass = 0.05 kg; position = (-0.8, 1.5, 0.04); }
bone L_Ind_M { mass = 0.05 kg; position = (-0.84, 1.5, 0.04); }
bone L_Ind_D { mass = 0.05 kg; position = (-0.88, 1.5, 0.04); }
joint JLI1 { source = L_Meta2; target = L_Ind_P; }
joint JLI2 { source = L_Ind_P; target = L_Ind_M; }
joint JLI3 { source = L_Ind_M; target = L_Ind_D; }

bone L_Mid_P { mass = 0.05 kg; position = (-0.82, 1.48, 0.0); }
bone L_Mid_M { mass = 0.05 kg; position = (-0.87, 1.48, 0.0); }
bone L_Mid_D { mass = 0.05 kg; position = (-0.92, 1.48, 0.0); }
joint JLM6 { source = L_Meta3; target = L_Mid_P; }
joint JLM7 { source = L_Mid_P; target = L_Mid_M; }
joint JLM8 { source = L_Mid_M; target = L_Mid_D; }

bone L_Rng_P { mass = 0.05 kg; position = (-0.8, 1.46, -0.04); }
bone L_Rng_M { mass = 0.05 kg; position = (-0.84, 1.46, -0.04); }
bone L_Rng_D { mass = 0.05 kg; position = (-0.88, 1.46, -0.04); }
joint JLR1 { source = L_Meta4; target = L_Rng_P; }
joint JLR2 { source = L_Rng_P; target = L_Rng_M; }
joint JLR3 { source = L_Rng_M; target = L_Rng_D; }

bone L_Pnk_P { mass = 0.05 kg; position = (-0.78, 1.44, -0.07); }
bone L_Pnk_M { mass = 0.05 kg; position = (-0.82, 1.44, -0.07); }
bone L_Pnk_D { mass = 0.05 kg; position = (-0.86, 1.44, -0.07); }
joint JLP1 { source = L_Meta5; target = L_Pnk_P; }
joint JLP2 { source = L_Pnk_P; target = L_Pnk_M; }
joint JLP3 { source = L_Pnk_M; target = L_Pnk_D; }
