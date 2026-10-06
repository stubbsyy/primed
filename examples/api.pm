f hello(req s) s
  ret "hello from primed"

f greet(req s) s
  v name = jstr(jget(req, "body"))
  ret jo("msg", "hi {name}")

f notes(req s) s
  v all = read(pm_home() + "/notes.txt")
  v n = count(lines(all))
  ret jo("notes", str2(n))

f main()
  route("/", hello)
  route("/greet", greet)
  route("/notes", notes)
  srv(8080)
