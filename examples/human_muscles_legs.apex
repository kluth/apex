// === MUSCLES: LEGS ===

// --- LEFT LEG ---
// Quadriceps (Primary Knee Extensor)
muscle L_RectusFemoris { source = L_Hip;   target = L_Patella; max_force = 1200.0 Nm; }
muscle L_VastusLat      { source = L_Femur; target = L_Patella; max_force = 1500.0 Nm; }
muscle L_VastusMed      { source = L_Femur; target = L_Patella; max_force = 1500.0 Nm; }

// Hamstrings (Primary Knee Flexor / Hip Extensor)
muscle L_BicepsFemoris  { source = Pelvis;  target = L_Fibula;  max_force = 1000.0 Nm; }
muscle L_Semitendinosus { source = Pelvis;  target = L_Tibia;   max_force = 900.0 Nm; }

// Gluteals (Hip Power)
muscle L_GluteusMax     { source = Sacrum;  target = L_Femur;   max_force = 2500.0 Nm; }
muscle L_GluteusMed     { source = Pelvis;  target = L_Femur;   max_force = 1500.0 Nm; }

// Calf (Ankle Plantarflexion)
muscle L_Gastrocnemius  { source = L_Femur; target = L_Calcaneus; max_force = 2000.0 Nm; }
muscle L_Soleus         { source = L_Tibia; target = L_Calcaneus; max_force = 1800.0 Nm; }

// --- RIGHT LEG ---
muscle R_RectusFemoris { source = R_Hip;   target = R_Patella; max_force = 1200.0 Nm; }
muscle R_VastusLat      { source = R_Femur; target = R_Patella; max_force = 1500.0 Nm; }
muscle R_VastusMed      { source = R_Femur; target = R_Patella; max_force = 1500.0 Nm; }

muscle R_BicepsFemoris  { source = Pelvis;  target = R_Fibula;  max_force = 1000.0 Nm; }
muscle R_Semitendinosus { source = Pelvis;  target = R_Tibia;   max_force = 900.0 Nm; }

muscle R_GluteusMax     { source = Sacrum;  target = R_Femur;   max_force = 2500.0 Nm; }
muscle R_GluteusMed     { source = Pelvis;  target = R_Femur;   max_force = 1500.0 Nm; }

muscle R_Gastrocnemius  { source = R_Femur; target = R_Calcaneus; max_force = 2000.0 Nm; }
muscle R_Soleus         { source = R_Tibia; target = R_Calcaneus; max_force = 1800.0 Nm; }
