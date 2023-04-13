#   Copyright 2023 Golden_Water, 火纹梅戚
#   SPDX-License-Identifier: AGPL-3.0-only

import json

from LoadXL import *

cls_d = load_class('data.xls')
chr_d = load_characters('data.xls', cls_d)

with open("../src/class.json", mode="w") as f:
    f.write(json.dumps(load_class_to_obj("data.xls"), ensure_ascii=False))

with open("../src/character.json", mode="w") as f:
    characters = load_characters_to_obj("data.xls", cls_d)
    for character in characters:
        character["init_attribute"] = {}
        character["init_attribute"]["class"] = character["init_stat"]["class"]
        character["init_attribute"]["LV"] = character["init_stat"]["LV"]
        character["init_attribute"]["HLV"] = character["init_stat"]["HLV"]
        del character["init_stat"]["class"]
        del character["init_stat"]["LV"]
        del character["init_stat"]["HLV"]
        character["init_attribute"]["stats"] = character["init_stat"]
        del character["init_stat"]

    f.write(json.dumps(characters, ensure_ascii=False))

while True:
    n = input('Please enter name: ')
    if n == exit:
        break
    if n not in chr_d:
        print('Character not found')
        continue
    c = chr_d[n]
    print(c)
    while True:
        cmd = input(
            """Enter 'up' to get one level up\nEnter 'change' to change your class\nEnter 'reset' to reset your character\nEnter 'exit' for another character\nPlease input your command:""")
        if cmd == 'exit':
            c.initialize()
            break
        elif cmd == 'reset':
            c.initialize()
        elif cmd == 'up':
            if c.level_up():
                print('Failed')
            else:
                print('Succeeded')
        elif cmd == 'change':
            while True:
                cls = input('Please enter your class:')
                if cls in cls_d:
                    if c.class_change(cls_d[cls]):
                        print('Failed')
                    else:
                        print('Succeeded')
                    break
                elif cls == 'exit':
                    break
                else:
                    print('Class not found')
        else:
            print('Command not found')
        print(str(c))
