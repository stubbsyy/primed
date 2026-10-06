t note { id i, text s }

f main()
  # struct literals + list ops + each
  v ns = [{note id: 1, text: "buy milk"}, {note id: 2, text: "ship v0.2"}]
  each n ns
    p "#{n.id}: {n.text}"

  # list builtins
  m tags = ["alpha", "beta"]
  push(tags, "gamma")
  p "count={count(tags)} first={get(tags, 0)} joined={join(tags, ",")}"

  # string ops
  v s = "  Hello PRIMED  "
  p "trim=[{trim(s)}] upper={upper(trim(s))} has={has(s, "PRIM")} idx={idx(s, "PRIM")}"
  p "words={count(split("a b c", " "))} rep={rep("ab", 3)} rev={rev("abc")}"

  # filter pattern with each + mutable list
  m kept = []
  each word split("apple banana avocado cherry", " ")
    if starts(word, "a")
      push(kept, word)
  p "a-words: {join(kept, " ")}"

  # numeric lists
  m nums = [5, 3, 9, 1]
  sorti(nums)
  m parts = []
  each n nums
    push(parts, str2(n))
  p "sorted: {join(parts, ",")} sum={sum(nums)}"
