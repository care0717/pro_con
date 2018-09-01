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

def solve(ps, cospa, prob_num, urikire)
  temp = prob_num <= 100 ? prob_num : 100
  index = 0
  cospa[temp].each{|i|
      if !urikire[i]
        index = i
        break
      end
  }
  point, num, bonus = ps[index]
  if num > temp
    return point*temp
  elsif num == temp
    return point*num+bonus
  else
    urikire[index] = true
    return point*num+bonus + solve(ps, cospa, prob_num-num, urikire)
  end
end

d, g = li()
ps = []
d.times{|i|
  ps << [(i+1)*100].concat(li())
}
cospa = Array.new(101)
1.upto(100){|i|
  aves = ps.map.with_index{|val, j|
    point, num, bonus = val
    num <= i ?  [point + bonus/num,j] : [point,j]}.sort_by {|ave, index| -ave}
  cospa[i] = []
  aves.each{|ave, index|
    cospa[i] << index
  }
}
all_prob = ps.reduce(0) {|sum, p| sum + p[1]}
p cospa
1.upto(all_prob){|i|
  urikire = {}
  0.upto(9){|j|
    urikire[j] = false
  }
  res = solve(ps, cospa, i, urikire)
  if res >= g
    puts i
    puts res
    break
  end

}
