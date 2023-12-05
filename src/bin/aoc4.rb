lines = 0
part1 = File.foreach("input.txt").map do |line|
    line = line.gsub(/Card *\d*:/, "").split("|")
    winning_numbers = line[0].split(" ").map {|num| num.to_i}
    my_numbers = line[1].split(" ").map {|num| num.to_i}

    winning = my_numbers.select {|num| winning_numbers.include? num}

    if winning.length == 0
        0
    else
        2 ** (winning.length - 1)
    end
    lines = lines + 1
end.sum

card_instances = Array.new(lines, 1)
part2 = File.foreach("input.txt").map do |line|
    card_number = $. - 1
    card_count = card_instances[card_number]

    line = line.gsub(/Card *\d*:/, "").split("|")
    winning_numbers = line[0].split(" ").map {|num| num.to_i}
    my_numbers = line[1].split(" ").map {|num| num.to_i}

    count = (my_numbers.select {|num| winning_numbers.include? num}.length)
    count = 0 unless count
    count = count

    card_count.times do |x|
        count.times do |idx|
            card_instances[card_number + idx + 1] += 1 unless (card_number + idx + 1) >= card_instances.length
        end
    end

    card_instances.sum
end.last

p "Part 1: #{part1}","Part 2: #{part2}"
