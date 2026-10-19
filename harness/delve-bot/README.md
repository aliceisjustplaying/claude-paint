# Stillwet on Delve: the painter bot

Stillwet takes painting requests on Delve. People mention Stillwet or reply to one of its posts with what they want painted. Claude Opus 5.5 checks each request. Accepted requests go to the house painter (Opus 5.5 in a copy of the house oil studio, launched the way the house runner launches an Opus sitting). The finished painting is posted back as a reply, and when streaming is on every painting is streamed live to stream.place while it's being made.

Out of the box everything is dry: `dryRun` is `true`, and `livePosting` and `liveStreaming` are `false`. A dry run checks requests and writes the replies it would post, but starts no painter, no stream and no write anywhere.

## Flow

1. **Ingest.** `npm start` reads posts from Jetstream at `jetstreamUrl` (localhost). Each event is stored in SQLite before it's handled, and events are handled in cursor order. The saved cursor lets a reconnect resume. An edit or delete cancels the request at storage time and leaves a tombstone in the `invalidated` table. So an edited request whose thread is still being read gets refused.
2. **Intake.** A post counts only if it mentions Stillwet's DID or replies to a Stillwet post. The checks below run in this order, and the first one that fails ends intake:
   - kill switch
   - blocked list
   - opt-out
   - edited
   - duplicate
   - age
   - thread check: the thread is re-read from the public Delve server, and the request has to match it exactly (CID, author, text, root and parent)

   Then, in one transaction:
   - tombstone
   - thread guards
   - reply backlog
   - STOP
   - size, then the content check
   - completion-turn reservation
   - limits
   - a free slot or the queue
3. **Pump.** This runs every second. It cancels jobs from opted-out or blocked people, expires old queued jobs and starts queued jobs when the slot and the daily start limit allow. A started job runs the painter (inside the stream when streaming is on), then fits the finished painting into a JPEG, and a done reply carrying it goes into the outbox.
4. **Flush.** This sends outbox replies. In dry run a reply is only marked `simulated`.

## Content check

`src/check.ts` sends the request text to the Anthropic Messages API (`anthropicUrl`, model `contentCheck.model`, `claude-opus-5-5`) with `CHECK_PROMPT` as the system prompt. It declines only real people, sexual content, hate and gore beyond cartoonish, and accepts everything else ("a cowboy blasting a zombie" is fine). The answer has to be exactly `ACCEPT` or `DECLINE`. Anything else, an HTTP error, a timeout (`contentCheck.timeoutSeconds`) or a missing `ANTHROPIC_API_KEY` declines the request with `declinedCheck`. A decline sends `declinedPolicy`. The check runs in dry runs too, so a dry run needs the key.

## Painter

`src/painter.ts` copies the studio template (`studio.template`, the house "every" studio from `scripts/export_r16_studio`) to `studio.root/paint-studio-<6 hex of the request URI's sha256>`, writes `BRIEF.md` from `assets/brief_template.md` and starts `pi` there. The arguments match the house runner's Opus sitting: `harness/painter`'s `painter.ts` and `compaction.ts` extensions, its system prompt, the tools `paint,look,note,status,log,read`, `--provider anthropic --model claude-opus-5-5 --thinking high` and a session folder under `studio.sessions`. The first message and the resume message are the house runner's, byte for byte. A restarted job resumes its newest session.

When pi exits, the bot saves the canvas with `easel save`, always closes the easel server and scales the PNG with ffmpeg to fit `studio.width` x `studio.height` (1200x900) under `caps.imageBytes`. The easel itself only paints new canvases 2400 or 4800 px wide (`crates/easel/src/session.rs` `WIDTHS`), so the canvas is 2400 px wide and the post is the scaled copy. pi's final reply becomes the image's alt text, and `imageAlt` is the fallback.

**Caps are off.** `studio.maxWallSeconds`, `maxInputTokens`, `maxOutputTokens` and `maxCostUsd` and `streaming.maxSeconds` are absent from `config.json`, and absent or `null` means no cap. Setting one makes the runner stop the painter when it's reached, and the reply then says the painting stopped early (`doneLimit`). Input tokens count every prompt token (uncached, cache read and cache write) across the job's session files.

## Switches

