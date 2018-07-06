N, C = gets.split.map(&:to_i)
ds = [] # [[x_11,...],...,[x_m1,...]]
C.times do
  line = gets.split.map(&:to_i)
  ds << line
end
cs = []
N.times do
  line = gets.split.map(&:to_i)
  cs << line
end
mod0 = Array.new(0)
mod1 = Array.new(0)
mod2 = Array.new(0)
for i in 0..(N-1) do
  for j in 0..(N-1) do
    modNum = ((i+1)+(j+1)) % 3
    if modNum == 0 then
      mod0.push(cs[i][j])
    elsif modNum == 1 then
      mod1.push(cs[i][j])
    else
      mod2.push(cs[i][j])
    end
  end
end
mod = [mod0, mod1, mod2]
costs = Array.new(0)
for c in 1..C do
  costs.push(mod.map{|m| m.reduce(0){ | sum, n | sum + ds[n-1][c-1] } })
end
res = 1000000000000000000000000000000000000000000000000000000
for c1 in 1..C do
  for c2 in 1..C do
    for c3 in 1..C do
      cost1 = costs[c1-1][0]
      cost2 = costs[c2-1][1]
      cost3 = costs[c3-1][2]
      if ( c1 != c2 && c2 != c3 && c3 != c1) then
        res = res > (cost1+cost2+cost3) ? (cost1+cost2+cost3) : res
      end
    end
  end
end
p res

