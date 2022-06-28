# @param {String[]} cpdomains
# @return {String[]}
def subdomain_visits(cpdomains)
  hash = Hash.new

  cpdomains.each do |cpdomain|
    visits, domains = cpdomain.split(' ')
    domains = domains.split('.')

    for i in 0...domains.size
      level = domains[i...domains.size].join('.')
      total_visits = 0
      if hash.has_key?(level)
        total_visits = hash[level]
      end
      hash[level] = total_visits + visits.to_i
    end
  end

  hash.map { |k, v| [v, k].join(' ') }
end
