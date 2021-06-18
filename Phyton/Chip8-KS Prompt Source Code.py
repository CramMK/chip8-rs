import os
import PySimpleGUI as sg

sg.theme('dark grey 9')
filename = sg.popup_get_file('Enter the file you wish to process',no_titlebar=True)
filenamewithquotes = (f'"{filename}"')

print(filename)

if filename == None:
    sg.popup('Please select a file',no_titlebar=True)
    quit()

os.system(f'cargo run -- {filenamewithquotes}')
