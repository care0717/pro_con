N=gets.to_i
x = gets.split(" ").map(&:to_i)
c = gets.split(" ").map(&:to_i)
v = gets.split(" ").map(&:to_i)
saihu = 0

index = c.index { |e| e > x.sum}
c.delete_at(index)
v.delete_at(index)

N.times { |i|
  saihu += x[i]
  index = c.index {|e| e < (saihu + x[i+1])}
  if index.empty?
    max_v = v.max
  else
    max_v = v[index].max
  end
  v.delete(max_v)
}
