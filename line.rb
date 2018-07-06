class Schedule
  attr_accessor :time, :distination
  def initialize(time, distination)
    @time = time
    @distination = distination
  end
end

def toMinute(hour, minute)
  hour*60+minute
end


ScheduleA1 = [*0..210].map{|i|  Schedule.new(toMinute(5,55)+i*5, i%2==0 ? "A7" : "A13")}.select{|i| i.time<toMinute(23,0)}
ScheduleA13 = [*0..110].map{|i| Schedule.new(toMinute(5,52)+i*10, "A1")}.select{|i| i.time<toMinute(23,0)}
ScheduleA13[-1].distination = "A7"
ScheduleA7 = [*0..110].map{|i| Schedule.new(toMinute(6,6)+i*10, "A1")}.select{|i| i.time<toMinute(23,0)}
ScheduleA7.insert(1, Schedule.new(toMinute(6,10), "A13"))


def getStationSchedule(currentStation)
  if currentStation.include?("A")
    currentNum =  currentStation[1..-1].to_i
    schedule = Array.new()
    if currentNum < 7 
      schedule += ScheduleA1.map{|i| 
        i.time+= getDistance(currentNum,1)
        i
      }
      schedule += ScheduleA7.select{|i| i.distination.include?("A1")}.map{|i| 
        i.time+=getDistance(currentNum,7)
        i
      }
      schedule += ScheduleA13.select{|i| i.distination.include?("A1")}.map{|i| 
        i.time+=getDistance(currentNum,13)
        i
      }
    end
  end
  schedule.sort_by {|i| i.time }
end

def getDistance(currentNum, targetNum)
  routeA = [3, 5, 2, 3, 4, 3, 4, 2, 2, 3, 6, 2]
  routeA[[currentNum-1, targetNum-1].min .. ([currentNum-1, targetNum-1].max-1)].sum
end

def getCanDists(currentNum, targetNum)
  if currentNum < targetNum 
    if targetNum > 7
      ["A13"]
    else
      ["A7", "A13"]
    end
  else
    if targetNum < 7
      ["A1"]
    else
      ["A1", "A7"]
    end
  end
end


def main()
  # このコードは引数と標準出力を用いたサンプルコードです。
  # このコードは好きなように編集・削除してもらって構いません。
  # ---
  # This is a sample code to use arguments and outputs.
  # Edit and remove this code as you like.
#  argv.each_index do |i|
#    v = argv[i]
 #   puts "argv[#{i}]: #{v}"
  #end
  cure = "A2"
  cureNum = cure[1..-1].to_i
  dist = "A10"
  distNum = dist[1..-1].to_i
  sch =  getStationSchedule(cure)
  dists = getCanDists(cureNum, distNum)
  time = toMinute(7, 0)
  res = sch.select{|i| i.time <= time && dists.include?(i.distination)}[-1]
  if res == nil && sch.select{|i| i.time <= time}[-1] == nil
  
    
end


main()
