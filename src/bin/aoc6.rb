part2 = false
input = File.foreach("input.txt").map do |line|
    line = line.gsub(/Time: */, "").gsub(/Distance: */, "").gsub(/  */, part2 ? "" : " ").split(" ").map {|num| num.to_i}
end

times = input[0]
distance = input[1]

total = 1
times.each_with_index do |time, i|
    sum = 0
    (0..time).map do |x|
        dist = (time - x) * x
        sum += 1 if dist > distance[i]
    end
    total *= sum
end

p total
