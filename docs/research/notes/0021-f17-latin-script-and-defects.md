# 0021 — Sources of F17: the Latin script in industrial HMIs, and the causes of the defects

Spike SP-019, 2026-10-15, for F17 (E15). The validation of E15 (VAL-014 to VAL-017) removed the HTML layer on the reason that industrial synoptic views are written in Latin script (ADR-0060), and found five defects whose causes had to be found before any fix. Every page below was read in the local library before it was cited; browser behaviors are read in their code where their documentation is silent (level 5, read only). No browser could be run on the machine of the spike: each cause is reproduced in the library's own model, in the font files of the machine, or in CoreText, and what remains to be seen in a browser is said.

## The scripts of industrial HMIs (ADR-0060)

| Finding                                                                                                                                                                                                                | Source                      |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- |
| A SCADA vendor's views are translated per user: "Localization lets you display projects in multiple languages", "users in different regions" seeing "the same project in their preferred language"; no script is named | `REF-IGNITION-LOCALIZATION` |
| ISA-101, the HMI standard, is paid; no readable edition nor free part on text or scripts was found                                                                                                                     | `REF-ISA-101` [unverified]  |

**No readable source says that industrial HMIs mostly use the Latin script**, nor which other scripts appear and where. The one vendor manual read treats languages as a translation of the same views, chosen by each user, which means a view may be shown in a script other than the one it was drawn in. Vendor pages on Chinese, Japanese, Korean and Arabic in HMI runtimes were found only on third-party blogs and resellers' listings, not read as sources (research protocol: a blog never alone).

So ADR-0060's reason stands as the Product Owner's scope decision (the editor targets the Latin script; Greek, Cyrillic, Hebrew and right to left kept), not as a fact the sources confirm. The question is left to the Product Owner in SP-019's report; nothing in F17 depends on it, since ADR-0060 removes the HTML layer whatever the answer.
