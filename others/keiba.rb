class Array
  def inverse_sum
    temp = 0.0
    self.each { |i|
      temp += 1.0 / i
    }
    temp.round(4)
  end
end

tan_odds = [
  2.8, 13.4, 123.8, 3.4, 3.7, 13.3, 52.6, 74.4, 6.6
  #14.7, 72.6, 1.4, 2.3, 151.8, 18.0, 66.8
]
uma_odds = [
  [ 23.1, 154.8, 7.7, 11.5, 27, 68.7, 117.9, 14.8 ],
  [ 40.2, 828, 58.7, 62.6, 160.9, 303.2, 621.4, 90.1 ],
  [ 353, 1116, 58.7, 62.6, 160.9, 303.2, 621.4, 90.1 ]
  #[347.2, 37.4, 54.0, 662.7, 169.6, 607.5],
  #[1,1,1,1,1,1],
  #[23.0, 86.8, 5.4, 50.7, 13, 177.8]
]
tan_odds.length.times {|i|
  puts("馬#{i+1}の単勝オッズは、#{tan_odds[i]}")
}
puts("オッズの逆数和は#{tan_odds.inverse_sum}")
puts("単勝を馬単に変えた場合")
change_to_tan = []
flag = true
uma_odds.length.times { |i|
  changed_odds = 1.0 / (uma_odds[i].inverse_sum)
  puts("  馬#{i+1}のオッズは,#{changed_odds.round(3)}")
  if changed_odds > tan_odds[i]
    flag = false
    change_to_tan.push(i+1)
    tan_odds[i] = changed_odds
  end
}
if flag
  change_to_tan.push("存在しない")
end
puts("以上より、単勝から馬単に変えた方がいい馬は#{change_to_tan}")
odds_inverse = tan_odds.inverse_sum
if odds_inverse < 1
  puts("変えた場合、オッズの逆数和は#{odds_inverse}<1となり、絶対得する")
else
  puts("変えても、オッズの逆数和は#{odds_inverse}>1となり、損をする")
end
