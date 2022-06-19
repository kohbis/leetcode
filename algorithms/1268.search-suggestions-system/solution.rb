# @param {String[]} products
# @param {String} search_word
# @return {String[][]}
def suggested_products(products, search_word)
  products.sort!
  res = []

  (0...search_word.size).each do |i|
    suggests = []
    prefix = search_word[0..i]

    products.each do |product|
      break unless suggests.size < 3

      suggests << product if product.start_with?(prefix)
    end

    res << suggests
  end

  res
end
