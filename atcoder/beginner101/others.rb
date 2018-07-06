def s(n)
  ret=0
  while n>0
    ret+=n%10
    n/=10
  end
  ret
end
k=[*1..9]
1.upto(9){|i|k<<10*i+9}
2.upto(9){|i|
  d=10**i
  nnn=d-1
  100.times{|j|
    k<<j*d+nnn
  }
}
10.upto(16){|i|
  d=10**i
  nnn=d-1
  1000.times{|j|
    k<<j*d+nnn
  }
}
k.sort!
k.uniq!
min=10**18
ans=[]
k.reverse.each{|a|
  f=a.to_f/s(a)
  if f<=min
    min=f
    ans<<a
  end
}

n=gets.to_i
puts ans.reverse.take(n)
