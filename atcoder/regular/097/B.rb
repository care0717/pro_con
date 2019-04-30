def getStr(s, index, length)
  if S.length > index+length 
    s[index..(index+length)]
  end
end

def solve(s, indexs, length, res)
  if indexs.min+length == s.length || length > 5
    return res
  end
  indexs.each{|i|
    res.push(getStr(s, i, length))
  }
  solve(s, indexs, length+1, res)
end

S = gets.chop
K = gets.to_i
min = "a"
result = Array.new()
while result.length < K do
  res = Array.new()
  min_indexs = Array.new()
  offset = 0
  while S.index(min, offset) != nil do
    min_indexs.push(S.index(min, offset))
    offset = min_indexs[-1]+1
  end
  if min_indexs.length > 0 
    res.push(min)
    result.concat(solve(S, min_indexs, 1, res).compact.uniq)
  end
  min = min.succ
end

puts  result.sort[K-1]
