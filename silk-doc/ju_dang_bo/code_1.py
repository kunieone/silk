import random
from collections import deque
from enum import Enum

# class SkilType:
#     def __init__(self, label, extra_names):
#         self.label = label
#         self.extra_names = extra_names

class SkilType(Enum):
    ju = ("聚气", ["聚"])
    dang = ("格挡", ["挡", "中挡", "大挡", "超挡", "极挡"])
    bo = ("发波", ["波", "中波", "大波", "超波", "极波"])

    def __init__(self, label, extra_names):
        self.label = label
        self.extra_names = extra_names

class Action:
    def __init__(self, skil_type, level):
        self.skil_type = skil_type
        self.level = level

    def __str__(self):
        return self.skil_type.extra_names[self.level - 1]

class Player:
    def __init__(self):
        self.ju = 0
        self.dang = None
        self.actions = deque()

    def ju_action(self, level):
        self.dang = None
        self.ju += 1
        self.actions.append(Action(SkilType.ju, level))

    def dang_action(self, level):
        consume = level - 1
        if self.ju < consume:
            raise ValueError("Not enough ju to perform dang action")
        else:
            self.ju -= consume
        self.dang = level
        self.actions.append(Action(SkilType.dang, level))

    def bo_action(self, level):
        self.dang = None
        if self.ju < level:
            raise ValueError("Not enough ju to perform bo action")
        else:
            self.ju -= level
        self.actions.append(Action(SkilType.bo, level))

class Game:
    def __init__(self):
        self.human = Player()
        self.machine = Player()
        self.judger = Judger()
        self.is_over = False

    def start(self):
        print('''
        游戏说明...
        ''')

        while not self.is_over:
            user_input = input("请输入你的动作：")
            try:
                self.process_human_input(user_input)
                self.process_machine_input()
                self.print_actions()
                result = self.judger.judge(self.human, self.machine)
                self.handle_result(result)
            except ValueError as e:
                print(f"错误：{e}")

    def process_human_input(self, user_input):
        if user_input == 'a':
            self.human.ju_action(1)
        elif user_input in ['q', 'w', 'e', 'r', 't']:
            self.human.dang_action(int(user_input))
        elif user_input in ['1', '2', '3', '4', '5']:
            self.human.bo_action(int(user_input))
        else:
            raise ValueError("无效的输入，请重新输入。")

    def process_machine_input(self):
        action = random.choice(['a', 'q', 'w', 'e', 'r', 't', '1', '2', '3', '4', '5'])
        if action == 'a':
            self.machine.ju_action(1)
        elif action in ['q', 'w', 'e', 'r', 't']:
            self.machine.dang_action(int(action))
        elif action in ['1', '2', '3', '4', '5']:
            self.machine.bo_action(int(action))

    def print_actions(self):
        print(f'你 {self.human.actions} ->>     <<- {self.machine.actions} 机器')

    def handle_result(self, result):
        if result == 0:
            print(f'本回合平局。你:聚气点数({self.human.ju}),机器人:聚气点数({self.machine.ju})')
        elif result == 3:
            print('恭喜你，你赢得了游戏！')
            self.is_over = True
        elif result == 4:
            print('很遗憾，你输掉了游戏。')
            self.is_over = True

class Judger:
    def judge(self, human, machine):
        action_h = human.actions.popleft()
        action_m = machine.actions.popleft()

        if action_h.skil_type == SkilType.dang and action_m.skil_type == SkilType.dang \
                or action_m.skil_type == SkilType.ju and action_h.skil_type == SkilType.ju:
            return 0
        elif action_h.skil_type == SkilType.bo and action_m.skil_type == SkilType.ju:
            return 3
        elif action_m.skil_type == SkilType.bo and action_h.skil_type == SkilType.ju:
            return 4
        elif action_h.skil_type == SkilType.bo and action_m.skil_type == SkilType.bo:
            return 0 if action_h.level == action_m.level else 3 if action_h.level > action_m.level else 4
        elif action_h.skil_type == SkilType.bo and action_m.skil_type == SkilType.dang:
            return 0 if action_h.level == action_m.level else 3 if action_h.level > action_m.level else 0
        elif action_h.skil_type == SkilType.dang and action_m.skil_type == SkilType.bo:
            return 0 if action_h.level == action_m.level else 0 if action_h.level > action_m.level else 4
        return 0

# 实例化游戏对象并开始游戏
game = Game()
game.start()
