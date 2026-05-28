// === MUSCLES: ARMS & HANDS (High Fidelity) ===

// --- ROTATOR CUFF & SHOULDER ---
muscle L_Supraspinatus { source = L_Scapula; target = L_Humerus; max_force = 500.0 Nm; }
muscle R_Supraspinatus { source = R_Scapula; target = R_Humerus; max_force = 500.0 Nm; }
muscle L_Infraspinatus { source = L_Scapula; target = L_Humerus; max_force = 600.0 Nm; }
muscle R_Infraspinatus { source = R_Scapula; target = R_Humerus; max_force = 600.0 Nm; }
muscle L_Deltoid_Ant   { source = L_Clavicle;target = L_Humerus; max_force = 1000.0 Nm; }
muscle R_Deltoid_Ant   { source = R_Clavicle;target = R_Humerus; max_force = 1000.0 Nm; }
muscle L_Deltoid_Post  { source = L_Scapula; target = L_Humerus; max_force = 1000.0 Nm; }
muscle R_Deltoid_Post  { source = R_Scapula; target = R_Humerus; max_force = 1000.0 Nm; }

// --- UPPER ARM (Elbow control) ---
muscle L_Biceps_Long   { source = L_Scapula; target = L_Radius;  max_force = 900.0 Nm; }
muscle R_Biceps_Long   { source = R_Scapula; target = R_Radius;  max_force = 900.0 Nm; }
muscle L_Biceps_Short  { source = L_Scapula; target = L_Radius;  max_force = 800.0 Nm; }
muscle R_Biceps_Short  { source = R_Scapula; target = R_Radius;  max_force = 800.0 Nm; }
muscle L_Triceps_Long  { source = L_Scapula; target = L_Ulna;    max_force = 1100.0 Nm; }
muscle R_Triceps_Long  { source = R_Scapula; target = R_Ulna;    max_force = 1100.0 Nm; }
muscle L_Triceps_Lat   { source = L_Humerus; target = L_Ulna;    max_force = 1000.0 Nm; }
muscle R_Triceps_Lat   { source = R_Humerus; target = R_Ulna;    max_force = 1000.0 Nm; }

// --- FOREARM (Wrist & Finger Power) ---
muscle L_FlexorCarpRad { source = L_Humerus; target = L_Meta2;   max_force = 400.0 Nm; }
muscle R_FlexorCarpRad { source = R_Humerus; target = R_Meta2;   max_force = 400.0 Nm; }
muscle L_FlexorCarpUln { source = L_Humerus; target = L_Meta5;   max_force = 400.0 Nm; }
muscle R_FlexorCarpUln { source = R_Humerus; target = R_Meta5;   max_force = 400.0 Nm; }
muscle L_ExtenCarpRad  { source = L_Humerus; target = L_Meta2;   max_force = 400.0 Nm; }
muscle R_ExtenCarpRad  { source = R_Humerus; target = R_Meta2;   max_force = 400.0 Nm; }

// --- INTRINSIC HANDS (Thumb & Grip) ---
muscle L_OpponensPol   { source = L_Trapezium;target = L_Meta1;  max_force = 300.0 Nm; }
muscle R_OpponensPol   { source = R_Trapezium;target = R_Meta1;  max_force = 300.0 Nm; }
muscle L_AdductorPol   { source = L_Capitate; target = L_Thumb_P;max_force = 250.0 Nm; }
muscle R_AdductorPol   { source = R_Capitate; target = R_Thumb_P;max_force = 250.0 Nm; }
muscle L_Interossei_1  { source = L_Meta1;    target = L_Meta2;  max_force = 150.0 Nm; }
muscle R_Interossei_1  { source = R_Meta1;    target = R_Meta2;  max_force = 150.0 Nm; }
