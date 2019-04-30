class Wavelet
  def initialize(list)
    height = Math.log2(list.max).floor + 1
    b = list.size
    res = Array.new(height)
    zero_count = Array.new(height, 0)
    list.each{|l|
      temp = l.to_s(2).rjust(height, "0")
      height.times{|i|
        if res[i] == nil
          res[i] = ""
        end
        zero_count[i] += if temp[i]=="0" then 1 else 0 end
        res[i] << temp[i]
      }
    }

    start_point = {}
    sorted_list = list.clone
    sort_index = [*0..b-1]
    1.upto(height){|i|
      temp_sorted_list = sorted_list.clone
      temp_sort_index = sort_index.clone
      if i != height
        temp0 = ""
        temp1 = ""
        res_c = res[i].clone
        b.times{|j|
          if res[i-1][j] == "0"
            temp0 << res_c[temp_sort_index[j]]
            sorted_list[temp0.length-1] = temp_sorted_list[j]
            sort_index[temp0.length-1] = j
          else
            temp1 << res_c[temp_sort_index[j]]
            sorted_list[zero_count[i-1] + temp1.length-1] = temp_sorted_list[j]
            sort_index[zero_count[i-1] + temp1.length - 1] = j
          end
        }
        temp0 << temp1
        res[i] = temp0
      else
        temp0 = 0
        temp1 = 0
        b.times{|j|
          if res[i-1][j] == "0"
            temp0 += 1
            sorted_list[temp0-1] = temp_sorted_list[j]
          else
            temp1 += 1
            sorted_list[zero_count[i-1] + temp1-1] = temp_sorted_list[j]
          end
        }
      end
    }
    before = -1000000000000000000
    b.times{|i|
      if sorted_list[i] != before
        start_point[sorted_list[i]] = i
        before = sorted_list[i]
      end
    }

    big_rank_size = Math.log2(b).floor**2
    small_rank_size = Math.log2(b).floor/2
    big_rank = Array.new(height)
    small_rank = Array.new(height)
    height.times{|i|
      big_rank[i] = [0]
      small_rank[i] = Array.new((b/big_rank_size).ceil)
      0.upto(b/big_rank_size){|j|
        base = res[i][j*big_rank_size..(j+1)*big_rank_size-1]
        small_rank[i][j] = [0]
        0.upto(base.length/small_rank_size){|k|
          small_list = base[k*small_rank_size..(k+1)*small_rank_size-1]
          if small_list != ""
            small_rank[i][j] << small_rank[i][j][k]+small_list.count("1")
          end
        }
        big_rank[i] << big_rank[i][j]+base.count("1")
      }
    }

    table = []
    (2**small_rank_size).times{|i|
      table << i.to_s(2).count("1")
    }

    @start_point = start_point
    @length = b
    @big_rank_size = big_rank_size
    @small_rank_size = small_rank_size
    @height = height
    @bit_list = res
    @big_rank = big_rank
    @small_rank = small_rank
    @table = table
    @zero_count = zero_count
    puts @bit_list
    puts ""
  end

  def rank(index, num)
    target_bits = num.to_s(2).rjust(@height, "0")
    next_index = index
    @height.times{|i|
      next_index = count(target_bits[i], next_index, i)
      next_index += if target_bits[i] == "1" then @zero_count[i]  else 0 end
    }
    return next_index - @start_point[num]
  end

  def count(bit, index, depth)
    big_index  = index/@big_rank_size
    small_index = (index-big_index*@big_rank_size)/@small_rank_size
    bits = @bit_list[depth][(big_index*@big_rank_size+small_index*@small_rank_size)..(big_index*@big_rank_size+(small_index+1)*@small_rank_size)-1].ljust(@small_rank_size, "0")
    mask_size = index-(big_index*@big_rank_size+small_index*@small_rank_size)
    if mask_size != 0
      table_index = (bits.to_i(2) >> mask_size << mask_size)
    else
      table_index = 0
    end
    bit1_num =  @big_rank[depth][big_index] + @small_rank[depth][big_index][small_index] + @table[table_index]
    if bit == "1"
      bit1_num
    else
      index - bit1_num
    end
  end



end

def in_group_of(string, group_size)
  size = string.length
  res_size = (size/group_size.to_f).ceil
  res = Array.new(res_size)
  res[0] = ""
  i = 0
  string.each_char{|c|
    if (res[i].length < group_size)
      res[i] << c
    else
      i += 1
      res[i] = c
    end
  }
  return res
end

def wavelet(list)
  height = Math.log2(list.max).floor + 1
  b = list.size
  res = Array.new(height)
  res_split = Array.new(height)
  big_rank_size = Math.log2(b).floor**2
  small_rank_size = Math.log2(b).floor/2
  big_rank = Array.new(height)
  small_rank = Array.new(height)

  list.each{|l|
    temp = l.to_s(2).rjust(height, "0")
    height.times{|i|
      if res[i] == nil
        res[i] = ""
      end
      res[i] << temp[i]
    }
  }

  height.times{|i|
    big_rank[i] = [0]
    small_rank[i] = Array.new((b/big_rank_size.to_f).ceil)
    0.upto(b/big_rank_size){|j|
      base = res[i][j*big_rank_size..(j+1)*big_rank_size-1]
      small_rank[i][j] = [0]
      0.upto(base.length/small_rank_size){|k|
        small_list = base[k*small_rank_size..(k+1)*small_rank_size-1]
        if small_list != ""
          small_rank[i][j] << small_rank[i][j][k]+small_list.count("1")
        end
      }
      big_rank[i] << big_rank[i][j]+base.count("1")
    }
  }

  table = []
  (2**small_rank_size).times{|i|
    table << i.to_s(2).count("1")
  }
  p [res, big_rank, small_rank]
  p table
end


list = [5, 4, 5, 5, 2, 1, 5, 6, 1, 3, 5, 0]
w = Wavelet.new(list)
puts w.rank(7,1)


