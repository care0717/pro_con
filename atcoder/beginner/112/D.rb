def s()
  gets().chomp
end
def i()
  gets.to_i
end
def li()
  gets.split.map(&:to_i)
end
def ili()
  n = gets.to_i
  as = gets.split.map(&:to_i)
  [n, as]
end
class Array
  def sum
    total = 0
    self.each do |item|
      total += item
    end
    total
  end
end

def solve(pos,m)
  x, y = pos
  query = ""
  if x < 0  
    query += "L"*(x.abs)
  else
    query += "R"*(x.abs)
  end
  if y <  0  
    query += "D"*(y.abs)
  else
    query += "U"*(y.abs)
  end
  if query.size < m
    query += "LR"*((m-query.size)/2)
  end
  query
end


n = i()
originPos = []
pos = []
n.times{
  x, y = li()
  originPos.push([x,y])
  pos.push([x.abs, y.abs])
}
evenOrOdd = pos[0].sum % 2
max = 0
pos.each{|x, y|
  if evenOrOdd != (x+y)%2
    puts -1
    exit 
  end
  max = [max, x+y].max
}
puts max
puts Array.new(max, 1).join(" ")
originPos.each{|p|
  puts solve(p, max)
}
