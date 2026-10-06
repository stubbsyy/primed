f greet(n s) s
  ret "hi {n}"

f main()
  p greet(arg(1) ? "world")
