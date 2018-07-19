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

def get_bits(num)
  res = []
  (2**num).times{|i|
    res << i.to_s(2).rjust(num, "0")
  }
  return res
end

def naive(s, x, y)
  rui = s.count("T")
  bits =  get_bits(rui)
  dir = [1, 0, -1, 0]
  bits.each{|bit|
    vec = 0
    t_pos = 0
    pos = [0,0]
    s.each_char{|c|
      if c == "F"
        pos[0] += dir[vec]
        pos[1] += dir[vec^1]
      else
        if bit[t_pos] == "0"
          vec += 1
        else
          vec -= 1
        end
        if vec == 4 || vec == -5
          vec = 0
        end
        t_pos += 1
      end
    }
    p pos

    if pos[0] == x && pos[1] == y
      # puts "Yes"
      #  exit
    end
  }
  puts "No"
end

def check(moves, target)
  len = moves.length
  if len == 0
    return target == 0
  end
  dp = Array.new(len){Hash.new()}
  dp[0][moves[0]] = true
  dp[0][-moves[0]] = true
  (1...len).each do |i|
    dp[i - 1].each do |key, _|
      dp[i][key+moves[i]] = true
      dp[i][key-moves[i]] = true
    end
  end
  return dp[len - 1][target]
end


s = s()
x, y = li()
t_cnt = s.count("T")

moveCount = [Array.new(t_cnt/2+1, 0),Array.new(t_cnt/2+1, 0)]

currentVec = 0
j = 0
s.each_char{|c|
  if c=="F"
    moveCount[currentVec][j] += 1
  else
    if currentVec == 1
      j+=1
    end
    currentVec ^= 1
  end
}

if t_cnt == s.length
  if x==0 && y==0
    puts  "Yes"
  else
    puts "No"
  end
  exit
end

offset = moveCount[0].shift

if check(moveCount[0], x-offset)&&check(moveCount[1], y)
  puts "Yes"
else
  puts "No"
end



