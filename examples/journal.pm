f db() s
  ret home() + "/journal.txt"

f main()
  v cmd = arg(1) ? "today"
  v txt = read(db())

  if cmd == "write"
    m entry = arg(2) ? ""
    if entry == ""
      entry = ask("entry: ")
    if entry == ""
      p "nothing to write"
      quit(1)
    append(db(), "{today()} {entry}\n")
    p "saved [{today()}] {entry}"
  elif cmd == "today"
    m found = 0
    each row lines(txt)
      if len(row) > 0
        if starts(row, today())
          p cut(row, 11, len(row))
          found = found + 1
    if found == 0
      p "no entry for {today()}. write one: journal write <text>"
  elif cmd == "find"
    v q = arg(2) ? ""
    if q == ""
      p "usage: journal find <text>"
      quit(1)
    m hits = 0
    each row lines(txt)
      if len(row) > 0
        if has(lower(row), lower(q))
          p row
          hits = hits + 1
    if hits == 0
      p "no matches for {q}"
  elif cmd == "all"
    if len(txt) == 0
      p "journal is empty"
    each row lines(txt)
      if len(row) > 0
        p row
  elif cmd == "clear"
    write(db(), "")
    p "journal cleared"
  e
    p "commands: write [text] | today | find <text> | all | clear"
