# APEX
**Anatomical Procedural Expression Language**

APEX is a declarative, biological modeling and simulation operating system. Geometry, physiology, and movement emerge exclusively from anatomical relationships rather than explicit keyframe animations or direct coordinate manipulations.

## Architecture
APEX strictly adheres to **Hexagonal Architecture** and **Domain-Driven Design (DDD)**. 

### Core Paradigms
- **Parse, Don't Validate:** Mathematical impossibilities (e.g. negative mass) are caught at compile time.
- **Data-Oriented Design (DOD):** Memory is laid out in strict Arena-based structures (`Topology`) for cache-line locality and rapid Entity Component System (ECS) iteration.
- **Extended Position Based Dynamics (XPBD):** Used for unconditionally stable simulation of extremely stiff differentials (mythic biomaterials).

## Documentation 
- **[CHANGELOG](./CHANGELOG.md)**: Sequential version tracking.
- **[WIKI](./docs/WIKI.md)**: The Living Domain Lexicon and architectural guidelines.
- **[GEMINI AUDIT LEDGER](./GEMINI.md)**: Immutable chronological AI operational ledger.