def root(x, par)
  if(par[x] == x)
    return x
  else
    return par[x] = root(par[x])
  end
end

N, M = gets.split.map(&:to_i)
ps = gets.split.map(&:to_i).map{|i| i-1}
par = [*0..N-1]
res = []
M.times { 
  x = gets.split.map{|s| s.to_i-1}

  if res.flatten.include?(x[0]) || res.flatten.include?(x[1])
    if res.flatten.include?(x[0]) && res.flatten.include?(x[1])
      left, right = getIndexs(res, x)
      res[left] = res[left].concat(res[right])
      res.delete_at(right)
    else
      res.length.times{|i|
        if res[i].include?(x[0]) 
          res[i] << x[1]
        elsif res[i].include?(x[1]) 
          res[i] << x[0]
        end
      }
    end
  else
    res << x
  end
}
others =  [*0..N-1]-res.flatten
sum = (ps.values_at(*others) & others).length
res.each{|list|
  sum += (ps.values_at(*list) & list).length
}
puts sum



