# coding: utf-8

require 'benchmark'

8.times{|i|
  n = 10**i
  res = Array.new(n, 0)
  size=res.size

  result = Benchmark.realtime do
    a = res[size-10..size-1]
    puts a
  end
  puts "処理概要 #{result}s"
}