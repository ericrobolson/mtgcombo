# Eric Olson (c) 2015 MTG Combo
# This file goes through the entire list of combos on essentialmagic.com/combos
# and enters them into combofinder.py

import os
import time
from selenium import webdriver
from selenium.webdriver.common.keys import Keys


# run combofind on http
def f_ComboFind(http):
  cmd = 'python3 combofinder.py ' + http
  os.system(cmd)


# get text 
def f_GetText(http):
  textise_driver = webdriver.Firefox()
  textise_driver.get("http://www.textise.net")
  textise_driver.implicitly_wait(4)
  elem = textise_driver.find_element_by_name("in")
  elem.send_keys(http)
  elem.send_keys(Keys.RETURN)
  http = textise_driver.current_url
  textise_driver.close()
  return http


# Run through the entire webpage
driver = webdriver.Firefox()
driver.get("http://www.essentialmagic.com/COMBOS")


# Run through the entire website to get all combos. NOTE: TAKES A LONG TIME
loop = 1
while loop !=0:
  try:
    # input textise version into comboFind
    url = f_GetText(driver.current_url)
    f_ComboFind(url)
    print(loop)
    loop +=1
    driver.find_element_by_class_name('LightButton').click()
    
  except:
    loop = 0


driver.close()
