#   Copyright 2023 Golden_Water, 火纹梅戚
#   SPDX-License-Identifier: AGPL-3.0-only

import xlrd

from Character import Character
from Class import Class


def load_class(filepath: str) -> dict[str, Class]:
    r_dict = dict()
    wb = xlrd.open_workbook(filepath)
    sh_cls = wb.sheet_by_name('兵种')
    keys = sh_cls.row_values(1)
    for r in range(2, 61):
        v = sh_cls.row_values(r)
        c = Class(v[0], v[1])
        d3 = [dict() for _ in range(3)]
        for i in range(3):
            for cnt in range(2 + i * 9, 11 + i * 9):
                d3[i][keys[cnt]] = int(v[cnt]) * (100 if i != 1 else 1)
        c.correctness, c.growth_rate, c.upper_limit = d3
        r_dict[c.name] = c
    return r_dict


def load_class_to_obj(filepath: str) -> list[dict]:
    result = []
    wb = xlrd.open_workbook(filepath)
    sh_cls = wb.sheet_by_name('兵种')
    keys = sh_cls.row_values(1)
    for r in range(2, 61):
        v = sh_cls.row_values(r)
        c = {"name": v[0], "type": v[1]}
        d3 = [dict() for _ in range(3)]
        for i in range(3):
            for cnt in range(2 + i * 9, 11 + i * 9):
                d3[i][keys[cnt]] = int(v[cnt]) * (100 if i != 1 else 1)
        c["correctness"], c["growth_rate"], c["upper_limit"] = d3
        result.append(c)
    return result


def load_characters(filepath: str, class_dict: dict[str, Class]) -> dict[str, Character]:
    Character.cls_dict = class_dict
    r_dict = dict()
    wb = xlrd.open_workbook(filepath)
    sh_chr = wb.sheet_by_name('个人')
    keys = sh_chr.row_values(0)
    for r in range(1, 42):
        v = sh_chr.row_values(r)
        c = Character(v[0])
        init_stat = dict()
        growth_rate = dict()
        correctness = dict()
        init_stat['class'] = class_dict[v[1]]
        init_stat['LV'] = int(v[2])
        init_stat['HLV'] = int(v[3]) if v[3] != '--' else 0
        for i in range(4, 13):
            stat = int(v[i]) * 100
            stat -= class_dict[v[1]].correctness[keys[i]]
            init_stat[keys[i]] = stat
        for i in range(13, 22):
            growth_rate[keys[i]] = int(v[i])
        for i in range(22, 31):
            correctness[keys[i]] = int(v[i] if v[i] else '0') * 100
        c.init_stat = init_stat
        c.growth_rate = growth_rate
        c.correctness = correctness
        c.initialize()
        r_dict[c.chn_name] = c
    return r_dict


def load_characters_to_obj(filepath: str, class_dict: dict[str, Class]) -> list[dict]:
    Character.cls_dict = class_dict
    result = []
    wb = xlrd.open_workbook(filepath)
    sh_chr = wb.sheet_by_name('个人')
    keys = sh_chr.row_values(0)
    for r in range(1, 42):
        v = sh_chr.row_values(r)
        c = {"name": v[0]}
        init_stat = dict()
        growth_rate = dict()
        correctness = dict()
        init_stat['class'] = v[1]
        init_stat['LV'] = int(v[2])
        init_stat['HLV'] = int(v[3]) if v[3] != '--' else 0
        for i in range(4, 13):
            stat = int(v[i]) * 100
            stat -= class_dict[v[1]].correctness[keys[i]]
            init_stat[keys[i]] = stat
        for i in range(13, 22):
            growth_rate[keys[i]] = int(v[i])
        for i in range(22, 31):
            correctness[keys[i]] = int(v[i] if v[i] else '0') * 100
        c["init_stat"] = init_stat
        c["growth_rate"] = growth_rate
        c["correctness"] = correctness
        result.append(c)
    return result
