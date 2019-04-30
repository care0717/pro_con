class Heap_Tree
  attr_reader :value
  def initialize()
    @value = [0]
  end

  def up_heap(v)
    @value.push(v)
    index = @value.length-1
    while (@value[index/2] < @value[index] and index != 1)
      tmp = @value[index/2]
      @value[index/2] =  @value[index]
      @value[index] = tmp
      index = index/2
    end
  end

  def has_child(index)
    2*index <= @value.length-1
  end

  def get_max_child_index(index)
    if 2*index+1 <= @value.length-1
      @value[2*index] >= @value[2*index+1] ? 2*index : 2*index+1
    else
      2*index
    end
  end

  def down_heap
    len = @value.length
    if len == 2
      return @value.pop
    end

    res = @value[1]
    @value[1] = @value.pop
    index = 1

    if !has_child(index)
      return res
    end

    child_index = get_max_child_index(index)
    while (@value[index] <= @value[child_index])
      tmp = @value[child_index]
      @value[child_index] =  @value[index]
      @value[index] = tmp
      index = child_index
      if !has_child(index)
        break
      end
      child_index = get_max_child_index(index)
    end
    res
  end

end
