awk '
  {
    for(i = 1; i <= NF; i++) {
      arr[NR ,i] = $i
    }
  }
  END {
    for(j = 1; j <= NF; j++) {
      str = arr[1 ,j];
      for(i = 2; i <= NR; i++) {
        str = str" "arr[i ,j];
      }
      print str
    }
  }
' file.txt
