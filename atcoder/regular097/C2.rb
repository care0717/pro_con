def root(x, par)
  if(par[x] == x)
    return x
  else
    return par[x] = root(par[x], par)
  end
end

def unite(x, y, par)
  x = root(x, par)
  y = root(y, par)
  if (x==y) 
    return par;
  end
  par[x] = y
  return par
end

def same(x, y, par)
  root(x, par) == root(y, par)
end

N, M = gets.split.map(&:to_i)
ps = gets.split.map(&:to_i).map{|i| i-1}
par = [*0..N-1]
M.times { 
  x = gets.split.map{|s| s.to_i-1}
  x0_root = root(x[0], par)
  x1_root = root(x[1], par)
  if x0_root == x[0] 
    par[x[0]] = x1_root
  elsif x1_root == x[1]
    par[x[1]] = x0_root
  else
    par = unite(x[0], x[1], par)
  end
}
count = 0
N.times{|i|
  if same(i, ps[i], par) 
    count += 1
  end
}
puts count

