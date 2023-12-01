part1 = File.foreach("input.txt").map do |line|
    line = line.gsub(/[a-z\n]*/, "")
    "#{line[0]}#{line[-1]}".to_i
end.sum

part2 = File.foreach("input.txt").map do |line|
    line = line.gsub("one", "one1one")
               .gsub("two", "two2two")
               .gsub("three", "three3three")
               .gsub("four", "four4four")
               .gsub("five", "five5five")
               .gsub("six", "six6six")
               .gsub("seven", "seven7seven")
               .gsub("eight", "eight8eight")
               .gsub("nine", "nine9nine")
               .gsub("zero", "zero0zero")
               .gsub(/[a-z\n]*/, "")
    "#{line[0]}#{line[-1]}".to_i
end.sum

p "Part 1: #{part1}","Part 2: #{part2}"
