# The reader's record of SONF painter 1 (Sonnet 5.5 max), written as a chain's would be, for SON5F to continue.
import sys; sys.path.insert(0, "~/tmp/gallery-fcf9c110/r19")
import r19_chains as C
rd = C.RUN / "SONF"; n = 1; d = C.A / "paint-studio-58dfc6"
logs = [p for s in C.load_sittings(rd, n) for p in s.get("sessions", []) if C.Path(p).exists()]
assert logs, "no session logs"
out = rd / "p1_record.md"
rb = ((C.HERE / "reader_brief.md").read_text().replace("{LOG}", ", ".join(logs))
      .replace("{JOURNAL}", str(d / "notes/journal.md")).replace("{OUT}", str(out)))
(rd / "p1_reader_brief.md").write_text(rb)
print("logs:", logs)
C.run(C.reader_cmd(f"Read {rd}/p1_reader_brief.md and do what it says."), rd, rd / "p1_reader_final.txt", rd / "p1_reader_err.txt")
print("record", "written" if out.exists() else "MISSING")
