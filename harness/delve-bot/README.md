# Stillwet on Delve: dry-run prototype

This prototype receives painting requests sent to Stillwet on Delve and decides what to do with each one. It writes the job files and the reply it would post. It never launches a painter and never posts to Delve. When a job finishes, the reply carries an existing Stillwet painting (r01-coast) and says so. Nothing here is deployed. It holds no secrets, has made no live writes and has no PR.

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
   - size, then policy
   - completion-turn reservation
   - limits
   - a free slot or the queue
3. **Pump.** This runs every second. It cancels jobs from opted-out or blocked people, expires old queued jobs and starts queued jobs when the slot and the daily start limit allow. "Running" calls a stub that does nothing. Then the job is marked done, and a done reply with the existing painting goes into the outbox.
4. **Flush.** This sends outbox replies. In dry run a reply is only marked `simulated`.

## Config defaults (`config.json`)

| Setting | Value | What it does |
| --- | --- | --- |
| `did` / `handle` | `did:plc:stillwetprototype` / `stillwet.invalid` | Placeholder identity, not a real account |
| `dryRun` | `true` | `npm start` refuses to run without it |
| `database` | `data/stillwet.sqlite` | SQLite state, plus a `.lock` file holding the PID |
| `killFile` | `data/STOP` | Kill switch file |
| `jetstreamUrl` | `ws://127.0.0.1:6008/subscribe` | Event feed |
| `appviewUrl` | `https://api.delve.town` | Public reads: thread, feed, profile |
| `pdsUrl` | `https://pds.delve.town` | Writes go only to a `127.0.0.1` mock, so this real host is refused |
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
| `studio` | oil, 1200x900, engine 7, `claude-opus-5-5`, 7200 s, 500000 in / 50000 out tokens, $10 | Copied into each `job.json`. Nothing enforces it yet because nothing launches |

The schema in `src/config.ts` caps every value above. For example, `globalPerHour` can't go past 40 and `running` can't go past 4.

## Reply templates (`assets/replies.json`)

These are fixed text. No model writes or checks them at runtime.

| Key | Text |
| --- | --- |
| `accepted` | Got it! I've reserved a painter slot and written up the job. This prototype is a stub, so no painter is launched. |
| `queued` | Thanks! Your request is in line at position {position}, waiting for a free painter slot or room under the daily start limit. This prototype is a stub, so no painter is launched. |
| `declinedPolicy` | Sorry, I can't take this one. This prototype only paints short requests built from a small list of safe subjects, like cartoon monsters, dragons, cats or the coast. Real people, sexual content, hate and graphic gore are always out. |
| `declinedLimit` | Sorry, a request limit has been reached, so I can't take this one. |
| `declinedSize` | Sorry, that request is too long for me to take. A shorter description would fit. |
| `declinedQueue` | Sorry, the line is full right now, so I can't take your request. |
| `done` | Here is an existing Stillwet painting. This prototype doesn't paint new requests, so it wasn't made for yours. |
| `stopped` | Done. You're opted out and any pending request is canceled. Your opt-out stays in place, and a new request won't turn it back on. |
| `expired` | Sorry, your request expired before a painter slot opened, so it's been closed. |
| `imageAlt` | Oil painting of a calm sea at dusk under a crescent moon. A tall sailing ship and a smaller one rest on the water, and a lone figure in a long cloak stands on a dark beach beside a half-buried anchor. |

## Content policy: safe words only

A request passes only if all of these are true once `@stillwet.invalid` is removed:
- it uses letters, spaces and `. , ! ? -` only
- every word is on one of the two lists below
- at least one word is a subject

Nothing tries to judge whether a free-form request is fictional. Everything else gets `declinedPolicy`.

