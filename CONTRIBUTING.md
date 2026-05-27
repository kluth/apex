# CONTRIBUTING TO APEX

The APEX Protocol is a set of engineering laws that govern this repository. By contributing, you agree to uphold these standards.

## THE ENGINEERING LAWS

1. **Micro-Increments:** A single Pull Request MUST NOT exceed **150 lines of code** (excluding documentation and tests).
2. **Strict TDD:** Follow the Red-Green-Refactor cycle. Every PR must include proof of failing tests being resolved by implementation.
3. **Complexity Limit:** Cyclomatic complexity (McCabe) per function MUST be **< 10**.
4. **Result Pattern:** Exceptions for control flow are strictly forbidden. Use `Result<T, E>`.
5. **Contract-First:** Define interfaces/contracts before implementation.
6. **Documentation:** Every feature or ADR change must be reflected in the Wiki.

## WORKFLOW

1. Find/Create an Issue.
2. Propose a conceptual plan in the issue.
3. Once approved, create a feature branch.
4. Execute TDD cycle.
5. Ensure 100% pass rate in CI/CD.
6. Submit PR (squash/rebase only).

## COMMIT STANDARDS

We use [Conventional Commits](https://www.conventionalcommits.org/).
- `feat:` for new features.
- `fix:` for bug fixes.
- `docs:` for documentation updates.
- `chore:` for maintenance.
- `refactor:` for code changes that neither fix a bug nor add a feature.
