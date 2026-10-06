t task { id i, text s, done b }

f db() s
  ret home() + "/todo.txt"

f load() s
  ret read(db())

f save(line s)
  write(db(), line)

f next_id() i
  v txt = load()
  if len(txt) == 0
    ret 1
  v rows = lines(txt)
  v last = get(rows, count(rows) - 1)
  v parts = split(last, "|")
  ret int(get(parts, 0)) + 1

f main()
  v cmd = arg(1) ? "list"
  v txt = load()

  if cmd == "add"
    v what = arg(2) ? ""
    if what == ""
      p "usage: todo add <text>"
      quit(1)
    v line = "{next_id()}|{what}|0"
    if len(txt) > 0
      append(db(), line + "\n")
    e
      save(line + "\n")
    p "task {next_id() - 1}: added {what}"
  elif cmd == "done"
    v id = int(arg(2) ? "0")
    if id == 0
      p "usage: todo done <id>"
      quit(1)
    m out = []
    each row lines(txt)
      v parts = split(row, "|")
      if int(get(parts, 0)) == id
        push(out, "{get(parts, 0)}|{get(parts, 1)}|1")
      e
        push(out, row)
    save(join(out, "\n") + "\n")
    p "task {id} done"
  elif cmd == "rm"
    v id = int(arg(2) ? "0")
    if id == 0
      p "usage: todo rm <id>"
      quit(1)
    m out = []
    each row lines(txt)
      v parts = split(row, "|")
      if not int(get(parts, 0)) == id
        push(out, row)
    save(join(out, "\n") + "\n")
    p "task {id} removed"
  elif cmd == "list"
    if len(txt) == 0
      p "no tasks"
    e
      each row lines(txt)
        if len(row) > 0
          v parts = split(row, "|")
          if get(parts, 2) == "1"
            p "[x] #{get(parts, 0)} {get(parts, 1)}"
          e
            p "[ ] #{get(parts, 0)} {get(parts, 1)}"
  elif cmd == "clear"
    save("")
    p "cleared"
  e
    p "commands: add <text> | done <id> | rm <id> | list | clear"
