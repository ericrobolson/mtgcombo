# Eric Olson (c) 2015, MTGCOMBO
# This file is used to initialize the database, and to load the JSON 
# objects into it. 

import sqlite3
import json
import sys
import os


COLORS = [('Blue'), ('Red'), ('Black'), ('White'), ('Green'), ('Colorless')]


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
    # colors can be Red, Blue, Green, White, Black, Colorless
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


# update colorless cards in mtg
def f_update_colorless():
  db = sqlite3.connect("AllThings.sqlite3")
  cursor = db.cursor()

  # select all cards without a color 
  cursor.execute("SELECT AllCards.name FROM AllCards EXCEPT SELECT AllColors.name FROM AllColors")
    
  # add colorless to the cards
  c = 'Colorless'
  all_names = cursor.fetchall()
  print(all_names)
  for n in all_names:
    name = n[0]
    cursor.execute("INSERT INTO AllColors VALUES (?, ?)", (c, name))

  db.commit()
  db.close()


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
          c = "Colorless"
          cursor.execute("INSERT INTO AllColors VALUES (?, ?)", (c, name))
          colors = "None"

      # The card was entered already	
      except:
        print("The card: " + name + " is already entered.")
  
  # close and commit the database 
  db.commit()
  db.close()

  # update colorless cards, things that might not have had a color assigned
  f_update_colorless()


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


# create a view, and find the card combos with the card name, and exclude the provided colors
def f_combofind(card_name, ignore_colors = []):
  db = sqlite3.connect("AllThings.sqlite3")
  cursor = db.cursor()

  # create a view with combos
  cursor.execute("""CREATE TEMP VIEW v_combo AS 
      SELECT name from AllCombos WHERE card = '""" + card_name + "'")

  # create a view with all the cards to each combo
  cursor.execute("""CREATE TEMP VIEW v_allcards AS
    SELECT name as n1, card as card FROM v_combo
    JOIN
    (SELECT name as n2, card as card FROM AllCombos) WHERE n1 == n2""")

  # create new view without combos that contain certain color(s)
  i_c = "', '".join(ignore_colors)

  # Created a view with all cards/colors
  # now need to work on excluding combos of a certain color; can exclude
  # cards of a certain color however.
  cursor.execute("""CREATE TEMP VIEW v_f AS
    SELECT n1, color, card
      FROM v_allcards
      JOIN   
      AllColors
      ON v_allcards.card = AllColors.name

    EXCEPT
      SELECT v_allcards.n1 AS n2, color AS c, card as card2
      FROM AllColors, v_allcards
      WHERE c NOT IN ('""" + i_c + "')")

  # create the final view; excluding cards and combos that
  # are not of the chosen colors
  cursor.execute("""CREATE TEMP VIEW v_final AS
    SELECT n1, card FROM v_allcards
    EXCEPT
    SELECT n1 AS n2, card FROM v_f WHERE n1 == n2
  """)

  # SELECT the final table
  cursor.execute("""SELECT DISTINCT GROUP_CONCAT(card, '; ') FROM v_final GROUP BY n1""") 

  # print the final table of combos
  results = cursor.fetchall()
  for i in results:
    # split combo into cards, and print
    combo = i[0].split("; ")
    for j in combo:
      print(j)
    print()

  db.close() 


#############################################################################
############### Command Line Interface to search for combos #################
#############################################################################

if len(sys.argv) > 1:
  # format: python3 database.py CARDNAME COLOR1 COLOR2 ETC
  card = sys.argv[1]
  colors = []  
  index = 0

  # check to see if it's a real card
  db = sqlite3.connect("AllThings.sqlite3")
  cursor = db.cursor()
  cursor.execute("SELECT * FROM AllCards WHERE name = '" + card + "'")
  results = cursor.fetchall()
  if not results:
    print("Enter in format: 'CARDNAME' Red Blue White Black Green Colorless")
    print("Note: Colors are optional and may be entered in any order")
  db.close()
  
  # populate the colors
  for i in sys.argv:
    if index >1:
      if i in COLORS:
        colors.append(i)
    index +=1

  # search for the combo
  f_combofind(card, colors)
