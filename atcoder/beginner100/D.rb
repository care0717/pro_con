def solve(data, m)
  return 0 if m == 0
  cands = []
  [-1, 1].each do |a|
    [-1, 1].each do |b|
      [-1, 1].each do |c|
        o = []
        data.each do |x, y, z|
          o << a*x + b*y + c*z
        end
        cands << o.sort[-m..-1].inject(:+)
      end
    end
  end
  cands.max
end
 
 
n, m = gets.split.map(&:to_i)
data = n.times.map{ gets.split.map(&:to_i) }
 
p solve(data, m)
