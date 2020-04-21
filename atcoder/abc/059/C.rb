N=gets.to_i
list=gets.split(" ").map(&:to_i)
sum = 0
res = 0

def sign(i)
  if (i>0) then
    1
  elsif i < 0 then
    -1
  else
    0
  end
end
if(list[0] == 0) then
  temp = 0
  flag= true
  (N-1).times{|i|
    temp += list[i+1]
    if(temp != 0 && flag) then
      res +=1

      if((i+1)%2 == 0) then
        list[0] = sign(list[i+1])
      else
        list[0] = -sign(list[i+1])

      end
      flag = false
    end
    if(i == N-2 && flag) then
      res += 1
      list[0]=1
    end
  }
end

N.times{|i|
  before_sum = sum
  sum += list[i]
  if (sum*before_sum> 0) then
    if(sum > 0) then
      res += (sum+1)
      sum = -1
    else
      res += (-sum+1)
      sum = 1
    end
  elsif sum*before_sum==0 then
    if(before_sum < 0 )then
      res += 1
      sum = 1
    elsif(before_sum > 0) then
      sum = -1
      res += 1
    end
  end
}

puts(res)
