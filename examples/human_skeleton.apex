organism AverageHuman {
    // 1. ANATOMY (The Bones)
    include "human_core.apex";
    include "human_head.apex";
    include "human_ribs.apex";
    include "human_arms.apex";
    include "human_legs.apex";

    // 2. MYOFASCIAL (The Muscles)
    include "human_muscles_torso.apex";
    include "human_muscles_arms.apex";
    include "human_muscles_legs.apex";

    // 3. NEURAL (The Control System)
    include "human_neural.apex";
}
