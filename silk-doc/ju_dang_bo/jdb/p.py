# 导入模块
import random
import time

# 定义常量
MAX_QI = 10  # 气的最大值
MIN_QI = 0  # 气的最小值
MAX_LEVEL = 5  # 招数的最大等级
MIN_LEVEL = 1  # 招数的最小等级
JU = "w"  # 聚的键位
DANG = "e"  # 挡的键位
BO = "q"  # 波的键位
QUIT = "x"  # 退出的键位
DELAY = 1  # 回合间隔时间（秒）


# 定义玩家类
class Player:
    def __init__(self, name):
        self.name = name  # 玩家的名字
        self.qi = 0  # 玩家的气
        self.action = None  # 玩家的招数
        self.level = None  # 玩家的招数等级

    def ju(self):
        # 聚气，气加一，招数为聚
        self.qi += 1
        self.action = JU
        self.level = None
        print(f"{self.name}使用了聚，气增加了1点，现在有{self.qi}点气。")

    def dang(self, level):
        # 挡，消耗气，招数为挡，等级为输入的值
        self.qi -= level - 1
        self.action = DANG
        self.level = level
        print(f"{self.name}使用了{level}级挡，消耗了{level}点气，现在有{self.qi}点气。")

    def bo(self, level):
        # 波，消耗气，招数为波，等级为输入的值
        self.qi -= level
        self.action = BO
        self.level = level
        print(f"{self.name}使用了{level}级波，消耗了{level}点气，现在有{self.qi}点气。")

    def choose_action(self):
        # 玩家选择招数，输入键位和等级，根据气的限制和招数的规则进行判断和执行
        while True:
            # 输入键位
            key = input(
                f"{self.name}请选择你的招数，输入{JU}聚气，输入{DANG}或{BO}加数字使用挡或波，输入{QUIT}退出游戏："
            )
            # 判断是否退出
            if key == QUIT:
                print("你选择了退出游戏，游戏结束。")
                return False
            # 判断是否聚气
            elif key == JU:
                # 气不能超过最大值
                if self.qi < MAX_QI:
                    self.ju()
                    return True
                else:
                    print("你的气已经达到最大值，不能再聚气了，请重新选择。")
                    continue
            # 判断是否使用挡或波
            elif key[0] in [DANG, BO]:
                # 输入的键位长度必须大于1
                if len(key) > 1:
                    # 输入的键位除了第一个字符外，其他必须是数字
                    if key[1:].isdigit():
                        # 转换输入的数字为整数
                        level = int(key[1:])
                        # 招数等级必须在最小值和最大值之间
                        if MIN_LEVEL <= level <= MAX_LEVEL:
                            # 如果使用挡，消耗的气等于招数等级
                            if key[0] == DANG:
                                cost = level
                            # 如果使用波，消耗的气等于招数等级加一
                            elif key[0] == BO:
                                cost = level + 1
                            # 判断气是否足够
                            if self.qi >= cost:
                                # 如果气足够，执行相应的招数
                                if key[0] == DANG:
                                    self.dang(level)
                                elif key[0] == BO:
                                    self.bo(level)
                                return True
                            else:
                                # 如果气不够，提示重新选择
                                print("你的气不够使用这个招数，请重新选择。")
                                continue
                        else:
                            # 如果招数等级不在范围内，提示重新选择
                            print(f"招数等级必须在{MIN_LEVEL}到{MAX_LEVEL}之间，请重新选择。")
                            continue
                    else:
                        # 如果输入的键位不是数字，提示重新选择
                        print("你输入的招数等级不是数字，请重新选择。")
                        continue
                else:
                    # 如果输入的键位长度不大于1，提示重新选择
                    print("你输入的招数等级缺失，请重新选择。")
                    continue
            else:
                # 如果输入的键位不符合规则，提示重新选择
                print("你输入的招数无效，请重新选择。")
                continue


