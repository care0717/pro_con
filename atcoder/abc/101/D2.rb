def digit_sum(n)
  n.to_s.each_char.map {|c| c.to_i }.reduce(:+)
end

K = gets.to_i

res = [*1..9]
res = res.concat([*1..9].map{|i| i*10+9})
res = res.concat([*1..9].map{|i| i*100+99})
1.upto(15){|n|
  base = 10**n
  10.times{|i|
    10.times{|j|
      10.times{|k|
        res << base*(100*i+10*j+k)+base-1
      }
    }
  }
}
res = res.sort.uniq
result = []
(res.length-100).times{|i|
  flag = true
  2.times{|j|
    if (res[i]/digit_sum(res[i]).to_f > res[i+j+1]/digit_sum(res[i+j+1]).to_f)
      flag = false
    end
  }
  if (flag)
    result << res[i]
  end
}

res = []
(result.length-20).times{|i|
  flag = true
  20.times{|j|
    if (result[i]/digit_sum(result[i]).to_f > result[i+j+1]/digit_sum(result[i+j+1]).to_f)
      flag = false
    end
  }
  if (flag)
    res << result[i]
  end
}
puts res[0..K-1]
