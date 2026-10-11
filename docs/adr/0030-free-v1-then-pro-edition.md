# ADR-0030 — A finished free V1, then new features in a paid edition

**Status**: Accepted (Product Owner, 2026-10-11)

## Context

The backlog grows feature by feature with no end set. The Product Owner wants a stop: one complete, free product that anyone can use, and a paid product for the features that only companies need. In their words: « Ouais, super, faire du libre, c'est cool, mais il faut vraiment mettre un stop. » And on storage: « pour la V1, on ne va sortir qu'un logiciel en local et point barre. Il n'y aura pas de côté serveur. »

The repository is public under the Apache License 2.0, and only the agent writes changes (ADR-0026): no outside contribution has to be relicensed, so the same author can build a paid product on the library.

## Decision

1. **A free V1, as complete as possible.** Everything integrated before the V1 release is free and stays in this repository. Its features are chosen by the Product Owner as the work goes on; it is a real product, not a demo.
2. **After V1, the free product only gets bug fixes.** Every new feature goes to the **pro edition**: a private project of the Product Owner, hosted elsewhere, sold as a service or self-hosted.
3. **What belongs to the pro edition**: features that companies want and an individual user does not ("des trucs de test, quand c'est un utilisateur lambda, personne n'a envie de faire ça") — testing tools, open APIs for other services, project management, servers, a faster engine. Ideas are kept in [editions](../domain/editions.md), never as stories in this backlog.
4. **The free product is local only and stays simple**: it runs on the user's computer, with a working folder and files saved one by one, like a desktop vector editor; no server side at all, not even a local server (this replaces the local server of Q5). Synoptic views, not drawing. A feature that makes it complicated is either left out or goes to the pro edition.
5. **The agent checks the scope**: when a request, refinement or idea looks like a pro feature, it says so and asks the Product Owner where it goes; it never puts one in a `ready` story on its own.

## Consequences

- The free product shows the idea behind the project, and anyone may improve it under its licence.
- Every epic of the backlog makes V1 (Q22 in the [domain](../domain/)); the local server planned in epic E11 leaves V1, E11 keeps files in a working folder; E07 publishes libraries locally only. The application shell is still open (Q23).
- The free library must stay usable by another product: plain data, portable core (ADR-0025), public API documented. Nothing pro is written here first.
- Pro ideas are written in this public repository and visible to everyone, by the Product Owner's choice: this first project should go as far as possible, ideas included.

## References

ADR-0017, ADR-0025, ADR-0026, [editions](../domain/editions.md), `LICENSE` (Apache License 2.0)
