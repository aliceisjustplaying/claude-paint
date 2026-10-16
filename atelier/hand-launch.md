# Hand launch: @FOLDER@ in agy

Gemini 3.8 Flash, painting @NUMBER@ in the atelier. Studio: @STUDIO@. Agent: easel-painter. Model: @MODEL@.

## Steps

1. In a new terminal, from the studio:

   ```sh
   cd @STUDIO@
   agy --agent easel-painter --model '@MODEL@'
   ```

   If agy rejects that label, `--model gemini-3.8-flash-high` is the selector form. The label already sets High, so there's no `--effort`, and there's no skip-permissions flag.

2. If agy asks whether to trust the project, yes only marks the folder as trusted.

3. Before anything is typed, the header shows the agent easel-painter and the model @MODEL@. On "Agent Not Found" or a different model, Ctrl+C exits with nothing sent. The `/model` picker can save a new global default, so it stays closed.

4. The first message is the hello, exactly as written:

   > @HELLO@

5. After one reply, `/mcp` shows the server `easel` running with eight tools: paint, look, note, status, log, read, end_sitting and notebook. If the panel shows the server's path, it ends in `@FOLDER@/bin/mcp/server.py`. With another path, missing tools or a changed model, the session ends there, before the second message.

6. The second message is the launch message, exactly as written:

   > @LAUNCH@

7. The only tool calls to approve are mcp_easel_paint, mcp_easel_look, mcp_easel_note, mcp_easel_status, mcp_easel_log, mcp_easel_read, mcp_easel_end_sitting and mcp_easel_notebook. Every other request gets denied.

8. The session stays open. Sittings go on through `end_sitting` with nothing more pasted.

Ctrl+C in the agy terminal interrupts the turn, and exiting agy ends the session. An oil chunk the easel has already started finishes on its own.

## When the painting is done

`@STUDIO@/out/easel/sittings.json` holds `"complete": true` once a sitting adds no paint and has a whole-canvas look and a detail crop in normal color. This then hangs it:

```sh
@FINISH@
```

It closes the easel, checks a replay of the log, renders the painting, hangs it on the walls, carries the notebook home and commits the text records to the local stillwet-runs-data clone, with no push. It reads the painter's closing reply from the viewer feed, `@FEED@`, which the agy watcher (`harness/agy/watch.py`) writes. Without the feed, the reply goes in `@FINAL@` before the finish.
