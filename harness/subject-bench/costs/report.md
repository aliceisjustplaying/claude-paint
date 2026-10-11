# Two-chunk opening cost calibration

## Outcome

**r31 Opus 5.5: five responses through two successful main-canvas chunks, $0.2196074 API-equivalent.** Logged tokens: 12 uncached input, 81,697 cache reads, 30,812 five-minute cache writes, zero one-hour writes and 2,458 output including 1,197 reasoning tokens.

The sanitized prefix preserves original lines 1–27. Paint calls/results are 22/23 and 26/27. All five usage records were rechecked against the surviving ledger. The Lua matched archived main chunks 1 and 2 before the cloud backing replacement. The first two responses execute reads/status and complete zero paint chunks. Their cost is $0.122124; first paint is response four.

**Setup-only caveat:** chunk 1 opens the canvas and prints dimensions/tubes; chunk 2 mixes palette piles. Neither lays a stroke. A look call is generated alongside chunk 2, with its result beyond the stop. The full response is charged once.

## PR

None; reconstruction uses local surviving Drive inputs only. No repository/git writes or model/network calls.

## Needs

The other nine models need real bench opening usage for measured counts. These scenarios assume the same Opus tokenization, reading, reasoning and actions. They are not prediction bounds. No additional private archived transcript is required for this ballpark.

## Cost

| Target | Same workload, cached | Same workload, all input uncached |
|---|---:|---:|
| Claude Haiku 5.5 CPA | $0.0059 | $0.0125 |
| Claude Sonnet 5.5 CPA | $0.1098 | $0.2496 |
| Claude Opus 5.5 CPA | $0.2196 | $0.4992 |
| GPT 6.1 Sol Codex | $0.0944 | $0.2496 |
| GPT 6 Luna Codex | $0.0051 | $0.0125 |
| Kimi K3 Go | $0.1539 | $0.3744 |
| DeepSeek V4.1 Flash Go | $0.0127 | $0.0367 |
| GLM 5.3 Flash Go | $0.0083 | $0.0181 |
| MiMo V2.6 Pro Go | $0.0158 | $0.0511 |
| Gemini 3.8 Flash OpenRouter | $0.0385 | $0.0936 |

Rates are from the local pricing handoff checked **2026-10-10**, without a refresh. `rates.csv` preserves sources, tiers and notes. Pricing is per response, including cache input in prompt length; reasoning is already in output. Anthropic cache writes retain their write rates. For non-Anthropic targets, 30,812 newly written tokens become ordinary input: 30,824 uncached input and 81,697 cached input in the cached scenario. The cold scenario prices 112,521 aggregate prompt tokens as ordinary input across the same five responses. None of those individual responses enters a high-context tier.

Claude CPA, Codex and Go labels are requested bench routes; the historical provider field does not establish subscription billing. These are public API-equivalent values, not subscription invoices, marginal seat charges or quota deductions. Claude uses Anthropic list rates; Sol/Luna use OpenAI standard list rates; Kimi/DeepSeek/GLM use verified Zen-equivalent rates. MiMo uses an exact-model OpenRouter alternative-route benchmark because direct/Zen Go pricing was not verified. Gemini uses exact-model OpenRouter rates. Cache retention fees, fast/batch modifiers and unlogged fees are not invented.

## Lesson

Two requests are not two chunks. Count successful main-canvas paint results, then include every model request up to that boundary, including reads, scratch and failed attempts. A two-chunk stop can reveal setup decisions without a visible subject. Record real bench usage to replace cross-model repricing assumptions.

Durable files in `handoffs/subject-bench/costs/`: `report.md`, `measured_opening.json`, `opening_calibration.tsv`, `opening-costs.json` and `rates.csv`. Extended analyzers and historical amortization were intentionally not reconstructed.
