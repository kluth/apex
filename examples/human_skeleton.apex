organism AverageHuman {
    // Torso and Head
    bone Pelvis { mass = 15.0 kg; }
    bone Spine  { mass = 10.0 kg; }
    bone Head   { mass = 5.0 kg; }
    
    joint Hip { source = Pelvis; target = Spine; }
    joint Neck { source = Spine; target = Head; }

    // Left Leg
    bone LeftThigh { mass = 8.0 kg; }
    bone LeftTibia { mass = 4.0 kg; }
    joint LeftHip { source = Pelvis; target = LeftThigh; }
    joint LeftKnee { source = LeftThigh; target = LeftTibia; }

    // Right Leg
    bone RightThigh { mass = 8.0 kg; }
    bone RightTibia { mass = 4.0 kg; }
    joint RightHip { source = Pelvis; target = RightThigh; }
    joint RightKnee { source = RightThigh; target = RightTibia; }

    // Left Arm
    bone LeftHumerus { mass = 3.0 kg; }
    bone LeftRadius  { mass = 2.0 kg; }
    joint LeftShoulder { source = Spine; target = LeftHumerus; }
    joint LeftElbow    { source = LeftHumerus; target = LeftRadius; }

    // Right Arm
    bone RightHumerus { mass = 3.0 kg; }
    bone RightRadius  { mass = 2.0 kg; }
    joint RightShoulder { source = Spine; target = RightHumerus; }
    joint RightElbow    { source = RightHumerus; target = RightRadius; }
}
