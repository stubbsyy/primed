use uuid[v4]

f main()
  v id = uuid::Uuid::new_v4().to_string()
  p "id: {id}"
  p "len: {len(id)}"
