organism AverageHuman {
    // === TORSO & HEAD (6 Nodes) ===
    bone Pelvis     { mass = 15.0 kg; position = (0.0, 0.9, 0.0); }
    bone SpineLower { mass = 4.0 kg;  position = (0.0, 1.1, 0.0); }
    bone SpineMid   { mass = 4.0 kg;  position = (0.0, 1.3, 0.0); }
    bone Chest      { mass = 10.0 kg; position = (0.0, 1.5, 0.0); }
    bone Neck       { mass = 1.5 kg;  position = (0.0, 1.65, 0.0); }
    bone Head       { mass = 5.0 kg;  position = (0.0, 1.85, 0.0); }
    
    joint T1 { source = Pelvis;     target = SpineLower; }
    joint T2 { source = SpineLower; target = SpineMid; }
    joint T3 { source = SpineMid;   target = Chest; }
    joint T4 { source = Chest;      target = Neck; }
    joint T5 { source = Neck;       target = Head; }

    // === LEFT ARM (5 Nodes) ===
    bone L_Clavicle { mass = 0.5 kg; position = (-0.1, 1.5, 0.0); }
    bone L_Shoulder { mass = 1.0 kg; position = (-0.2, 1.5, 0.0); }
    bone L_Elbow    { mass = 2.0 kg; position = (-0.45, 1.5, 0.0); }
    bone L_Wrist    { mass = 0.8 kg; position = (-0.7, 1.5, 0.0); }
    bone L_Hand     { mass = 0.4 kg; position = (-0.85, 1.5, 0.0); }
    
    joint LA1 { source = Chest;      target = L_Clavicle; }
    joint LA2 { source = L_Clavicle; target = L_Shoulder; }
    joint LA3 { source = L_Shoulder; target = L_Elbow; }
    joint LA4 { source = L_Elbow;    target = L_Wrist; }
    joint LA5 { source = L_Wrist;    target = L_Hand; }

    // === RIGHT ARM (5 Nodes) ===
    bone R_Clavicle { mass = 0.5 kg; position = (0.1, 1.5, 0.0); }
    bone R_Shoulder { mass = 1.0 kg; position = (0.2, 1.5, 0.0); }
    bone R_Elbow    { mass = 2.0 kg; position = (0.45, 1.5, 0.0); }
    bone R_Wrist    { mass = 0.8 kg; position = (0.7, 1.5, 0.0); }
    bone R_Hand     { mass = 0.4 kg; position = (0.85, 1.5, 0.0); }
    
    joint RA1 { source = Chest;      target = R_Clavicle; }
    joint RA2 { source = R_Clavicle; target = R_Shoulder; }
    joint RA3 { source = R_Shoulder; target = R_Elbow; }
    joint RA4 { source = R_Elbow;    target = R_Wrist; }
    joint RA5 { source = R_Wrist;    target = R_Hand; }

    // === LEFT LEG (4 Nodes) ===
    bone L_Hip   { mass = 2.0 kg; position = (-0.15, 0.85, 0.0); }
    bone L_Knee  { mass = 4.5 kg; position = (-0.15, 0.45, 0.0); }
    bone L_Ankle { mass = 1.2 kg; position = (-0.15, 0.1, 0.0); }
    bone L_Foot  { mass = 0.8 kg; position = (-0.15, 0.05, 0.15); }
    
    joint LL1 { source = Pelvis;  target = L_Hip; }
    joint LL2 { source = L_Hip;   target = L_Knee; }
    joint LL3 { source = L_Knee;  target = L_Ankle; }
    joint LL4 { source = L_Ankle; target = L_Foot; }

    // === RIGHT LEG (4 Nodes) ===
    bone R_Hip   { mass = 2.0 kg; position = (0.15, 0.85, 0.0); }
    bone R_Knee  { mass = 4.5 kg; position = (0.15, 0.45, 0.0); }
    bone R_Ankle { mass = 1.2 kg; position = (0.15, 0.1, 0.0); }
    bone R_Foot  { mass = 0.8 kg; position = (0.15, 0.05, 0.15); }
    
    joint RL1 { source = Pelvis;  target = R_Hip; }
    joint RL2 { source = R_Hip;   target = R_Knee; }
    joint RL3 { source = R_Knee;  target = R_Ankle; }
    joint RL4 { source = R_Ankle; target = R_Foot; }
}