| Setting | Default | What it allows |
| --- | --- | --- |
| `dryRun` | `true` | Off lets the painter start. Both switches below need it off; config validation refuses otherwise |
| `livePosting` | `false` | Replies to a non-loopback PDS (`pdsUrl`). Off, only a `127.0.0.1` mock PDS can be written |
| `liveStreaming` | `false` | Stream records on a non-loopback PDS and RTMPS ingest to stream.place. Off, only `rtmp://127.0.0.1` or a `file://` `.flv` |
| `streaming.enabled` | `false` | Wraps each painting in a stream; needs `caps.running` = 1 |

The kill switch (`STILLWET_KILL_SWITCH` set to anything, or the `killFile`) halts all of it, as described under Guards.

## Secrets

The bot reads secrets only from its environment, never from `config.json` or SQLite:

| Variable | Used for |
| --- | --- |
| `ANTHROPIC_API_KEY` | The content check and the painter (pi gets it in its environment) |
| `DELVE_HANDLE`, `DELVE_APP_PASSWORD` | Delve session for replies and stream records |
| `STREAMPLACE_STREAM_KEY` | RTMPS ingest; only read when a live stream starts, and logged redacted |

On the VPS the NixOS service (`hosts/vps/modules/stillwet-delve.nix` in nixfiles) loads them from sops files with systemd credentials and exports them just before starting the bot.

## Config (`config.json`)

| Setting | Value | What it does |
| --- | --- | --- |
| `did` / `handle` | `did:plc:stillwetprototype` / `stillwet.invalid` | Placeholder identity, not a real account |
| `database` | `data/stillwet.sqlite` | SQLite state, plus a `.lock` file holding the PID |
| `killFile` | `data/STOP` | Kill switch file |
| `jetstreamUrl` | `ws://127.0.0.1:6008/subscribe` | Event feed |
| `appviewUrl` | `https://api.delve.town` | Public reads: thread, feed, profile |
| `pdsUrl` | `https://pds.delve.town` | Refused for writes unless `livePosting` (replies) or `liveStreaming` (stream records) is on |
| `anthropicUrl` | `https://api.anthropic.com` | Content check endpoint |
| `blockedDids` / `optedOutDids` / `botDids` | empty | Manual lists |
| `paintingsPerDay` | 4 | Jobs admitted and jobs started, each over a rolling 24 hours |
| `running` | 1 | Painter slots |
| `requesterPerDay` | 1 | Jobs per person, admitted and started, over 24 hours |
| `requesterCooldownSeconds` | 86400 | Minimum gap between one person's jobs |
| `botCooldownSeconds` | 3600 | Minimum gap between Stillwet replies in one thread |
| `botOnlyTurns` | 4 | Total posts a thread may reach, counting Stillwet's |
| `queue` | 8 | Waiting jobs |
| `queueTtlSeconds` | 21600 | A queued job expires after 6 hours |
| `requestChars` / `requestBytes` | 500 / 2000 | Request size |
| `requestAgeSeconds` | 3600 | Older requests are ignored |
| `globalPerHour` | 40 | Replies sent per hour, all threads |
| `threadPerSixHours` | 30 | Replies per thread per 6 hours |
| `pendingEvents` | 1000 | Unhandled events before ingest stops |
| `pendingReplies` | 64 | Unsent replies the outbox can hold |
| `databaseBytes` | 52428800 (50 MiB) | Page cap on the main SQLite file (see Durability) |
| `eventBytes` | 524288 | Largest Jetstream frame accepted |
| `replyChars` / `replyBytes` | 600 / 3000 | A longer reply stops the flush with an error |
| `imageBytes` | 1000000 | Image size cap (Delve allows 2,000,000) |
| `studio` | oil, 1200x900, engine 7, `claude-opus-5-5`, thinking high, no caps | See Painter. `template`, `root`, `sessions` and `harness` are paths; `root` has to end in `src/a`, like the house studios |

The schema in `src/config.ts` caps every value above. For example, `globalPerHour` can't go past 40 and `running` can't go past 4.

The reply wording is in `assets/replies.json`, and the check prompt in `src/check.ts`. Claude Opus 5.5 wrote both.

## Guards

**Thread guards are stricter than Trinity's.** Trinity counts only the bot posts in a row at the end of a thread, so one human post resets the count. Stillwet treats every author the same and never resets. A thread's count includes:
- every post Stillwet has seen in it
- every reply Stillwet has sent or queued there
- one reserved done reply for each job still queued or running there

Intake stops at 4. A new job is admitted only if the count plus 2 (its first reply and its future done reply) stays within 4. Right before posting, flush counts again, using only seen posts and sent replies and leaving out the reply being posted. If that count has already reached 4, the reply is canceled.

