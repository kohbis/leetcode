SEED     = [*('a'..'z'), *('A'..'Z'), *('0'..'9')]
KEY_LEN  = 6
BASE_URL = "http://tinyurl.com/"

@urls = {}

def get_random
  key = ""
  KEY_LEN.times { key += SEED.sample }

  key
end

# Encodes a URL to a shortened URL.
#
# @param {string} longUrl
# @return {string}
def encode(longUrl)
  key = ""
  while @urls[key] || key.length == 0
    key = get_random()
  end
  @urls[key] = longUrl

  BASE_URL + key
end

# Decodes a shortened URL to its original URL.
#
# @param {string} shortUrl
# @return {string}
def decode(shortUrl)
  key = shortUrl.gsub(BASE_URL, "")

  @urls[key]
end


# Your functions will be called as such:
# decode(encode(url))

