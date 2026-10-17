# Streamplace research

Read on October 16, 2026. Source checkout: `streamplace/streamplace` at `425f1beb2b8e6ee55edccc1533a15c8ed4c68714`. These are sourced protocol findings, not a successful live integration. No account, stream key or authenticated production request was used.

## Ingest and stream keys

Streamplace accepts RTMP/RTMPS and WHIP. Its [OBS guide](https://stream.place/docs/guides/start-streaming/obs/) prefers RTMP and currently gives `rtmps://rtmp.stream.place:1935/live`, not the old `rtmps://stream.place:1935/live`. A publisher appends `/STREAM_KEY` to that server URL. Native server source strips `/live/` and passes the remainder to `MakeMediaSigner`: [pkg/api/rtmp_server.go:30–42](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/pkg/api/rtmp_server.go#L30-L42). This prototype plans RTMPS because stock ffmpeg supports it without a separate WHIP-enabled build.

The same guide supplies `https://stream.place` as the WHIP server with the key as its Bearer token. This isn't an invented `/whip` URL: the source registers POST `/` as WHIP at [pkg/api/api.go:287](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/pkg/api/api.go#L287), plus `/api/ingest/webrtc` and `/api/ingest/webrtc/:key` at lines 222–223. [pkg/api/playback.go:70–100](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/pkg/api/playback.go#L70-L100) requires `application/sdp` and supports either `Authorization: Bearer <key>` or the path key. The [development guide](https://stream.place/docs/guides/start-contributing/streaming-in-development/) demonstrates the latter with a WHIP-enabled ffmpeg. WHIP uses H.264 plus Opus; RTMP uses H.264 plus AAC. The encoding guide calls for one-second keyframes and no B-frames, which the prototype enforces with `-g <fps> -bf 0`.

Station endpoints can be discovered with public GET [`/xrpc/place.stream.ingest.getIngestUrls`](https://stream.place/docs/api/operations/placestreamingestgetingesturls/). [pkg/spxrpc/place_stream_ingest.go](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/pkg/spxrpc/place_stream_ingest.go) honors configured advertised endpoints, otherwise derives RTMP/RTMPS and WHIP endpoints. Its secure response can use type `rtmps`, although the generated API prose only lists `rtmp` and `whip`. The prototype does no discovery or production network calls; `streaming.ingestUrl` holds the documented RTMPS base without a key.

Keys are signing credentials, not random strings chosen by the bot. The user logs in to Streamplace, opens its live dashboard and generates a key. Current [js/web/src/lib/store/slices/blueskySlice.ts:493–530](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/js/web/src/lib/store/slices/blueskySlice.ts#L493-L530) generates an exportable secp256k1 keypair, concatenates private key bytes with the account DID bytes and encodes that as base58btc multibase. It writes the public key to the account's PDS with `com.atproto.repo.createRecord`, collection `place.stream.key`, record fields `$type`, `signingKey` (the public `did:key`), `createdAt` and `createdBy`. Only the public half goes in that record.

The [signing documentation](https://docs.stream.place/docs/video-metadata/signing/) describes the resulting C2PA segment signing and metadata preferences. [pkg/api/stream_key.go](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/pkg/api/stream_key.go) decodes the private credential, resolves its DID and verifies the matching public signing-key record and station authorization. A fabricated key won't work. The prototype doesn't generate, publish or provision keys or metadata preferences. A future authorized deployment would receive an already generated key through `STREAMPLACE_STREAM_KEY`; dry run doesn't even read it. ffmpeg receives the real value only in local mock mode. Child stdout is restricted to ffmpeg progress, stderr is discarded and all surfaced errors are fixed strings. The key is absent from logs, config and stored job/reply/record data; in an eventual RTMP deployment it would still be visible to same-host process inspection as part of ffmpeg's argv.

## Announcing and ending a livestream

The current lexicon remains [`place.stream.livestream`](https://stream.place/docs/lex-reference/place-stream-livestream/), with a TID record key. Pinned [lexicons/place/stream/livestream.json](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/lexicons/place/stream/livestream.json) requires only `title` and `createdAt`. The title has `maxGraphemes: 140` and `maxLength: 1400`. Relevant optional fields include `url`, `canonicalUrl`, `lastSeenAt`, `endedAt`, `idleTimeoutSeconds`, `post`, `thumb` and notification settings. `url` is a stream-location/replication hint; `canonicalUrl` is the viewer-facing URL.

The current web dashboard builds a record with `url: <station>/<DID>`, `createdAt`, `lastSeenAt` and idle-timeout settings: [js/web/src/components/dashboard/stream-info.tsx:167–207](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/js/web/src/components/dashboard/stream-info.tsx#L167-L207). It calls `place.stream.live.startLivestream` for a new record. That convenience endpoint can add a thumbnail and a Bluesky announcement. Its [lexicon](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/lexicons/place/stream/live/startLivestream.json) explicitly permits creating the record manually. The backend's actual PDS write is `com.atproto.repo.createRecord` with `repo: session.DID` and `collection: place.stream.livestream`: [pkg/spxrpc/place_stream_live.go:678–689](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/pkg/spxrpc/place_stream_live.go#L678-L689).

The prototype takes that direct-record path, not the convenience endpoint, and doesn't create a Bluesky post or upload a thumbnail. After ffmpeg reports its first output frame, it creates one record in the bot DID's repository:

```json
{
  "repo": "did:plc:stillwetprototype",
  "collection": "place.stream.livestream",
  "rkey": "<new 13-character TID>",
  "validate": true,
  "record": {
    "$type": "place.stream.livestream",
    "title": "Stillwet studio prototype stub: no painter runs and no new painting is made",
    "url": "https://stream.place/did:plc:stillwetprototype",
    "canonicalUrl": "https://stream.place/did:plc:stillwetprototype",
    "createdAt": "<ISO timestamp>",
    "idleTimeoutSeconds": 30
  }
}
```

This identity is still a placeholder. Stopping media and ending the announcement are separate operations: the former ends capture/ingest; the latter GETs the current record and PUTs the same key with `endedAt` and `swapRecord: <current CID>`, preserving station updates. Nothing is deleted. This follows the current stop implementation in [pkg/spxrpc/place_stream_live.go:786–824](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/pkg/spxrpc/place_stream_live.go#L786-L824) and the station finalizer in [pkg/statedb/queue_processor.go:544–583](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/pkg/statedb/queue_processor.go#L544-L583). The prototype ends by updating the record even after the kill switch, since that is cleanup rather than a new stream. Failed finalization is surfaced as an error; there is no crash-durable retry queue or guarantee that an unapproved future station would finalize an idle record.

## Can the Streamplace identity also be the Delve identity?

Yes at the identity/repository level: both use an AT Protocol DID and a PDS, rather than separate required Streamplace and Delve account systems. The [Streamplace prerequisites](https://stream.place/docs/guides/start-streaming/obs/) say an AT Protocol account is used to log in. Key records and livestream records are written under that logged-in DID. Its [OAuth client](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/js/web/src/lib/oauth.ts) and [resolver](https://github.com/streamplace/streamplace/blob/425f1beb2b8e6ee55edccc1533a15c8ed4c68714/js/web/src/lib/oauth-resolver.ts) route identity login through Streamplace's OAuth proxy; they don't require a `bsky.social` handle suffix.

Delve's operator confirms that `town.delve.*` and `app.bsky.*` records can coexist in the same PDS account in this [October 4 explanation](https://delve.town/profile/did:plc:hr6mywilgrfqqvoxo6t4ldo2/post/3mx3kkhufqc2x). Streamplace's `place.stream.*` collections can likewise live in that repository. A Delve-only profile doesn't automatically become a Bluesky profile. Actual Streamplace login to the future bot's PDS, any profile prerequisites, station allowlists and indexing still need an authorized integration check; none was exercised here. The prototype deliberately reuses `config.did` for both record families and the stream URL, rather than introducing a second identity. Mock PDS authentication uses existing fake `DELVE_HANDLE`/`DELVE_APP_PASSWORD` credentials; that isn't a production OAuth implementation.

Optional reply links use `town.delve.richtext.facet#link`, with UTF-8 byte offsets. The deployed [Delve client schema](https://delve.town/static/js/main.1afd7f99.js), read again for this task, defines `town.delve.richtext.facet` with a `link` feature containing a required URI and a byte slice with `byteStart`/`byteEnd` (offset 2704579 in that bundle). A test verifies that slicing the reply's UTF-8 bytes gives the exact stream URL.

## Measured locally versus not verified

`tests/streaming.test.ts` uses a synthetic HTTP viewer on cloud loopback port 8766, actual headless Chromium/ffmpeg to a local FLV file sink and a separate mocked PDS. It checks decoded pixel color, H.264 dimensions/frame count, AAC, start/end writes, CID-swap preservation, uncertain-create recovery, errors, timer, kill, cancellation, shutdown, single-stream admission, config ceilings, URL facets and production-write rejection. No real oil viewer, painter, account, RTMPS server or WHIP endpoint was used. A live Streamplace stream and real PDS schema/indexing behavior remain unverified.

Synthetic CPU probe: a changing-color canvas at 960x540/10 fps, local file output, two x264 threads, one 10-second hold after startup. On this 4-vCPU AMD EPYC cloud machine, GNU time measured 11.60 seconds elapsed, 3.12 seconds user CPU plus 1.05 seconds system CPU (about 0.36 core averaged over that interval). ffprobe counted 106 H.264 frames at 10 fps, 10.621 seconds of media and AAC audio. Largest-process max RSS was 205624 KiB, not the sum across processes. This is not a real oil-viewer benchmark or a host-sizing guarantee. Chromium rendering and a real painter add unmeasured cost; x264's two-thread setting doesn't cap the browser. Toolchain: Node 24.21.0, ffmpeg 6.1.1, Chrome 155.0.8059.84, playwright-core 1.56.1. Scratch media isn't included in the handoff.
