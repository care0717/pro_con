N, K = gets.chomp.split.map(&:to_i)

K2 = 2 * K
imos = Array.new(K2){Array.new(K, 0)}

N.times do |i|
  x, y, c = gets.chomp.split
  x = x.to_i % K2
  y = y.to_i % K2

  # change condition to
  # 0 <= x < KX, 0 <= y < KY, 'B'
  if c == 'B'
    if y >= K
      x = (x + K) % K2
      y -= K
    end
  else
    if y >= K
      y -= K
    else
      x = (x + K) % K2
    end
  end

  # imos count for each rectangle
  #
  # ..###.
  # ..*##.
  # ##...#    # main, left-bottom, right-bottom
  #
  # #...##
  # #...*#
  # .###..    # main, left-bottom, further-left

  # main
  imos[x][y] += 1
  imos[x + K][y] -= 1 if x < K

  # left-bottom
  if y > 0 && x > 0
    lbx = [0, x - K].max
    imos[lbx][0] += 1
    imos[x][y] += 1
    imos[x][0] -= 1
    imos[lbx][y] -= 1
  end

  # right-bottom
  if y > 0 && x < K
    imos[x + K][0] += 1
    imos[x + K][y] -= 1
  end

  # further-left
  if x > K + 1
    imos[0][y] += 1
    imos[x - K][y] -= 1
  end
end

(1...K2).each do |i|
  (0...K).each do |j|
    imos[i][j] += imos[i - 1][j]
  end
end

(0...K2).each do |i|
  (1...K).each do |j|
    imos[i][j] += imos[i][j - 1]
  end
end
p imos

max = 0
(0...K2).each do |i|
  (0...K).each do |j|
    max = imos[i][j] if max < imos[i][j]
  end
end

print max