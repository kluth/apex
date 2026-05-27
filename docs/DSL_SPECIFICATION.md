# APEX DSL Specification (v0.1)

## Overview
APEX is a declarative language for modeling biological organisms. It focuses on anatomical relationships and physical properties rather than explicit movement commands.

## EBNF Grammar
```ebnf
<Program>       ::= <Pragma>* <Organism>
<Pragma>        ::= "#[" "apex" "(" <Attribute> ")" "]"
<Organism>      ::= "organism" <Identifier> "{" <Substrate>? <Anatomy>* "}"
<Anatomy>       ::= <Bone> | <Joint> | <Muscle> | <Synapse> | <Skin>

<Bone>          ::= "bone" <Identifier> "{" <Property>* "}"
<Property>      ::= <Identifier> "=" <Value> <Unit> ";"

<Unit>          ::= "kg" | "m" | "Nm" | "rad"
```

## Core Components

### 1. Organism
The top-level container for an anatomical model.
```apex
organism Biped { ... }
```

### 2. Bone
Represents a rigid physical body. Requires a `mass` property.
```apex
bone Femur {
    mass = 2.0 kg;
}
```

### 3. Physical Units
APEX enforces strict unit types to prevent biomechanical scaling errors:
- `kg`: Mass (Kilograms)
- `m`: Length (Meters)
- `Nm`: Torque/Moment (Newton-meters)
- `rad`: Angle (Radians)

## Biological Constraints
- **Unique Identifiers:** Every bone, joint, and muscle must have a unique ID within the organism.
- **Positive Mass:** Mass must be a strictly positive non-zero value.
- **Attachment Integrity:** Joints and muscles must reference existing bones.
