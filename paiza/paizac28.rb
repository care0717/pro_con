input_num = gets.to_i
input_list = []
input_num.times {
    input_list.push(gets.split(" "))
}
res = 0
input_list.each {|data|
    count = 0
    if data[0] == data[1] then
       res += 2
    elsif data[0].length == data[1].length then
        data[0].length.times {|i|
            if data[0][i] != data[1][i] then
                count +=1
            end
        }
        if count == 1 then
            res +=1
        end
    end
}
puts res
