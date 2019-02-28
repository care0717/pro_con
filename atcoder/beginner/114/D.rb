PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]
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
def eratosthenes(size)
  prime_list = []
  is_prime = Array.new(size, true)
  is_prime[0] = false
  is_prime[1] = false

  2.upto(size-1){|i|
    if is_prime[i]
      prime_list << i
      (2).upto((size.to_f/i).ceil-1){|j|
        is_prime[j*i] = false
      }
    end
  }
  [prime_list, is_prime]
end

def calc_divisor(n)
  hash = {}
  PRIMES.each{ |p|
    count = 0
    if n%p == 0
      m = n
      while(m%p == 0)
        count += 1
        m /= p
      end
    end
    hash[p] = count
  }
  return hash
end

def factorial(n, r)
  res = 1
  ((n-r+1)..n).each{|i|
    res *= i
  }
  res
end
def combination(n,r)
  factorial(n,r)/factorial(r,r)
end

n = i()
hashs = []
(1..n).each{|i|
  hashs.push(calc_divisor(i))
}
divisors = Hash[calc_divisor(1).map{|k,v|  [k, v+1]}]
hashs.each{|hash|
  PRIMES.each{|p|
    divisors[p] += hash[p] 
  }
}

divisors.delete_if {|key, val| val < 3 }
sevenfive_num = divisors.count {|k,v| v >= 75 }
twnfive_num = divisors.count {|k,v| v >= 25 } 
fiveteen_num = divisors.count {|k,v| v >= 15 } 
five_num = divisors.count {|k,v| v >= 5 }
three_num = divisors.count {|k,v| v >= 3 } 
sevenfive_num += fiveteen_num*(five_num-1)
sevenfive_num += twnfive_num*(three_num-1)
sevenfive_num += (three_num-2)*combination(five_num, 2)
p sevenfive_num