# 定义电脑类，继承玩家类
class Computer(Player):
    def __init__(self, name):
        super().__init__(name)  # 调用父类的初始化方法

    def choose_action(self):
        # 电脑选择招数，随机生成键位和等级，根据气的限制和招数的规则进行判断和执行
        while True:
            # 随机生成键位
            key = random.choice([JU, DANG, BO])
            # 判断是否聚气
            if key == JU:
                # 气不能超过最大值
                if self.qi < MAX_QI:
                    self.ju()
                    return True
                else:
                    continue
            # 判断是否使用挡或波
            elif key in [DANG, BO]:
                # 随机生成招数等级
                level = random.randint(MIN_LEVEL, MAX_LEVEL)
                # 如果使用挡，消耗的气等于招数等级
                if key == DANG:
                    cost = level
                # 如果使用波，消耗的气等于招数等级加一
                elif key == BO:
                    cost = level + 1
                # 判断气是否足够
                if self.qi >= cost:
                    # 如果气足够，执行相应的招数
                    if key == DANG:
                        self.dang(level)
                    elif key == BO:
                        self.bo(level)
                    return True
                else:
                    continue


# 定义游戏类
class Game:
    def __init__(self):
        self.player = Player("你")  # 创建玩家对象
        self.computer = Computer("电脑")  # 创建电脑对象
        self.winner = None  # 游戏的胜者

    def compare(self):
        # 比较双方的招数，判断胜负，返回True表示游戏结束，返回False表示游戏继续
        # 如果双方都使用聚，游戏继续
        if self.player.action == JU and self.computer.action == JU:
            print("双方都使用了聚，气各自增加了1点，继续下一回合。")
            return False
        # 如果双方都使用挡，游戏继续
        elif self.player.action == DANG and self.computer.action == DANG:
            print("双方都使用了挡，没有任何效果，继续下一回合。")
            return False
        # 如果双方都使用波，比较招数等级，等级高的一方获胜，游戏结束
        elif self.player.action == BO and self.computer.action == BO:
            if self.player.level > self.computer.level:
                print(f"你使用了{self.player.level}级波，电脑使用了{self.computer.level}级波，你获胜了！")
                self.winner = self.player
            elif self.player.level < self.computer.level:
                print(f"你使用了{self.player.level}级波，电脑使用了{self.computer.level}级波，你失败了。")
                self.winner = self.computer
            else:
                print(f"你和电脑都使用了{self.player.level}级波，平局，继续下一回合。")
            return True
        # 如果一方使用挡，一方使用波，判断挡是否抵挡住波，游戏结束
        elif (self.player.action == DANG and self.computer.action == BO) or (
            self.player.action == BO and self.computer.action == DANG
        ):
            if self.player.action == DANG:
                print(f"你使用了{self.player.level}级挡，电脑使用了{self.computer.level}级波。")
            else:
                print(f"你使用了{self.player.level}级波，电脑使用了{self.computer.level}级挡。")
            # 判断挡是否抵挡住波
            if self.player.level >= self.computer.level:
                if self.player.action == DANG:
                    print("你成功抵挡住了电脑的波，继续下一回合。")
                else:
                    print("你的波抵挡住了电脑的挡，你获胜了！")
                    self.winner = self.player
            else:
                if self.player.action == DANG:
                    print("你未能抵挡住电脑的波，你失败了。")
                else:
                    print("你的波被电脑的挡抵挡住了，继续下一回合。")
            return True
        # 如果一方使用聚，一方使用挡或波，游戏继续
        elif (
            self.player.action == JU
            and (self.computer.action == DANG or self.computer.action == BO)
        ) or (
            (self.player.action == DANG or self.player.action == BO)
            and self.computer.action == JU
        ):
            if self.player.action == JU:
                print(f"你使用了聚，电脑使用了{self.computer.action}。")
            else:
                print(f"你使用了{self.player.action}，电脑使用了聚。")
            return False

    def play(self):
        # 游戏主循环
        print("游戏开始！")
        while True:
            # 显示玩家和电脑的气值
            print(
                f"{self.player.name}的气值：{self.player.qi}，{self.computer.name}的气值：{self.computer.qi}"
            )
            # 玩家选择招数
            if not self.player.choose_action():
                break
            # 电脑选择招数
            if not self.computer.choose_action():
                break
            # 比较招数，判断胜负
            if self.compare():
                break
            # 间隔一段时间
            time.sleep(DELAY)
        # 显示游戏结果
        if self.winner:
            print(f"{self.winner.name}获胜！")
        else:
            print("游戏结束，平局。")


# 创建游戏对象并开始游戏
game = Game()
game.play()
