# Eric Olson (c) 2015, MTGCOMBO
# This file is used to initialize the database, and to load the JSON 
# objects into it. 

import sqlite3
import json

# if the database doesn't exist, run this function to create it.
def f_init_database():
  try:
    db = sqlite3.connect("AllThings.sqlite3")
    cursor = db.cursor()

    # table for cards
    cursor.execute('''CREATE TABLE AllCards
        (name TEXT PRIMARY KEY UNIQUE NOT NULL,
        cmc INTEGER,
        text TEXT)''' )

    # table for colors
    cursor.execute("""CREATE TABLE AllColors
        (color TEXT NOT NULL,
        name TEXT NOT NULL, 
        FOREIGN KEY (name) REFERENCES AllCards(name),
        PRIMARY KEY (color, name))"""
        )

    # table for combos
    cursor.execute("""CREATE TABLE AllCombos
        (name TEXT NOT NULL,
        card TEXT NOT NULL,
        FOREIGN KEY (card) REFERENCES AllCards(name),
        PRIMARY KEY (name, card))"""
        )
     
    db.commit()
    db.close()

  except:
    print("Database exists.")


# run this function to create/update card entries based off of AllCards.json
def f_update_AllCards():
  # load our database, and our json file
  db = sqlite3.connect("AllThings.sqlite3")
  with open('AllCards.json') as data_file:
    # for each subfield in Json:
    data = json.load(data_file)

    # put into database
    for i in data:
      name = data[i]["name"]
      cmc = 0
      text = " "
      # cmc
      try:
        cmc = data[i]["cmc"]
      except:
        cmc = 0
      # text
      try:
        text = data[i]["text"]
      except:
        text = " "

      # try to insert it into database
      try:
        cursor = db.cursor()
        t = [name, cmc, text]
        cursor.execute("INSERT INTO AllCards VALUES (?, ? ,?)", (name, cmc, text))

        # insert colors for the card
        try:
          for c in data[i]["colors"]:
            cursor.execute("INSERT INTO AllColors VALUES (?, ?)", (c, name))
        except:
          colors = "None"

      # The card was entered already	
      except:
        print("The card: " + name + " is already entered.")
    
   
  # close and commit the database 
  db.commit()
  db.close()


# add a combo into the database
def f_add_Combo(combo_name, card_name):
  db = sqlite3.connect("AllThings.sqlite3")
  cursor = db.cursor()
 
  # enter the combo + card into the database
  try:
    print(cursor.fetchall())
    cursor.execute("INSERT INTO AllCombos VALUES (?, ?)", 
        (combo_name, card_name))
  except:
    print("The combo " + combo_name + " with card " +
        card_name + " already exists.")
  
  db.commit()
  db.close()




def f_search(card_name, ignore_colors = []):
  db = sqlite3.connect("AllThings.sqlite3")
  cursor = db.cursor()

  # Example statement on how to select combos including two cards
#  cursor.execute("""SELECT * FROM (SELECT name AS n1 FROM AllCombos WHERE card = 'Heartless Hidetsugu') 
#      JOIN
#      (SELECT name AS n2 FROM AllCombos WHERE card =  'Overblaze') WHERE n1 == n2
#      """
#      )

  # Example statement on how to select combos with one card
  cursor.execute("SELECT * FROM(SELECT name as c1 FROM AllCombos WHERE card = '" + card_name + "')")
  # make the list of combos printable
  l = cursor.fetchall()

  # print the cards in the combo
  for i in l:
    c_n = ''.join(i[0])

    cursor.execute("SELECT card FROM AllCombos WHERE name = '" + c_n + "'"
        )
    
    # save as a list
    j = cursor.fetchall()

    # get the colors in each combo
    c = []
    for k in j:
      card_name = ''.join(k)
     
      # get color
      cursor.execute("SELECT color FROM AllColors WHERE name = '" + card_name + "'")
      colors = cursor.fetchall()
      c.append(colors)

    # now, we can select the colors we want, and picking color COLORS = [],
    # we can check if in c, every color matches COLORS. It makes more sense
    # to do it so that we select colors WE DON"T WANT. E.g. that way it can
    # still display monocolored combos, as well as artifact combos.

    # example:
    # 1: pick colors you DON"T WANT
    # for item in j, if it's colors match the colors WE DON"T WANT, remove it
    to_print = 1
    for z in c:
      # if it's not a color we don't want:
      for y in z:
        for x in y:
          for i_c in ignore_colors:
            if x == i_c:
              to_print = 0
 
    if to_print == 1:
      string = []
      for z in j:
        for y in z:
          print(y)
          string.append(y)
#        print(str(j) + str(c))   ## Print the cards + color
      print(" ")
     
  db.commit()
  db.close()


# search for a card, ignore colors:
f_search("Merieke Ri Berit", [])

# Update info
# f_init_database()
# f_update_AllCards()
