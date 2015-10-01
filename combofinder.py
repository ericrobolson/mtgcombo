# Eric Olson (c) 2015, MTGCOMBO
# This file is used to add combos from www.essentialmagic.com/COMBOS
# to populate the database.
# Use lynx to get text dumps, and use Selenium to get the webpages

import os
import database
import sys


# You can pass in the website to scan as an argument.
cmd = 'lynx -dump -nomargins -dont_wrap_pre ' + sys.argv[1] + ' > output.txt'
os.system(cmd)
f = open('output.txt', 'r+')


# go throuh each line in text file and add combos
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
  
  # split our string s into a list of words
  words = s.split()
  w = []
  # remove 'Buy' from words
  for i in words:
    if i != 'Buy':
      w.append(i)
  
  # check to see if w fits the format 'card' + 'card'; use recursion
  # isacombo lets us know if the string fits the combo format or not
  isacombo = 0
  index = 0
  plus_index = []
  preindex = 0
  length_w = len(w)

  # check if there is a '+', and if so, it's a combo. Grab the indexes of
  # the '+'s so we can create the combo
  for i in w:
    if i == '+' and index < length_w:
      isacombo = 1    
      plus_index.append(index)

    index +=1

  # create the combo
  if isacombo == 1:
    index = 0
    combo_name = "".join(w)
    # add the cards to the combo	
    for i in plus_index:
      # add the cards between the +'s
      if index < (len(plus_index) -1):
        # enter it into the database
        cardname = " ".join(w[preindex : (i)])
        try:
          database.f_add_Combo(combo_name, cardname)
        except:
          print("Error with card name: " + cardname)
        preindex = i +1

      # if it's the last +, add the cards before and after it
      if index == (len(plus_index) - 1):
        # enter card before + into the database
        cardname = " ".join(w[preindex : (i)])
        try:
          database.f_add_Combo(combo_name, cardname)
        except:
          print("Error with card name: " + cardname)

        # enter card after + into the database
        cardname =" ".join(w[(plus_index[index] +1) :])
        try:
          database.f_add_Combo(combo_name, cardname)
        except:
          print("Error with card name: " + cardname)

      index +=1 


#close and delete the file
f.close()
cmd = 'rm output.txt'
os.system(cmd)
