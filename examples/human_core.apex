// === HUMAN CORE: PELVIS & VERTEBRAL COLUMN ===

// 1. PELVIS (The Root Anchor)
bone Pelvis { mass = 15.0 kg; position = (0.0, 0.9, 0.0); mesh = "anatomy/human/pelvis"; }

// 2. SACRUM & COCCYX
bone Sacrum { mass = 1.0 kg; position = (0.0, 0.85, 0.0); }
bone Coccyx { mass = 0.2 kg; position = (0.0, 0.8, -0.05); }
joint JS0 { source = Sacrum; target = Coccyx; }
joint JS1 { source = Pelvis; target = Sacrum; }

// 3. LUMBAR VERTEBRAE (L5 - L1)
bone L5 { mass = 0.5 kg; position = (0.0, 0.92, 0.0); }
bone L4 { mass = 0.5 kg; position = (0.0, 0.97, 0.0); }
bone L3 { mass = 0.5 kg; position = (0.0, 1.02, 0.0); }
bone L2 { mass = 0.5 kg; position = (0.0, 1.07, 0.0); }
bone L1 { mass = 0.5 kg; position = (0.0, 1.12, 0.0); }
joint JL5 { source = Sacrum; target = L5; }
joint JL4 { source = L5; target = L4; }
joint JL3 { source = L4; target = L3; }
joint JL2 { source = L3; target = L2; }
joint JL1 { source = L2; target = L1; }

// 4. THORACIC VERTEBRAE (T12 - T1)
bone T12 { mass = 0.4 kg; position = (0.0, 1.17, 0.0); }
bone T11 { mass = 0.4 kg; position = (0.0, 1.21, 0.0); }
bone T10 { mass = 0.4 kg; position = (0.0, 1.25, 0.0); }
bone T9  { mass = 0.4 kg; position = (0.0, 1.29, 0.0); }
bone T8  { mass = 0.4 kg; position = (0.0, 1.33, 0.0); }
bone T7  { mass = 0.4 kg; position = (0.0, 1.37, 0.0); }
bone T6  { mass = 0.4 kg; position = (0.0, 1.41, 0.0); }
bone T5  { mass = 0.4 kg; position = (0.0, 1.45, 0.0); }
bone T4  { mass = 0.4 kg; position = (0.0, 1.49, 0.0); }
bone T3  { mass = 0.4 kg; position = (0.0, 1.53, 0.0); }
bone T2  { mass = 0.4 kg; position = (0.0, 1.57, 0.0); }
bone T1  { mass = 0.4 kg; position = (0.0, 1.61, 0.0); }
joint JT12 { source = L1; target = T12; }
joint JT11 { source = T12; target = T11; }
joint JT10 { source = T11; target = T10; }
joint JT9  { source = T10; target = T9; }
joint JT8  { source = T9; target = T8; }
joint JT7  { source = T8; target = T7; }
joint JT6  { source = T7; target = T6; }
joint JT5  { source = T6; target = T5; }
joint JT4  { source = T5; target = T4; }
joint JT3  { source = T4; target = T3; }
joint JT2  { source = T3; target = T2; }
joint JT1  { source = T2; target = T1; }

// 5. CERVICAL VERTEBRAE (C7 - C1 / Atlas)
bone C7 { mass = 0.3 kg; position = (0.0, 1.65, 0.0); }
bone C6 { mass = 0.3 kg; position = (0.0, 1.68, 0.0); }
bone C5 { mass = 0.3 kg; position = (0.0, 1.71, 0.0); }
bone C4 { mass = 0.3 kg; position = (0.0, 1.74, 0.0); }
bone C3 { mass = 0.3 kg; position = (0.0, 1.77, 0.0); }
bone C2 { mass = 0.3 kg; position = (0.0, 1.80, 0.0); }
bone C1 { mass = 0.3 kg; position = (0.0, 1.83, 0.0); }
joint JC7 { source = T1; target = C7; }
joint JC6 { source = C7; target = C6; }
joint JC5 { source = C6; target = C5; }
joint JC4 { source = C5; target = C4; }
joint JC3 { source = C4; target = C3; }
joint JC2 { source = C3; target = C2; }
joint JC1 { source = C2; target = C1; }

bone Head { mass = 5.0 kg; position = (0.0, 1.95, 0.0); }
joint JH { source = C1; target = Head; }
