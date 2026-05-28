// === RIGHT HAND: METACARPALS & PHALANGES ===

// Metacarpals
bone R_Meta1 { mass = 0.1 kg; position = (0.72, 1.52, 0.05); }
bone R_Meta2 { mass = 0.1 kg; position = (0.75, 1.5, 0.03); }
bone R_Meta3 { mass = 0.1 kg; position = (0.76, 1.48, 0.0); }
bone R_Meta4 { mass = 0.1 kg; position = (0.75, 1.46, -0.03); }
bone R_Meta5 { mass = 0.1 kg; position = (0.73, 1.44, -0.05); }

joint JRM1 { source = R_Carpal; target = R_Meta1; }
joint JRM2 { source = R_Carpal; target = R_Meta2; }
joint JRM3 { source = R_Carpal; target = R_Meta3; }
joint JRM4 { source = R_Carpal; target = R_Meta4; }
joint JRM5 { source = R_Carpal; target = R_Meta5; }

// Thumb (2 Phalanges)
bone R_Thumb_P { mass = 0.05 kg; position = (0.75, 1.55, 0.08); }
bone R_Thumb_D { mass = 0.05 kg; position = (0.78, 1.58, 0.1); }
joint JRT1 { source = R_Meta1;   target = R_Thumb_P; }
joint JRT2 { source = R_Thumb_P; target = R_Thumb_D; }

// Fingers (3 Phalanges each)
bone R_Ind_P { mass = 0.05 kg; position = (0.8, 1.5, 0.04); }
bone R_Ind_M { mass = 0.05 kg; position = (0.84, 1.5, 0.04); }
bone R_Ind_D { mass = 0.05 kg; position = (0.88, 1.5, 0.04); }
joint JRI1 { source = R_Meta2; target = R_Ind_P; }
joint JRI2 { source = R_Ind_P; target = R_Ind_M; }
joint JRI3 { source = R_Ind_M; target = R_Ind_D; }

bone R_Mid_P { mass = 0.05 kg; position = (0.82, 1.48, 0.0); }
bone R_Mid_M { mass = 0.05 kg; position = (0.87, 1.48, 0.0); }
bone R_Mid_D { mass = 0.05 kg; position = (0.92, 1.48, 0.0); }
joint JRM6 { source = R_Meta3; target = R_Mid_P; }
joint JRM7 { source = R_Mid_P; target = R_Mid_M; }
joint JRM8 { source = R_Mid_M; target = R_Mid_D; }

bone R_Rng_P { mass = 0.05 kg; position = (0.8, 1.46, -0.04); }
bone R_Rng_M { mass = 0.05 kg; position = (0.84, 1.46, -0.04); }
bone R_Rng_D { mass = 0.05 kg; position = (0.88, 1.46, -0.04); }
joint JRR1 { source = R_Meta4; target = R_Rng_P; }
joint JRR2 { source = R_Rng_P; target = R_Rng_M; }
joint JRR3 { source = R_Rng_M; target = R_Rng_D; }

bone R_Pnk_P { mass = 0.05 kg; position = (0.78, 1.44, -0.07); }
bone R_Pnk_M { mass = 0.05 kg; position = (0.82, 1.44, -0.07); }
bone R_Pnk_D { mass = 0.05 kg; position = (0.86, 1.44, -0.07); }
joint JRP1 { source = R_Meta5; target = R_Pnk_P; }
joint JRP2 { source = R_Pnk_P; target = R_Pnk_M; }
joint JRP3 { source = R_Pnk_M; target = R_Pnk_D; }
