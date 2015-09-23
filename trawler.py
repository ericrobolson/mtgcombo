# Eric Olson (c) 2015 MTG Combo
# This file goes through the entire list of combos on essentialmagic.com/combos
# and enters them into combofinder.py
import os
from selenium import webdriver


# run combofind on http
def f_ComboFind(http):
  cmd = 'python3 combofinder.py ' + http
  os.system(cmd)

# Run through the entire webpage
loop = 1
driver = webdriver.Firefox()
driver.get("http://www.essentialmagic.com/COMBOS")
while loop ==1:

  try:
    driver.find_element_by_class_name('LightButton').click()
    f_comboFind(driver.current_url)
  except:
    loop = 0


driver.close()
