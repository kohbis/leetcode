# @param {String[]} emails
# @return {Integer}
def num_unique_emails(emails)
  emails.map { |email|
    plus, at = email.index('+'), email.index('@')
    local = (plus.nil? ? email[...at] : email[...plus]).gsub('.', '')
    local << email[email.index('@')..]
  }.uniq.size
end
