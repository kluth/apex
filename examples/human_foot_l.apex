// === LEFT FOOT: ANATOMICAL WIREFRAME ===

// Tarsals (7)
bone L_Talus      { mass = 0.2 kg; position = (-0.15, 0.12, -0.02); }
bone L_Calcaneus  { mass = 0.2 kg; position = (-0.15, 0.08, -0.05); }
bone L_Navicular  { mass = 0.1 kg; position = (-0.14, 0.08, 0.02); }
bone L_Cuboid     { mass = 0.1 kg; position = (-0.18, 0.08, 0.0); }
bone L_Cunei_Med  { mass = 0.1 kg; position = (-0.13, 0.06, 0.05); }
bone L_Cunei_Int  { mass = 0.1 kg; position = (-0.15, 0.06, 0.05); }
bone L_Cunei_Lat  { mass = 0.1 kg; position = (-0.17, 0.06, 0.05); }

// Ankle & Foot Connections
joint JLT1 { source = L_Tibia; target = L_Talus; }
joint JLT2 { source = L_Talus; target = L_Calcaneus; }
joint JLT3 { source = L_Talus; target = L_Navicular; }
joint JLT4 { source = L_Calcaneus; target = L_Cuboid; }
joint JLT5 { source = L_Navicular; target = L_Cunei_Med; }
joint JLT6 { source = L_Navicular; target = L_Cunei_Int; }
joint JLT7 { source = L_Navicular; target = L_Cunei_Lat; }
joint JLT8 { source = L_Cuboid;    target = L_Cunei_Lat; }

// Metatarsals (5)
bone L_Meta_1 { mass = 0.1 kg; position = (-0.13, 0.05, 0.12); }
bone L_Meta_2 { mass = 0.1 kg; position = (-0.15, 0.05, 0.13); }
bone L_Meta_3 { mass = 0.1 kg; position = (-0.17, 0.05, 0.14); }
bone L_Meta_4 { mass = 0.1 kg; position = (-0.19, 0.05, 0.13); }
bone L_Meta_5 { mass = 0.1 kg; position = (-0.21, 0.05, 0.11); }

joint JLFM1 { source = L_Cunei_Med; target = L_Meta_1; }
joint JLFM2 { source = L_Cunei_Int; target = L_Meta_2; }
joint JLFM3 { source = L_Cunei_Lat; target = L_Meta_3; }
joint JLFM4 { source = L_Cuboid;    target = L_Meta_4; }
joint JLFM5 { source = L_Cuboid;    target = L_Meta_5; }

// Toes
bone L_Toe1_P { mass = 0.05 kg; position = (-0.13, 0.02, 0.18); }
bone L_Toe1_D { mass = 0.05 kg; position = (-0.13, 0.01, 0.22); }
joint JLT9 { source = L_Meta_1; target = L_Toe1_P; }
joint JLT10 { source = L_Toe1_P; target = L_Toe1_D; }

bone L_Toe2_P { mass = 0.05 kg; position = (-0.15, 0.02, 0.19); }
bone L_Toe2_M { mass = 0.05 kg; position = (-0.15, 0.01, 0.21); }
bone L_Toe2_D { mass = 0.05 kg; position = (-0.15, 0.01, 0.23); }
joint JLT11 { source = L_Meta_2; target = L_Toe2_P; }
joint JLT12 { source = L_Toe2_P; target = L_Toe2_M; }
joint JLT13 { source = L_Toe2_M; target = L_Toe2_D; }

bone L_Toe3_P { mass = 0.05 kg; position = (-0.17, 0.02, 0.20); }
bone L_Toe3_M { mass = 0.05 kg; position = (-0.17, 0.01, 0.22); }
bone L_Toe3_D { mass = 0.05 kg; position = (-0.17, 0.01, 0.24); }
joint JLT14 { source = L_Meta_3; target = L_Toe3_P; }
joint JLT15 { source = L_Toe3_P; target = L_Toe3_M; }
joint JLT16 { source = L_Toe3_M; target = L_Toe3_D; }

bone L_Toe4_P { mass = 0.05 kg; position = (-0.19, 0.02, 0.19); }
bone L_Toe4_M { mass = 0.05 kg; position = (-0.19, 0.01, 0.21); }
bone L_Toe4_D { mass = 0.05 kg; position = (-0.19, 0.01, 0.23); }
joint JLT17 { source = L_Meta_4; target = L_Toe4_P; }
joint JLT18 { source = L_Toe4_P; target = L_Toe4_M; }
joint JLT19 { source = L_Toe4_M; target = L_Toe4_D; }

bone L_Toe5_P { mass = 0.05 kg; position = (-0.21, 0.02, 0.17); }
bone L_Toe5_M { mass = 0.05 kg; position = (-0.21, 0.01, 0.19); }
bone L_Toe5_D { mass = 0.05 kg; position = (-0.21, 0.01, 0.20); }
joint JLT20 { source = L_Meta_5; target = L_Toe5_P; }
joint JLT21 { source = L_Toe5_P; target = L_Toe5_M; }
joint JLT22 { source = L_Toe5_M; target = L_Toe5_D; }
