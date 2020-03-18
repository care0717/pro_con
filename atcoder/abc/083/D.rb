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

def other(s)
  if s=="0"
    "1"
  else
    "0"
  end
end

S = s()
len = S.length
before = S[0]

switch = 0
if S[0] == "0"
  switch = 1
end
1.upto(len-1){|i|
  if before == "1" && S[i] != before
    switch += 1
  end
  before = S[i]
}

score = Array.new(switch)
switch.times{|i|
  score[i] =  {"start": 0, "len": 0, "left": 0, "right": 0}
}

i=0
zero_start = 0
while S[i] != "0"
  zero_start += 1
  i+=1
end
score[0][:left] = zero_start
score[0][:start] = zero_start
score[0][:len] = 1
s = S[zero_start..-1]
before = s[0]
j = 0
1.upto(s.length-1){|i|
  if before != s[i] && s[i] == "0"
    j += 1
    score[j][:left] = score[j-1][:right]
    score[j][:start] = i + zero_start
  end
  if s[i] == "0"
    score[j][:len] +=  1
  else
    score[j][:right] +=  1
  end
  before = s[i]
}

result = 0
switch.times{|i|
  sum = (score[i][:left] < score[i][:right] ? score[i][:left] : score[i][:right]) + score[i][:len]
  result = sum > result ? sum : result
}
puts result
