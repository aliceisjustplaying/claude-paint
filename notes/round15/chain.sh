#!/bin/bash
# Round 15 painter chain: three painters in sequence, each in a fresh
# studio exported from branch r15-base (no git, no build artifacts), each
# given only the previous painters' craft notes. Painters run without the
# user's global context files, skills or prompt templates.
L=~/tmp/paint-r6-b943b1ca/r15
B=~/tmp/paint-r6-b943b1ca/briefs
for N in 1 2 3; do
  D=~/src/a/paint-r15-p$N
  echo "$(date +%H:%M) p$N: preparing $D"
  rm -rf "$D"; mkdir -p "$D"
  git -C ~/src/a/claude-paint archive r15-base | tar -x -C "$D"
  rm -f "$D/.gitignore"
  for k in $(seq 1 $((N-1))); do
    f=~/src/a/paint-r15-p$k/notes/craft_r15_p$k.md
    [ -f "$f" ] || { echo "$(date +%H:%M) p$N: missing $f, stopping"; exit 1; }
    cp "$f" "$D/notes/"
  done
  (cd "$D" && timeout 1800 cargo build --release -q -p paintings >"$L/p${N}_build.log" 2>&1) || { echo "p$N: build failed"; exit 1; }
  echo "$(date +%H:%M) p$N: painting"
  (cd "$D" && pi --print --no-context-files --no-skills --no-prompt-templates \
     --provider anthropic --model claude-opus-5-5 --thinking high \
     "Complete your task autonomously. Read $B/r15_p$N.md and follow it exactly. That file is your whole brief. Your FINAL message is the reply it asks for." \
     </dev/null >"$L/p${N}_final.txt" 2>"$L/p${N}_err.txt")
  echo "$(date +%H:%M) p$N: done (exit $?); $(ls $D/out/*.png 2>/dev/null | wc -l) renders; craft: $(wc -l <$D/notes/craft_r15_p$N.md 2>/dev/null)"
done
echo "$(date +%H:%M) chain finished"
