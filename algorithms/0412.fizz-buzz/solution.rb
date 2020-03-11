# @param {Integer} n
# @return {String[]}
def fizz_buzz(n)
  res = []
  (1..n).each do |num|
    if num % 15 == 0
      res.push "FizzBuzz"
    elsif num % 3 == 0
      res.push "Fizz"
    elsif num % 5 == 0
      res.push "Buzz"
    else
      res.push num.to_s
    end
  end
  res
end
