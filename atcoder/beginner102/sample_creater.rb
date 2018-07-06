n = gets.to_i
puts n
res = []
random = Random.new(88)
n.times{
  res << random.rand(100000000)
}
puts res.join(" ")