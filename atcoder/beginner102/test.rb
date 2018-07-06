n = gets.to_i
as = gets.split.map(&:to_i)
a = []
b = []
n.times{|i|
  Math.log2(n).floor.times{
    a = a.push(1)
    b = b.unshift(1)
  }
}
#p a