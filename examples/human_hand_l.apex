// === LEFT HAND: ANATOMICAL WIREFRAME ===

// Carpals - Proximal Row
bone L_Scaphoid   { mass = 0.05 kg; position = (-0.68, 1.51, 0.04); }
bone L_Lunate     { mass = 0.05 kg; position = (-0.68, 1.51, 0.02); }
bone L_Triquetrum { mass = 0.05 kg; position = (-0.68, 1.51, 0.0); }
bone L_Pisiform   { mass = 0.05 kg; position = (-0.68, 1.51, -0.02); }

// Carpals - Distal Row
bone L_Trapezium  { mass = 0.05 kg; position = (-0.70, 1.50, 0.05); }
bone L_Trapezoid  { mass = 0.05 kg; position = (-0.70, 1.50, 0.03); }
bone L_Capitate   { mass = 0.05 kg; position = (-0.70, 1.50, 0.01); }
bone L_Hamate     { mass = 0.05 kg; position = (-0.70, 1.50, -0.02); }

// Wrist connections
joint J_LWr1 { source = L_Radius; target = L_Scaphoid; }
joint J_LWr2 { source = L_Ulna;   target = L_Triquetrum; }

// Carpal Inter-connections (The Gitter)
joint J_LC1 { source = L_Scaphoid; target = L_Lunate; }
joint J_LC2 { source = L_Lunate;   target = L_Triquetrum; }
joint J_LC3 { source = L_Triquetrum; target = L_Pisiform; }
joint J_LC4 { source = L_Scaphoid; target = L_Trapezium; }
joint J_LC5 { source = L_Lunate;   target = L_Capitate; }
joint J_LC6 { source = L_Trapezium; target = L_Trapezoid; }
joint J_LC7 { source = L_Trapezoid; target = L_Capitate; }
joint J_LC8 { source = L_Capitate;  target = L_Hamate; }

// Metacarpals
bone L_Meta1 { mass = 0.1 kg; position = (-0.72, 1.52, 0.05); }
bone L_Meta2 { mass = 0.1 kg; position = (-0.75, 1.5, 0.03); }
bone L_Meta3 { mass = 0.1 kg; position = (-0.76, 1.48, 0.0); }
bone L_Meta4 { mass = 0.1 kg; position = (-0.75, 1.46, -0.03); }
bone L_Meta5 { mass = 0.1 kg; position = (-0.73, 1.44, -0.05); }

joint JLM1 { source = L_Trapezium; target = L_Meta1; }
joint JLM2 { source = L_Trapezoid; target = L_Meta2; }
joint JLM3 { source = L_Capitate;  target = L_Meta3; }
joint JLM4 { source = L_Hamate;    target = L_Meta4; }
joint JLM5 { source = L_Hamate;    target = L_Meta5; }

// Thumb
bone L_Thumb_P { mass = 0.05 kg; position = (-0.75, 1.55, 0.08); }
bone L_Thumb_D { mass = 0.05 kg; position = (-0.78, 1.58, 0.1); }
joint JLT1 { source = L_Meta1;   target = L_Thumb_P; }
joint JLT2 { source = L_Thumb_P; target = L_Thumb_D; }

// Fingers
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
