// === MUSCLES: ARMS ===

// --- LEFT ARM ---
// Shoulder/Chest power
muscle L_Deltoid       { source = L_Scapula; target = L_Humerus; max_force = 1200.0 Nm; }
muscle L_Pectoralis    { source = Sternum;   target = L_Humerus; max_force = 1800.0 Nm; }

// Biceps/Triceps (Elbow flex/ext)
muscle L_Biceps        { source = L_Scapula; target = L_Radius;  max_force = 800.0 Nm; }
muscle L_Triceps       { source = L_Scapula; target = L_Ulna;    max_force = 900.0 Nm; }

// Forearm (Wrist control)
muscle L_Brachiorad    { source = L_Humerus; target = L_Radius;  max_force = 500.0 Nm; }

// --- RIGHT ARM ---
muscle R_Deltoid       { source = R_Scapula; target = R_Humerus; max_force = 1200.0 Nm; }
muscle R_Pectoralis    { source = Sternum;   target = R_Humerus; max_force = 1800.0 Nm; }

muscle R_Biceps        { source = R_Scapula; target = R_Radius;  max_force = 800.0 Nm; }
muscle R_Triceps       { source = R_Scapula; target = R_Ulna;    max_force = 900.0 Nm; }

muscle R_Brachiorad    { source = R_Humerus; target = R_Radius;  max_force = 500.0 Nm; }
