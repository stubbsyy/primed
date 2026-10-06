f path() s
  ret home() + "/notes.txt"

f main()
  v cmd = arg(1) ? "list"
  v db = path()

  if cmd == "add"
    v note = arg(2) ? ""
    if note == ""
      p "usage: notes add <text>"
      quit(1)
    append(db, note + "\n")
    p "added: {note}"
  elif cmd == "list"
    if argn() < 1
      p "no notes"
    v txt = read(db)
    if len(txt) == 0
      p "no notes"
    e
      p txt
  elif cmd == "clear"
    write(db, "")
    p "cleared"
  e
    p "commands: add <text> | list | clear"
