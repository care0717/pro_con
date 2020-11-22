class Tree
  attr_reader :value, :children
  def initialize(v, c=[])
    @value = v
    @children = c
  end

  def search(v)
    return true if @value == v
    @children.each{ |c|
      return true if c.search(v)
    }
    false
  end
end

t1 = Tree.new(1)
t2 = Tree.new(1, [
  Tree.new(2),
  Tree.new(3),
  Tree.new(4, [
    Tree.new(5)
    ])
  ])
puts(t2.search(6))

class Binary_tree
  attr_reader :value, :left, :right
  def initialize(v, l=nil, r=nil)
    @value = v
    @left = l
    @right = r
  end

  def search(v)
    return true if @value == v
    return true if @left != nil and @left.search(v)
    return true if @right != nil and @right.search(v)
    false
  end
end