```text
subjects:  cat, cats, dog, dogs, rabbit, rabbits, owl, owls, fox, foxes, dragon, dragons, monster, monsters, robot, robots, mountain, mountains, forest, forests, tree, trees, lake, lakes, ocean, coast, flower, flowers, cloud, clouds, moon, sun, castle, castles
modifiers: a, an, the, and, with, by, in, on, under, over, near, at, of, is, please, paint, draw, picture, painting, me, for, some, one, two, three, small, big, tiny, blue, red, green, yellow, purple, orange, pink, white, black, golden, bright, dark, soft, gentle, quiet, sleepy, sleeping, happy, friendly, silly, cartoon, cartoonish, fictional, imaginary, sunset, sunrise, night, snow, rain, mist, stars, sitting, flying, walking, garden, watercolor, oil
```

This declines plenty of harmless requests, on purpose. Checked against the real lists:

```text
accepted  Paint a sleepy cat
accepted  Please paint two friendly cartoon dragons flying over the ocean at sunset!
declined  Paint a goblin                      (not on the list)
declined  Paint my cat                        ("my" isn't on the list)
declined  Paint a lighthouse on the coast     ("lighthouse")
declined  Paint a blue sunset                 ("sunset" is a modifier, so there's no subject)
declined  Paint the cat's garden              (apostrophe)
declined  Paint a cat 🐱                      (emoji)
declined  Paint a famous real person
```

## Guards

**Thread guards are stricter than Trinity's.** Trinity counts only the bot posts in a row at the end of a thread, so one human post resets the count. This prototype treats every author the same and never resets. A thread's count includes:
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

STOP records a permanent opt-out in SQLite and cancels that person's jobs and unsent replies. Nothing in the prototype removes an opt-out. At most one `stopped` confirmation is queued per person. A full outbox or a thread guard can suppress it, but the opt-out and cancellation still hold. So not every STOP gets an acknowledgment.

**Kill switch.** Setting `STILLWET_KILL_SWITCH` (to any value, even an empty one) or creating `data/STOP` halts all outward activity and job work: thread reads, job starts and completions, and replies. The posting code checks it again before every request. Jetstream events are still stored, and opt-outs, edits and deletes still cancel. Other requests that arrive while it's on are skipped, not saved for later.

## Durability

State changes are `BEGIN IMMEDIATE` transactions on SQLite in WAL mode. The tables are:
- `events` and `seen`
- `invalidated` and `optouts`
- `observed`
- `jobs` and `outbox`
- `meta`

Public reads and job-file writes run outside transactions. Job files (`data/jobs/<sha256 of request URI>/job.json` and `BRIEF.md`) are rewritten from the stored job row whenever the job runs, which repairs a missed write.

Every reply is an outbox row first. Its record key is the first 32 hex digits of the SHA-256 of `<request URI>#<reply kind>`, so a retry always targets the same key and can't create a second record. The writer reads that key before anything else. If the record already exists, it checks that the record matches exactly and writes nothing. Otherwise it reserves the attempt (the cap and cooldown checks above), creates the record and reads it back to verify.

The database refuses to open under a different DID or `dryRun` setting. `databaseBytes` sets `max_page_count` on the main database file only, and only under `npm start`. The WAL and shared-memory files and the job folders come on top, so it isn't a total disk limit.

## Image record

The done reply has this shape. The blob is the real `assets/finished.png` (1000x714, 453803 bytes), and the CID was computed from its bytes. `root` and `parent` are strong references (URI plus CID). Here they're the same because the demo request starts its own thread. In a deeper thread `root` is the thread's first post and `parent` is the request.

