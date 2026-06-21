# 0045 FIRST-PROOF-CERT mutation triage register

- Final evidence SHA: `9a071b6e32ebc5b6126645a9db257d453399c028`
- Canonical denominator: 2,901 display-name identities
- Completion manifest: `reports/0045_first_proof_cert_mutation_completion_manifest.md`
- Final missed manifest: `reports/0045_first_proof_cert_mutation_final_missed.txt` (empty, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`)
- Final timeout manifest: `reports/0045_first_proof_cert_mutation_final_timeout.txt` (empty, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`)
- Accepted baseline-miss file: `.cargo/mutants-baseline-misses.txt` remains empty, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`

## Final Survivors

None. The complete shard union contains `missed=0` and `timeout=0`; no reserved survivor-fix ticket is opened from this run.

## Historical 0044 Completion Blocker Reconciliation

| Historical item | 0045 disposition |
|---|---|
| 0044 classified 2,384 of 2,901 identities before wrapper timeout | Superseded by complete 0045 union: 2,901 of 2,901 identities assigned and reconciled. |
| 0044 unclassified remainder: 517 identities | Superseded by complete 0045 union: missing canonical identities = 0. |
| 0044 final missed/timeout floor | Resolved for this denominator: final missed file and final timeout file are empty. |

## Aggregate Outcome

| Caught | Missed | Timeout | Unviable | Union equals canonical | Pairwise disjoint |
|---:|---:|---:|---:|---|---|
| 2277 | 0 | 0 | 624 | True | True |
