#   Copyright 2023 Golden_Water, 火纹梅戚
#   SPDX-License-Identifier: AGPL-3.0-only

from typing import Optional


class Class:
    name: str
    type: str
    growth_rate: dict
    correctness: dict
    upper_limit: dict

    def __init__(self, name: str, typ: str, growth_rate: Optional[dict] = None, correctness: Optional[dict] = None,
                 upper_limit: Optional[dict] = None):
        self.name = name
        self.type = typ
        self.growth_rate = growth_rate
        self.upper_limit = upper_limit
        self.correctness = correctness

    def __eq__(self, other) -> bool:
        return self.name == other.name

    def is_spc(self) -> bool:
        return self.type == '特殊'

    def is_adv(self) -> bool:
        return self.type == '高级'

    def is_bsc(self) -> bool:
        return self.type == '基础'
