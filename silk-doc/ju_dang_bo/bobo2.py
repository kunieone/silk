# 导入 random 模块和 colorama 包
import random

from colorama import Fore, Style

# 初始化 colorama
# Fore.init()

# 定义玩家和电脑的初始气数、血量、名字
player_mag = 0
player_blood = 3
player_name = input("请输入你的名字：")

computer_mag = 0
computer_blood = 3
computer_name = "必应"

# 定义游戏的回合数、胜利条件
rounds = 10
win_blood = 0

# 输出欢迎信息
print(f"\n\033[1;33m欢迎来到波波攒游戏，{player_name}！\033[0m")
print(f"\033[1;34m你的对手是{computer_name}，它是一个智能的机器人。\033[0m")
print(
    f"游戏共有\033[1;33m{rounds}\033[0m回合，每回合你和\033[1;35m{computer_name}\033[0m都可以选择攒（\033[1;32mw\033[0m）、波（\033[1;31mq\033[0m）、防御（\033[1;36me\033[0m）。"
)
print(
    f"你和\033[1;35m{computer_name}\033[0m初始都有\033[1;31m{player_blood}\033[0m点血量，每次攒（\033[1;32mw\033[0m）可以增加1点气，每次波（\033[1;31mq\033[0m）可以消耗相应等级的气并对对方造成相同等级的伤害，每次防御（\033[1;36me\033[0m）可以抵挡对方的波。"
)
print(
    f"当你或者\033[1;35m{computer_name}\033[0m的血量降到\033[1;31m{win_blood}\033[0m点或者回合数用完时，游戏结束。血量多的一方获胜，如果血量相同，则平局。"
)
print("\033[1;33m让我们开始吧！\033[0m")

# 定义游戏的主循环
while True:
    # 输出当前的气数、血量
    print(
        f"\n\033[1;33m[状态]\033[0m {player_name}的气数：\033[1;32m{player_mag}\033[0m，血量：\033[1;31m{player_blood}\033[0m"
    )
    print(
        f"\033[1;33m[状态]\033[0m {computer_name}的气数：\033[1;32m{computer_mag}\033[0m，血量：\033[1;31m{computer_blood}\033[0m"
    )

    # 获取玩家的输入，检查是否合法
    player_action = input(
        f"\033[1;33m[动作]\033[0m 请输入你的动作：攒（\033[1;32mw\033[0m）、波（\033[1;31mq\033[0m）、防御（\033[1;36me\033[0m）或退出（\033[1;33mx\033[0m）："
    )
    if player_action == "x":
        print("\033[1;33m[状态]\033[0m 你选择了退出游戏，再见！")
        break
    elif player_action not in ["w", "q", "e"] and not (
        player_action.isdigit() and 1 <= int(player_action.__len__) <= 5
    ):
        print("\033[1;31m[错误]\033[0m 你输入的动作不合法，请重新输入！")
        continue
    elif player_action.startswith("q") and not player_action[1:].isdigit():
        print("\033[1;31m[错误]\033[0m 你输入的动作不合法，请重新输入！")
        continue

    elif player_action == "q" and player_mag <= 0:
        print("\033[1;31m[错误]\033[0m 你没有足够的气来发动波，请重新输入！")
        continue

    # 处理攻击和防御
    if player_action.isdigit():
        level = int(player_action)
        if player_mag < level:
            print("\033[1;31m[错误]\033[0m 你没有足够的气来进行这个等级的攻击或防御，请重新输入！")
            continue
        player_mag -= level
        player_action = "q" * level  # 使用相应等级的波

    # 生成电脑的随机动作，检查是否合法
    computer_action = random.choice(["w", "q", "e"])
    while computer_action == "q" and computer_mag <= 0:
        computer_action = random.choice(["w", "q", "e"])

    # 输出双方的动作
    print(
        f"\033[1;33m[动作]\033[0m {player_name}选择了{'攒' if player_action == 'w' else '波' if player_action == 'q' else '防御'}，"
        f"{computer_name}选择了{'攒' if computer_action == 'w' else '波' if computer_action == 'q' else '防御'}。"
    )

    # 判断双方的动作，更新血量
    if player_action == "w":
        player_mag += 1
        print(f"\033[1;33m[动作]\033[0m {player_name}增加了1点气。")
    elif player_action.startswith("q"):
        level = len(player_action)
        if computer_action.startswith("q"):
            if len(computer_action) > level:
                computer_blood -= level
                print(
                    f"\033[1;33m[效果]\033[0m {computer_name}的波强于{player_name}，{player_name}受到了{level}点伤害。"
                )
            elif len(computer_action) < level:
                player_blood -= level
                print(
                    f"\033[1;33m[效果]\033[0m {player_name}的波强于{computer_name}，{computer_name}受到了{level}点伤害。"
                )
            else:
                print("\033[1;33m[效果]\033[0m 双方的波相互抵消，没有造成伤害。")
        elif computer_action == "e":
            print(f"\033[1;33m[效果]\033[0m {computer_name}成功防御了{player_name}的波，没有受到伤害。")
        else:
            player_blood -= level
            print(
                f"\033[1;33m[效果]\033[0m {player_name}成功发动波，{computer_name}受到了{level}点伤害。"
            )
    elif player_action == "e":
        if computer_action.startswith("q"):
            level = len(computer_action)
            print(f"\033[1;33m[效果]\033[0m {computer_name}的波被{player_name}成功防御，没有受到伤害。")
        # else:
        #     print("\033[1;33m[效果]\033[0m 没有发生任何事情。")

    if computer_action == "w":
        computer_mag += 1
        print(f"\033[1;33m[动作]\033[0m {computer_name}增加了1点气。")
    elif computer_action.startswith("q"):
        level = len(computer_action)
        if player_action == "e":
            print(f"\033[1;33m[效果]\033[0m {player_name}成功防御了{computer_name}的波，没有受到伤害。")
        else:
            player_blood -= level
            print(
                f"\033[1;33m[效果]\033[0m {computer_name}成功发动波，{player_name}受到了{level}点伤害。"
            )
    elif computer_action == "e":
        if player_action.startswith("q"):
            level = len(player_action)
            print(f"\033[1;33m[效果]\033[0m {player_name}的波被{computer_name}成功防御，没有受到伤害。")
        # else:
        #     print("\033[1;33m[效果]\033[0m 没有发生任何事情。")

    # 判断游戏是否结束，输出结果
    if player_blood == win_blood or computer_blood == win_blood or rounds == 1:
        print("\n\033[1;33m[状态]\033[0m 游戏结束！")
        if player_blood > computer_blood:
            print(f"\033[1;34m{player_name}\033[0m，恭喜你，你赢了！")
        elif player_blood < computer_blood:
            print(f"\033[1;34m{player_name}\033[0m，很遗憾，你输了！")
        else:
            print(
                f"\033[1;34m{player_name}\033[0m和\033[1;35m{computer_name}\033[0m打成了平局！"
            )
        break
    else:
        # 更新回合数，继续游戏
        rounds -= 1
        print("\033[1;33m[状态]\033[0m 游戏继续！")
