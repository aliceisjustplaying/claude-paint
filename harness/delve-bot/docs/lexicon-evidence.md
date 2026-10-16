# Delve post image embed evidence

Observed 2026-10-16 at approximately 00:04 UTC. Read-only research: all external requests were HTTP GET. No repository files were changed.

## Confirmed published record

The post record `$type` is **`town.delve.feed.post`**. Its image embed `$type` is **`town.delve.embed.images`**, not `app.bsky.embed.images`.

Primary source: [PDS getRecord](https://pds.delve.town/xrpc/com.atproto.repo.getRecord?repo=did%3Aplc%3Axatf4zv5rzinoaa2oiwgvteu&collection=town.delve.feed.post&rkey=3mxwsxsb2eg25).

- URI: `at://did:plc:xatf4zv5rzinoaa2oiwgvteu/town.delve.feed.post/3mxwsxsb2eg25`
- Record CID: `bafyreibxh3a57vioir532h2miwpnna4bv5lm4izyf6j6kjkeubaqgjt6wi`
- Full unmodified response: `image-post-record.json`

The record's exact image embed is:

```json
{
  "$type": "town.delve.embed.images",
  "images": [
    {
      "alt": "Show 221 on swarm.art: Cells, growing, theme: grow out from a single seed in the middle. A drawing made together by 19 hands, agents and people, one move at a time.",
      "aspectRatio": { "height": 1500, "width": 1200 },
      "image": {
        "$type": "blob",
        "ref": {
          "$link": "bafkreieuaepsc2vpgwyexu534mbnectzoe2movyzqp2c3hfcpolguuc7gu"
        },
        "mimeType": "image/jpeg",
        "size": 403108
      }
    }
  ]
}
```

The full record also has `createdAt`, `text`, `langs` and `facets`; see the raw response for the complete example. Blob `ref` is a JSON CID link object, not a URL or bare CID string. Blob `size` is the numeric byte count.

## Public client schema and publisher

Primary source: [Delve's deployed client JavaScript](https://delve.town/static/js/main.1afd7f99.js), downloaded from the script referenced by [the public app](https://delve.town). Saved as `delve-main.js`. Relevant verbatim excerpts and zero-based JavaScript string offsets are in `client-schema-excerpts.txt`.

The deployed schema declares:

| Object | Fields and constraints in the client schema |
| --- | --- |
| `town.delve.feed.post` | Required `text` and `createdAt`; optional `embed` union includes images, video, gallery, external, record and recordWithMedia. |
| `town.delve.embed.images` | Required `images` array, maximum length **4**. |
| Each images item | Required `image` blob accepting `image/*`, maximum size **2,000,000** bytes; required `alt` string; optional `aspectRatio`. |
| `town.delve.embed.defs#aspectRatio` | Required integer `width` and `height`, each minimum **1**. |
| `town.delve.embed.images#view` | `images` array containing `thumb`, `fullsize`, `alt` and optional `aspectRatio`. This is the rendered appview shape, not the stored blob shape. |

The publisher code explicitly constructs `{$type:"town.delve.embed.images",images:...}` and each item as `{image:(await x(...)).blob,alt:e.alt,aspectRatio:{width:n,height:r}}`. It does not insert an item `$type` for the ordinary images array. The live PDS record above confirms that serialization.

There is also a distinct gallery format in the same deployed client schema and publisher:

- Embed `$type`: **`town.delve.embed.gallery`** with an `items` array, maximum length **20**.
- Each item has `$type`: **`town.delve.embed.gallery#image`**, `image` blob accepting `image/*` up to 2,000,000 bytes, `alt` string and required `aspectRatio`.
- Gallery's view uses `town.delve.embed.gallery#view` and `town.delve.embed.gallery#viewImage`; its image URL field is `thumbnail`, unlike ordinary images' `thumb`.

Gallery is sourced from the deployed schema and publisher, not from a published gallery record in this research. No independent raw lexicon JSON was located; the constraints above are from the actual deployed runtime schema. The ordinary images type and blob shape are independently verified against a PDS record.

## Feed corroboration

Primary source: [requested public feed, limit 20](https://api.delve.town/xrpc/town.delve.feed.getFeed?feed=at%3A%2F%2Fdid%3Aplc%3Aqzqct2rrq4u2gmy5g3mjxske%2Ftown.delve.feed.generator%2Ftown&limit=20).

Saved complete response: `feed.json`; HTTP 200 headers: `feed-headers.txt`. The PDS record above also appeared in the feed. A local comparison confirmed that its PDS `value` and appview `record` were identical. Extracted record plus appview embed: `image-post-appview.json`. Other extracted embedded records: `feed-image-records.json`.

The appview embed for this post has `$type: "town.delve.embed.images#view"` and image `thumb` / `fullsize` URLs. The stored record instead contains `$type: "town.delve.embed.images"` with the `image` blob object shown above. Those two shapes are not interchangeable.

Primary endpoint configuration source: [public settings.json](https://delve.town/settings.json), saved as `settings.json`, names `https://pds.delve.town`, `https://api.delve.town` and town feed creator `did:plc:qzqct2rrq4u2gmy5g3mjxske`.

## Integrity

SHA-256 of downloaded sources:

```text
029ffa7feb678507ff9caaef7876a875e85be25634d1c17134e884f8e322f701  feed.json
2727a3fab0c885d5d4e73c2b69f62f8e7be5291382a8c2eec64eec3871c93d64  image-post-record.json
63434b6ef60df30ddc925968b722710d30d867e479d5afcf302bf651217c529f  delve-main.js
```
