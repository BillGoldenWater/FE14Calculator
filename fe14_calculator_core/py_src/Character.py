#   Copyright 2023 Golden_Water, 火纹梅戚
#   SPDX-License-Identifier: AGPL-3.0-only

from Class import Class
from typing import Any, Optional


class Character:
    """
    Describe a character
    chn_name: Chinese name of a character
    init_stat: A dictionary containing initial states of a character
    growth_rate: A dictionary containing growth rates of a character
    current_stat: A dictionary containing current states of a character
    A dictionary stores lv, hlv, class, hp, str, mag, skl, spd, lck, def, res, bld
    """
    cls_dict: dict[str, Class]
    chn_name: str
    init_stat: dict[str: Any]
    growth_rate: dict[str: int]
    correctness: dict[str: int]
    current_stat: dict[str: Any]

    def __init__(self, chn_name: str, init_stat: Optional[dict] = None, growth_rate: Optional[dict] = None,
                 correctness: Optional[dict] = None):
        self.chn_name = chn_name
        self.init_stat = init_stat
        self.correctness = correctness
        self.growth_rate = growth_rate

    def __str__(self):
        s = ''
        for key in self.current_stat:
            if key in self.growth_rate:
                value = str(self.get_total_stat(key))
            elif key == 'class':
                value = self.current_stat[key].name
            else:
                value = str(self.current_stat[key])
            s += key + ' : ' + value + '\n'
        return s

    def initialize(self):
        self.current_stat = self.init_stat.copy()
        for stat in self.growth_rate:
            self.current_stat[stat] += self.growth_rate[stat]
            self.current_stat[stat] = min(self.get_upper_limit(stat), self.get_total_stat(stat)) - self.get_cor_stat(
                stat)

    def get_current_class(self) -> Class:
        cls = self.current_stat['class']
        return Character.cls_dict[cls.name]

    def get_max_lv(self) -> int:
        if self.get_current_class().is_spc():
            return 40
        else:
            return 20

    def get_upper_limit(self, stat: str) -> int:
        return self.get_current_class().upper_limit[stat] + self.correctness[stat]

    def get_cor_stat(self, stat: str) -> int:
        return self.get_current_class().correctness[stat]

    def get_total_stat(self, stat: str) -> int:
        current_class = self.get_current_class()
        return self.current_stat[stat] + current_class.correctness[stat]

    def calculate_growth(self, stat: str, enhanced: bool = False, doubled: bool = False) -> int:
        doubled_index = 2 if doubled else 1
        enhanced_index = 1 if enhanced else 0
        return self.growth_rate[stat] + self.get_current_class().growth_rate[
            stat] * doubled_index + 15 * enhanced_index

    def level_up(self, enhanced: bool = False, doubled: bool = False) -> bool:
        if self.get_lv() >= self.get_max_lv():
            return True
        self.current_stat['LV'] += 1
        for stat in self.growth_rate:
            self.current_stat[stat] += self.calculate_growth(stat, enhanced, doubled)
            self.current_stat[stat] = min(self.get_upper_limit(stat), self.get_total_stat(stat)) - self.get_cor_stat(
                stat)
        return False

    def get_lv(self) -> int:
        return self.current_stat['LV']

    def get_hlv(self) -> int:
        return self.current_stat['HLV']

    def get_total_lv(self) -> int:
        return self.get_lv() + self.get_hlv()

    def class_change(self, dst_class: Class) -> bool:
        current_class: Class = self.get_current_class()
        if current_class == dst_class and self.get_max_lv() > self.get_lv():
            return True
        if dst_class.is_adv():
            if current_class.is_spc() and self.get_lv() < 21:
                return True
            if current_class.is_bsc() and self.get_lv() < 10:
                return True
        lv = 1
        if dst_class.is_spc():
            if current_class.is_adv():
                lv = 21
            elif current_class.is_spc():
                if self.get_lv() >= 21:
                    lv = 21
                else:
                    lv = 1
            else:
                lv = 1
        hlv = max(self.get_total_lv() - lv, 0)
        self.current_stat['class'] = dst_class
        self.current_stat['HLV'] = hlv
        self.current_stat['LV'] = lv
        return False
