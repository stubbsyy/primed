use rand

f main()
  v n = int(arg(1) ? "1")
  m rolls = []
  m i = 0
  w i < n
    push(rolls, str2(rand::random_range(1..7)))
    i = i + 1
  p "rolls: {join(rolls, ",")}"