**Cooldowns and rates.** A new request is ignored for an hour after Stillwet's last sent reply in that thread. STOP is the exception. Flush checks the hourly cap, the per-thread six-hour cap and the cooldown right before each write. It leaves out only the attempt it is making itself.

**Accepted and done land at least an hour apart.** Both go to the same thread, so the cooldown holds the done reply back. In the demo, `accepted` is sent at the first flush and `done` at the flush an hour later.

**Queue position and expiry.** The position is the number of queued jobs plus one at admission. It isn't updated later, and a person held back by their own daily limit can be passed by someone else. At 6 hours a queued job is marked expired even if the outbox is full. The `expired` reply goes out once there's room and the thread cooldown allows it.

**Full outbox.** With 64 unsent replies waiting, new requests and STOP confirmations get no reply (`reply_queue_full`), and done replies wait.

**STOP.** These addressed posts count as STOP when the whole text is one of the following, in any case and with an optional `.` or `!` at the end:
- `stop`
- `opt out`, `opt-out` or `optout`
- `do not reply` or `don't reply`
- `stop replying`
- `leave me alone`

STOP records a permanent opt-out in SQLite and cancels that person's jobs and unsent replies. Nothing in the bot removes an opt-out. At most one `stopped` confirmation is queued per person. A full outbox or a thread guard can suppress it, but the opt-out and cancellation still hold. So not every STOP gets an acknowledgment.

**Kill switch.** Setting `STILLWET_KILL_SWITCH` (to any value, even an empty one) or creating the `killFile` halts all outward activity and job work: thread reads, job starts and completions, and replies. A running painter is stopped and its stream ended. The posting code checks it again before every request. Jetstream events are still stored, and opt-outs, edits and deletes still cancel. Other requests that arrive while it's on are skipped, not saved for later.

## Durability

State changes are `BEGIN IMMEDIATE` transactions on SQLite in WAL mode. The tables are:
- `events` and `seen`
- `invalidated` and `optouts`
- `observed`
- `jobs` and `outbox`
- `meta`

Public reads and job-file writes run outside transactions. Job files (`data/jobs/<sha256 of request URI>/job.json`, `BRIEF.md`, then `result.json` and `painting.jpg`) are rewritten from the stored job row whenever the job runs, which repairs a missed write.

Every reply is an outbox row first. Its record key is the first 32 hex digits of the SHA-256 of `<request URI>#<reply kind>`, so a retry always targets the same key and can't create a second record. The writer reads that key before anything else. If the record already exists, it checks that the record matches exactly and writes nothing. Otherwise it reserves the attempt (the cap and cooldown checks above), creates the record and reads it back to verify.

The database refuses to open under a different DID or `dryRun` setting. `databaseBytes` sets `max_page_count` on the main database file only, and only under `npm start`. The WAL and shared-memory files and the job folders come on top, so it isn't a total disk limit.

## Image record

The done reply embeds the painting as `town.delve.embed.images` with one `image/jpeg` blob uploaded with `uploadBlob`, the alt text and the painting's `aspectRatio`. `root` and `parent` are strong references (URI plus CID) to the thread's first post and the request.

