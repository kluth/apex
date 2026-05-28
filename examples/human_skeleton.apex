organism AverageHuman {
    // 1. ANATOMY (The Bones - ~206 Nodes)
    include "human_core.apex";
    include "human_head.apex";
    include "human_ribs.apex";
    include "human_arms.apex";
    include "human_legs.apex";

    // 2. MYOFASCIAL (The Muscles - ~150+ Functional Groups)
    include "human_muscles_torso.apex";
    include "human_muscles_arms.apex";
    include "human_muscles_legs.apex";

    // 3. NEURAL (The Control System - 43 Nerve Pairs)
    include "human_neural.apex";
    include "human_nerves.apex";

    // 4. SENSORY (The Feedback System - Proprioceptors)
    include "human_receptors.apex";
}
