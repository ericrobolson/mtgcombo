# Eric Olson (c) 2015, MTGCOMBO
# This file is used to initialize the database, and to load the JSON 
# objects into it. 

import sqlite3
import json

# if the database doesn't exist, run this function to create it.
def f_init():
  db = sqlite3.connect("AllCards.sqlite3")
  cursor = db.cursor()
  cursor.execute("""CREATE TABLE AllCards
      (name TEXT PRIMARY KEY UNIQUE NOT NULL,
      cmc INTEGER,
      colors TEXT,
      text TEXT)""" )
  db.commit()
  db.close()


# run this function to create/update card entries based off of AllCards.json
def f_update_AllCards():
  # load our database, and our json file
  db = sqlite3.connect("AllCards.sqlite3")
  with open('AllCards.json') as data_file:
    # for each subfield in Json:
    data = json.load(data_file)

    # put into database
    for i in data:
      name = data[i]["name"]
      cmc = 0
      colors = " "
      text = " "

      # cmc
      try:
        cmc = data[i]["cmc"]
      except:
        cmc = 0
      
      # colors
      try:
        colors = data[i]["colors"] 
      except:
        colors = "None"
      
      # text
      try:
        text = data[i]["text"]
      except:
        text = " "

  
    # try to insert it into database
      try:
        cursor = db.cursor()
        t = [name, cmc, colors, text]
        cursor.execute("INSERT INTO AllCards VALUES (?, ?, ? ,?)", (name, cmc, "N/A", text))
      except:
        print("The card: " + name + " is already entered.")
    
   
  # close and commit the database 
  db.commit()
  db.close()



f_update_AllCards()