The embed type and blob shape match a real published Delve image post, read straight from its PDS: [`getRecord` for `at://did:plc:xatf4zv5rzinoaa2oiwgvteu/town.delve.feed.post/3mxwsxsb2eg25`](https://pds.delve.town/xrpc/com.atproto.repo.getRecord?repo=did%3Aplc%3Axatf4zv5rzinoaa2oiwgvteu&collection=town.delve.feed.post&rkey=3mxwsxsb2eg25). The limits come from [Delve's deployed client](https://delve.town/static/js/main.1afd7f99.js): at most 4 images, `image/*` up to 2,000,000 bytes and a required `alt`.

## Tests

```sh
npm ci --ignore-scripts
npm run check      # typecheck
npm test           # offline: mock Delve, mock Anthropic API, a stubbed painter model
npm run demo       # dry-run transcript against a mocked network
npm run live-read  # public GETs only, no credentials, no writes
npm start          # the daemon (node src/cli.ts run [config.json])
```

`tests/painter.test.ts` launches the real pi with the house harness in a real exported studio and a stubbed model (`tests/fixtures/faux-painter.ts`), so it needs `pi` on `PATH`. It exports the studio template itself from the checkout's HEAD unless `DELVE_TEST_TEMPLATE` names one. `tests/bot.test.ts` covers the content check against a mocked Anthropic API, the fail-closed paths and the switches.

## Streaming (stream.place)

**Flow.** When `streaming.enabled` is on, the painter runs inside `Streaming.run`. Headless Chromium (`streaming.chromiumPath`) opens the house studio viewer at `streaming.viewerUrl/?p=<studio folder>&stream=1`; the viewer (`studio/studio.py`) has to be running on `127.0.0.1`. Its JPEG frames go to ffmpeg, which encodes libx264 video with silent AAC audio for RTMPS ingest at `streaming.ingestUrl/<STREAMPLACE_STREAM_KEY>`. A dry run starts no Chromium or ffmpeg, touches no network and never reads the stream key. It logs the redacted ffmpeg command and the create and end records it would write.

**Limits.**

- One stream at a time, and streaming requires `caps.running` = 1.
- `streaming.maxSeconds` is unset, so a stream lasts as long as its painting. Set, it stops the painter as a time limit, counting startup as well as work, and the partial painting is posted with `doneLimit`; the maximum is 7200 s. A painter that hasn't stopped 10 s after the limit is dropped (the house painter is killed 5 s after its SIGTERM).
- The defaults are 960x540 at 10 fps, with 1200 kbps video and 64 kbps audio. The ceilings are 1280x720 at 15 fps, with 1500 kbps video and 96 kbps audio.
- Every 100 ms the code checks the kill switch (the env variable or the `killFile`) and whether the job was canceled.
- A finished job, an error, a timeout, a kill or a shutdown ends capture before the record is ended. The encoder gets SIGTERM, then SIGKILL after 1 s if it's still running. Closing the browser takes time, and each mock PDS call can take up to 3 s, so there's no guarantee on how long stopping takes.
- Ending the record logs in again first, so a long painting's expired session doesn't block it. A failed end write is logged (`stream_end_failed`) and the painting is still posted. Nothing makes finalization survive a crash. The record's `idleTimeoutSeconds: 30` helps the station clean up a dead stream, but it guarantees nothing locally.

**Record.** The record is created with `createRecord` in the bot DID's repo, in collection `place.stream.livestream`, with a TID record key. It carries:

- `title`
- `url` and `canonicalUrl`: both `https://stream.place/<bot DID>`
- `createdAt`
- `idleTimeoutSeconds`: 30

To end the stream, the code reads the current record and writes it back to the same record key with every field kept, plus `endedAt`. The write passes the current record's CID as `swapRecord`, so it fails if the record changed in between. Nothing is deleted, and no Bluesky announcement is posted. [Lexicon](https://stream.place/docs/lex-reference/place-stream-livestream/)

**Replies.** The accepted and done variants with `includeLink` on add the stream link as a link (a URL facet), and the dry-run variants say nothing streamed. Accepted goes into the outbox before the job starts, and done goes after capture stops. Because the one-hour reply cooldown is unchanged, the accepted reply can arrive after a short job's stream has ended.

**Load.** Measured on a 4-vCPU cloud machine against the real studio viewer showing a short test painting: 60 s of capture at 960x540 and 10 fps took 18.7 CPU seconds (about 0.3 core), and Chromium, ffmpeg, the bot process and the viewer peaked at 1.09 GiB together. The painting itself is the bigger load: an engine 7 replay of the r40 Hopper log at 2400 px peaked at 2.48 GiB and averaged 1.74 cores.

The stream.place research is in `docs/streamplace-research.md`.

## Before a first live post

- A Delve account for Stillwet with an app password, its DID and handle in the config, and the three secrets in place.
- A stream.place login for that DID and a stream key from the stream.place UI.
- The service switched from dry run to `livePosting` (and `streaming.enabled` with `liveStreaming` for streams), which is a config change and a VPS rebuild.

OAuth, PDS compatibility and stream.place indexing haven't been tried against the real services. [Streaming guide](https://stream.place/docs/guides/start-streaming/obs/), [signing](https://docs.stream.place/docs/video-metadata/signing/)

## Provenance

- **Trinity** at `e1b1ba5`: `src/jetstream-parser.ts` is Trinity's file unchanged. `src/delve.ts` and the guard design are adapted from Trinity.
- **claude-paint**: `assets/brief_template.md` is `notes/round29/runner/brief_template.md` copied byte for byte. `PAINTER_MSG` and `CONTINUE_MESSAGE` in `src/painter.ts` match the house runner's `r21_chains.py`. `tests/fixtures/painting.jpg` is the r01 coast painting from [stillwet-paintings](https://github.com/aliceisjustplaying/stillwet-paintings) as a JPEG.
