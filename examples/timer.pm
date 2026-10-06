f main()
  v mins = int(arg(1) ? "25")
  v label = arg(2) ? "focus"
  if mins < 1 or mins > 180
    p "usage: timer <minutes 1-180> [label]"
    quit(1)

  p "=== {label}: {mins} min ==="
  p "start {clock()}"

  m left = mins
  w left > 0
    p "[{label}] {left} min left"
    sleep(60000)
    left = left - 1

  p "=== done: {label} ==="
  p "end {clock()}"
  p "nice work."
