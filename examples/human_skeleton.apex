organism AverageHuman {
    // === TORSO & HEAD ===
    // Core centerline positions
    bone Pelvis { mass = 15.0 kg; position = (0.0, 0.9, 0.0); }
    bone LowerSpine { mass = 5.0 kg; position = (0.0, 1.1, 0.0); }
    bone Chest  { mass = 15.0 kg; position = (0.0, 1.3, 0.0); }
    bone Neck   { mass = 2.0 kg; position = (0.0, 1.5, 0.0); }
    bone Head   { mass = 5.0 kg; position = (0.0, 1.7, 0.0); }
    
    joint J1 { source = Pelvis; target = LowerSpine; }
    joint J2 { source = LowerSpine; target = Chest; }
    joint J3 { source = Chest; target = Neck; }
    joint J4 { source = Neck; target = Head; }

    // === LEFT LEG ===
    bone LThigh { mass = 8.0 kg; position = (-0.15, 0.85, 0.0); }
    bone LTibia { mass = 4.0 kg; position = (-0.15, 0.45, 0.0); }
    bone LFoot  { mass = 1.5 kg; position = (-0.15, 0.05, 0.1); }
    
    joint JL1 { source = Pelvis; target = LThigh; }
    joint JL2 { source = LThigh; target = LTibia; }
    joint JL3 { source = LTibia; target = LFoot; }

    // === RIGHT LEG ===
    bone RThigh { mass = 8.0 kg; position = (0.15, 0.85, 0.0); }
    bone RTibia { mass = 4.0 kg; position = (0.15, 0.45, 0.0); }
    bone RFoot  { mass = 1.5 kg; position = (0.15, 0.05, 0.1); }
    
    joint JR1 { source = Pelvis; target = RThigh; }
    joint JR2 { source = RThigh; target = RTibia; }
    joint JR3 { source = RTibia; target = RFoot; }

    // === LEFT ARM ===
    bone LShoulder { mass = 1.0 kg; position = (-0.2, 1.4, 0.0); }
    bone LHumerus  { mass = 3.0 kg; position = (-0.4, 1.2, 0.0); }
    bone LRadius   { mass = 2.0 kg; position = (-0.4, 0.9, 0.0); }
    bone LHand     { mass = 0.5 kg; position = (-0.4, 0.7, 0.0); }
    
    joint JLA1 { source = Chest; target = LShoulder; }
    joint JLA2 { source = LShoulder; target = LHumerus; }
    joint JLA3 { source = LHumerus; target = LRadius; }
    joint JLA4 { source = LRadius; target = LHand; }

    // === RIGHT ARM ===
    bone RShoulder { mass = 1.0 kg; position = (0.2, 1.4, 0.0); }
    bone RHumerus  { mass = 3.0 kg; position = (0.4, 1.2, 0.0); }
    bone RRadius   { mass = 2.0 kg; position = (0.4, 0.9, 0.0); }
    bone RHand     { mass = 0.5 kg; position = (0.4, 0.7, 0.0); }
    
    joint JRA1 { source = Chest; target = RShoulder; }
    joint JRA2 { source = RShoulder; target = RHumerus; }
    joint JRA3 { source = RHumerus; target = RRadius; }
    joint JRA4 { source = RRadius; target = RHand; }
}
