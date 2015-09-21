# Eric Olson (c) 2015, MTGCOMBO
# This file is used to add combos from www.essentialmagic.com/COMBOS
# to populate the database.
# Use lynx to get text dumps, and use Selenium to get the webpages
# Use Beautiful Soup???
# http://www.gregreda.com/2013/03/03/web-scraping-101-with-python/
# http://www.crummy.com/software/BeautifulSoup/bs4/doc/
import os


#1: download files with lynx into text file; open file
cmd = 'lynx -dump www.essentialmagic.com/COMBOS > output.txt'
os.system(cmd)
f = open('output.txt', 'r+')


#2: go throuh each line in text file:

## XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
## there is a potential error; where we have a combo with many cards on 
## different lines. Need to fix this somehow.

for line in f:
  line2 = line
  newlist = []  

  # this lets us know when to skip characters in our list
  skipping = 0

  # remove '[ ... ]' in each line
  for c in line2:
    # check to see if it should skip characters
    if c == '[':
      skipping = 1
      continue
    
    # check if it should stop skipping characters
    if skipping ==1:
      if c != ']':
        continue
      else:
        skipping = 0
        continue

    # if it's not skipping, copy to a new list    
    if skipping == 0:
      newlist += c

  # make our list a new string
  s = "".join(newlist)
  print(s)
      

#3:   check if it fits the format [card] buy + [card] buy + [card] buy + ...
#     if so, then create a new combo and enter it, otherwise, go to next line

#4: go the the next page and repeat the process

#close and delete the file
f.close()

cmd = 'rm output.txt'
os.system(cmd)
