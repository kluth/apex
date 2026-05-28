// === RIGHT FOOT: FULL METATARSALS & 5 TOES ===

bone R_Meta_1 { mass = 0.1 kg; position = (0.14, 0.05, 0.12); }
bone R_Meta_2 { mass = 0.1 kg; position = (0.16, 0.05, 0.13); }
bone R_Meta_3 { mass = 0.1 kg; position = (0.18, 0.05, 0.14); }
bone R_Meta_4 { mass = 0.1 kg; position = (0.20, 0.05, 0.13); }
bone R_Meta_5 { mass = 0.1 kg; position = (0.22, 0.05, 0.11); }

joint JRFM1 { source = R_Calc; target = R_Meta_1; }
joint JRFM2 { source = R_Calc; target = R_Meta_2; }
joint JRFM3 { source = R_Calc; target = R_Meta_3; }
joint JRFM4 { source = R_Calc; target = R_Meta_4; }
joint JRFM5 { source = R_Calc; target = R_Meta_5; }

// Toes
bone R_Toe1_P { mass = 0.05 kg; position = (0.14, 0.02, 0.18); }
bone R_Toe1_D { mass = 0.05 kg; position = (0.14, 0.01, 0.22); }
joint JRT1 { source = R_Meta_1; target = R_Toe1_P; }
joint JRT2 { source = R_Toe1_P; target = R_Toe1_D; }

bone R_Toe2_P { mass = 0.05 kg; position = (0.16, 0.02, 0.19); }
bone R_Toe2_D { mass = 0.05 kg; position = (0.16, 0.01, 0.23); }
joint JRT3 { source = R_Meta_2; target = R_Toe2_P; }
joint JRT4 { source = R_Toe2_P; target = R_Toe2_D; }

bone R_Toe3_P { mass = 0.05 kg; position = (0.18, 0.02, 0.20); }
bone R_Toe3_D { mass = 0.05 kg; position = (0.18, 0.01, 0.24); }
joint JRT5 { source = R_Meta_3; target = R_Toe3_P; }
joint JRT6 { source = R_Toe3_P; target = R_Toe3_D; }

bone R_Toe4_P { mass = 0.05 kg; position = (0.20, 0.02, 0.19); }
bone R_Toe4_D { mass = 0.05 kg; position = (0.20, 0.01, 0.23); }
joint JRT7 { source = R_Meta_4; target = R_Toe4_P; }
joint JRT8 { source = R_Toe4_P; target = R_Toe4_D; }

bone R_Toe5_P { mass = 0.05 kg; position = (0.22, 0.02, 0.17); }
bone R_Toe5_D { mass = 0.05 kg; position = (0.22, 0.01, 0.20); }
joint JRT9 { source = R_Meta_5; target = R_Toe5_P; }
joint JRT10 { source = R_Toe5_P; target = R_Toe5_D; }
