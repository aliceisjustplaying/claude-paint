(•_•) Round28 Hopper was stopped after the user changed the requested painter to Monet. Its canvas and sketch marks, journal, Lua log and provider session remain in `paint-studio-002de5`. The easel was closed and saved, the new runner/Pi processes terminated and `art.stillwet.painter.r28` removed from launchd. [Stop receipt](readiness/stopped.json).

The frozen source commit is `f2b324457e36fe762cb65dc30aefd0428b735d79`. It adds the approved prompt bundle to the R27 source without engine, material or harness changes. The abandoned runtime worktree and its generated build cache were removed after shutdown; the Git commit and exported studio remain. [Approved prompt diff](prompt-changes.diff), [export receipt](readiness/export.log).

The replacement is a fresh [Round29 Monet](../round29/LAUNCH.md), with its own source snapshot, studio and persistent runner.
