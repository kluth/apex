// === LEFT FOOT: FULL METATARSALS & 5 TOES ===

bone L_Meta_1 { mass = 0.1 kg; position = (-0.14, 0.05, 0.12); }
bone L_Meta_2 { mass = 0.1 kg; position = (-0.16, 0.05, 0.13); }
bone L_Meta_3 { mass = 0.1 kg; position = (-0.18, 0.05, 0.14); }
bone L_Meta_4 { mass = 0.1 kg; position = (-0.20, 0.05, 0.13); }
bone L_Meta_5 { mass = 0.1 kg; position = (-0.22, 0.05, 0.11); }

joint JLFM1 { source = L_Calc; target = L_Meta_1; }
joint JLFM2 { source = L_Calc; target = L_Meta_2; }
joint JLFM3 { source = L_Calc; target = L_Meta_3; }
joint JLFM4 { source = L_Calc; target = L_Meta_4; }
joint JLFM5 { source = L_Calc; target = L_Meta_5; }

// Toes
bone L_Toe1_P { mass = 0.05 kg; position = (-0.14, 0.02, 0.18); }
bone L_Toe1_D { mass = 0.05 kg; position = (-0.14, 0.01, 0.22); }
joint JLT1 { source = L_Meta_1; target = L_Toe1_P; }
joint JLT2 { source = L_Toe1_P; target = L_Toe1_D; }

bone L_Toe2_P { mass = 0.05 kg; position = (-0.16, 0.02, 0.19); }
bone L_Toe2_D { mass = 0.05 kg; position = (-0.16, 0.01, 0.23); }
joint JLT3 { source = L_Meta_2; target = L_Toe2_P; }
joint JLT4 { source = L_Toe2_P; target = L_Toe2_D; }

bone L_Toe3_P { mass = 0.05 kg; position = (-0.18, 0.02, 0.20); }
bone L_Toe3_D { mass = 0.05 kg; position = (-0.18, 0.01, 0.24); }
joint JLT5 { source = L_Meta_3; target = L_Toe3_P; }
joint JLT6 { source = L_Toe3_P; target = L_Toe3_D; }

bone L_Toe4_P { mass = 0.05 kg; position = (-0.20, 0.02, 0.19); }
bone L_Toe4_D { mass = 0.05 kg; position = (-0.20, 0.01, 0.23); }
joint JLT7 { source = L_Meta_4; target = L_Toe4_P; }
joint JLT8 { source = L_Toe4_P; target = L_Toe4_D; }

bone L_Toe5_P { mass = 0.05 kg; position = (-0.22, 0.02, 0.17); }
bone L_Toe5_D { mass = 0.05 kg; position = (-0.22, 0.01, 0.20); }
joint JLT9 { source = L_Meta_5; target = L_Toe5_P; }
joint JLT10 { source = L_Toe5_P; target = L_Toe5_D; }
