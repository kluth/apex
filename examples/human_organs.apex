// === VISCERAL LAYER: INTERNAL ORGANS ===

// 1. BRAIN
organ Brain {
    anchor = Frontal;
    mass = 1.4 kg;
    volume = 0.0013 m;
}

// 2. HEART
organ Heart {
    anchor = Sternum;
    mass = 0.3 kg;
    volume = 0.0003 m;
}

// 3. LUNGS
organ Lung_L {
    anchor = L_Rib5;
    mass = 0.5 kg;
    volume = 0.002 m;
}
organ Lung_R {
    anchor = R_Rib5;
    mass = 0.5 kg;
    volume = 0.002 m;
}

// 4. LIVER
organ Liver {
    anchor = R_Rib10;
    mass = 1.5 kg;
    volume = 0.0015 m;
}

// 5. KIDNEYS
organ Kidney_L {
    anchor = L1;
    mass = 0.15 kg;
    volume = 0.0001 m;
}
organ Kidney_R {
    anchor = L1;
    mass = 0.15 kg;
    volume = 0.0001 m;
}
