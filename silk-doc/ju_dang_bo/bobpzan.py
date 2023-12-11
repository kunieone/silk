# 导入 random 模块
import random

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

# 定义游戏的动作
actions = ["+", "-", "="]  # 攒、波、防
action_names = {"+": "攒", "-": "波", "=": "防"}

# 输出欢迎信息
print(f"欢迎来到波波攒游戏，{player_name}！")
print(f"你的对手是{computer_name}，它是一个智能的机器人。")
print(
    f"游戏共有{rounds}回合，每回合你和{computer_name}都可以选择{action_names['+']}、{action_names['-']}或{action_names['=']}。"
)
print(
    f"你和{computer_name}初始都有{player_blood}点血量，每次{action_names['+']}可以增加1点气，每次{action_names['-']}可以消耗1点气并对对方造成1点伤害，每次{action_names['=']}可以抵挡对方的{action_names['-']}。"
)
print(f"当你或者{computer_name}的血量降到{win_blood}点或者回合数用完时，游戏结束。血量多的一方获胜，如果血量相同，则平局。")
print("让我们开始吧！")

# 定义游戏的主循环
while True:
    # 输出当前的回合数、气数、血量
    # 输出当前的回合数、气数、血量
    print(f"第{rounds}回合")
    print(f"{player_name}的气数：\033[1m{player_mag}\033[0m，血量：\033[1m{player_blood}\033[0m")
    print(f"{computer_name}的气数：\033[1m{computer_mag}\033[0m，血量：\033[1m{computer_blood}\033[0m")


    # 获取玩家的输入，检查是否合法
    player_action = input(
        f"请输入你的动作：{action_names['+']}（{actions[0]}）、{action_names['-']}（{actions[1]}）、{action_names['=']}（{actions[2]}）或退出（q）："
    )
    if player_action == "q":
        print("你选择了退出游戏，再见！")
        break
    elif player_action not in actions:
        print("你输入的动作不合法，请重新输入！")
        continue
    elif player_action == "-" and player_mag <= 0:
        print("你没有足够的气来发动{action_names['-']}，请重新输入！")
        continue

    # 生成电脑的随机动作，检查是否合法
    computer_action = random.choice(actions)
    while computer_action == "-" and computer_mag <= 0:
        computer_action = random.choice(actions)

    # 输出双方的动作
    print(
        f"{player_name}选择了{action_names[player_action]}，{computer_name}选择了{action_names[computer_action]}。"
    )

    # 判断双方的动作，更新气数、血量
    # 判断双方的动作，更新气数、血量
    if player_action == "+":
        player_mag += 1
        print(f"{player_name}增加了1点气。")
    elif player_action == "-":
        player_mag -= 1
        if computer_action != "=":
            computer_blood -= 1
            print(f"{player_name}消耗了1点气，对{computer_name}造成了1点伤害。")
        else:
            print(f"{player_name}消耗了1点气，但被{computer_name}防御了。")
    elif player_action == "=":
        if computer_action == "-":
            computer_mag -= 1
            print(f"{player_name}防御了{computer_name}的波，{computer_name}消耗了1点气。")
        else:
            print(f"{player_name}防御了，但没有效果。")

    if computer_action == "+":
        computer_mag += 1
        print(f"{computer_name}增加了1点气。")
    elif computer_action == "-":
        computer_mag -= 1
        if player_action != "=":
            player_blood -= 1
            print(f"{computer_name}消耗了1点气，对{player_name}造成了1点伤害。")
        else:
            print(f"{computer_name}消耗了1点气，但被{player_name}防御了。")
    elif computer_action == "=":
        if player_action == "-":
            player_mag -= 1
            print(f"{computer_name}防御了{player_name}的波，{player_name}消耗了1点气。")
        else:
            print(f"{computer_name}防御了，但没有效果。")


    # 判断游戏是否结束，输出结果
    if player_blood == win_blood or computer_blood == win_blood or rounds == 1:
        print("游戏结束！")
        if player_blood > computer_blood:
            print(f"恭喜你，{player_name}，你赢了！")
        elif player_blood < computer_blood:
            print(f"很遗憾，{player_name}，你输了！")
        else:
            print(f"{player_name}，你和{computer_name}打成了平局！")
        break
    else:
        # 更新回合数，继续游戏
        rounds -= 1
        print("游戏继续！")
