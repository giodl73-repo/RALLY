# RALLY Review Panel

Use this panel for changes to shared game-simulation, validation, comparison,
and evidence contracts. Product repositories retain creative and economic
policy.

## Active Roles

| Role | Protects | Invoke when |
|---|---|---|
| [Harness Boundary Engineer](parliament/harness-boundary-engineer.md) | Product-neutral core | Adding mechanics, adapters, or dependencies |
| [Simulation Auditor](parliament/simulation-auditor.md) | Deterministic runs and traces | Changing seeds, events, dice, ordering, or reports |
| [Evidence Packet Reviewer](parliament/evidence-packet-reviewer.md) | Inspectable validation evidence | Changing findings, comparisons, manifests, or JSON |
| [Consumer Advocate](stakeholders/consumer-advocate.md) | Adoption without workflow loss | Changing public contracts or migration policy |
| [Privacy Reviewer](stakeholders/privacy-reviewer.md) | Separation from private playtest content | Adding fixtures, examples, packets, or publication |

## Core Tensions

| Pulls | Against | Because |
|---|---|---|
| Harness Boundary Engineer | Consumer Advocate | Consumer convenience can import game policy into the shared core. |
| Simulation Auditor | Consumer Advocate | Strict reproducibility can constrain legitimate local adapters. |
| Evidence Packet Reviewer | Privacy Reviewer | Complete evidence can expose private campaign or playtest details. |
| Consumer Advocate | Evidence Packet Reviewer | A simpler migration can omit evidence required to prove compatibility. |

## Review Order

1. Privacy Reviewer removes unsafe source material.
2. Simulation Auditor and Evidence Packet Reviewer establish reproducible proof.
3. Harness Boundary Engineer enforces neutral ownership.
4. Consumer Advocate judges whether adoption earns its migration cost.