```json
{
  "$type": "town.delve.feed.post",
  "text": "Here is an existing Stillwet painting. This prototype doesn't paint new requests, so it wasn't made for yours.",
  "createdAt": "2026-10-16T08:00:00.000Z",
  "reply": {
    "root":   { "uri": "at://did:plc:requester1/town.delve.feed.post/demo1", "cid": "bafy-demo1" },
    "parent": { "uri": "at://did:plc:requester1/town.delve.feed.post/demo1", "cid": "bafy-demo1" }
  },
  "embed": {
    "$type": "town.delve.embed.images",
    "images": [
      {
        "image": {
          "$type": "blob",
          "ref": { "$link": "bafkreiabtjsk5hdjujpwuatraw6drv2x3n2ohsqvcoccsxm4hcczlem3vq" },
          "mimeType": "image/png",
          "size": 453803
        },
        "alt": "Oil painting of a calm sea at dusk under a crescent moon. A tall sailing ship and a smaller one rest on the water, and a lone figure in a long cloak stands on a dark beach beside a half-buried anchor.",
        "aspectRatio": { "width": 1000, "height": 714 }
      }
    ]
  }
}
```

The embed type and blob shape match a real published Delve image post, read straight from its PDS: [`getRecord` for `at://did:plc:xatf4zv5rzinoaa2oiwgvteu/town.delve.feed.post/3mxwsxsb2eg25`](https://pds.delve.town/xrpc/com.atproto.repo.getRecord?repo=did%3Aplc%3Axatf4zv5rzinoaa2oiwgvteu&collection=town.delve.feed.post&rkey=3mxwsxsb2eg25). The limits come from [Delve's deployed client](https://delve.town/static/js/main.1afd7f99.js): at most 4 images, `image/*` up to 2,000,000 bytes and a required `alt`.

## What has actually run

**Live public reads** (2026-10-16 02:16 Stockholm time) used no credentials and wrote nothing. There were 3 public GET requests: the town feed (20 posts), one thread (5 posts) and one profile. All 20 posts came back `unaddressed_or_invalid`, as expected, because Stillwet's DID here is a placeholder nobody can mention. Zero replies were queued.

**The dry-run demo** is the opposite: synthetic requests against a mocked Delve server.
- "Paint a sleepy cat" was `accepted`.
- "Paint a friendly cartoon monster" was `queued` at position 1.
- "Paint a famous real person" got `declinedPolicy`.

Both jobs finished with the existing painting. Every reply ended as `simulated`, with `pdsWrites: 0`.

## Commands

```sh
npm ci --ignore-scripts
npm run check      # typecheck
npm test           # offline tests with a mock network
npm run demo       # the dry-run demo above
npm run live-read  # public GETs only, no credentials, no writes
npm start          # dry-run daemon
```

`npm start` is optional and none of the tests launch it. It connects only to the configured Jetstream on localhost. For each addressed post it reads the thread from the public Delve server. It refuses to run with `dryRun` off.

## Before this could go live

None of this is done or approved.

- **Account.** Stillwet needs its own Delve account and an app password, given to the service as `DELVE_HANDLE` and `DELVE_APP_PASSWORD` from a secret store. Credentials never go in `config.json` or SQLite. Today the code reads them only on the mock path, and the tests use fake values.
- **Writer.** The production writer is hard-disabled: posting throws `live_writes_disabled_in_prototype` for anything but a `127.0.0.1` PDS. Changing that needs explicit authorization and a code change.
- **Painters.** Real painters need a separate, approved sandbox host. No location has been chosen, and alice-m3p is untouched.
- **Safeguards.** A trusted runner has to enforce the `studio` cost, time, token and canvas limits. Today they're data only.

## Provenance

- **Trinity** at `e1b1ba5`: `src/jetstream-parser.ts` is Trinity's file unchanged. `src/delve.ts` and the guard design are adapted from Trinity.
- **claude-paint** at `de23782a`: `assets/brief_template.md` is `notes/round29/runner/brief_template.md` copied byte for byte, so the brief is the existing house text. The only things inserted are the accepted request as its opening, the studio folder name and the reading line. The studio settings travel separately in `job.json`. `assets/finished.png` and `assets/RECORD.md` are from `archive/collection/gallery/r01-coast/`.
- Nothing comes from ACS, Golden Gate or Haiku.

## Streaming (stream.place)

Streaming is opt-in. `streaming.enabled` and `streaming.includeLink` both default to `false`, and `dryRun` defaults to `true`. Nothing here is live.

**Flow.** When streaming is enabled, the painter stub runs inside `Streaming.run`. Headless Chromium opens `http://127.0.0.1:8766/?p=<job directory>/studio&stream=1`. Its JPEG frames go to ffmpeg, which encodes libx264 video with silent AAC audio for the planned RTMPS ingest. OBS isn't used.

Production ingest and the production PDS are blocked in code even with `dryRun=false`. Mock mode accepts only a `file://` `.flv` output or `rtmp://127.0.0.1/live`, plus a PDS at a loopback address (127.0.0.1). A dry run starts no Chromium or ffmpeg, touches no network and never reads the stream key. It logs the redacted ffmpeg command and the create and end records it would write.

**Limits.**

- One stream at a time, and streaming requires `caps.running` = 1.
- The timer defaults to 7200 s, which is also the maximum. It counts startup as well as work.
- The defaults are 960x540 at 10 fps, with 1200 kbps video and 64 kbps audio. The ceilings are 1280x720 at 15 fps, with 1500 kbps video and 96 kbps audio.
- Every 100 ms the code checks the kill switch (the env variable or `data/STOP`) and whether the job was canceled.
- A finished job, an error, a timeout, a kill or a shutdown ends capture before the record is ended. The encoder gets SIGTERM, then SIGKILL after 1 s if it's still running. Closing the browser takes time, and each mock PDS call can take up to 3 s, so there's no guarantee on how long stopping takes.
- A failed cleanup is reported, but nothing makes finalization survive a crash. The record's `idleTimeoutSeconds: 30` helps the station clean up a dead stream, but it guarantees nothing locally.

**Record.** The record is created with `createRecord` in the bot DID's repo, in collection `place.stream.livestream`, with a TID record key. It carries:

- `title`
- `url` and `canonicalUrl`: both `https://stream.place/<bot DID>`
- `createdAt`
- `idleTimeoutSeconds`: 30

To end the stream, the code reads the current record and writes it back to the same record key with every field kept, plus `endedAt`. The write passes the current record's CID as `swapRecord`, so it fails if the record changed in between. Nothing is deleted, and no Bluesky announcement is posted. [Lexicon](https://stream.place/docs/lex-reference/place-stream-livestream/)

**Replies.** The optional accepted and done variants add the stream link as a link (a URL facet), and the dry-run variants say no stream ran. Accepted goes into the outbox before the job starts, and done goes after capture stops. Because the one-hour reply cooldown is unchanged, the accepted reply can arrive after a short job's stream has ended.

**Going live needs:**

- Code changes, approved separately, that remove the blocks on production ingest and the production PDS and the daemon's dry-run-only checks. A key or a config change alone can't turn streaming on.
- A Streamplace login with the bot's atproto DID, which can be the Delve PDS identity.
- A stream key generated in the Streamplace UI and stored as the secret `STREAMPLACE_STREAM_KEY`.
- Approval for the login and for the public record writes. Neither has been given.
- A capture host where the existing local viewer on port 8766 can see the studio. No host is chosen, alice-m3p wasn't accessed and the stub creates no studio content.

OAuth and PDS compatibility and Streamplace indexing haven't been tested. [Streaming guide](https://stream.place/docs/guides/start-streaming/obs/), [signing](https://docs.stream.place/docs/video-metadata/signing/)

**CPU caveat.** One synthetic animated color canvas, written to a local file at 960x540 and 10 fps:

- 11.60 s of wall time for 10.621 s of H.264 (106 frames)
- 4.17 CPU seconds, about 0.36 of a core
- 205,624 KiB peak memory for the largest single process, not for everything together

Load with the real oil viewer hasn't been measured, and limiting x264 to 2 threads doesn't cap Chrome's CPU.

The detailed research is in `docs/streamplace-research.md` in the STIL-113 handoff.
