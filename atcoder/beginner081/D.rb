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

n, as = ili()

max = as.max
max_index = as.index{|a| a == max}
min = as.min
min_index = as.index{|a| a == min}


if min >= 0
  puts n-1
  (n-1).times{|i|
    puts (i+1).to_s + " "+ (i+2).to_s
  }
elsif max <= 0
  puts n-1
  (n-1).times{|i|
    puts (n-i).to_s + " " +(n-i-1).to_s
  }
else
  res = []
  if max.abs >= min.abs
    n.times{|i|
      if as[i] < 0
        res << [max_index+1, i+1]
      end
    }
    puts n-1+res.length
    res.each{|r|
      puts r.join(" ")
    }
    (n-1).times{|i|
      puts (i+1).to_s + " "+ (i+2).to_s
    }
  else
    n.times{|i|
      if as[i] > 0
        res << [min_index+1, i+1]
      end
    }
    puts n-1+res.length
    res.each{|r|
      puts r.join(" ")
    }
    (n-1).times{|i|
      puts (n-i).to_s + " "+ (n-i-1).to_s
    }
  end
end
