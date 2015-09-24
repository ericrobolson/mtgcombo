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




def f_search():
  db = sqlite3.connect("AllThings.sqlite3")
  cursor = db.cursor()


  # Example statement on how to select combos including two cards
  cursor.execute("""SELECT * FROM (SELECT name AS n1 FROM AllCombos WHERE card = 'Heartless Hidetsugu') 
      JOIN
      (SELECT name AS n2 FROM AllCombos WHERE card =  'Overblaze') WHERE n1 == n2
      """
      )

  # Example statement on how to select combos with one card
  def f_one(card_name):
    cursor.execute("SELECT * FROM AllCombos WHERE card = " + card_name)


  f_one("Tainted Strike")
  print(cursor.fetchall())
     
  db.commit()
  db.close()


#f_search()

# Update info
# f_init_database()
# f_update_AllCards()
