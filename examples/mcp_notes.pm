tool add_note(text s) s "add a note to the store"
  append(pm_home() + "/notes.txt", text + "\n")
  ret "added: {text}"

tool list_notes(x s) s "list all notes"
  v all = read(pm_home() + "/notes.txt")
  if len(all) == 0
    ret "no notes"
  ret all

tool note_count(x s) s "count notes"
  v all = read(pm_home() + "/notes.txt")
  ret str2(count(lines(all)))
