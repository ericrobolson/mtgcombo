# Eric Olson (c) 2015,  MTGCOMBO

MTG Combo finder
  This is a program that is build to allow users to select cards, then to 
find overlapping combos/enter in new combos.

The database, AllThings.sqlite3, is finished. No need to update unless needed.

database.py:
  This is used to set up the initial database full of cards. It is also used to enter in combinations, 
as well as search. You can search for a combo using the format: "python3 database.py CARDNAME RED GREEN ... "
in the command line.

combofinder.py:
  This is used to crawl a website to populate our database with combinations. You can enter in an http 
address for it do crawl through through the command line.

trawler.py:
  This uses Selenium to go through a website, and to add new pages for combofinder to crawl through. 
Don't run this unless you need to populate the combos, as it take a very 
long time to use.

Technology used:
Python3: sqlite3, selenium web driver, sys, os
MTGJson.com: AllCards.json
EssentialMagic.com/COMBOS : Used to initially populate the combo list

