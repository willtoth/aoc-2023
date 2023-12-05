def part1(id, line)
    game = id.split(" ")[1].to_i
    max_r = 0
    max_g = 0
    max_b = 0
    turns = line.split(";").select do |turn|
        turn.split(",").each do |score|
            if score.include?("red")
                max_r = [score.to_i, max_r].max
            end
            if score.include?("green")
                max_g = [score.to_i, max_g].max
            end
            if score.include?("blue")
                max_b = [score.to_i, max_b].max
            end
        end
    end
    return game if max_r <= 12 && max_g <= 13 && max_b <= 14
    return 0
end

part1 = File.foreach("input.txt").map do |line|
    parts = line.split(":")
    part1(parts[0], parts[1])
end

p part1.sum

def part2(id, line)
    game = id.split(" ")[1].to_i
    max_r = 0
    max_g = 0
    max_b = 0
    turns = line.split(";").select do |turn|
        turn.split(",").each do |score|
            if score.include?("red")
                max_r = [score.to_i, max_r].max
            end
            if score.include?("green")
                max_g = [score.to_i, max_g].max
            end
            if score.include?("blue")
                max_b = [score.to_i, max_b].max
            end
        end
    end
    return max_r * max_g * max_b
end

part2 = File.foreach("input.txt").map do |line|
    parts = line.split(":")
    part2(parts[0], parts[1])
end

p part2.sum
