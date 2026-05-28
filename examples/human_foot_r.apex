// === RIGHT FOOT: TARSALS, METATARSALS & PHALANGES ===

// Tarsals (7)
bone R_Talus      { mass = 0.2 kg; position = (0.15, 0.12, -0.02); }
bone R_Calcaneus  { mass = 0.2 kg; position = (0.15, 0.08, -0.05); }
bone R_Navicular  { mass = 0.1 kg; position = (0.14, 0.08, 0.02); }
bone R_Cuboid     { mass = 0.1 kg; position = (0.18, 0.08, 0.0); }
bone R_Cunei_Med  { mass = 0.1 kg; position = (0.13, 0.06, 0.05); }
bone R_Cunei_Int  { mass = 0.1 kg; position = (0.15, 0.06, 0.05); }
bone R_Cunei_Lat  { mass = 0.1 kg; position = (0.17, 0.06, 0.05); }

joint JRTar1 { source = R_Tibia; target = R_Talus; }
joint JRTar2 { source = R_Talus; target = R_Calcaneus; }
joint JRTar3 { source = R_Talus; target = R_Navicular; }
joint JRTar4 { source = R_Calcaneus; target = R_Cuboid; }
joint JRTar5 { source = R_Navicular; target = R_Cunei_Med; }
joint JRTar6 { source = R_Navicular; target = R_Cunei_Int; }
joint JRTar7 { source = R_Navicular; target = R_Cunei_Lat; }

// Metatarsals (5)
bone R_Meta_1 { mass = 0.1 kg; position = (0.13, 0.05, 0.12); }
bone R_Meta_2 { mass = 0.1 kg; position = (0.15, 0.05, 0.13); }
bone R_Meta_3 { mass = 0.1 kg; position = (0.17, 0.05, 0.14); }
bone R_Meta_4 { mass = 0.1 kg; position = (0.19, 0.05, 0.13); }
bone R_Meta_5 { mass = 0.1 kg; position = (0.21, 0.05, 0.11); }

joint JRFM1 { source = R_Cunei_Med; target = R_Meta_1; }
joint JRFM2 { source = R_Cunei_Int; target = R_Meta_2; }
joint JRFM3 { source = R_Cunei_Lat; target = R_Meta_3; }
joint JRFM4 { source = R_Cuboid;    target = R_Meta_4; }
joint JRFM5 { source = R_Cuboid;    target = R_Meta_5; }

// Toes (14 Phalanges: 2 for hallux, 3 for others)
bone R_Toe1_P { mass = 0.05 kg; position = (0.13, 0.02, 0.18); }
bone R_Toe1_D { mass = 0.05 kg; position = (0.13, 0.01, 0.22); }
joint JRT1 { source = R_Meta_1; target = R_Toe1_P; }
joint JRT2 { source = R_Toe1_P; target = R_Toe1_D; }

bone R_Toe2_P { mass = 0.05 kg; position = (0.15, 0.02, 0.19); }
bone R_Toe2_M { mass = 0.05 kg; position = (0.15, 0.01, 0.21); }
bone R_Toe2_D { mass = 0.05 kg; position = (0.15, 0.01, 0.23); }
joint JRT3 { source = R_Meta_2; target = R_Toe2_P; }
joint JRT4 { source = R_Toe2_P; target = R_Toe2_M; }
joint JRT5 { source = R_Toe2_M; target = R_Toe2_D; }

bone R_Toe3_P { mass = 0.05 kg; position = (0.17, 0.02, 0.20); }
bone R_Toe3_M { mass = 0.05 kg; position = (0.17, 0.01, 0.22); }
bone R_Toe3_D { mass = 0.05 kg; position = (0.17, 0.01, 0.24); }
joint JRT6 { source = R_Meta_3; target = R_Toe3_P; }
joint JRT7 { source = R_Toe3_P; target = R_Toe3_M; }
joint JRT8 { source = R_Toe3_M; target = R_Toe3_D; }

bone R_Toe4_P { mass = 0.05 kg; position = (0.19, 0.02, 0.19); }
bone R_Toe4_M { mass = 0.05 kg; position = (0.19, 0.01, 0.21); }
bone R_Toe4_D { mass = 0.05 kg; position = (0.19, 0.01, 0.23); }
joint JRT9 { source = R_Meta_4; target = R_Toe4_P; }
joint JRT10{ source = R_Toe4_P; target = R_Toe4_M; }
joint JRT11{ source = R_Toe4_M; target = R_Toe4_D; }

bone R_Toe5_P { mass = 0.05 kg; position = (0.21, 0.02, 0.17); }
bone R_Toe5_M { mass = 0.05 kg; position = (0.21, 0.01, 0.19); }
bone R_Toe5_D { mass = 0.05 kg; position = (0.21, 0.01, 0.20); }
joint JRT12 { source = R_Meta_5; target = R_Toe5_P; }
joint JRT13 { source = R_Toe5_P; target = R_Toe5_M; }
joint JRT14 { source = R_Toe5_M; target = R_Toe5_D; }
