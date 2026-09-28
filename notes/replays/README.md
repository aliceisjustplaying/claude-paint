# Timelapses of the Rust-program paintings

Rounds 1, 2, 7-9 (middle arm) and 10-15 were Rust programs. A timelapse of one is **the final program
replayed**, stroke by stroke in code order, not the painter's process (that's the looks slideshow).

`r10_frames.rs` + `r10_frames.patch` (round 10's engine, base 45a63b3, shared by all three arms): a frame
recorder in the paint crate, off unless `PAINT_FRAMES_DIR` is set (`PAINT_FRAMES_WIDTH`, long side, default
1920; `PAINT_FRAMES_EVERY`, seconds of hand time, default 60). Round 10 paints with hand time off, so a pass is
one batch of tiles; with frames on, a pass runs its tiles in consecutive groups (same order, same stroke ids)
with a frame between them. The painting is unchanged: at 1000 px the PNG is byte-identical with frames on and
off, and all three round 10 paintings rendered at 3200 with frames are pixel-identical to their original finals.

    git worktree add --detach <dir> r10-arm3 && cd <dir> && git apply r10_frames.patch && cp r10_frames.rs crates/paint/src/frames.rs
    PAINT_FRAMES_DIR=frames/r10-3 target/release/r10_summer --width 3200 --out r10-3.png   # after cargo build --release -p paintings --bin r10_summer
    scripts/replay_clip r10-3.png r10-3-1920.mp4 --reuse --frames-dir frames/r10-3 --width 1920 --pace dynamic --gamma 0.4 --ramp 0.3 --length 20 --open-hold 0.25

Round 10 (2026-09-28): 702-743 s a painting at 3200 (with a replay batch running alongside), 1,529-1,873 frames.
Mid-pass frames show tiles half done (checkerboard and banding in the skies, a striped crown on the lime).
