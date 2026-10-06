# r18r: resume of round 18's DSK, MIMO, BUN, GLM

- Runner: r18r_open.py (copy of ../r18g/r18g_open.py). RUN = ../r18/run; four opencode-go lanes; never exports.
- Backup of ../r18/run before any change: ~/tmp/r18-resume-054c85eb/r18_run_backup
- Markers moved to p1.{done,painted,finished}.before_resume; old outputs renamed *_finished_before_resume.{png,lua}, p1_final/p1_finish* *_before_resume.
- Easel: bin/easel in the four studios replaced by the r17-base 40db9c4 build (has 5656ef2); old binaries and pre-swap logs in scratch pre_swap/<studio>/.
- 2026-09-27 17:51 launched (pid 81962/81978); 18:00 HALTED by hand: `bin/easel open` deadlocks replaying every lane's log
  (main thread in paint::handling::Canvas::run_plans -> rayon in_worker_cold/LockLatch::wait_and_reset, all workers asleep, 0% CPU,
  no socket). Old binary deadlocks the same on MIMO's log (scratch repro_old). New sittings left 'running' -> retaken as 'interrupted' on rerun.
- Relaunch after an easel fix: swap bin/easel again, then `nohup uv run r18r_open.py >> ../r18/run/runner.log 2>&1 &`.
