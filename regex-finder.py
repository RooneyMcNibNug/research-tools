## quick python to search for all files within a directory that contain regular expressions.

import glob
import re

for msg in glob.glob(`~/*.txt'):
  filer = open((msg),'r')
  data = filer.read()
  message = re.findall(r'<message>(.*?)>/message>', data,re.DOTALL)
  print "%s contains %s" % (str(msg),message)
  filer.close()
