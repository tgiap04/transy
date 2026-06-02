# CLAUDE.md

Behavioral guidelines to reduce common LLM coding mistakes. Merge with project-specific instructions as needed.

**Tradeoff:** These guidelines bias toward caution over speed. For trivial tasks, use judgment.

## 1. Think Before Coding

**Don't assume. Don't hide confusion. Surface tradeoffs.**

Before implementing:
- State your assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them - don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

## 2. Simplicity First

**Minimum code that solves the problem. Nothing speculative.**

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask yourself: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

## 3. Surgical Changes

**Touch only what you must. Clean up only your own mess.**

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated dead code, mention it - don't delete it.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: Every changed line should trace directly to the user's request.

## 4. Goal-Driven Execution

**Define success criteria. Loop until verified.**

Transform tasks into verifiable goals:
- "Add validation" → "Write tests for invalid inputs, then make them pass"
- "Fix the bug" → "Write a test that reproduces it, then make it pass"
- "Refactor X" → "Ensure tests pass before and after"

For multi-step tasks, state a brief plan:
```
1. [Step] → verify: [check]
2. [Step] → verify: [check]
3. [Step] → verify: [check]
```

Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

---

## 5. Commit Discipline

**Never commit or push unless the user explicitly asks.** When asked:
- Commit only the files relevant to the request — no unrelated changes.
- Use conventional commits (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
- No AI references in commit messages.
- Keep commits focused and atomic.

---

## 6. Performance & Algorithm Quality

**Default to optimal complexity. Degrade only with a documented reason.**

### Complexity Targets

| Data Size | Target Complexity |
|-----------|-------------------|
| Lookup / membership | O(1) — HashMap, HashSet, index |
| Search (sorted) | O(log n) — binary search, BTree |
| Single-pass transform | O(n) |
| Sort-dependent | O(n log n) |
| Nested loops | Question aggressively — often avoidable |

**Rules:**
- If you write O(n²) or worse, comment why — and why the faster alternative doesn't fit.
- No N+1 queries. Batch fetch, join, or preload. One query for N records, not N queries for 1 record each.
- When iterating: prefer iterator chains over mutable accumulators. Lazy evaluation saves memory.
- Recursion: ensure tail-call or bounded depth. Unbounded recursion on user input is a crash.

### Resource Budgets

| Resource | Mindset |
|----------|---------|
| **RAM** | Don't load the entire dataset when you need one field. Stream, paginate, or project. |
| **CPU** | Cache repeated computations. Memoize pure functions. Avoid re-parsing / re-serializing the same data. |
| **Time** | Cold start matters. Defer heavy init until first use. Parallelize independent work. |
| **I/O** | Batch reads/writes. Buffer when streaming. Never `await` inside a tight loop when you can `join!`. |

### Scalability Check

Before finalizing any data structure or algorithm, ask:
- *Does this hold if input grows 100×? 1000×?*
- *Does this allocate proportional to input size? Can it be constant?*
- *If this runs on a 1-core 512MB VM, does it survive peak load?*

When in doubt, pick the constant-memory, linear-scan approach. You can always optimize later — you cannot un-crash a production OOM.

---

**These guidelines are working if:** fewer unnecessary changes in diffs, fewer rewrites due to overcomplication, and clarifying questions come before implementation rather than after mistakes.