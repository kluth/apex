organism AverageHuman {
    // === CORE (5 Nodes) ===
    bone Pelvis     { mass = 15.0 kg; position = (0.0, 0.9, 0.0); }
    bone Spine_Low  { mass = 5.0 kg;  position = (0.0, 1.1, 0.0); }
    bone Chest      { mass = 15.0 kg; position = (0.0, 1.4, 0.0); }
    bone Neck       { mass = 2.0 kg;  position = (0.0, 1.6, 0.0); }
    bone Head       { mass = 5.0 kg;  position = (0.0, 1.8, 0.0); }
    
    // === LEFT ARM (5 Nodes) ===
    bone L_Clavicle { mass = 0.5 kg; position = (-0.1, 1.45, 0.0); }
    bone L_Shoulder { mass = 1.0 kg; position = (-0.25, 1.45, 0.0); }
    bone L_Elbow    { mass = 2.0 kg; position = (-0.5, 1.3, 0.0); }
    bone L_Wrist    { mass = 0.8 kg; position = (-0.75, 1.15, 0.0); }
    bone L_Hand     { mass = 0.4 kg; position = (-0.85, 1.1, 0.0); }

    // === RIGHT ARM (5 Nodes) ===
    bone R_Clavicle { mass = 0.5 kg; position = (0.1, 1.45, 0.0); }
    bone R_Shoulder { mass = 1.0 kg; position = (0.25, 1.45, 0.0); }
    bone R_Elbow    { mass = 2.0 kg; position = (0.5, 1.3, 0.0); }
    bone R_Wrist    { mass = 0.8 kg; position = (0.75, 1.15, 0.0); }
    bone R_Hand     { mass = 0.4 kg; position = (0.85, 1.1, 0.0); }

    // === LEFT LEG (4 Nodes) ===
    bone L_Hip      { mass = 2.0 kg; position = (-0.2, 0.9, 0.0); }
    bone L_Knee     { mass = 4.5 kg; position = (-0.2, 0.45, 0.0); }
    bone L_Ankle    { mass = 1.2 kg; position = (-0.2, 0.1, 0.0); }
    bone L_Foot     { mass = 0.8 kg; position = (-0.2, 0.05, 0.15); }

    // === RIGHT LEG (4 Nodes) ===
    bone R_Hip      { mass = 2.0 kg; position = (0.2, 0.9, 0.0); }
    bone R_Knee     { mass = 4.5 kg; position = (0.2, 0.45, 0.0); }
    bone R_Ankle    { mass = 1.2 kg; position = (0.2, 0.1, 0.0); }
    bone R_Foot     { mass = 0.8 kg; position = (0.2, 0.05, 0.15); }

    // Logical connections (Joints are needed for the AIR graph, but the viewer will use direct names)
    joint J_C1 { source = Pelvis; target = Spine_Low; }
    joint J_C2 { source = Spine_Low; target = Chest; }
    joint J_C3 { source = Chest; target = Neck; }
    joint J_C4 { source = Neck; target = Head; }
}
