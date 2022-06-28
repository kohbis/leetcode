# @param {String} path
# @return {String}
def simplify_path(path)
  stack = []

  path.split('/') do |dir|
    case dir
    when '', '.'
      # do nothing
    when '..'
      stack.pop if stack.any?
    else
      stack.push(dir)
    end
  end

  '/' + stack.join('/')
end
