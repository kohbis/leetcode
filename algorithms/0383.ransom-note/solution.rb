# @param {String} ransom_note
# @param {String} magazine
# @return {Boolean}
def can_construct(ransom_note, magazine)
  mag_chars = magazine.chars
  can = true
  ransom_note.chars do |c|
    idx = mag_chars.find_index(c)
    if idx
      mag_chars.delete_at(idx)
    else
      break can = false
    end
  end
  can
end

